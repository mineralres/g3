#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::info;
use tauri::Manager;
mod config;
use config::*;
use rust_share_util::*;
mod trader;
use std::io;
use std::sync::mpsc::*;
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, subscribe::CollectExt, EnvFilter};
mod command;
use command::*;
mod db;
use db::*;

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
    let (cta_es, mut cta_er) = tokio::sync::mpsc::channel(1000);
    let db = Database::new(g3conf, cta_es);
    let state = StateTpye::new(db);
    tauri::Builder::default()
        .manage(state)
        .setup(|app| {
            // listen to the `event-name` (emitted on any window)
            let _id = app.listen_global("event", |event| {
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
            let main_window = app.get_window("main").unwrap();
            tokio::spawn(async move {
                while let Some(e) = cta_er.recv().await {
                    main_window.emit("cta-event", e).unwrap();
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
            delete_account,
            order_rows,
            get_order_row,
            trade_rows,
            get_trade_row,
            position_detail_rows,
            get_position_detail_row,
            position_rows,
            get_position_row,
            instrument_rows,
            get_instrument_row
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
