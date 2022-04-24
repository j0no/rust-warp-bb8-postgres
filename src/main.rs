use dotenv::dotenv;
use warp::{http::{Method }, Filter};
use tokio_postgres::config::Config;
use bb8_postgres::{PostgresConnectionManager, bb8, tokio_postgres};
use bb8::Pool;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{IpAddr, Ipv4Addr};
use std::env;


type DBPool = Pool<PostgresConnectionManager<tokio_postgres::NoTls>>;

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

#[derive(Deserialize, Serialize)]
struct Response {
    message: Option<String>
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_config = env::var("DB_INFO").expect("unable to get DB_INFO from env variables").as_str().parse::<Config>().expect("unable to parse DB_INFO");
    let manager = PostgresConnectionManager::new(db_config, tokio_postgres::NoTls);
    let pool = Pool::builder().build(manager).await.unwrap();
    
    let cors = warp::cors()
    .allow_any_origin()
    .allow_header("Content-Type")
    .allow_methods(&[Method::GET, Method::POST, Method::DELETE, Method::PUT]);

    let root = warp::path::end().map(|| "api");

    let hello = warp::get()
    .and(warp::path!("hello"))
    .and(with_db(pool.clone()))
    .and(warp::query::<HashMap<String, String>>())
    .and_then(|pool: DBPool, _p: HashMap<String, String>| async move {
        let connection = pool.get().await.expect("pool error");
        let rows = connection.query("select \'hello world\' ", &[]).await.unwrap();
        let hello = Response {
            message: rows[0].get(0)
        };
        Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json(&hello))
    })
    .with(cors);

    
    let host_address : Ipv4Addr = match env::var("HOST_ADDRESS") {
        Ok(host_string) => host_string.parse().unwrap(),
        Err(_) => Ipv4Addr::new(127, 0, 0, 1)
    };
    
    let host = IpAddr::V4(host_address);

    let port : u16 = match env::var("WARP_PORT") {
        Ok(port_string) => port_string.parse().unwrap(),
        Err(_) => 3030
    };
    
    let routes = root.or(hello);
    
    println!("\n  Server running on http://{}:{}", "localhost", port);

    warp::serve(routes)
        .run((host, port))
        .await;
}
