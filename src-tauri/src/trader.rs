use crate::config::*;
use ctp_futures::trader_api::*;
use ctp_futures::*;
use futures::StreamExt;
use log::info;
use rust_share_util::*;
use std::ffi::CString;
use tokio::sync::mpsc::*;
use tokio::sync::oneshot;

pub struct Trader {
    conf: TradingAccount,
    api: Box<CThostFtdcTraderApi>,
    exit_signal_receiver: tokio::sync::oneshot::Receiver<String>,
    request_id: i32,
}

impl Trader {
    pub fn init(conf: TradingAccount, sink: Sender<(String, CThostFtdcTraderSpiOutput)>) -> Self {
        let conf1 = conf.clone();
        let (exit_sender, exit_receiver) = oneshot::channel::<String>();
        let broker_id = conf.broker_id;
        let account = conf.account;
        let fens_trade_front = conf.fens_trade_front.as_str();
        let trade_front = conf.trade_front.as_str();
        let auth_code = conf.auth_code.as_str();
        let user_product_info = conf.user_product_info.as_str();
        let app_id = conf.app_id.as_str();
        let password = conf.password.as_str();
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
        // 处理登陆初始化查询
        info!("{} 初始化查询完成.", account);
        tokio::spawn(async move {
            let key = format!("{}:{}", broker_id, account);
            while let Some(spimsg) = stream.next().await {
                sink.send((key.clone(), spimsg)).await.unwrap();
            }
            exit_sender.send("exited".to_string()).unwrap();
        });
        Trader {
            conf: conf1,
            api,
            exit_signal_receiver: exit_receiver,
            request_id: 10,
        }
    }

    pub async fn delete(self) {
        // self.api.release();
        // Box::leak(api);
        self.exit_signal_receiver.await.unwrap();
    }

    pub fn get_request_id(&mut self) -> i32 {
        self.request_id += 1;
        self.request_id
    }

    pub fn handle_spi_msg(&mut self, spi_msg: &CThostFtdcTraderSpiOutput) {
        let conf = &self.conf;
        let broker_id = conf.broker_id.as_str();
        let account = conf.account.as_str();
        let fens_trade_front = conf.fens_trade_front.as_str();
        let trade_front = conf.trade_front.as_str();
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
                info!("OnFrontConnected");
            }
            OnFrontDisconnected(p) => {
                info!("on front disconnected {:?} 直接Exit ", p);
                std::process::exit(-1);
            }
            OnRspAuthenticate(ref p) => {
                if p.p_rsp_info.as_ref().unwrap().ErrorID == 0 {
                    let mut req = CThostFtdcReqUserLoginField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.UserID, account);
                    set_cstr_from_str_truncate_i8(&mut req.Password, password);
                    let request_id = self.get_request_id();
                    self.api.req_user_login(&mut req, request_id);
                } else {
                    info!("RspAuthenticate={:?}", p);
                    std::process::exit(-1);
                }
            }
            OnRspUserLogin(ref p) => {
                if p.p_rsp_info.as_ref().unwrap().ErrorID == 0 {
                    let u = p.p_rsp_user_login.unwrap();
                } else {
                    info!("Trade RspUserLogin = {:?}", print_rsp_info!(&p.p_rsp_info));
                }
                let mut req = CThostFtdcSettlementInfoConfirmField::default();
                set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                let request_id = self.get_request_id();
                let result = self.api.req_settlement_info_confirm(&mut req, request_id);
                if result != 0 {
                    info!("ReqSettlementInfoConfirm={}", result);
                }
            }
            OnRspSettlementInfoConfirm(ref _p) => {
                let mut req = CThostFtdcQryTradingAccountField::default();
                set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                let request_id = self.get_request_id();
                let result = self.api.req_qry_trading_account(&mut req, request_id);
                if result != 0 {
                    info!("ReqQueryTradingAccount={}", result);
                }
            }
            OnRspQryTradingAccount(ref p) => {
                if let Some(taf) = p.p_trading_account {
                    info!(
                        "查询账户资金完成.  account={} trading_day={:?} balance={}",
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
                        info!("ReqQryInvestorPositionDetail = {:?}", result);
                    }
                }
            }
            OnRspQryInvestorPositionDetail(ref detail) => {
                if let Some(d) = detail.p_investor_position_detail {
                    info!("d={:?}", d);
                }
                if detail.b_is_last {
                    info!("查询持仓明细完成");
                    let mut req = CThostFtdcQryInvestorPositionField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                    let request_id = self.get_request_id();
                    let result = self.api.req_qry_investor_position(&mut req, request_id);
                    if result != 0 {
                        info!("ReqQueryPosition={}", result);
                    }
                }
            }
            OnRspQryInvestorPosition(ref p) => {
                if let Some(p) = p.p_investor_position {
                    info!("pos={:?}", p);
                }
                if p.b_is_last {
                    info!("查询持仓完成");
                    let mut req = CThostFtdcQryInstrumentField::default();
                    let request_id = self.get_request_id();
                    let result = self.api.req_qry_instrument(&mut req, request_id);
                    if result != 0 {
                        info!("ReqQryInstrument = {:?}", result);
                    }
                }
            }
            OnRspQryInstrument(ref p) => {
                if let Some(instrument) = p.p_instrument {
                    info!(
                        "inst=[{:?}:{:?}]",
                        gb18030_cstr_to_str_i8(&instrument.ExchangeID),
                        gb18030_cstr_to_str_i8(&instrument.InstrumentID)
                    );
                }
                if p.b_is_last {
                    // 查询行情
                    info!("查询合约完成");
                    let mut req = CThostFtdcQryDepthMarketDataField::default();
                    let request_id = self.get_request_id();
                    let result = self.api.req_qry_depth_market_data(&mut req, request_id);
                    if result != 0 {
                        info!("ReqQryDepthMarketData= {:?}", result);
                    }
                }
            }
            OnRspQryDepthMarketData(ref p) => {
                if p.p_depth_market_data.is_some() {}
                if p.b_is_last {
                    info!("查询行情完成 l={}", 0);
                    let mut req = CThostFtdcQryOrderField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                    let request_id = self.get_request_id();
                    let result = self.api.req_qry_order(&mut req, request_id);
                    if result != 0 {
                        info!("ReqQryOrder = {:?}", result);
                    }
                }
            }
            OnRspQryOrder(ref p) => {
                if p.p_order.is_some() {}

                if p.b_is_last {
                    info!("查询委托完成 l={}", 0);
                    let mut req = CThostFtdcQryTradeField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                    let request_id = self.get_request_id();
                    let result = self.api.req_qry_trade(&mut req, request_id);
                    if result != 0 {
                        info!("ReqQryTrade = {:?}", result);
                    }
                }
            }
            OnRspQryTrade(ref p) => {
                if let Some(trade) = p.p_trade {}
                if p.b_is_last {
                    info!("查询成交明细完成 l={}", 0);
                }
            }
            OnRspQryInstrumentCommissionRate(ref p) => {
                // 未处理
                if p.p_instrument_commission_rate.is_some() {
                    let cr = p.p_instrument_commission_rate.unwrap();
                }
                if p.b_is_last {}
            }
            _ => {}
        }
    }
}
