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
    
    println!("Anfrage an Energy Charts API: {}", url);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .context("Fehler beim Senden der HTTP-Anfrage")?;
    
    if !response.status().is_success() {
        anyhow::bail!("API-Anfrage fehlgeschlagen: {}", response.status());
    }
    
    let energy_data: EnergyChartsResponse = response
        .json()
        .await
        .context("Fehler beim Parsen der JSON-Antwort")?;
    
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
        return "Keine Preisdaten verfügbar".to_string();
    }
    
    let average = calculate_average_price(&data.price);
    let min_price = data.price.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_price = data.price.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    format!(
        "📊 Strompreis-Statistiken für heute:\n\
         💰 Durchschnittspreis: {:.2} {}\n\
         📉 Niedrigster Preis:  {:.2} {}\n\
         📈 Höchster Preis:     {:.2} {}\n\
         📋 Anzahl Datenpunkte: {}",
        average, data.unit,
        min_price, data.unit,
        max_price, data.unit,
        data.price.len()
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔌 Strompreise Deutschland - Energy Charts API");
    println!("==============================================");
    
    // Heutiges Datum ermitteln
    let today = Local::now().date_naive();
    let date_string = today.format("%Y-%m-%d").to_string();
    
    println!("📅 Abfrage für Datum: {}", date_string);
    
    // Strompreise abrufen
    match fetch_energy_prices(&date_string).await {
        Ok(energy_data) => {
            println!("\n✅ Daten erfolgreich abgerufen!");
            println!("📄 Lizenz: {}", energy_data.license_info);
            
            // Statistiken anzeigen
            println!("\n{}", format_price_statistics(&energy_data));
            
            // Zeitliche Aufschlüsselung (optional, erste 5 Einträge)
            if !energy_data.unix_seconds.is_empty() && !energy_data.price.is_empty() {
                println!("\n⏰ Erste 5 Stundenpreise:");
                for (&timestamp, &price) in energy_data.unix_seconds
                    .iter()
                    .zip(energy_data.price.iter())
                    .take(5) 
                {
                    let datetime = chrono::DateTime::from_timestamp(timestamp, 0)
                        .map(|dt| dt.format("%H:%M").to_string())
                        .unwrap_or_else(|| "Unbekannt".to_string());
                    
                    println!("   {}:00 Uhr: {:.2} {}", datetime, price, energy_data.unit);
                }
                
                if energy_data.price.len() > 5 {
                    println!("   ... und {} weitere Stunden", energy_data.price.len() - 5);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Fehler beim Abrufen der Strompreise: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
