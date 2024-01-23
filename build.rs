use std::env;
use std::path::PathBuf;

fn main() {
    dotenvy::dotenv().ok();
    let db = env::var("DATABASE_URL").expect("Environment variable DATABASE_URL not set");
    let db = PathBuf::from(db);

    if !db.exists() {
        println!("cargo:rustc-env=SQLX_OFFLINE=1")
    }
}
