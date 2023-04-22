#[cfg(test)]
mod tests {
    use super::*;
    use ckb_std::{
        ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context},
        ckb_types::{
            bytes::Bytes,
            core::{TransactionBuilder, TransactionView},
            packed::{CellDep, CellInput, CellOutput},
            prelude::*,
        },
    };
    use ckb_tool::ckb_error::assert_error_eq;
    use ckb_tool::ckb_script::ScriptError;

    const MAX_CYCLES: u64 = 100_0000;

    fn build_test_context(
        threshold: u8,
        pubkey_hashes: Vec<Bytes>,
        witnesses: Vec<Bytes>,
    ) -> (Context, TransactionView) {
        let mut context = Context::default();
        let multisig_bin: Bytes = Loader::default().load_binary("multisig");
        let multisig_out_point = context.deploy_contract(multisig_bin);
        let always_success_out_point = context.deploy_contract(ALWAYS_SUCCESS.clone());

        let lock_script = context
            .build_script(&always_success_out_point, Default::default())
            .expect("script");
        let lock_script_dep = CellDep::new_builder()
            .out_point(always_success_out_point)
            .build();

        let multisig_script_args = {
            let mut args = vec![threshold];
            for pubkey_hash in &pubkey_hashes {
                args.extend_from_slice(pubkey_hash);
            }
            args.into()
        };

        let multisig_script = context
            .build_script(&multisig_out_point, multisig_script_args)
            .expect("script");
        let multisig_script_dep = CellDep::new_builder().out_point(multisig_out_point).build();

        let input_out_point = context.create_cell(
            CellOutput::new_builder()
                .capacity(1000u64.pack())
                .lock(multisig_script.clone())
                .build(),
            Bytes::new(),
        );

        let input = CellInput::new_builder()
            .previous_output(input_out_point)
            .build();

        let output = CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build();

        let tx = TransactionBuilder::default()
            .input(input)
            .output(output)
            .output_data(Default::default())
            .cell_dep(lock_script_dep)
            .cell_dep(multisig_script_dep)
            .witnesses(witnesses.pack())
            .build();

        (context, tx)
    }

    #[test]
    fn test_spend_multisig_cell_with_valid_signatures() {
        let threshold = 2;
        let pubkey_hashes = vec![
            Bytes::from(&hex::decode("pubkey_hash_1").unwrap()[..]),
            Bytes::from(&hex::decode("pubkey_hash_2").unwrap()[..]),
            Bytes::from(&hex::decode("pubkey_hash_3").unwrap()[..]),
        ];

        let valid_witnesses = vec![
            Bytes::from(&hex::decode("valid_signature_1").unwrap()[..]),
            Bytes::from(&hex::decode("valid_signature_2").unwrap()[..]),
        ];

        let (mut context, tx) = build_test_context(threshold, pubkey_hashes, valid_witnesses);
        let tx = context.complete_tx(tx);

        let cycles = context
            .verify_tx(&tx, MAX_CYCLES)
            .expect("spend multisig cell with valid signatures should pass");
        println!("cycles: {}", cycles);
    }
}
