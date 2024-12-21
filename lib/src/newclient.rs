// use std::net::{IpAddr, Ipv4Addr};
// use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

// use bevy::math::VectorSpace;
use bevy::prelude::*;
use jeffy_quintet::client::certificate::CertificateVerificationMode;
use jeffy_quintet::shared::channels::ChannelType;
use jeffy_quintet::{client::*, shared::channels::ChannelsConfiguration};
// use jeffy_quintet::{client::*, server::*, shared::channels::ChannelsConfiguration};

// use jeffy_quintet::server::certificate::CertificateRetrievalMode;
use connection::ClientEndpointConfiguration;

use crate::game::{CHUNKSYS, CURRSEED, RECEIVED_WORLD};
use crate::server_types::Message;
//use crate::{add_player_to_scene, ChildJId, JId, JMoveState, JMyCollider, JMyId, JMyPlayer, JOtherPlayers};



static mut GLOBAL_CLIENT: *mut QuintetClient = std::ptr::null_mut();


#[derive(Component)]
pub struct InterpolationThing {
    pub lastpos: Transform,
    pub newpos: Transform,
    pub t: f32
}

impl InterpolationThing {
    pub fn new(pos: Transform) -> Self {
        Self {
            lastpos: pos,
            newpos: pos,
            t: 0.0
        }
    }
    pub fn update(&mut self, pos: Transform) {
        self.t = 0.0;
        self.lastpos = self.newpos;
        self.newpos = pos;
    }
}





#[derive(Resource, Default)]
pub struct PlayerUpdateTimer(pub Timer);

// pub fn send_my_location(
//     myid: Res<JMyId>,
//     mut client: ResMut<QuintetClient>,
//     time: Res<Time>,
//     mut timer: ResMut<PlayerUpdateTimer>,
//     query: Query<&Transform, With<JMyCollider>>,
//     query2: Query<&JMoveState, With<JMyPlayer>>,
// ) {
//     if timer.0.tick(time.delta()).just_finished() {
//         let me = query.single();
//         for mov in query2.iter() {
//             (*client).connection_mut().send_message_on(
//                 0,
//                 Message::PlayerUpdate(myid.uuid, me.translation, me.rotation, me.scale, mov.moving),
//             );
//         }
        
//     }
// }
pub static mut THEENTEREDADDRESS: String = String::new();
pub static mut ADDRESSENTERED: AtomicBool = AtomicBool::new(false);

pub fn start_connection(mut client: ResMut<QuintetClient>) {
    // unsafe { GLOBAL_CLIENT = &mut *client as *mut QuintetClient };
    // while !unsafe { ADDRESSENTERED.load(Ordering::Relaxed) } {
    //     thread::sleep(Duration::from_millis(500));
    // }

    // let address = unsafe { THEENTEREDADDRESS.clone() }; // Remove any trailing newline characters
    println!("Starting connection0");
    // handle potential error?
    let _ = client.open_connection(
        ClientEndpointConfiguration::from_strings(
                             "127.0.0.1:6000",
                             "0.0.0.0:0"
                         ).unwrap(),
        CertificateVerificationMode::SkipVerification,
        ChannelsConfiguration::from_types(vec![
            ChannelType::Unreliable,
            ChannelType::Unreliable,
            ChannelType::OrderedReliable,
            ChannelType::OrderedReliable,
        ])
        .unwrap(),
    );
    println!("Starting connection1");

    match client.connection_mut().send_message_on(3, Message::Hello(100)) {
        Ok(_) => {
            println!("Sent bytes!");
        },
        Err(_) => {
            println!("Failed to send");
        }
    }
    
}

// pub fn update_otherplayers_interps(
//     mut otherplayerbodies: Query<(&mut Transform, &JId, &mut InterpolationThing)>,
//     time: Res<Time>
// ) {
//     for (mut transform, id, mut interp) in otherplayerbodies.iter_mut() {
//         interp.t = (interp.t + time.delta_seconds() * 4.0).min(1.0);
//         //println!("INterep: {}", interp.t);
//         (*transform).translation = interp.lastpos.translation.lerp(interp.newpos.translation, interp.t);
//         (*transform).rotation = interp.lastpos.rotation.slerp(interp.newpos.rotation, interp.t);
//         (*transform).scale = Vec3::ONE;
 
//     }
// }

pub fn handle_server_messages(
    _commands: Commands,
    mut client: ResMut<QuintetClient>,
    //mut opl: ResMut<JOtherPlayers>,
    //mut otherplayerbodies: Query<(&mut Transform, &JId, &mut InterpolationThing)>, /*...*/
    //mut animstates: Query<(&mut JMoveState, &ChildJId)>
) {
    //client.connection_mut().send_message_on(3, Message::Hello(unsafe { CURRSEED.load(Ordering::Relaxed) }));

    println!("Checking for server messages...");
    loop {
        match client.connection_mut().receive_message::<Message>() {
            Ok(None) => {
                //println!("none");
            }
            Ok(Some(message)) => {
                //println!("Finally receiing");
    
                match message {
        
                    (channelId, Message::Hello(seed)) => {
                        println!("Received: {}", seed)
                    }
                    (channelId, Message::WorldRealInfo(info)) => {
                        unsafe {
                            if let Some(ref chunksys) = CHUNKSYS {
                                println!("Received seed {} from server.", info.seed);
                                chunksys.change_seed(info.seed);
                            } else {
                                panic!("CHUNKSYS is not initialized when received seed from server");
                            }

                            RECEIVED_WORLD.store(true, Ordering::Relaxed);
                        }
                        
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
        
                    // }
                    _ => {
                            println!("Received something else");
                    }
        
        
        
                    // Match on your own message types ...
                    //(channelid, Message::PlayerUpdate(uuid, trans, rot, scale, mov)) => {
                       // println!("Got on {}: {}", channelid, uuid);
        
                        // if !opl.list.contains(&uuid) {
                        //     opl.list.insert(uuid);
        
                        //     add_player_to_scene(&mut commands, &asset_server, trans, uuid);
                        // } else {
                        //     for (mut transform, id, mut interp) in otherplayerbodies.iter_mut() {
                        //         let newtrans = Transform {
                        //             translation: trans,
                        //             rotation: rot,
                        //             scale,
                        //         };
        
                                
                                
                                
                        //         if id.uuid == uuid {
                        //             (*interp).update(newtrans);
                        //             //(*transform) = newtrans; was causing glitch movement
                        //         }
                        //     }
                        //     for (mut movestate, jid) in animstates.iter_mut() {
                        //         if jid.uuid == uuid {
                        //             movestate.moving = mov;
                        //         }
                        //     }
                        // }
                    //} //ServerMessage::ClientConnected { client_id, username} => {/*...*/}
                      //ServerMessage::ClientDisconnected { client_id } => {/*...*/}
                      //ServerMessage::ChatMessage { client_id, message } => {/*...*/}
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
    

}
