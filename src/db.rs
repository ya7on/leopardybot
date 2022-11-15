use crate::conf::get_config;
use crate::error::{Error, Result};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn create_db() -> Result<DatabaseConnection> {
    info!("Connection to Database");
    let c = get_config();
    let mut opt = ConnectOptions::new(c.db.clone());
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await?;

    info!("Running migrations");
    Migrator::up(&db, None).await?;

    Ok(db)
}

pub fn clone_db(db: &DatabaseConnection) -> Result<DatabaseConnection> {
    match db {
        DatabaseConnection::SqlxPostgresPoolConnection(pool) => {
            Ok(DatabaseConnection::SqlxPostgresPoolConnection(pool.clone()))
        }
        _ => Err(Error("Cannot clone DatabaseConnection".to_owned())),
    }
}
