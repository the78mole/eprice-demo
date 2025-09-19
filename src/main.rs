use anyhow::{Context, Result};
use chrono::Local;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct EnergyChartsResponse {
    license_info: String,
    unix_seconds: Vec<i64>,
    price: Vec<f64>,
    unit: String,
    #[allow(dead_code)]
    deprecated: bool,
}

async fn fetch_energy_prices(date: &str) -> Result<EnergyChartsResponse> {
    let url = format!(
        "https://api.energy-charts.info/price?country=de&start={}&end={}",
        date, date
    );

    println!("Request to Energy Charts API: {}", url);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .context("Error sending HTTP request")?;

    if !response.status().is_success() {
        anyhow::bail!("API request failed: {}", response.status());
    }

    let energy_data: EnergyChartsResponse = response
        .json()
        .await
        .context("Error parsing JSON response")?;

    Ok(energy_data)
}

fn calculate_average_price(prices: &[f64]) -> f64 {
    if prices.is_empty() {
        return 0.0;
    }

    let sum: f64 = prices.iter().sum();
    sum / prices.len() as f64
}

fn format_price_statistics(data: &EnergyChartsResponse) -> String {
    if data.price.is_empty() {
        return "No price data available".to_string();
    }

    let average = calculate_average_price(&data.price);
    let min_price = data.price.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_price = data.price.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    format!(
        "📊 Electricity Price Statistics for Today:\n\
         💰 Average Price: {:.2} {}\n\
         📉 Lowest Price:  {:.2} {}\n\
         📈 Highest Price: {:.2} {}\n\
         📋 Data Points:   {}",
        average,
        data.unit,
        min_price,
        data.unit,
        max_price,
        data.unit,
        data.price.len()
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔌 Germany Electricity Prices - Energy Charts API");
    println!("==================================================");

    // Get today's date
    let today = Local::now().date_naive();
    let date_string = today.format("%Y-%m-%d").to_string();

    println!("📅 Query for date: {}", date_string);

    // Fetch electricity prices
    match fetch_energy_prices(&date_string).await {
        Ok(energy_data) => {
            println!("\n✅ Data successfully retrieved!");
            println!("📄 License: {}", energy_data.license_info);

            // Display statistics
            println!("\n{}", format_price_statistics(&energy_data));

            // Time breakdown (optional, first 5 entries) - filter to current day only
            if !energy_data.unix_seconds.is_empty() && !energy_data.price.is_empty() {
                // Filter data to only include timestamps from the requested date
                let filtered_data: Vec<(i64, f64)> = energy_data
                    .unix_seconds
                    .iter()
                    .zip(energy_data.price.iter())
                    .filter_map(|(&timestamp, &price)| {
                        if let Some(datetime) = chrono::DateTime::from_timestamp(timestamp, 0) {
                            let local_datetime = datetime.with_timezone(&chrono::Local);
                            // Check if the date matches the requested date
                            if local_datetime.date_naive() == today {
                                Some((timestamp, price))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                if !filtered_data.is_empty() {
                    println!("\n⏰ First 5 hourly prices:");
                    for (timestamp, price) in filtered_data.iter().take(5) {
                        let datetime = chrono::DateTime::from_timestamp(*timestamp, 0)
                            .map(|dt| dt.with_timezone(&chrono::Local).format("%H:%M").to_string())
                            .unwrap_or_else(|| "Unknown".to_string());

                        println!("   {}:00: {:.2} {}", datetime, price, energy_data.unit);
                    }

                    if filtered_data.len() > 5 {
                        println!("   ... and {} more hours", filtered_data.len() - 5);
                    }
                } else {
                    println!("\n⏰ No hourly prices found for the requested date");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error retrieving electricity prices: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
