use futures::executor::block_on;
use std::net::SocketAddr;
use std::net::IpAddr;
use std::net::Ipv4Addr;

mod tlv_parser;
mod pair_setup;

pub fn start(server: &crate::AccessoryServer) {
    let mut app = tide::new();
    app.at("/pair-setup").post(pair_setup);
    // TODO -- eat the async here for now until I get my arms around the rest of 
    // the language more
    let _ = block_on(app.listen(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), server.port)));
}

async fn pair_setup(mut req: tide::Request<()>) -> tide::Result {
    // TODO should be returning 4xx here
    let body : Vec<u8> = req.body_bytes().await.unwrap();
    let params = tlv_parser::parse(&body);

    let _result = match params.get(&pair_setup::kTLVType_State) {
        Some(x) if x[0] == 1 => pair_setup::m1(&params),
        _ => panic!("Unknown state")
    };

    // TODO should be returning result here (need encoder)
    Ok("TODO".into())
}
