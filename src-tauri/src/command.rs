use crate::config::*;
use crate::db::Database;
use log::{error, info};
use tauri::Manager;
use tokio::sync::Mutex;

pub type StateTpye = Mutex<Database>;

#[tauri::command]
pub async fn close_splashscreen(
    window: tauri::Window,
    database: tauri::State<'_, StateTpye>,
) -> Result<(), String> {
    info!("close_splashscreen");
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    window.get_window("main").unwrap().show().unwrap();
    info!("Sync traders on start");
    database.lock().await.sync_traders().await;
    Ok(())
}

#[tauri::command]
pub async fn account_list(
    _window: tauri::Window,
    database: tauri::State<'_, StateTpye>,
) -> Result<Vec<TradingAccountRow>, String> {
    let v = database.lock().await.account_rows().await;
    Ok(v)
}

#[tauri::command]
pub async fn instrument_rows(
    _window: tauri::Window,
    database: tauri::State<'_, StateTpye>,
) -> Result<Vec<InstrumentRow>, String> {
    Ok(database.lock().await.instrument_rows().await)
}

#[tauri::command]
pub async fn get_instrument_row(
    _window: tauri::Window,
    broker_id: String,
    account: String,
    key: String,
    database: tauri::State<'_, StateTpye>,
) -> Result<Option<InstrumentRow>, String> {
    Ok(database
        .lock()
        .await
        .get_instrument_row(&broker_id, &account, &key)
        .await)
}

#[tauri::command]
pub async fn trade_rows(
    _window: tauri::Window,
    database: tauri::State<'_, StateTpye>,
) -> Result<Vec<TradeRow>, String> {
    Ok(database.lock().await.trade_rows().await)
}

#[tauri::command]
pub async fn get_trade_row(
    _window: tauri::Window,
    broker_id: String,
    account: String,
    key: String,
    database: tauri::State<'_, StateTpye>,
) -> Result<Option<TradeRow>, String> {
    Ok(database
        .lock()
        .await
        .get_trade_row(&broker_id, &account, &key)
        .await)
}

#[tauri::command]
pub async fn position_detail_rows(
    _window: tauri::Window,
    database: tauri::State<'_, StateTpye>,
) -> Result<Vec<PositionDetailRow>, String> {
    Ok(database.lock().await.position_detail_rows().await)
}

#[tauri::command]
pub async fn get_position_detail_row(
    _window: tauri::Window,
    broker_id: String,
    account: String,
    key: String,
    database: tauri::State<'_, StateTpye>,
) -> Result<Option<PositionDetailRow>, String> {
    Ok(database
        .lock()
        .await
        .get_position_detail_row(&broker_id, &account, &key)
        .await)
}

#[tauri::command]
pub async fn position_rows(
    _window: tauri::Window,
    database: tauri::State<'_, StateTpye>,
) -> Result<Vec<PositionRow>, String> {
    Ok(database.lock().await.position_rows().await)
}

#[tauri::command]
pub async fn get_position_row(
    _window: tauri::Window,
    broker_id: String,
    account: String,
    key: String,
    database: tauri::State<'_, StateTpye>,
) -> Result<Option<PositionRow>, String> {
    Ok(database
        .lock()
        .await
        .get_position_row(&broker_id, &account, &key)
        .await)
}

#[tauri::command]
pub async fn order_rows(
    _window: tauri::Window,
    database: tauri::State<'_, StateTpye>,
) -> Result<Vec<OrderRow>, String> {
    Ok(database.lock().await.order_rows().await)
}

#[tauri::command]
pub async fn get_order_row(
    _window: tauri::Window,
    broker_id: String,
    account: String,
    key: String,
    database: tauri::State<'_, StateTpye>,
) -> Result<Option<OrderRow>, String> {
    Ok(database
        .lock()
        .await
        .get_order_row(&broker_id, &account, &key)
        .await)
}

#[tauri::command]
pub async fn default_account(
    _window: tauri::Window,
    _database: tauri::State<'_, StateTpye>,
) -> Result<TradingAccount, String> {
    Ok(TradingAccount::default())
}

#[tauri::command]
pub async fn add_account(
    _window: tauri::Window,
    account: TradingAccount,
    db: tauri::State<'_, StateTpye>,
) -> Result<(), String> {
    info!("add account = {:?}", account);
    if account.account.len() == 0 {
        return Err("账号不能为空".to_string());
    } else if account.broker_id.len() == 0 {
        return Err("broker_id不能为空".to_string());
    }
    let mut db = db.lock().await;
    {
        let conf = &mut db.conf;
        if let Some(_a) = conf.accounts.iter().find(|a| a.account == account.account) {
            error!(
                "账户[{}:{}]不能重复添加",
                account.broker_id, account.account
            );
            return Err("账号已存在".to_string());
        }
        conf.accounts.push(account);
        conf.save(G3Config::default_path()).unwrap();
    }
    db.sync_traders().await;
    Ok(())
}

#[tauri::command]
pub async fn delete_account(
    _window: tauri::Window,
    broker_id: String,
    account: String,
    db: tauri::State<'_, StateTpye>,
) -> Result<(), String> {
    info!("delete account = [{}:{}]", broker_id, account);
    let mut db = db.lock().await;
    {
        let conf = &mut db.conf;
        conf.accounts
            .retain(|ta| !(ta.account == account && ta.broker_id == broker_id));
        conf.save(G3Config::default_path()).unwrap();
    }
    db.sync_traders().await;
    Ok(())
}

#[tauri::command]
pub async fn my_custom_command(
    window: tauri::Window,
    number: usize,
    _database: tauri::State<'_, StateTpye>,
) -> Result<CustomResponse, String> {
    println!("Called from {}", window.label());
    let result: Option<String> = some_other_function().await;
    if let Some(message) = result {
        Ok(CustomResponse {
            message,
            other_val: 42 + number,
        })
    } else {
        Err("No result".into())
    }
}

#[derive(serde::Serialize)]
pub struct CustomResponse {
    pub message: String,
    pub other_val: usize,
}

pub async fn some_other_function() -> Option<String> {
    Some("response".into())
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub message: String,
}
