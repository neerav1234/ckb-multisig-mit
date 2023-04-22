#![no_std]
#![no_main]

use ckb_std::{
    ckb_constants::Source,
    ckb_types::{packed::*, prelude::*},
    debug,
    high_level::{
        load_cell_capacity, load_cell_data, load_cell_lock_hash, load_script_hash,
        load_witness_args, QueryIter,
    },
};
use core::result::Result;

const N: usize = 2; // Number of required signatures
const PUBKEYS: [&str; 2] = ["public_key_1", "public_key_2"]; // List of public keys for signers

fn main() -> Result<(), i8> {
    let mut sig_count = 0;
    let mut used_keys = vec![];

    // Verify that the script code hash is correct
    let script_hash = load_script_hash()?;
    let expected_code_hash: [u8; 32] = [2u8; 32];
    if script_hash.as_slice() != expected_code_hash.as_ref() {
        return Err(-1);
    }

    // Verify that the witness argument contains the correct number of signatures
    let witness_args = load_witness_args(0, Source::GroupInput)?;
    if witness_args.input_type().is_none() {
        return Err(-2);
    }
    let witness_len = witness_args.input_type().unwrap().raw_data().len();
    if witness_len % 65 != 0 {
        return Err(-3);
    }
    sig_count = witness_len / 65;

    // Verify that the lock arguments match the public keys of the signers
    for i in 0..sig_count {
        let witness_data = witness_args.input_type().unwrap().raw_data();
        let witness_bytes = witness_data
            .slice(i * 65, (i + 1) * 65)
            .raw_data()
            .to_vec();
        let mut pubkey_bytes = [0u8; 65];
        pubkey_bytes.copy_from_slice(&witness_bytes[..]);
        let pubkey = secp256k1::PublicKey::from_slice(&pubkey_bytes)?;
        let lock_hash = load_cell_lock_hash(i, Source::GroupInput)?;
        if lock_hash.as_slice() != pubkey.serialize_uncompressed()[1..].as_ref() {
            return Err(-4);
        }
        let pubkey_hex = hex::encode(&pubkey.serialize());
        if !PUBKEYS.contains(&pubkey_hex.as_str()) || used_keys.contains(&pubkey_hex) {
            return Err(-5);
        }
        used_keys.push(pubkey_hex);
    }

    // Verify that the required number of signatures have been provided
    if sig_count < N {
        return Err(-6);
    }

    // Verify that the capacity has not been changed
    for i in 0..sig_count {
        let cell_capacity = load_cell_capacity(i, Source::GroupInput)?;
        if cell_capacity != 0 {
            return Err(-7);
        }
    }

    Ok(())
}
