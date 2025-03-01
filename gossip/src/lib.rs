#![cfg_attr(RUSTC_WITH_SPECIALIZATION, feature(min_specialization))]
#![allow(clippy::integer_arithmetic)]

pub mod cluster_info;
pub mod cluster_info_metrics;
pub mod crds;
pub mod crds_entry;
pub mod crds_gossip;
pub mod crds_gossip_error;
pub mod crds_gossip_pull;
pub mod crds_gossip_push;
pub mod crds_shards;
pub mod crds_value;
mod deprecated;
pub mod duplicate_shred;
pub mod epoch_slots;
pub mod gossip_error;
pub mod gossip_service;
#[macro_use]
pub mod legacy_contact_info;
pub mod ping_pong;
mod received_cache;
pub mod weighted_shuffle;

#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate matches;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate solana_frozen_abi_macro;

#[macro_use]
extern crate solana_metrics;
