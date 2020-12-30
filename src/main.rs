fn main() {
    let mut builder = env_logger::builder();
    builder.parse_filters("libmdns=info");
    builder.parse_filters("hap=info");
    builder.init();

    let server = haprs::AccessoryServer  {
        name: "HAPrs".to_owned(),
        identifier: "11:22:33:44:55:66".to_owned(),
        accessory_type: 5,
        port: 5000,
        config_number: 1
    };

    haprs::start(server);
}
