use std::str::FromStr;

use borsh::BorshDeserialize;

use crate::config::ClankerConfig;
use crate::update::{update_pool, UpdatePoolArgs};
use sanctum_solana_cli_utils::TxSendMode;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_readonly_account::keyed::Keyed;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair, signer::Signer, sysvar,
};
use spl_stake_pool_interface::{StakePool, ValidatorList};

pub async fn update(pool: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = ClankerConfig::get_config();

    let rpc = RpcClient::new_with_commitment(config.rpc_url, CommitmentConfig::confirmed());
    let pool = Pubkey::from_str(pool).unwrap();

    let payer = Keypair::from_base58_string(&config.payer_private_key);
    let payer: &dyn Signer = &payer;

    let mut fetched = rpc
        .get_multiple_accounts(&[pool, sysvar::clock::ID])
        .await
        .unwrap();

    let _ = fetched.pop().unwrap().unwrap();
    let stake_pool_acc = fetched.pop().unwrap().unwrap();

    let program_id = stake_pool_acc.owner;

    let stake_pool = StakePool::deserialize(&mut stake_pool_acc.data.as_slice()).unwrap();

    let validator_list_acc = rpc.get_account(&stake_pool.validator_list).await.unwrap();

    let ValidatorList { validators, .. } =
        ValidatorList::deserialize(&mut validator_list_acc.data.as_slice()).unwrap();

    update_pool(UpdatePoolArgs {
        rpc: &rpc,
        send_mode: TxSendMode::SendActual,
        payer,
        program_id,
        stake_pool: Keyed {
            pubkey: pool,
            account: &stake_pool_acc,
        },
        validator_list_entries: &validators,
        fee_limit_cb: 100_000,
    })
    .await;

    Ok(())
}
