use crate::config::*;
use crate::trader;
use crate::trader::*;
use itertools::Itertools;
use log::error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn ta_key(broker_id: &str, account: &str) -> String {
    format!("{broker_id}:{account}")
}

pub struct Database {
    pub conf: G3Config,
    pub traders: std::collections::HashMap<String, Arc<Mutex<Trader>>>,
    pub cta_event_sender: tokio::sync::mpsc::Sender<CtaEvent>,
}

impl Database {
    pub async fn sync_traders(&mut self) {
        for ta in self.conf.accounts.iter().filter(|ta| {
            if ta.account.len() == 0 {
                error!("[{}:{}] account不能为空", ta.broker_id, ta.account);
                return false;
            }
            true
        }) {
            let broker = self
                .conf
                .brokers
                .iter()
                .find(|b| b.broker_id == ta.broker_id);
            if let Some(broker) = broker {
                let key = format!("{}:{}", ta.broker_id, ta.account);
                if !self.traders.contains_key(&key) {
                    let trader = trader::Trader::init(
                        ta.clone(),
                        broker.clone(),
                        self.cta_event_sender.clone(),
                    );
                    match trader {
                        Ok(trader) => {
                            self.traders.insert(key, trader);
                        }
                        Err(e) => {
                            error!("Init trader {}", e);
                        }
                    }
                }
            } else {
                error!(
                    "{}:{} 没有找到经纪商[broker_id={}]",
                    ta.broker_id, ta.account, ta.broker_id
                );
            }
        }
        let delete_list = self
            .traders
            .iter()
            .filter(|(k, _v)| {
                self.conf
                    .accounts
                    .iter()
                    .find(|ta| format!("{}:{}", ta.broker_id, ta.account) == **k)
                    .is_none()
            })
            .map(|(k, _v)| k.clone())
            .collect::<Vec<_>>();
        for k in delete_list.iter() {
            if let Some(trader) = self.traders.remove(k) {
                if let Some(sender) = trader.lock().await.exit_sender.take() {
                    sender.send("exit".to_string()).unwrap();
                }
            }
        }
    }
    pub fn new(g3conf: G3Config, cta_es: tokio::sync::mpsc::Sender<CtaEvent>) -> Self {
        let db = Database {
            conf: g3conf,
            traders: std::collections::HashMap::new(),
            cta_event_sender: cta_es,
        };
        db
    }

    pub async fn order_rows(&self) -> Vec<OrderRow> {
        let mut v = vec![];
        for (_, t) in self.traders.iter() {
            let t = t.lock().await;
            for (_, o) in t.cta.orders.iter() {
                v.push(o.clone());
            }
        }
        v
    }

    pub async fn get_order_row(
        &self,
        broker_id: &str,
        account: &str,
        key: &str,
    ) -> Option<OrderRow> {
        if let Some(t) = self.traders.get(&ta_key(broker_id, account)) {
            t.lock().await.cta.orders.get(key).cloned()
        } else {
            None
        }
    }

    pub async fn trade_rows(&self) -> Vec<TradeRow> {
        let mut v = vec![];
        for (_, t) in self.traders.iter() {
            let t = t.lock().await;
            for (_, t) in t.cta.trades.iter() {
                v.push(t.clone());
            }
        }
        v
    }
    pub async fn get_trade_row(
        &self,
        broker_id: &str,
        account: &str,
        key: &str,
    ) -> Option<TradeRow> {
        if let Some(t) = self.traders.get(&ta_key(broker_id, account)) {
            t.lock().await.cta.trades.get(key).cloned()
        } else {
            None
        }
    }

    pub async fn position_rows(&self) -> Vec<PositionRow> {
        let mut v = vec![];
        for (_, t) in self.traders.iter() {
            let t = t.lock().await;
            for (_, t) in t.cta.positions.iter() {
                v.push(t.clone());
            }
        }
        v
    }
    pub async fn get_position_row(
        &self,
        broker_id: &str,
        account: &str,
        key: &str,
    ) -> Option<PositionRow> {
        if let Some(t) = self.traders.get(&ta_key(broker_id, account)) {
            t.lock().await.cta.positions.get(key).cloned()
        } else {
            None
        }
    }

    pub async fn position_detail_rows(&self) -> Vec<PositionDetailRow> {
        let mut v = vec![];
        for (_, t) in self.traders.iter() {
            let t = t.lock().await;
            for (_, t) in t.cta.position_details.iter() {
                v.push(t.clone());
            }
        }
        v
    }
    pub async fn get_position_detail_row(
        &self,
        broker_id: &str,
        account: &str,
        key: &str,
    ) -> Option<PositionDetailRow> {
        if let Some(t) = self.traders.get(&ta_key(broker_id, account)) {
            t.lock().await.cta.position_details.get(key).cloned()
        } else {
            None
        }
    }

    pub async fn instrument_rows(&self) -> Vec<InstrumentRow> {
        let mut v = vec![];
        for (_, t) in self.traders.iter() {
            let t = t.lock().await;
            for (_, t) in t.cta.instruments.iter() {
                v.push(t.clone());
            }
            break;
        }
        v
    }
    pub async fn get_instrument_row(
        &self,
        broker_id: &str,
        account: &str,
        key: &str,
    ) -> Option<InstrumentRow> {
        if let Some(t) = self.traders.get(&ta_key(broker_id, account)) {
            t.lock().await.cta.instruments.get(key).cloned()
        } else {
            None
        }
    }

    pub async fn account_rows(&self) -> Vec<TradingAccountRow> {
        let mut v = self
            .conf
            .accounts
            .iter()
            .map(|a| {
                let mut row = TradingAccountRow::default();
                row.broker_id = a.broker_id.clone();
                row.account = a.account.clone();
                row.password = a.password.clone();
                row.front_group = a.front_group.clone();
                row
            })
            .collect_vec();
        for row in v.iter_mut() {
            if let Some(trader) = self.traders.get(&ta_key(&row.broker_id, &row.account)) {
                let trader = trader.lock().await;
                row.status = trader.status();
                row.status_description = trader.status_description();
                row.equity = trader.cta.ta.Balance;
                row.margin = trader.cta.ta.CurrMargin;
                row.closed_profit = trader.cta.ta.CloseProfit;
                row.position_profit = trader.cta.ta.PositionProfit;
                row.available = trader.cta.ta.Available;
                row.frozen_margin = trader.cta.ta.FrozenMargin;
                row.frozen_commission = trader.cta.ta.FrozenCommission;
                if let Some(b) = self
                    .conf
                    .brokers
                    .iter()
                    .find(|b| b.broker_id == trader.conf.broker_id)
                {
                    row.broker_name = b.name.clone();
                    if let Some(fg) = b.fronts.iter().find(|fg| fg.id == row.front_group) {
                        row.front_group_name = fg.name.clone();
                    }
                }
            }
        }
        v
    }
}
