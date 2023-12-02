use crate::config::*;
use bincode::{Decode, Encode};
use ctp_futures::trader_api::*;
use ctp_futures::*;
use futures::StreamExt;
use log::{info, warn};
use rust_share_util::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::CString;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::sync::Mutex;

#[derive(Decode, Encode, Debug, Clone, Serialize, Deserialize)]
pub enum CtaStatus {
    UnKown,
    Connected,
    Disconnected,
    AuthenticateFailed,
    AuthenticateSucceeded,
    LoginFailed,
    LoginSucceeded,
}

impl Default for CtaStatus {
    fn default() -> Self {
        CtaStatus::UnKown
    }
}

#[derive(Decode, Encode, Debug, Clone, Serialize, Deserialize)]
pub struct CtaEvent {
    tp: String,
    b: String,
    a: String,
    key: String,
}

#[derive(Debug, Clone, Default)]
pub struct CtpTradingAccount {
    pub ta: CThostFtdcTradingAccountField,
    pub status: CtaStatus,
    pub status_description: String,
    pub orders: HashMap<String, OrderRow>,
    pub trades: HashMap<String, TradeRow>,
    pub positions: HashMap<String, PositionRow>,
    pub position_details: HashMap<String, PositionDetailRow>,
    pub instruments: HashMap<String, InstrumentRow>,
}

pub struct Trader {
    pub conf: TradingAccount,
    pub broker: TradingBroker,
    pub cta: CtpTradingAccount,
    pub api: Box<CThostFtdcTraderApi>,
    pub exit_sender: Option<tokio::sync::oneshot::Sender<String>>,
    pub event_sender: tokio::sync::mpsc::Sender<CtaEvent>,
    request_id: i32,
}

