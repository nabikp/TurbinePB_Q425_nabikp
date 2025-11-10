use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::Client;
use anchor_client::Cluster;
use anchor_lang::prelude::*;
use solana_sdk::signature::{read_keypair_file, Keypair};
use std::rc::Rc;

use metaplex_core_example::program::MetaplexCoreExample;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load your devnet wallet
    let wallet_path = "/Users/sangi/.config/solana/devnet.json";
    let payer = read_keypair_file(wallet_path)?;
    let payer_rc = Rc::new(payer);

    // Connect to devnet
    let client = Client::new_with_options(Cluster::Devnet, payer_rc.clone(), CommitmentConfig::confirmed());

    // Get the deployed program
    let program_id = "3SBr7mg13DbMyhEzgYvwGb4SLfd9iEiCfJ2r83f2YPiE".parse()?;
    let program = client.program(program_id);

    // Call the mint_asset instruction
    let tx = program
        .request()
        .accounts(metaplex_core_example::accounts::MintAsset {
            user: payer_rc.pubkey(),
            mpl_core_program: mpl_core::ID,
            system_program: solana_sdk::system_program::ID,
        })
        .args(metaplex_core_example::instruction::MintAsset {})
        .send()?;

    println!("NFT minted! Transaction signature: {}", tx);

    Ok(())
}
