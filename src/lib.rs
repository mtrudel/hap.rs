mod discovery;
mod web;

pub struct AccessoryServer {
    pub name: String,
    pub identifier: String,
    pub accessory_type: u16,
    pub port: u16,
    pub config_number: u16
}

pub fn start(server: AccessoryServer) {
    discovery::start(&server);
    web::start(&server);
}
