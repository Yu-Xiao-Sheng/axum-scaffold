use axum_scaffold::configuration::{get_configuration, init_mysql_pool, init_tracing, run};

#[tokio::main]
async fn main() {
    let settings = get_configuration();
    init_tracing(&settings);
    let mysql_pool = init_mysql_pool(&settings).await;
    run(settings, mysql_pool).await;
}
