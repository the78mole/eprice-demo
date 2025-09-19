# Disclaimer

This project was completely created with GitHub Copilot (Claude Sonnet 4). The prompt was as follows:

```text
Please create a Rust project that queries today's electricity prices via the API https://api.energy-charts.info/ and calculates the average price.
```

# Germany Electricity Prices 🔌

A Rust program that queries current electricity prices for Germany via the Energy Charts API and calculates the average price.

## Features

- 📊 Fetch daily electricity prices for Germany
- 💰 Calculate average price
- 📈 Display minimum and maximum prices
- ⏰ List the first 5 hourly prices
- 🛡️ Robust error handling
- 🎨 User-friendly output with emojis

## Installation

Make sure Rust is installed. If not, install it from [rustup.rs](https://rustup.rs/).

## Usage

```bash
# Clone/download project
git clone <repository-url>
cd electricity-prices-api

# Run program
cargo run
```

## Example Output

```
🔌 Germany Electricity Prices - Energy Charts API
==================================================
📅 Query for date: 2025-09-19
Request to Energy Charts API: https://api.energy-charts.info/price?country=de&start=2025-09-19&end=2025-09-19

✅ Data successfully retrieved!
📄 License: CC BY 4.0 (creativecommons.org/licenses/by/4.0) from Bundesnetzagentur | SMARD.de

📊 Electricity Price Statistics for Today:
💰 Average Price: 96.48 EUR / MWh
📉 Lowest Price:  0.00 EUR / MWh
📈 Highest Price: 409.24 EUR / MWh
📋 Data Points:   24

⏰ First 5 hourly prices:
   22:00: 74.09 EUR / MWh
   23:00: 72.47 EUR / MWh
   00:00: 73.45 EUR / MWh
   01:00: 73.70 EUR / MWh
   02:00: 74.02 EUR / MWh
   ... and 19 more hours
```

## API Source

This program uses the [Energy Charts API](https://api.energy-charts.info/) from the Bundesnetzagentur.

**Data License:** CC BY 4.0 (creativecommons.org/licenses/by/4.0) from Bundesnetzagentur | SMARD.de

## Dependencies

- `tokio` - Asynchronous runtime
- `reqwest` - HTTP client
- `serde` - JSON serialization/deserialization
- `chrono` - Date/time processing
- `anyhow` - Enhanced error handling

## Technical Details

- The program calls the Energy Charts API for the current date
- The API provides hourly electricity prices in EUR/MWh
- All 24 hours of a day are retrieved and statistically evaluated
- Timestamps are converted from Unix timestamps to readable times

## License

This project is under the MIT License.