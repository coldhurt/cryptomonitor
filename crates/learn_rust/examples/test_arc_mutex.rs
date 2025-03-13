use std::{collections::HashMap, sync::{Arc, Mutex}};

#[tokio::main]
async fn main(){

    let db = Arc::new(Mutex::new(HashMap::new()));

    let write_db = db.clone();
    let w_handle = tokio::spawn(async move {
        let mut db = write_db.lock().unwrap();
        db.insert("test", "test_value");
    });

    let read_db = db.clone();
    let r_handle = tokio::spawn(async move{
        println!("{}", read_db.lock().unwrap().get("test").unwrap_or(&"none"));
    });

    _ = tokio::join!(w_handle, r_handle);
}