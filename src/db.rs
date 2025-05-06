use sqlx::PgPool;

pub async fn connect_pg() -> PgPool {
    PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Postgres connection failed")
}
