use std::net::{IpAddr, Ipv4Addr};

use bevy::math::VectorSpace;
use bevy::prelude::*;
use jeffy_quintet::client::certificate::CertificateVerificationMode;
use jeffy_quintet::shared::channels::ChannelType;
use jeffy_quintet::{client::*, server::*, shared::channels::ChannelsConfiguration};

use jeffy_quintet::server::certificate::CertificateRetrievalMode;
use connection::ClientEndpointConfiguration;

use crate::server_types::Message;
//use crate::{add_player_to_scene, ChildJId, JId, JMoveState, JMyCollider, JMyId, JMyPlayer, JOtherPlayers};






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

pub fn start_connection(mut client: ResMut<QuintetClient>) {
    client.open_connection(
        ClientEndpointConfiguration::from_ips(
            IpAddr::V4(Ipv4Addr::new(69, 62, 174, 8)),
            6000,
            IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            0,
        ),
        CertificateVerificationMode::SkipVerification,
        ChannelsConfiguration::from_types(vec![
            ChannelType::Unreliable,
            ChannelType::Unreliable,
            ChannelType::OrderedReliable,
            ChannelType::OrderedReliable,
        ])
        .unwrap(),
    );
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut client: ResMut<QuintetClient>,
    //mut opl: ResMut<JOtherPlayers>,
    //mut otherplayerbodies: Query<(&mut Transform, &JId, &mut InterpolationThing)>, /*...*/
    //mut animstates: Query<(&mut JMoveState, &ChildJId)>
) {
    //println!("Checking for server messages...");
    while let Ok(Some(message)) = client.connection_mut().receive_message::<Message>() {
       // println!("Finally receiing");

        match message {
            _ => {
                    
            }
            (channelid, Message::Disconnect) => {

            },
            (channelid, Message::ChestUpdate(chestloc, slotindex, slot)) => {

            },
            (channelid, Message::BlockSet(servec3, blockid)) => {

            },
            (channelid, Message::InvUpdate(slotindex, slot)) => {

            },
            (channelid, Message::ItemToYourMouse(slot)) => {

            },
            (channelid, Message::MobUpdate) => {

            },
            (channelid, Message::MobUpdateBatch) => {

            },
            (channelid, Message::MultiBlockSet(blocks)) => {

            },
            (channelid, Message::RequestWorldInfo) => {

            },
            (channelid, Message::TellYouMyID(id1, id2)) => {

            },
            (channelid, Message::YourId(id1, id2)) => {

            },
            (channelid, Message::TimeUpdate(newtime)) => {

            },
            (channelid, Message::WorldInfo(chestregbytes, pt, udmbytes, seed)) => {
                //ChestReg, Pt, Udm, Seed
            },
            (channelid, Message::PlayerUpdate(uuid, pos, rot)) => {

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
}
