use sea_orm::prelude::*;
use serde::Deserialize;
use std::time::Duration;
use tokio::net::TcpListener;

#[derive(Deserialize)]
pub struct Setting {
    pub app_port: u16,
    pub db: DBSetting,
}

#[derive(Deserialize)]
pub struct DBSetting {
    #[serde(rename = "type")]
    pub db_type: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
    pub username: String,
    pub password: String,
}

fn load() -> Result<Setting, config::ConfigError> {
    config::Config::builder()
        .add_source(config::File::with_name("configration"))
        .build()?
        .try_deserialize::<Setting>()
}

pub async fn get() -> (DbConn, TcpListener) {
    let setting = load().expect("Failed to load configuration");

    let db_url = format!(
        "{}://{}:{}@{}:{}/{}",
        setting.db.db_type,
        setting.db.username,
        setting.db.password,
        setting.db.host,
        setting.db.port,
        setting.db.db_name
    );

    let mut opt = sea_orm::ConnectOptions::new(&db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    let db_conn = sea_orm::Database::connect(opt)
        .await
        .expect("连接数据库失败");

    let listner = TcpListener::bind(format!("127.0.0.1:{}", setting.app_port))
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to port {}", setting.app_port));

    (db_conn, listner)
}
