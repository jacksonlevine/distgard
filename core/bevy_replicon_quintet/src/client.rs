use bevy::{
    app::{App, Plugin, PostUpdate, PreUpdate},
    prelude::{IntoSystemConfigs, IntoSystemSetConfigs, Res, ResMut},
};
use jeffy_quintet::{
    client::{QuintetClient, QuintetClientPlugin},
    shared::QuintetSyncUpdate,
};
use bevy_replicon::{
    client::ClientSet,
    core::ClientId,
    prelude::{RepliconClient, RepliconClientStatus},
};

pub struct RepliconQuintetClientPlugin;

impl Plugin for RepliconQuintetClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuintetClientPlugin::default())
            .configure_sets(
                PreUpdate,
                ClientSet::ReceivePackets.after(QuintetSyncUpdate),
            )
            .add_systems(
                PreUpdate,
                (
                    Self::set_connecting.run_if(jeffy_quintet::client::client_connecting),
                    Self::set_disconnected.run_if(jeffy_quintet::client::client_just_disconnected),
                    Self::set_connected.run_if(jeffy_quintet::client::client_just_connected),
                    Self::receive_packets.run_if(jeffy_quintet::client::client_connected),
                )
                    .chain()
                    .in_set(ClientSet::ReceivePackets),
            )
            .add_systems(
                PostUpdate,
                Self::send_packets
                    .in_set(ClientSet::SendPackets)
                    .run_if(jeffy_quintet::client::client_connected),
            );
    }
}

impl RepliconQuintetClientPlugin {
    fn set_disconnected(mut client: ResMut<RepliconClient>) {
        client.set_status(RepliconClientStatus::Disconnected);
    }

    fn set_connecting(mut client: ResMut<RepliconClient>) {
        client.set_status(RepliconClientStatus::Connecting);
    }

    fn set_connected(mut client: ResMut<RepliconClient>, Quintet_client: Res<QuintetClient>) {
        let client_id = match Quintet_client.connection().client_id() {
            Some(id) => Some(ClientId::new(id)),
            None => None,
        };

        client.set_status(RepliconClientStatus::Connected { client_id });
    }

    fn receive_packets(
        mut Quintet_client: ResMut<QuintetClient>,
        mut replicon_client: ResMut<RepliconClient>,
    ) {
        let Some(connection) = Quintet_client.get_connection_mut() else {
            return;
        };

        while let Some((channel_id, message)) = connection.try_receive_payload() {
            replicon_client.insert_received(channel_id, message);
        }
    }

    fn send_packets(
        Quintet_client: ResMut<QuintetClient>,
        mut replicon_client: ResMut<RepliconClient>,
    ) {
        let Some(connection) = Quintet_client.get_connection() else {
            return;
        };
        for (channel_id, message) in replicon_client.drain_sent() {
            connection.try_send_payload_on(channel_id, message);
        }
    }
}
