#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ctp_futures::trader_api::*;
use log::info;
use tauri::Manager;
mod config;
use config::*;
use rust_share_util::*;
use std::io::Error;
use tokio::sync::Mutex;

pub fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    info!("close_splashscreen");
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    window.get_window("main").unwrap().show().unwrap();
}

struct Database {
    conf: G3Config,
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
    let conf = &mut db.lock().await.conf;
    if let Some(a) = conf.accounts.iter().find(|a| a.account == account.account) {
        return Err("账号已存在".to_string());
    }
    conf.accounts.push(account);
    conf.save(G3Config::default_path())
        .map_err(|e| (e.to_string()))
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

use std::io;
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, subscribe::CollectExt, EnvFilter};

struct MyWriter {}
impl std::io::Write for MyWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let out_str = String::from_utf8_lossy(buf).to_string();
        print!("{}", out_str);
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
    let (non_blocking2, _guard) = tracing_appender::non_blocking(MyWriter {});

    let collector = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(fmt::Subscriber::new().with_writer(io::stdout))
        .with(fmt::Subscriber::new().with_writer(non_blocking2))
        .with(fmt::Subscriber::new().with_writer(non_blocking));
    tracing::collect::set_global_default(collector).expect("Unable to set a global collector");

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing::info!("testttt");
    check_make_dir(".cache");
    let g3conf = G3Config::load(G3Config::default_path()).unwrap_or(G3Config::default());
    let db = Database { conf: g3conf };
    let state = StateTpye::new(db);
    tauri::Builder::default()
        .manage(state)
        .setup(|app| {
            // listen to the `event-name` (emitted on any window)
            let id = app.listen_global("event", |event| {
                info!("got event-name with payload {:?}", event.payload());
            });
            // unlisten to the event using the `id` returned on the `listen_global` function
            // an `once_global` API is also exposed on the `App` struct
            // app.unlisten(id);

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
                loop {
                    main_window
                        .emit(
                            "test-event",
                            Payload {
                                message: "Test event from rs!".into(),
                            },
                        )
                        .unwrap();
                    info!("emit test-event");
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            close_splashscreen,
            my_custom_command,
            account_list,
            add_account,
            default_account
        ])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
