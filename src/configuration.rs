use std::path::Path;
use std::sync::Arc;
use axum::Extension;
use config::Config;
use serde::Deserialize;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use tracing::{info, Level};
use tracing_appender::non_blocking::NonBlocking;
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::router::create_app;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub active: String,
    pub port: u16,
    pub log_level: String,
    pub log_path: String,
}

#[derive(Deserialize, Debug)]
pub struct ActiveSettings {
    pub mysql_url: String,
}

pub struct AppState {
    pub mysql_pool: MySqlPool,
}

impl AppState {
    pub fn new(mysql_pool: MySqlPool) -> Arc<Self> {
        Arc::new(Self { mysql_pool })
    }
}

pub fn get_configuration() -> Settings {
    let settings = Config::builder()
        .add_source(config::File::with_name("src/resources/configuration"))
        .build();
    settings
        .unwrap()
        .try_deserialize()
        .expect("Failed to read configuration file.")
}

pub fn get_active_settings(active: &String) -> ActiveSettings {
    let settings = Config::builder()
        .add_source(config::File::with_name(
            format!("src/resources/configuration-{}", active).as_str(),
        ))
        .build();
    settings
        .unwrap()
        .try_deserialize()
        .expect("Failed to read database configuration file.")
}

pub fn get_log_level(settings: &Settings) -> Level {
    match settings.log_level.as_str() {
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "debug" => tracing::Level::DEBUG,
        _ => panic!("Invalid log level"),
    }
}

pub fn get_log_file_appender(settings: &Settings) -> RollingFileAppender {
    let log_path = settings.log_path.as_str();
    if !Path::new(log_path).exists() {
        std::fs::create_dir_all(log_path).unwrap();
    }
    tracing_appender::rolling::daily(log_path, "api.log")
}

pub fn init_tracing(settings: &Settings) {
    let log_level = get_log_level(&settings);
    let std_io_layer = fmt::layer()
        .with_writer(std::io::stdout.with_max_level(log_level.clone()))
        .with_span_events(FmtSpan::CLOSE);
    if "dev".eq(&settings.active) {
        tracing_subscriber::registry().with(std_io_layer).init();
    } else {
        let file_appender = get_log_file_appender(&settings);
        let (file_writer, _guard) = NonBlocking::new(file_appender);
        let file_layer = fmt::layer()
            .with_writer(file_writer.with_max_level(log_level.clone()))
            .with_ansi(false)
            .with_span_events(FmtSpan::CLOSE);
        tracing_subscriber::registry().with(std_io_layer).with(file_layer).init();
    }
    info!("tracing init success.");
}

pub async fn init_mysql_pool(settings: &Settings) -> sqlx::Pool<sqlx::MySql> {
    let active_settings = get_active_settings(&settings.active);
    MySqlPoolOptions::new()
        .max_connections(50)
        .connect(&active_settings.mysql_url)
        .await
        .expect("Failed to create MySQL pool")
}

pub async fn run(settings: Settings, mysql_pool: MySqlPool) {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", settings.port))
        .await
        .expect(format!("Failed to bind port->{}", settings.port).as_str());
    let app = create_app().layer(Extension(AppState::new(mysql_pool)));
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}