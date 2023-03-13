#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ctp_futures::trader_api::*;
use log::{error, info};
use tauri::Manager;
mod config;
use config::*;
use rust_share_util::*;
use std::io::Error;
use tokio::sync::Mutex;
mod trader;
use std::io;
use std::sync::mpsc::*;
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, subscribe::CollectExt, EnvFilter};
use trader::*;

#[tauri::command]
async fn close_splashscreen(
    window: tauri::Window,
    database: tauri::State<'_, StateTpye>,
) -> Result<(), String> {
    info!("close_splashscreen");
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    window.get_window("main").unwrap().show().unwrap();
    info!("Sync traders on start");
    database.lock().await.sync_traders();
    Ok(())
}

struct Database {
    conf: G3Config,
    traders: std::collections::HashMap<String, Trader>,
    sink_sender: tokio::sync::mpsc::Sender<(String, CThostFtdcTraderSpiOutput)>,
}

impl Database {
    pub fn sync_traders(&mut self) {
        for ta in self.conf.accounts.iter().filter(|ta| {
            if ta.account.len() == 0 {
                error!("[{}:{}] account不能为空", ta.broker_id, ta.account);
                return false;
            }
            if ta.trade_front.len() == 0 {
                error!("[{}:{}] trade_front不能为空", ta.broker_id, ta.account);
                return false;
            }
            true
        }) {
            let key = format!("{}:{}", ta.broker_id, ta.account);
            if !self.traders.contains_key(&key) {
                let sink = self.sink_sender.clone();
                let trader = trader::Trader::init(ta.clone(), sink);
            }
        }
    }
    pub fn new(g3conf: G3Config) -> Self {
        let (sink_sender, mut sink_receiver) = tokio::sync::mpsc::channel(1000);
        let db = Database {
            conf: g3conf,
            traders: std::collections::HashMap::new(),
            sink_sender,
        };
        tokio::spawn(async move {
            info!("start receive spi message");
            while let Some((key, message)) = sink_receiver.recv().await {
                info!("[{}] GOT = {:?}", key, message);
            }
            info!("exit receive spi message");
        });
        db
    }
}

type StateTpye = Mutex<Database>;

#[derive(serde::Serialize)]
struct CustomResponse {
    message: String,
    other_val: usize,
}

async fn some_other_function() -> Option<String> {
    Some("response".into())
}

#[tauri::command]
async fn account_list(
    window: tauri::Window,
    database: tauri::State<'_, StateTpye>,
) -> Result<Vec<TradingAccount>, String> {
    Ok(database.lock().await.conf.accounts.clone())
}

#[tauri::command]
async fn default_account(
    window: tauri::Window,
    database: tauri::State<'_, StateTpye>,
) -> Result<TradingAccount, String> {
    Ok(TradingAccount::default())
}

#[tauri::command]
async fn add_account(
    window: tauri::Window,
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
        if let Some(a) = conf.accounts.iter().find(|a| a.account == account.account) {
            return Err("账号已存在".to_string());
        }
        conf.accounts.push(account);
        conf.save(G3Config::default_path()).unwrap();
    }
    db.sync_traders();
    Ok(())
}

#[tauri::command]
async fn delete_account(
    window: tauri::Window,
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
    db.sync_traders();
    Ok(())
}

#[tauri::command]
async fn my_custom_command(
    window: tauri::Window,
    number: usize,
    database: tauri::State<'_, StateTpye>,
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
// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

struct FrontLogWriter {
    log_sender: std::sync::Mutex<Sender<String>>,
}
impl std::io::Write for FrontLogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let out_str = String::from_utf8_lossy(buf).to_string();
        self.log_sender.lock().unwrap().send(out_str).unwrap();
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// Register the command:
#[tokio::main]
async fn main() {
    LogTracer::init().unwrap();
    let file_appender = tracing_appender::rolling::hourly(".cache", "example.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let (log_sender, log_receiver) = channel();
    let (non_blocking2, _guard) = tracing_appender::non_blocking(FrontLogWriter {
        log_sender: std::sync::Mutex::new(log_sender),
    });

    let collector = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(fmt::Subscriber::new().with_writer(io::stdout))
        .with(fmt::Subscriber::new().with_writer(non_blocking2))
        .with(fmt::Subscriber::new().with_writer(non_blocking));
    tracing::collect::set_global_default(collector).expect("Unable to set a global collector");

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    check_make_dir(".cache");
    let g3conf = G3Config::load(G3Config::default_path()).unwrap_or(G3Config::default());
    let db = Database::new(g3conf);
    let state = StateTpye::new(db);
    tauri::Builder::default()
        .manage(state)
        .setup(|app| {
            // listen to the `event-name` (emitted on any window)
            let id = app.listen_global("event", |event| {
                info!("got event-name with payload {:?}", event.payload());
            });
            // emit the `event-name` event to all webview windows on the frontend
            app.emit_all(
                "event-name",
                Payload {
                    message: "Tauri is awesome!".into(),
                },
            )
            .unwrap();
            let main_window = app.get_window("main").unwrap();
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                while let Ok(i) = log_receiver.recv() {
                    main_window
                        .emit("new-log-line", Payload { message: i })
                        .unwrap();
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            close_splashscreen,
            my_custom_command,
            account_list,
            add_account,
            default_account,
            delete_account
        ])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if event.window().label() == "log" {
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
