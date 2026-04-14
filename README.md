# canary 🐦

A dead simple CLI tool that watches stock or crypto ticker, alerting you when it goes above or below a certain threshold.

# Features
* Live price polling from Yahoo Finance
* Desktop notifications for when thresholds are crossed

# Usage

Create a `config.json` in your current working directory:
```json
{
    "ticker": "BTC-USD",
    "interval": 2,
    "alert_above": 74408.00,
    "alert_below": 74211.77
}
```

Then
```
cargo run
```
If there is no `config.json`, a default config is used.