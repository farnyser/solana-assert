use solana_sdk::{signer::{keypair::Keypair, Signer}};
use solana_sdk::signature::Signature;
use anchor_client::{Client};
use std::ops::Deref;
use anchor_client::solana_client::rpc_config::RpcSendTransactionConfig;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;

pub fn assert_balance_gte<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    signer_wallet: &Keypair,
    associated_token_account: &Pubkey,
    amount: u64
) -> Result<Signature> {
    let program = client.program(assert_program::ID)?;
    let sig = program
        .request()
        .signer(&signer_wallet)
        .accounts(assert_program::accounts::TokenAccount {
            account: *associated_token_account,
        })
        .args(assert_program::instruction::AssertBalanceGte {
            amount
        })
        .send_with_spinner_and_config(RpcSendTransactionConfig{
            skip_preflight: true,
            preflight_commitment: None,
            encoding: None,
            max_retries: None,
            min_context_slot: None,
        })?;

    Ok(sig)
}

pub fn assert_balance_lte<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    signer_wallet: &Keypair,
    associated_token_account: &Pubkey,
    amount: u64
) -> Result<Signature> {
    let program = client.program(assert_program::ID)?;
    let sig = program
        .request()
        .signer(&signer_wallet)
        .accounts(assert_program::accounts::TokenAccount {
            account: *associated_token_account,
        })
        .args(assert_program::instruction::AssertBalanceLte {
            amount
        })
        .send_with_spinner_and_config(RpcSendTransactionConfig{
            skip_preflight: true,
            preflight_commitment: None,
            encoding: None,
            max_retries: None,
            min_context_slot: None,
        })?;

    Ok(sig)
}