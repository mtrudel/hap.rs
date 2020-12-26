fn main() {
    let mut builder = env_logger::builder();
    builder.parse_filters("libmdns=info");
    builder.init();

    let responder = libmdns::Responder::new().unwrap();
    let _svc = responder.register(
        "_hap._tcp".to_owned(),
        "HAP".to_owned(),
        5000,
        &["c#=2", "ff=0", "id=11:22:33:44:55:77", "md=hapco", "pv=1.1", "s#=1", "sf=1", "ci=2", "sh=T15I+A=="],
    );

    loop {
        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    }
}
