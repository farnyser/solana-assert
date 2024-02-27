use std::fmt::Debug;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::{Client, Cluster};
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;
use solana_sdk::signer::EncodableKey;
use solana_sdk::commitment_config::CommitmentConfig;
use clap::{arg, Parser, Subcommand};
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use assert_client::{assert_balance_gte, assert_balance_lte};

#[derive(Debug, Parser)]
#[command(name = "assert_client")]
#[command(about = "Assert CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// The wallet pubkey
    #[arg(short, long)]
    wallet: String,

    #[arg(short, long, default_value = "http://localhost:8899")]
    cluster_url: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    AssertLowerThan {
        /// The token pubkey
        #[arg(long)]
        mint: String,
        /// Amount
        amount: u64,
    },
    AssertGreaterThan {
        /// The token pubkey
        #[arg(long)]
        mint: String,
        /// Amount
        amount: u64,
    },
}

fn main() {
    let args = Cli::parse();

    let url = args.cluster_url.parse::<Cluster>().unwrap();
    let wallet = Keypair::read_from_file(Path::new(&args.wallet)).unwrap();
    let payer = Rc::new(wallet);
    let client = Client::new_with_options(url.clone(), payer.clone(), CommitmentConfig::processed());

    let sig = match args.command {
        Commands::AssertLowerThan { mint, amount } => {
            let owner_pubkey = payer.pubkey();
            let mint_pubkey = Pubkey::from_str(&mint).unwrap();
            let associated_token_address = get_associated_token_address(&owner_pubkey, &mint_pubkey);
            assert_balance_lte(&client, &payer, &associated_token_address, amount)
        },
        Commands::AssertGreaterThan { mint, amount } => {
            let owner_pubkey = payer.pubkey();
            let mint_pubkey = Pubkey::from_str(&mint).unwrap();
            let associated_token_address = get_associated_token_address(&owner_pubkey, &mint_pubkey);
            assert_balance_gte(&client, &payer, &associated_token_address, amount)
        },
    };

    match sig {
        Ok(hash) => println!("-> {} !", hash),
        Err(err) => println!("ERROR {} !", err),
    }
}
