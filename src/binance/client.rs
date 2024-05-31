use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub struct BinanceClient {
    client: Client,
}

impl BinanceClient {
    pub fn new() -> Self {
        BinanceClient {
            client: Client::new(),
        }
    }

    pub async fn get_value(&self, symbol: &str, basis: &str, ma_length: Option<u32>) -> Result<f64, Box<dyn Error>> {
        match basis {
            "price" => {
                let url = format!(
                    "https://api.binance.com/api/v3/ticker/price?symbol={}",
                    symbol
                );
                let response = self.client.get(&url).send().await?;
                let json: Value = response.json().await?;
                Ok(json["price"].as_str().unwrap().parse::<f64>().unwrap())
            },
            "moving_average" => {
                let url = format!(
                    "https://api.binance.com/api/v3/klines?symbol={}&interval=1d&limit={}",
                    symbol,
                    ma_length.unwrap_or(5)
                );
                let response = self.client.get(&url).send().await?;
                let json: Value = response.json().await?;
                let closes: Vec<f64> = json.as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v[4].as_str().unwrap().parse::<f64>().unwrap())
                    .collect();
                let ma: f64 = closes.iter().sum::<f64>() / closes.len() as f64;
                Ok(ma)
            },
            _ => Err("Invalid basis".into()),
        }
    }
}
