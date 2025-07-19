use sqlx::{Pool, Postgres};

pub async fn create_conn_pool() -> Result<Pool<Postgres>, sqlx::Error> { 
    // Get the database URL from the environment
    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url).await;
    if let Ok(x) = pool {
        Ok(x)
    } else {
        Err(sqlx::Error::PoolClosed.into())
    }
}



