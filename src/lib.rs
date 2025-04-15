#![no_std]
#![allow(unexpected_cfgs)]

use pinocchio_pubkey::declare_id;

mod constants;
mod entrypoint;
mod errors;
mod instructions;
mod state;

declare_id!("74jRBM9U3qGHzxS59HxHAGuNYJBPkZYG5rBpSGWHdrzX");
