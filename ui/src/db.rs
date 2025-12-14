use dioxus::fullstack::Lazy;
use sea_orm::DatabaseConnection;

pub static DB: Lazy<DatabaseConnection> = Lazy::new(async || db::connect().await);

pub fn db() -> &'static DatabaseConnection {
    DB.get()
}
