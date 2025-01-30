use std::{net::Ipv4Addr, thread, time::Duration};

fn main() {
    let (server, server_callbacks) = steamworks::Server::init(
        Ipv4Addr::LOCALHOST,
        27015,
        27016,
        steamworks::ServerMode::Authentication,
        "1.0.0.0",
    )
    .unwrap();

    server.set_product("Spacewar");
    server.set_mod_dir("Spacewar");
    server.set_map_name("TestMap");
    server.set_game_description("Description");
    server.set_max_players(16);
    server.set_server_name("TestServer");
    server.set_dedicated_server(true);
    server.enable_heartbeats(true);
    server.log_on_anonymous();

    let utils = server_callbacks.networking_utils();

    utils.init_relay_network_access();
    
    loop {
        server_callbacks.run_callbacks();
        let status = utils.detailed_relay_network_status();
        println!(
            "status: {:?}, network_config: {:?}, any_relay: {:?}, message: {}",
            status.availability(),
            status.network_config(),
            status.any_relay(),
            status.debugging_message()
        );
        thread::sleep(Duration::from_millis(100));
    }
}