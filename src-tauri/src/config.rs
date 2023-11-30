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
    pub exchange: String,
    pub symbol: String,
    pub direction: i32,
    pub offset: i32,
    pub price: f64,
    pub volume: i32,
}
impl TradeRow {
    pub fn key(&self) -> String {
        format!("{}:{}:{}", self.exchange, self.symbol, self.trade_id)
    }
}
impl From<&CThostFtdcTradeField> for TradeRow {
    fn from(value: &CThostFtdcTradeField) -> Self {
        Self {
            broker_id: ascii_cstr_to_str_i8(&value.BrokerID).unwrap().to_string(),
            account: ascii_cstr_to_str_i8(&value.InvestorID).unwrap().to_string(),
            trade_id: ascii_cstr_to_str_i8(&value.TradeID).unwrap().to_string(),
            exchange: ascii_cstr_to_str_i8(&value.ExchangeID).unwrap().to_string(),
            symbol: ascii_cstr_to_str_i8(&value.InstrumentID)
                .unwrap()
                .to_string(),
            direction: value.Direction as i32,
            offset: value.OffsetFlag as i32,
            price: value.Price,
            volume: value.Volume,
        }
    }
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
    pub insert_time: String,
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
            insert_time: gb18030_cstr_to_str_i8(&o.InsertTime).to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct InstrumentRow {
    pub exchange: String,
    pub symbol: String,
    pub name: String,
}
impl InstrumentRow {
    pub fn key(&self) -> String {
        format!("{}:{}", self.exchange, self.symbol)
    }
}

impl From<&CThostFtdcInstrumentField> for InstrumentRow {
    fn from(value: &CThostFtdcInstrumentField) -> Self {
        Self {
            exchange: ascii_cstr_to_str_i8(&value.ExchangeID).unwrap().to_string(),
            symbol: ascii_cstr_to_str_i8(&value.InstrumentID)
                .unwrap()
                .to_string(),
            name: gb18030_cstr_to_str_i8(&value.InstrumentName).to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MarketDataRow {}
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct PositionDetailRow {
    pub exchange: String,
    pub symbol: String,
    pub direction: i32,
    pub volume: i32,
    pub volume_closed: i32,
    pub trade_id: String,
}
impl PositionDetailRow {
    pub fn key(&self) -> String {
        format!("{}:{}:{}", self.exchange, self.symbol, self.trade_id)
    }
}
impl From<&CThostFtdcInvestorPositionDetailField> for PositionDetailRow {
    fn from(value: &CThostFtdcInvestorPositionDetailField) -> Self {
        Self {
            exchange: ascii_cstr_to_str_i8(&value.ExchangeID).unwrap().to_string(),
            symbol: ascii_cstr_to_str_i8(&value.InstrumentID)
                .unwrap()
                .to_string(),
            direction: value.Direction as i32,
            volume: value.Volume,
            volume_closed: value.CloseVolume,
            trade_id: ascii_cstr_to_str_i8(&value.TradeID).unwrap().to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct PositionRow {
    pub broker_id: String,
    pub account: String,
    pub exchange: String,
    pub symbol: String,
    pub position: i32,
    pub direction: i32,
    pub open_cost: f64,
    pub open_amount: f64,
    pub open_volume: i32,
}
impl PositionRow {
    pub fn key(&self) -> String {
        format!("{}:{}:{}", self.exchange, self.symbol, self.direction)
    }
}
impl From<&CThostFtdcInvestorPositionField> for PositionRow {
    fn from(value: &CThostFtdcInvestorPositionField) -> Self {
        Self {
            broker_id: ascii_cstr_to_str_i8(&value.BrokerID).unwrap().to_string(),
            account: ascii_cstr_to_str_i8(&value.InvestorID).unwrap().to_string(),
            exchange: ascii_cstr_to_str_i8(&value.ExchangeID).unwrap().to_string(),
            symbol: ascii_cstr_to_str_i8(&value.InstrumentID)
                .unwrap()
                .to_string(),
            position: value.Position,
            direction: value.PosiDirection as i32,
            open_cost: value.OpenCost,
            open_amount: value.OpenAmount,
            open_volume: value.OpenVolume,
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
