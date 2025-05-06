use redis::AsyncCommands;

pub async fn store_checkin(key: &str, project_id: i32) {
    let client = redis::Client::open(std::env::var("REDIS_URL").unwrap()).unwrap();
    let mut con = client.get_async_connection().await.unwrap();

    let _: () = con
        .set_ex(format!("checkin:{}", key), project_id, 60 * 60)
        .await
        .unwrap();
}