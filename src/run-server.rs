#![feature(more_float_constants)]

mod common;
mod server;

use std::time::SystemTime;
use std::net::UdpSocket;

use bevy::{log::LogPlugin, prelude::*};
use bevy_easings::*;
use bevy_renet::{
    renet::{
        transport::{ServerAuthentication, ServerConfig},
        ConnectionConfig, RenetServer,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use renet::{
    transport::{NetcodeServerTransport, NetcodeTransportError}, 
    DefaultChannel,
};

use common::{
    message::*,
    components::*,
    resources::map::*,
    systems::physics::*,
};
use server::{
    resources::{ *,
        terrain::*,
    },
    systems::{
        actor::*,
        input::*,
        renet::*,
    },
};

const PROTOCOL_ID: u64 = 7;

fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
    for e in renet_error.read() {
        panic!("{}", e);
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        LogPlugin {
            level: bevy::log::Level::TRACE,
            filter:  "wgpu=error,bevy=warn,".to_owned()
                    +"server=trace,"
                    ,
            custom_layer: |_| None,
        },
        TransformPlugin,
        RenetServerPlugin,
        NetcodeServerPlugin,
        EasingsPlugin,
    ));

    app.add_event::<Do>();
    app.add_event::<Try>();

    app.add_systems(Update, (
        panic_on_error_system,
        send_do,
        do_input,
        do_manage_connections,
        do_move,
        try_discover,
        try_input,
        try_move,
        update_headings,
        update_offsets,
        write_try,
    ));

    app.add_systems(FixedUpdate, (
        generate_input,
    ));

    let (server, transport) = new_renet_server();
    app.init_resource::<Lobby>();
    app.init_resource::<InputQueues>();
    app.init_resource::<Map>();
    app.init_resource::<Terrain>();
    app.insert_resource(server);
    app.insert_resource(transport);

    app.run();
}
