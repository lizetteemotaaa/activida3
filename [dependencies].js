[dependencies]
rocket = "0.5.0-rc.1"
diesel = { version = "2.0.0", features = ["postgres"] }
rocket_sync_db_pools = { version = "0.1.0-rc.1", features = ["diesel_postgres_pool"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"