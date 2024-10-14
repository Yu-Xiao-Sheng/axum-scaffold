use tracing::info;
use tracing_appender::non_blocking::NonBlocking;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use axum_scaffold::configuration::{get_active_settings, get_configuration, get_log_file_appender, get_log_level, init_mysql_pool, init_tracing, run};

#[tokio::main]
async fn main() {
    let settings = get_configuration();
    init_tracing(&settings);
    let mysql_pool = init_mysql_pool(&settings).await;
    run(settings, mysql_pool).await;
}
