/*!
Provides integration for [`bevy_replicon`](https://docs.rs/bevy_replicon) for [`jeffy_quintet`](https://docs.rs/jeffy_quintet).
*/

use bevy::{app::PluginGroupBuilder, prelude::*};
use jeffy_quintet::shared::channels::{ChannelType, ChannelsConfiguration};
use bevy_replicon::prelude::*;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "client")]
use client::RepliconQuintetClientPlugin;
#[cfg(feature = "server")]
use server::RepliconQuintetServerPlugin;

pub struct RepliconQuintetPlugins;

impl PluginGroup for RepliconQuintetPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        #[cfg(feature = "server")]
        {
            group = group.add(RepliconQuintetServerPlugin);
        }

        #[cfg(feature = "client")]
        {
            group = group.add(RepliconQuintetClientPlugin);
        }

        group
    }
}

pub trait ChannelsConfigurationExt {
    /// Returns server channel configs that can be used to create [`ConnectionConfig`](renet::ConnectionConfig).
    fn get_server_configs(&self) -> ChannelsConfiguration;

    /// Same as [`RenetChannelsExt::get_server_configs`], but for clients.
    fn get_client_configs(&self) -> ChannelsConfiguration;
}
impl ChannelsConfigurationExt for RepliconChannels {
    fn get_server_configs(&self) -> ChannelsConfiguration {
        create_configs(self.server_channels(), self.default_max_bytes)
    }

    fn get_client_configs(&self) -> ChannelsConfiguration {
        create_configs(self.client_channels(), self.default_max_bytes)
    }
}

/// Converts replicon channels into Quintet channel configs.
fn create_configs(
    channels: &[RepliconChannel],
    _default_max_bytes: usize,
) -> ChannelsConfiguration {
    let mut Quintet_channels = ChannelsConfiguration::new();
    for channel in channels.iter() {
        Quintet_channels.add(match channel.kind {
            ChannelKind::Unreliable => ChannelType::Unreliable,
            ChannelKind::Unordered => ChannelType::UnorderedReliable,
            ChannelKind::Ordered => ChannelType::OrderedReliable,
        });
    }
    Quintet_channels
}
