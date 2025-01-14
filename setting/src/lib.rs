use serde::Deserialize;
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

pub async fn get() -> (sqlx::MySqlPool, TcpListener) {
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
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to MySQL");

    let listner = TcpListener::bind(format!("127.0.0.1:{}", setting.app_port))
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to port {}", setting.app_port));
    (pool, listner)

    // migrate!("../migrations")
    //     .run(&pool)
    //     .await
    //     .expect("Failed to migrate database");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let setting = load().expect("Failed to load configuration");
        assert_eq!(setting.app_port, 3000);
        assert_eq!(setting.db.db_type, "mysql");
    }
}
