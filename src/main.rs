use diesel::Connection;
use std::{env, thread};
use dotenv::dotenv;
use std::sync::Arc;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = Arc::new(database_url);

    unsafe {
        println!("PQisthreadsafe: {}", pq_sys::PQisthreadsafe());
    }
    let mut handles = Vec::new();
    for _ in 0..20 {
        let database_url = Arc::clone(&database_url);
        let handle = thread::spawn(move || {
            let _con = diesel::PgConnection::establish(&database_url); //?sslmode=disable");
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}
