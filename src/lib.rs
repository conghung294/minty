#![no_std]
#![allow(unexpected_cfgs)]
use pinocchio::{no_allocator, nostd_panic_handler, program_entrypoint};
use processor::process_instruction;

pub mod error;
pub mod instructions;
pub mod processor;
pub mod state;

pinocchio_pubkey::declare_id!("AM2HxttPeNu6RM3RcUHU8xFWYLKXsEg7BjahB15GAKrB");

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();
