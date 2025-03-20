use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(HashMap::new()));

    let write_db = Arc::clone(&db);
    let w_handle = tokio::spawn(async move {
        let mut db = write_db.lock().await;
        db.insert("test", "test_value");
    });

    let read_db: Arc<Mutex<HashMap<&str, &str>>> = Arc::clone(&db);
    let r_handle = tokio::spawn(async move {
        println!("{}", read_db.lock().await.get("test").unwrap_or(&"none"));
    });

    _ = tokio::join!(w_handle, r_handle);
}
