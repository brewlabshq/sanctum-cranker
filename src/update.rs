#[allow(unused_imports)]
use borsh::BorshDeserialize;
use sanctum_solana_cli_utils::TxSendMode;
use sanctum_spl_stake_pool_lib::account_resolvers::{
    CleanupRemovedValidatorEntries, UpdateStakePoolBalance, UpdateValidatorListBalance,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_readonly_account::keyed::Keyed;
use solana_sdk::{account::Account, pubkey::Pubkey, signer::Signer};
use spl_stake_pool_interface::{
    cleanup_removed_validator_entries_ix_with_program_id,
    update_stake_pool_balance_ix_with_program_id, UpdateValidatorListBalanceIxArgs,
    ValidatorStakeInfo,
};

use crate::tx_utils::{handle_tx_full, with_auto_cb_ixs};

const MAX_VALIDATORS_TO_UPDATE_PER_TX: usize = 11;

pub struct UpdatePoolArgs<'a> {
    pub rpc: &'a RpcClient,
    pub send_mode: TxSendMode,
    pub payer: &'a (dyn Signer + 'static),
    pub program_id: Pubkey,
    pub stake_pool: Keyed<&'a Account>,
    pub validator_list_entries: &'a [ValidatorStakeInfo],
    pub fee_limit_cb: u64,
}

// ignores entries already updated for this epoch
pub async fn update_pool(
    UpdatePoolArgs {
        rpc,
        send_mode,
        payer,
        program_id,
        stake_pool,
        validator_list_entries,
        fee_limit_cb,
    }: UpdatePoolArgs<'_>,
) {
    eprintln!("Updating pool");

    // Update validator list:
    let uvlb = UpdateValidatorListBalance { stake_pool };
    // just do the validator list sequentially
    for (i, chunk) in validator_list_entries
        .chunks(MAX_VALIDATORS_TO_UPDATE_PER_TX)
        .enumerate()
    {
        let start_index = i * MAX_VALIDATORS_TO_UPDATE_PER_TX;
        let ixs = vec![uvlb
            .full_ix_from_validator_slice(
                program_id,
                chunk,
                UpdateValidatorListBalanceIxArgs {
                    start_index: start_index.try_into().unwrap(),
                    no_merge: false,
                },
            )
            .unwrap()];
        let ixs = with_auto_cb_ixs(rpc, &payer.pubkey(), ixs, &[], fee_limit_cb).await;

        eprintln!(
            "Updating validator list [{}..{}]",
            start_index,
            std::cmp::min(
                start_index + MAX_VALIDATORS_TO_UPDATE_PER_TX,
                validator_list_entries.len()
            )
        );
        handle_tx_full(rpc, send_mode, &ixs, &[], &mut [payer]).await;
    }

    // Update pool:
    let final_ixs = vec![
        update_stake_pool_balance_ix_with_program_id(
            program_id,
            UpdateStakePoolBalance { stake_pool }
                .resolve_for_prog(&program_id)
                .unwrap(),
        )
        .unwrap(),
        cleanup_removed_validator_entries_ix_with_program_id(
            program_id,
            CleanupRemovedValidatorEntries { stake_pool }
                .resolve()
                .unwrap(),
        )
        .unwrap(),
    ];

    let final_ixs = with_auto_cb_ixs(rpc, &payer.pubkey(), final_ixs, &[], fee_limit_cb).await;

    eprintln!("Sending final update tx");
    handle_tx_full(rpc, send_mode, &final_ixs, &[], &mut [payer]).await;
}
