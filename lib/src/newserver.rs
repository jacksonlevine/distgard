use std::net::{IpAddr, Ipv4Addr};

use bevy::prelude::*;
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::shared::channels::ChannelType;
use bevy_quinnet::{client::*, server::*, shared::channels::ChannelsConfiguration};

use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use connection::ClientEndpointConfiguration;

use crate::server_types::NewMessage;

pub fn start_listening(mut server: ResMut<QuinnetServer>) {
    server
        .start_endpoint(
            ServerEndpointConfiguration::from_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 6000),
            CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: String::from("Test Server"),
            },
            ChannelsConfiguration::from_types(vec![
                ChannelType::Unreliable,
                ChannelType::Unreliable,
                ChannelType::OrderedReliable,
                ChannelType::OrderedReliable,
            ])
            .unwrap(),
        )
        .unwrap();
}

pub fn handle_client_messages(
    mut server: ResMut<QuinnetServer>,
    /*...*/
) {
    let mut endpoint = server.endpoint_mut();
    for client_id in endpoint.clients() {
        while let Some(message) = endpoint.try_receive_message_from::<JMessage>(client_id) {
            match message {
                (channelid, JMessage::PlayerUpdate(uuid, trans, rot, scale, moving)) => {
                    println!("Received {} {}", uuid, trans);
                    // Send a message to a group of clients
                    // if let Err(e) = endpoint.broadcast_message_on(1, message.1.clone()) {
                    //     println!("Error broadcasting message: {:?}", e);
                    // } else {
                    //     println!("Broadcasted message to all clients.");
                    // }

                    endpoint.send_group_message_on(
                        endpoint.clients().iter().filter(|s| **s != client_id),
                        1,
                        message.1.clone(),
                    );

                    /*...*/
                } // // Match on your own message types ...
                  //You can also use endpoint.broadcast_message, which will send a message to all connected clients.
                  // ClientMessage::Join { username} => {
                  //     // Send a messsage to 1 client
                  //     endpoint.send_message(client_id, ServerMessage::InitClient {/*...*/}).unwrap();
                  //     /*...*/
                  // }
                  // ClientMessage::Disconnect { } => {
                  //     // Disconnect a client
                  //     endpoint.disconnect_client(client_id);
                  //     /*...*/
                  // }
                  // ClientMessage::ChatMessage { message } => {
                  //     // Send a message to a group of clients
                  //     endpoint.send_group_message(
                  //             client_group, // Iterator of ClientId
                  //             ServerMessage::ChatMessage {/*...*/}
                  //         )
                  //         .unwrap();
                  //     /*...*/
                  // }
            }
        }
    }
}
