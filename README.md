# Disclaimer

Dieses Projekt wurde komplett mit GitHub Copilot (Claude Sonnet 4) erstellt. Der Prompt lautete wir folgt:

```text
Erstelle mir bitte ein Rust-Projekt, das über die API https://api.energy-charts.info/ die Strompreise für heute abfragt und den Durchschnittspreis ermittelt.
```

# Strompreise Deutschland 🔌

Ein Rust-Programm, das die aktuellen Strompreise für Deutschland über die Energy Charts API abfragt und den Durchschnittspreis berechnet.

## Features

- 📊 Abrufen der tagesaktuellen Strompreise für Deutschland
- 💰 Berechnung des Durchschnittspreises
- 📈 Anzeige von Minimum- und Maximum-Preisen
- ⏰ Auflistung der ersten 5 Stundenpreise
- 🛡️ Robuste Fehlerbehandlung
- 🎨 Benutzerfreundliche Ausgabe mit Emojis

## Installation

Stellen Sie sicher, dass Rust installiert ist. Falls nicht, installieren Sie es von [rustup.rs](https://rustup.rs/).

## Verwendung

```bash
# Projekt klonen/herunterladen
git clone <repository-url>
cd strompreise-api

# Programm ausführen
cargo run
```

## Beispiel-Ausgabe

```
🔌 Strompreise Deutschland - Energy Charts API
==============================================
📅 Abfrage für Datum: 2025-09-19
Anfrage an Energy Charts API: https://api.energy-charts.info/price?country=de&start=2025-09-19&end=2025-09-19

✅ Daten erfolgreich abgerufen!
📄 Lizenz: CC BY 4.0 (creativecommons.org/licenses/by/4.0) from Bundesnetzagentur | SMARD.de

📊 Strompreis-Statistiken für heute:
💰 Durchschnittspreis: 96.48 EUR / MWh
📉 Niedrigster Preis:  0.00 EUR / MWh
📈 Höchster Preis:     409.24 EUR / MWh
📋 Anzahl Datenpunkte: 24

⏰ Erste 5 Stundenpreise:
   22:00:00 Uhr: 74.09 EUR / MWh
   23:00:00 Uhr: 72.47 EUR / MWh
   00:00:00 Uhr: 73.45 EUR / MWh
   01:00:00 Uhr: 73.70 EUR / MWh
   02:00:00 Uhr: 74.02 EUR / MWh
   ... und 19 weitere Stunden
```

## API-Quelle

Dieses Programm nutzt die [Energy Charts API](https://api.energy-charts.info/) der Bundesnetzagentur.

**Lizenz der Daten:** CC BY 4.0 (creativecommons.org/licenses/by/4.0) from Bundesnetzagentur | SMARD.de

## Abhängigkeiten

- `tokio` - Asynchrone Runtime
- `reqwest` - HTTP-Client
- `serde` - JSON-Serialisierung/Deserialisierung
- `chrono` - Datum/Zeit-Verarbeitung
- `anyhow` - Erweiterte Fehlerbehandlung

## Technische Details

- Das Programm ruft die Energy Charts API für das aktuelle Datum auf
- Die API liefert stündliche Strompreise in EUR/MWh
- Alle 24 Stunden eines Tages werden abgerufen und statistisch ausgewertet
- Die Zeitstempel werden von Unix-Timestamps in lesbare Zeiten umgewandelt

## Lizenz

Dieses Projekt steht unter der MIT-Lizenz.