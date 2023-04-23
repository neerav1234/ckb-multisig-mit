use secp256k1::hashes::sha256;
use secp256k1::{Message, Secp256k1};
// use std::any::type_name;

// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

#[derive(Debug)]
enum Error {
    VectorLengthNotSame,
}

fn verify_signature(
    message: &Message,
    signature: &secp256k1::ecdsa::Signature,
    public_key: &secp256k1::PublicKey,
) -> bool {
    let secp = Secp256k1::new();
    secp.verify_ecdsa(message, signature, public_key).is_ok()
}

fn validate_multisig(
    threshold: u32,
    messages: &[Message],
    signatures: &[secp256k1::ecdsa::Signature],
    public_keys: &[secp256k1::PublicKey],
) -> Result<bool, Error> {
    if messages.len() != signatures.len() {
        return Err(Error::VectorLengthNotSame);
    }
    if messages.len() != public_keys.len() {
        return Err(Error::VectorLengthNotSame);
    } else {
        let mut valid_sig = 0;
        for i in 0..messages.len() {
            let pass = verify_signature(&messages[i], &signatures[i], &public_keys[i]);
            if pass {
                valid_sig += 1;
            }
        }
        return Ok(valid_sig >= threshold);
    }
}

fn main() {
    let threshold = 2;

    let secp = Secp256k1::new();

    let (secret_key1, public_key1) = secp.generate_keypair(&mut rand::thread_rng());
    let (secret_key2, public_key2) = secp.generate_keypair(&mut rand::thread_rng());
    let message = Message::from_hashed_data::<sha256::Hash>("Hello World!".as_bytes());
    let sig1 = secp.sign_ecdsa(&message, &secret_key1);
    let sig2 = secp.sign_ecdsa(&message, &secret_key2);

    let messages = vec![message, message];
    let signatures = vec![sig1, sig2];
    let public_keys = vec![public_key1, public_key2];

    let result: Result<bool, Error> =
        validate_multisig(threshold, &messages, &signatures, &public_keys);
    match result {
        Ok(value) => println!("The result is: {}", value),
        Err(e) => println!("An error occurred: {:?}", e),
    }

    // let sig = secp.sign_ecdsa(&message, &secret_key);
    // let pass = verify_signature(&message, &sig, &public_key);
    // println!("signature: {:?}", sig);
    // println!("public key: {:?}", public_key);
    // println!("pass: {:?}", pass);
    // println!("message type: {}", type_of(&message));
    // println!("signature type: {}", type_of(&sig));
}
