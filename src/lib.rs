mod discovery;

use warp::Filter;

pub struct AccessoryServer {
    pub name: String,
    pub identifier: String,
    pub accessory_type: u16,
    pub port: u16,
    pub config_number: u16
}

#[tokio::main]
pub async fn start(server: AccessoryServer) {
    discovery::start(server);

    let log = warp::log("hap");
    let routes = warp::any().map(|| "Hello, World!").with(log);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 5000))
        .await;
}
