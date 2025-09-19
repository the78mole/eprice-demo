use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use chrono_tz::Europe::Berlin;
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

async fn fetch_energy_prices(start_date: &str, end_date: &str) -> Result<EnergyChartsResponse> {
    let url = format!(
        "https://api.energy-charts.info/price?country=de&start={}&end={}",
        start_date, end_date
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

    // Get today's date in German timezone
    let today_utc = Utc::now();
    let today_berlin = today_utc.with_timezone(&Berlin);
    let today = today_berlin.date_naive();
    
    // Convert German date to UTC range for API query
    // We need to get the full day (00:00 - 23:59) in German time
    let berlin_start = Berlin.from_local_datetime(&today.and_hms_opt(0, 0, 0).unwrap()).unwrap();
    let berlin_end = Berlin.from_local_datetime(&today.and_hms_opt(23, 59, 59).unwrap()).unwrap();
    
    // Convert to UTC for API request
    let utc_start = berlin_start.with_timezone(&Utc);
    let utc_end = berlin_end.with_timezone(&Utc);
    
    let start_date = utc_start.format("%Y-%m-%d").to_string();
    let end_date = utc_end.format("%Y-%m-%d").to_string();

    println!("📅 Query for German date: {}", today.format("%Y-%m-%d"));
    println!("🌍 UTC range: {} {} to {} {}", 
        utc_start.format("%Y-%m-%d"), 
        utc_start.format("%H:%M:%S"),
        utc_end.format("%Y-%m-%d"), 
        utc_end.format("%H:%M:%S")
    );

    // Fetch electricity prices
    match fetch_energy_prices(&start_date, &end_date).await {
        Ok(energy_data) => {
            println!("\n✅ Data successfully retrieved!");
            println!("📄 License: {}", energy_data.license_info);

            // Time breakdown (all entries) - filter to current day only
            if !energy_data.unix_seconds.is_empty() && !energy_data.price.is_empty() {
                // Filter data to only include timestamps from the requested German date
                let filtered_data: Vec<(i64, f64)> = energy_data
                    .unix_seconds
                    .iter()
                    .zip(energy_data.price.iter())
                    .filter_map(|(&timestamp, &price)| {
                        if let Some(datetime) = chrono::DateTime::from_timestamp(timestamp, 0) {
                            let berlin_datetime = datetime.with_timezone(&Berlin);
                            // Check if the date matches the requested German date
                            if berlin_datetime.date_naive() == today {
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
                    println!("\n⏰ All hourly prices for today:");
                    for (timestamp, price) in filtered_data.iter() {
                        let datetime = chrono::DateTime::from_timestamp(*timestamp, 0)
                            .map(|dt| dt.with_timezone(&Berlin).format("%H:%M").to_string())
                            .unwrap_or_else(|| "Unknown".to_string());

                        println!("   {}: {:.2} {}", datetime, price, energy_data.unit);
                    }
                    
                    // Display statistics for filtered data only
                    let filtered_prices: Vec<f64> = filtered_data.iter().map(|(_, price)| *price).collect();
                    let filtered_response = EnergyChartsResponse {
                        license_info: energy_data.license_info.clone(),
                        unix_seconds: filtered_data.iter().map(|(ts, _)| *ts).collect(),
                        price: filtered_prices,
                        unit: energy_data.unit.clone(),
                        deprecated: energy_data.deprecated,
                    };
                    println!("\n{}", format_price_statistics(&filtered_response));
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
