pub fn start(accessory_server: &crate::AccessoryServer) -> libmdns::Service {
    let responder = libmdns::Responder::new().unwrap();
    responder.register(
        "_hap._tcp".to_owned(),
        accessory_server.name.to_owned(),
        accessory_server.port,
        &[&format!("c#={}", accessory_server.config_number), "ff=0", &format!("id={}", accessory_server.identifier), "md=hapco", "pv=1.1", "s#=1", "sf=1", "ci=2", "sh=T15I+A=="],
    )
}
