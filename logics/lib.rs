#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub mod asset_pool;
pub mod factory;
pub mod manager;
pub mod math;
pub mod rate_strategy;
pub mod registry;
pub mod service;
pub mod traits;
pub mod ui_data_providers;
