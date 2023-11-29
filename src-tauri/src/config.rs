use crate::trader::CtaStatus;
use ctp_futures::*;
use rust_share_util::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TradingAccountRow {
    pub broker_id: String,
    pub account: String,
    pub equity: f64,
    pub status: CtaStatus,
    pub status_description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TradeRow {
    pub broker_id: String,
    pub account: String,
    pub trade_id: String,
    pub symbol: String,
    pub direction: i32,
    pub offset: i32,
    pub price: f64,
    pub volume: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct OrderRow {
    pub front_id: i32,
    pub session_id: i32,
    pub order_ref: String,
    pub broker_id: String,
    pub account: String,
    pub order_sys_id: String,
    pub symbol: String,
    pub direction: i32,
    pub offset: i32,
    pub limit_price: f64,
    pub volume_total: i32,
    pub volume_total_original: i32,
    pub volume_traded: i32,
    pub status: i32,
    pub status_description: String,
}
impl OrderRow {
    pub fn key(&self) -> String {
        format!("{}:{}:{}", self.front_id, self.session_id, self.order_ref)
    }
}

impl From<&CThostFtdcOrderField> for OrderRow {
    fn from(o: &CThostFtdcOrderField) -> Self {
        Self {
            front_id: o.FrontID,
            session_id: o.SessionID,
            order_ref: ascii_cstr_to_str_i8(&o.OrderRef).unwrap().to_string(),
            broker_id: ascii_cstr_to_str_i8(&o.BrokerID).unwrap().to_string(),
            account: ascii_cstr_to_str_i8(&o.InvestorID).unwrap().to_string(),
            order_sys_id: ascii_cstr_to_str_i8(&o.OrderSysID).unwrap().to_string(),
            symbol: ascii_cstr_to_str_i8(&o.InstrumentID).unwrap().to_string(),
            direction: o.Direction as i32,
            offset: o.CombOffsetFlag[0] as i32,
            limit_price: o.LimitPrice,
            volume_total: o.VolumeTotal,
            volume_total_original: o.VolumeTotalOriginal,
            volume_traded: o.VolumeTraded,
            status: o.OrderStatus as i32,
            status_description: gb18030_cstr_to_str_i8(&o.StatusMsg).to_string(),
        }
    }
}

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
