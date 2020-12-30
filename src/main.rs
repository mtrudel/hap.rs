use warp::Filter;

#[tokio::main]
async fn main() {
    let mut builder = env_logger::builder();
    builder.parse_filters("libmdns=info");
    builder.parse_filters("hap=info");
    builder.init();

    let responder = libmdns::Responder::new().unwrap();
    let _svc = responder.register(
        "_hap._tcp".to_owned(),
        "HAP".to_owned(),
        5000,
        &["c#=2", "ff=0", "id=11:22:33:44:55:77", "md=hapco", "pv=1.1", "s#=1", "sf=1", "ci=2", "sh=T15I+A=="],
    );

    let log = warp::log("hap");
    let routes = warp::any().map(|| "Hello, World!").with(log);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 5000))
        .await;
}
