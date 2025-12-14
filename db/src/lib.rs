use std::env;

use migration::MigratorTrait as _;
use sea_orm::{Database, DatabaseConnection};
use tracing::info;

pub mod entities;
pub mod queries;

pub async fn connect() -> Result<DatabaseConnection, anyhow::Error> {
    let loc = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://db.sqlite".to_owned());
    info!(loc = loc, "Connecting to database");
    let db = Database::connect(loc).await?;
    migration::Migrator::up(&db, None).await?;

    Ok(db)
}
