use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TradingAccount {
    pub broker_id: String,
    pub account: String,
    pub password: String,
    pub md_front: String,
    pub trade_front: String,
    pub query_front: String,
    pub user_product_info: String,
    pub auth_code: String,
    pub app_id: String,
    pub route_type: String,
    pub money_password: String,
    pub fens_trade_front: String,
    pub fens_md_front: String,
    pub terminal_info: String,
    pub hd_serial: String,
    pub inner_ip_address: String,
    pub mac_address: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct G3Config {
    pub accounts: Vec<TradingAccount>,
}

impl G3Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let c = serde_json::from_reader(reader)?;
        Ok(c)
    }

    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        std::fs::write(path, serde_json::to_string_pretty(&self).unwrap())
    }

    pub fn default_path() -> &'static str {
        ".cache/g3config.json"
    }
}
