use crate::conf::get_config;
use crate::error::{Error, Result};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn create_db() -> Result<DatabaseConnection> {
    info!("Connection to Database");
    let c = get_config();
    let opt = ConnectOptions::new(c.db.clone());
    let db = Database::connect(opt).await.map_err(|err| {
        Error::DatabaseError(format!("Cannot create database connection. {}", err))
    })?;

    info!("Running migrations");
    Migrator::up(&db, None)
        .await
        .map_err(|err| Error::DatabaseError(format!("Cannot run migrations. {}", err)))?;

    Ok(db)
}

pub fn clone_db(db: &DatabaseConnection) -> Result<DatabaseConnection> {
    match db {
        DatabaseConnection::SqlxPostgresPoolConnection(pool) => {
            Ok(DatabaseConnection::SqlxPostgresPoolConnection(pool.clone()))
        }
        _ => Err(Error::DatabaseError(
            "Cannot clone DatabaseConnection".to_owned(),
        )),
    }
}
