use futures::executor::block_on;

pub fn start(_server: &crate::AccessoryServer) {
    let mut app = tide::new();
    app.at("/pair-setup").post(pair_setup);
    // TODO -- eat the async here for now until I get my arms around the rest of 
    // the language more
    let _ = block_on(app.listen("127.0.0.1:5000"));
}

async fn pair_setup(_req: tide::Request<()>) -> tide::Result {
    Ok("Hello".into())
}
