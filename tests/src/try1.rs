use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context, ckb_types::prelude::*};
use std::collections::HashSet;

const MAX_CYCLES: u64 = 10_000_000;

#[test]
fn test_multisig() {
    // Arrange
    let mut context = Context::default();
    let multisig_script = context
        .build_script(&ALWAYS_SUCCESS, vec![1, 2, 3])
        .expect("building script");

    let input1 = CellInput::new(
        CellOutPoint::new(context.create_cell(
            CellOutput::new_builder()
                .capacity(100_000_000u64.pack())
                .lock(multisig_script.clone())
                .build(),
        )),
        0,
    );

    let output1 = CellOutput::new_builder()
        .capacity(100_000_000u64.pack())
        .lock(multisig_script.clone())
        .build();

    let tx = TransactionBuilder::default()
        .input(input1)
        .output(output1)
        .build();

    // Act
    let result = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("verify multisig contract should success");

    // Assert
    assert!(result.is_ok());

    let inputs = vec![&input1];
    let outputs = vec![&output1];

    let witness = multisig_script.clone().into_witness();
    let signature = ckb_std::secp256k1_sign(
        &ckb_std::load_tx_hash().expect("load tx hash"),
        0,
        &ckb_std::load_privkey().expect("load privkey"),
        None,
    )
    .expect("signing should succeed");
    let witness_args = witness
        .as_builder()
        .push(Bytes::from(signature.to_vec()).pack())
        .build();

    let tx = TransactionBuilder::default()
        .input(input1.clone())
        .output(output1.clone())
        .witness(witness_args.as_bytes().pack())
        .build();

    // Act
    let result = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("verify multisig contract should success");

    // Assert
    assert!(result.is_ok());
}
