use std::net::{IpAddr, Ipv4Addr};

use bevy::prelude::*;
// use jeffy_quintet::client::certificate::CertificateVerificationMode;
use jeffy_quintet::shared::channels::ChannelType;
// use jeffy_quintet::{client::*, server::*, shared::channels::ChannelsConfiguration};
use jeffy_quintet::{server::*, shared::channels::ChannelsConfiguration};

use jeffy_quintet::server::certificate::CertificateRetrievalMode;
// use connection::ClientEndpointConfiguration;
// use uuid::Uuid;

use crate::server_types::*;


pub const SERVERSEED: u32 = 100;

pub fn start_listening(mut server: ResMut<QuintetServer>) {
    println!("Server starting!");
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
    mut server: ResMut<QuintetServer>,
    /*...*/
) {
    let endpoint = server.endpoint_mut();
    for client_id in endpoint.clients() {
        while let Some(message) = endpoint.try_receive_message_from::<Message>(client_id) {
            match message {
                (3, Message::Hello(100) ) => {
                    println!("Got hello 100, sending seed!");
                    endpoint.send_message_on(client_id, 3, Message::WorldRealInfo(WorldInfoYo{
                        seed: SERVERSEED
                    }));
                }
                _ => {
                    println!("Received message on {}", message.0);
                    // TODO: handle potential error


                    match endpoint.send_group_message_on(
                    endpoint.clients().iter(),//.filter(|s| **s != client_id),
                        3,
                        message.1.clone(),
                    ) {
                        Ok(_) => {
                            println!("Sent message to everyone");
                        }
                        Err(e) => {
                            println!("Err sending to everyone: {e}");
                        }
                    }

                    // if let Err(e) = endpoint.broadcast_message_on(3, message.1.clone()) {
                    //     println!("Error broadcasting message: {:?}", e);
                    // } else {
                    //     println!("Broadcasted message to all clients.");
                    // }
                    
                    

                }
                // (channelid, Message::Disconnect) => {

                // },
                // (channelid, Message::ChestUpdate(chestloc, slotindex, slot)) => {

                // },
                // (channelid, Message::BlockSet(servec3, blockid)) => {

                // },
                // (channelid, Message::InvUpdate(slotindex, slot)) => {

                // },
                // (channelid, Message::ItemToYourMouse(slot)) => {

                // },
                // (channelid, Message::MobUpdate) => {

                // },
                // (channelid, Message::MobUpdateBatch) => {

                // },
                // (channelid, Message::MultiBlockSet(blocks)) => {

                // },
                // (channelid, Message::RequestWorldInfo) => {

                // },
                // (channelid, Message::TellYouMyID(id1, id2)) => {

                // },
                // (channelid, Message::YourId(id1, id2)) => {

                // },
                // (channelid, Message::TimeUpdate(newtime)) => {

                // },
                // (channelid, Message::WorldInfo(chestregbytes, pt, udmbytes, seed)) => {
                //     //ChestReg, Pt, Udm, Seed
                // },
                // (channelid, Message::PlayerUpdate(uuid, pos, rot)) => {
                //     let uuid = Uuid::from_u64_pair(uuid.0, uuid.1);
                //     println!("Received {} {} {} {}", uuid, pos.0.x, pos.0.y, pos.0.z);
                //     // Send a message to a group of clients
                //     // if let Err(e) = endpoint.broadcast_message_on(1, message.1.clone()) {
                //     //     println!("Error broadcasting message: {:?}", e);
                //     // } else {
                //     //     println!("Broadcasted message to all clients.");
                //     // }

                //     // TODO: handle potential error
                //     let _ = endpoint.send_group_message_on(
                //         endpoint.clients().iter().filter(|s| **s != client_id),
                //         1,
                //         message.1.clone(),
                //     );

                //     /*...*/
                // }
                  // // Match on your own message types ...
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
