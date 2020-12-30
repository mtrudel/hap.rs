use warp::Filter;

#[tokio::main]
pub async fn start(server: &crate::AccessoryServer) {
    let log = warp::log("hap");
    let routes = warp::any().map(|| "Hello, World!").with(log);

    warp::serve(routes)
        .run(([0, 0, 0, 0], server.port))
        .await;
}