impl Trader {
    pub fn init(
        conf: TradingAccount,
        broker: TradingBroker,
        es: tokio::sync::mpsc::Sender<CtaEvent>,
    ) -> Arc<Mutex<Self>> {
        let conf1 = conf.clone();
        let (exit_sender, mut exit_receiver) = oneshot::channel::<String>();
        let broker_id = conf.broker_id;
        let account = conf.account;
        let ak = format!("{broker_id}:{account}");
        let fens_trade_front = conf.fens_trade_front.as_str();
        let trade_front = conf.trade_front.as_str();
        let _auth_code = conf.auth_code.as_str();
        let _user_product_info = conf.user_product_info.as_str();
        let _app_id = conf.app_id.as_str();
        let _password = conf.password.as_str();
        let flow_path = format!(".cache/ctp_futures_trade_flow_{}_{}//", broker_id, account);
        check_make_dir(&flow_path);
        let mut api = create_api(&flow_path, false);
        let mut stream = {
            let (stream, pp) = create_spi();
            api.register_spi(pp);
            stream
        };
        if fens_trade_front.len() > 0 {
            api.register_name_server(CString::new(fens_trade_front).unwrap());
        } else {
            api.register_front(CString::new(trade_front).unwrap());
            info!("register front {}", trade_front);
        }
        api.subscribe_public_topic(ctp_futures::THOST_TE_RESUME_TYPE_THOST_TERT_QUICK);
        api.subscribe_private_topic(ctp_futures::THOST_TE_RESUME_TYPE_THOST_TERT_QUICK);
        api.init();
        // let (api, mut api1) = trader_api::unsafe_clone_api(api);
        // 处理登陆初始化查询
        let cta = CtpTradingAccount::default();
        let trader = Trader {
            conf: conf1,
            cta,
            api,
            exit_sender: Some(exit_sender),
            request_id: 10,
            event_sender: es,
            broker,
        };
        let trader = Arc::new(Mutex::new(trader));
        let t1 = Arc::clone(&trader);
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    msg = stream.next() => {
                        if let Some(msg) = msg {
                            let mut t1 = t1.lock().await;
                            t1.handle_spi_msg(&msg).await;
                        }
                    }
                    _ = &mut exit_receiver => {
                        info!("[{ak}] exited on receiver, start to release api");
                        let trader = Arc::into_inner(t1).unwrap();
                        let mut trader = trader.into_inner();
                        trader.api.release();
                        Box::leak(trader.api);
                        break;
                    }
                }
            }
            info!("[{ak}] exited loop");
        });
        trader
    }

    fn key(&self) -> String {
        format!("{}:{}", self.conf.broker_id, self.conf.account)
    }

    fn get_request_id(&mut self) -> i32 {
        self.request_id += 1;
        self.request_id
    }

    pub fn status(&self) -> CtaStatus {
        self.cta.status.clone()
    }

    pub fn status_description(&self) -> String {
        self.cta.status_description.clone()
    }

    fn make_event(&self, tp: &str, key: &str) -> CtaEvent {
        CtaEvent {
            tp: tp.to_string(),
            b: self.conf.broker_id.clone(),
            a: self.conf.account.clone(),
            key: key.to_string(),
        }
    }

    async fn handle_spi_msg(&mut self, spi_msg: &CThostFtdcTraderSpiOutput) {
        let conf = &self.conf;
        let broker_id = conf.broker_id.as_str();
        let account = conf.account.as_str();
        let _fens_trade_front = conf.fens_trade_front.as_str();
        let _trade_front = conf.trade_front.as_str();
        let auth_code = conf.auth_code.as_str();
        let user_product_info = conf.user_product_info.as_str();
        let app_id = conf.app_id.as_str();
        let password = conf.password.as_str();
        use ctp_futures::trader_api::CThostFtdcTraderSpiOutput::*;
        match spi_msg {
            OnFrontConnected(_p) => {
                let mut req = CThostFtdcReqAuthenticateField::default();
                set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                set_cstr_from_str_truncate_i8(&mut req.UserID, account);
                set_cstr_from_str_truncate_i8(&mut req.AuthCode, auth_code);
                set_cstr_from_str_truncate_i8(&mut req.UserProductInfo, user_product_info);
                set_cstr_from_str_truncate_i8(&mut req.AppID, app_id);
                let request_id = self.get_request_id();
                self.api.req_authenticate(&mut req, request_id);
                info!("{} OnFrontConnected", self.key());
                self.cta.status = CtaStatus::Connected;
                self.event_sender
                    .send(self.make_event("OnFrontConnected", ""))
                    .await
                    .unwrap();
            }
            OnFrontDisconnected(p) => {
                info!("{} on front disconnected {:?} 直接Exit ", self.key(), p);
                self.cta.status = CtaStatus::Disconnected;
                self.event_sender
                    .send(self.make_event("OnFrontDisconnected", ""))
                    .await
                    .unwrap();

                return;
            }
            OnRspAuthenticate(ref p) => {
                if p.p_rsp_info.as_ref().unwrap().ErrorID == 0 {
                    let mut req = CThostFtdcReqUserLoginField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.UserID, account);
                    set_cstr_from_str_truncate_i8(&mut req.Password, password);
                    let request_id = self.get_request_id();
                    self.api.req_user_login(&mut req, request_id);
                    self.cta.status = CtaStatus::AuthenticateSucceeded;
                    self.event_sender
                        .send(self.make_event("OnRspAuthenticate", ""))
                        .await
                        .unwrap();
                } else {
                    info!("{} RspAuthenticate={:?}", self.key(), p);
                    self.cta.status = CtaStatus::AuthenticateFailed;
                    if let Some(p) = p.p_rsp_info {
                        self.cta.status_description =
                            gb18030_cstr_to_str_i8(&p.ErrorMsg).to_string();
                    }
                    self.event_sender
                        .send(self.make_event("OnRspAuthenticate", ""))
                        .await
                        .unwrap();
                    return;
                }
            }
            OnRspUserLogin(ref p) => {
                if p.p_rsp_info.as_ref().unwrap().ErrorID == 0 {
                    let _u = p.p_rsp_user_login.unwrap();
                    self.cta.status = CtaStatus::LoginSucceeded;
                } else {
                    self.cta.status = CtaStatus::LoginFailed;
                    if let Some(p) = p.p_rsp_info {
                        self.cta.status_description =
                            gb18030_cstr_to_str_i8(&p.ErrorMsg).to_string();
                        warn!(
                            "{} Trade RspUserLogin ErrorID={} ErrorMsg={}",
                            self.key(),
                            p.ErrorID,
                            gb18030_cstr_to_str_i8(&p.ErrorMsg)
                        );
                    }
                }
                self.event_sender
                    .send(self.make_event("OnRspUserLogin", ""))
                    .await
                    .unwrap();
                let mut req = CThostFtdcSettlementInfoConfirmField::default();
                set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                let request_id = self.get_request_id();
                let result = self.api.req_settlement_info_confirm(&mut req, request_id);
                if result != 0 {
                    info!("{} ReqSettlementInfoConfirm={}", self.key(), result);
                }
            }
            OnRspSettlementInfoConfirm(ref _p) => {
                let mut req = CThostFtdcQryTradingAccountField::default();
                set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                let request_id = self.get_request_id();
                let result = self.api.req_qry_trading_account(&mut req, request_id);
                if result != 0 {
                    info!("{} ReqQueryTradingAccount={}", self.key(), result);
                }
            }
            OnRspQryTradingAccount(ref p) => {
                if let Some(taf) = p.p_trading_account {
                    info!(
                        "{} 查询账户资金完成.  account={} trading_day={:?} balance={}",
                        self.key(),
                        gb18030_cstr_to_str_i8(&taf.AccountID),
                        gb18030_cstr_to_str_i8(&taf.TradingDay),
                        taf.Balance
                    );
                }
                if p.b_is_last {
                    let mut req = CThostFtdcQryInvestorPositionDetailField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                    let request_id = self.get_request_id();
                    // flow control query limitation
                    let result = self
                        .api
                        .req_qry_investor_position_detail(&mut req, request_id);
                    if result != 0 {
                        info!("{} ReqQryInvestorPositionDetail = {:?}", self.key(), result);
                    }
                }
            }
            OnRspQryInvestorPositionDetail(ref detail) => {
                if let Some(d) = &detail.p_investor_position_detail {
                    let p = PositionDetailRow::from(d);
                    if let Some(v) = self.cta.position_details.get_mut(&p.key()) {
                        *v = p;
                    } else {
                        self.cta.position_details.insert(p.key(), p);
                    }
                }
                if detail.b_is_last {
                    info!("{} 查询持仓明细完成", self.key());
                    let mut req = CThostFtdcQryInvestorPositionField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                    let request_id = self.get_request_id();
                    let result = self.api.req_qry_investor_position(&mut req, request_id);
                    if result != 0 {
                        info!("{} ReqQueryPosition={}", self.key(), result);
                    }
                }
            }
            OnRspQryInvestorPosition(ref p) => {
                if let Some(p) = &p.p_investor_position {
                    let p = PositionRow::from(p);
                    if let Some(v) = self.cta.positions.get_mut(&p.key()) {
                        *v = p;
                    } else {
                        self.cta.positions.insert(p.key(), p);
                    }
                }
                if p.b_is_last {
                    info!("{} 查询持仓完成", self.key());
                    let mut req = CThostFtdcQryInstrumentField::default();
                    let request_id = self.get_request_id();
                    let result = self.api.req_qry_instrument(&mut req, request_id);
                    if result != 0 {
                        info!("{} ReqQryInstrument = {:?}", self.key(), result);
                    }
                }
            }
            OnRspQryInstrument(ref p) => {
                if let Some(instrument) = &p.p_instrument {
                    let mut instrument = InstrumentRow::from(instrument);
                    instrument.broker_id = self.conf.broker_id.clone();
                    instrument.account = self.conf.account.clone();
                    if let Some(v) = self.cta.instruments.get_mut(&instrument.key()) {
                        *v = instrument;
                    } else {
                        self.cta.instruments.insert(instrument.key(), instrument);
                    }
                }
                if p.b_is_last {
                    // 查询行情
                    info!("{} 查询合约完成", self.key());
                    let mut req = CThostFtdcQryDepthMarketDataField::default();
                    let request_id = self.get_request_id();
                    let result = self.api.req_qry_depth_market_data(&mut req, request_id);
                    if result != 0 {
                        info!("{} ReqQryDepthMarketData= {:?}", self.key(), result);
                    }
                }
            }
            OnRspQryDepthMarketData(ref p) => {
                if p.p_depth_market_data.is_some() {}
                if p.b_is_last {
                    info!("{} 查询行情完成 l={}", self.key(), 0);
                    let mut req = CThostFtdcQryOrderField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                    let request_id = self.get_request_id();
                    let result = self.api.req_qry_order(&mut req, request_id);
                    if result != 0 {
                        info!("{} ReqQryOrder = {:?}", result, self.key());
                    }
                }
            }
            OnRspQryOrder(ref p) => {
                if let Some(o) = &p.p_order {
                    let o = OrderRow::from(o);
                    self.cta.orders.insert(o.key(), o);
                }

                if p.b_is_last {
                    info!("{} 查询委托完成 l={}", self.key(), self.cta.orders.len());
                    let mut req = CThostFtdcQryTradeField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                    let request_id = self.get_request_id();
                    let result = self.api.req_qry_trade(&mut req, request_id);
                    if result != 0 {
                        info!("{} ReqQryTrade = {:?}", self.key(), result);
                    }
                }
            }
            OnRspQryTrade(ref p) => {
                if let Some(trade) = &p.p_trade {
                    let trade = TradeRow::from(trade);
                    if let Some(v) = self.cta.trades.get_mut(&trade.key()) {
                        *v = trade;
                    } else {
                        self.cta.trades.insert(trade.key(), trade);
                    }
                }
                if p.b_is_last {
                    info!("{} 查询成交明细完成 l={}", self.key(), 0);
                    self.event_sender
                        .send(self.make_event("LoginCompleted", ""))
                        .await
                        .unwrap();
                }
            }
            OnRspQryInstrumentCommissionRate(ref p) => {
                // 未处理
                if p.p_instrument_commission_rate.is_some() {
                    let _cr = p.p_instrument_commission_rate.unwrap();
                }
                if p.b_is_last {}
            }
            OnRtnOrder(ref p) => {
                if let Some(order) = &p.p_order {
                    let o = OrderRow::from(order);
                    let k = o.key();
                    if let Some(p) = self.cta.orders.get_mut(&k) {
                        *p = o;
                    } else {
                        self.cta.orders.insert(k.clone(), o);
                    }
                    self.event_sender
                        .send(self.make_event("Order", &k))
                        .await
                        .unwrap();
                }
            }
            OnRtnTrade(ref p) => {
                if let Some(trade) = &p.p_trade {
                    let trade = TradeRow::from(trade);
                    let k = trade.key();
                    if let Some(v) = self.cta.trades.get_mut(&k) {
                        *v = trade;
                    } else {
                        self.cta.trades.insert(trade.key(), trade);
                    }
                    self.event_sender
                        .send(self.make_event("Trade", &k))
                        .await
                        .unwrap();
                }
            }
            _ => {}
        }
    }
}
