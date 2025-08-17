use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};

pub async fn connect(migrate: Option<bool>) -> DatabaseConnection {
    let url: String = match dotenvy::var("DATABASE_URL") {
        Ok(u) => u,
        Err(_) => std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };
    let url = &*url;
    let schema = dotenvy::var("SCHEMA").unwrap_or("public".to_string());
    let mut options = ConnectOptions::new(url);
    options
        .min_connections(2)
        .sqlx_logging(true)
        .set_schema_search_path("public");
    println!("Connecting to db: {}", url);
    let conn = Database::connect(options)
        .await
        .expect("Failed to connect to db");
    println!("Connected to db: {}", url);
    conn.execute_unprepared(format!("SET search_path TO {schema}").as_str())
        .await
        .expect("Failed to set schema");

    if migrate.unwrap_or(false) {
        Migrator::up(&conn, None).await.expect("Failed to migrate");
        println!("Migration applied");
    }

    conn
}
