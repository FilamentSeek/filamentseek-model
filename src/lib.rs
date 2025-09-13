use surrealdb::{Surreal, engine::remote::ws::Ws, opt::auth::Root};

pub mod product;

pub async fn surrealdb_client() -> Result<Surreal<surrealdb::engine::remote::ws::Client>, String> {
    let surreal_address = if let Ok(addr) = std::env::var("SURREAL_ADDRESS") {
        addr
    } else {
        return Err("SURREAL_ADDRESS environment variable not set".to_owned());
    };

    let surreal_username = if let Ok(user) = std::env::var("SURREAL_USERNAME") {
        user
    } else {
        return Err("SURREAL_USERNAME environment variable not set".to_owned());
    };

    let surreal_password = if let Ok(pass) = std::env::var("SURREAL_PASSWORD") {
        pass
    } else {
        return Err("SURREAL_PASSWORD environment variable not set".to_owned());
    };

    let surreal_namespace = if let Ok(ns) = std::env::var("SURREAL_NAMESPACE") {
        ns
    } else {
        return Err("SURREAL_NAMESPACE environment variable not set".to_owned());
    };

    let surreal_database = if let Ok(db) = std::env::var("SURREAL_DATABASE") {
        db
    } else {
        return Err("SURREAL_DATABASE environment variable not set".to_owned());
    };

    let db = Surreal::new::<Ws>(surreal_address)
        .await
        .map_err(|e| "Error connecting to SurrealDB: ".to_owned() + &e.to_string())?;

    db.signin(Root {
        username: &surreal_username,
        password: &surreal_password,
    })
    .await
    .map_err(|e| "Error signing in to SurrealDB: ".to_owned() + &e.to_string())?;

    db.use_ns(&surreal_namespace)
        .use_db(&surreal_database)
        .await
        .map_err(|e| "Error using namespace/database: ".to_owned() + &e.to_string())?;

    Ok(db)
}
