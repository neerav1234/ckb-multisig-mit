#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use alloc::vec::Vec;
use core::result::Result;

use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    debug, default_alloc,
    error::SysError,
    high_level::{load_script, load_witness_args},
    secp256k1,
};

use crate::error::Error;

mod error;

// Constants
const THRESHOLD_BYTES: usize = 1;
const PUBKEY_HASH_BYTES: usize = 20;

entry!(entry);
default_alloc!();

fn entry() -> i8 {
    match main() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}

fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();
    let threshold = args[0];
    let num_pubkey_hashes = (args.len() - THRESHOLD_BYTES) / PUBKEY_HASH_BYTES;

    if threshold as usize > num_pubkey_hashes {
        return Err(Error::Threshold);
    }

    let mut pubkey_hashes = Vec::with_capacity(num_pubkey_hashes);
    for i in 0..num_pubkey_hashes {
        let start = THRESHOLD_BYTES + i * PUBKEY_HASH_BYTES;
        let end = start + PUBKEY_HASH_BYTES;
        pubkey_hashes.push(args.slice(start, end));
    }

    let mut sigs_count = 0;

    for i in 0.. {
        let witness_args = match load_witness_args(i, Source::Input) {
            Ok(witness_args) => witness_args,
            Err(SysError::ItemMissing) => break,
            Err(err) => return Err(err.into()),
        };

        let lock = witness_args.lock().to_opt().ok_or(Error::WitnessLock)?;

        if lock.len() != 65 {
            return Err(Error::SignatureSize);
        }

        let mut signature_data = [0u8; 64];
        signature_data.copy_from_slice(&lock[..64]);
        let signature = secp256k1::Signature::parse(&signature_data);

        let recid = secp256k1::RecoveryId::parse(lock[64]).map_err(|_| Error::InvalidSignature)?;

        let mut hasher = blake2b_simd::Params::new().hash_length(32).to_state();
        hasher.update(&witness_args.as_bytes());
        let message = hasher.finalize();

        let pubkey = secp256k1::recover(
            &secp256k1::Message::parse(message.as_array_ref().unwrap()),
            &signature,
            &recid,
        )
        .map_err(|_| Error::InvalidSignature)?;

        let pubkey_hash = {
            let mut hasher = blake2b_simd::Params::new().hash_length(20).to_state();
            hasher.update(&pubkey.serialize_compressed());
            hasher.finalize()
        };

        if pubkey_hashes.iter().any(|hash| &pubkey_hash == hash) {
            sigs_count += 1;
            if sigs_count >= threshold as usize {
                return Ok(());
            }
        }
    }

    Err(Error::ThresholdNotMet)
}
