use std::error::Error;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::{Connection, Surreal};

pub async fn establish_connection(
    url: Option<String>,
    username: Option<String>,
    password: Option<String>,
) -> Result<Surreal<impl Connection>, Box<dyn Error>> {
    let url = url.unwrap_or_else(|| {
        panic!("Error: No url provided. Please provide a url in the config file or as an argument.")
    });
    let username = username.unwrap_or_else(|| {
        panic!("Error: No username provided. Please provide a username in the config file or as an argument.")
    });
    let password = password.unwrap_or_else(|| {
        panic!("Error: No password provided. Please provide a password in the config file or as an argument.")
    });

    let url = url.as_str();
    let db = Surreal::new::<Ws>(url).await?;

    db.signin(Root {
        username: username.as_str(),
        password: password.as_str(),
    });
    Ok(db)
}
