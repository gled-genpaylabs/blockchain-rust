//! cli process

use super::*;
use crate::blockchain::*;
use crate::server::*;
use crate::transaction::*;
use crate::utxoset::*;
use crate::wallets::*;
use bitcoincash_addr::Address;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    PrintChain,
    CreateWallet,
    ListAddresses,
    Reindex,
    StartNode {
        #[arg(short, long, default_value = "3000")]
        port: String,
    },
    StartMiner {
        #[arg(short, long, default_value = "3000")]
        port: String,
        #[arg(short, long)]
        address: String,
    },
    GetBalance {
        #[arg(short, long)]
        address: String,
    },
    CreateBlockchain {
        #[arg(short, long)]
        address: String,
    },
    Send {
        #[arg(short, long)]
        from: String,
        #[arg(short, long)]
        to: String,
        #[arg(short, long)]
        amount: i32,
        #[arg(short, long, default_value_t = false)]
        mine: bool,
    },
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            command: Commands::PrintChain,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        info!("run app");
        let cli = Cli::parse();
        match &cli.command {
            Commands::PrintChain => {
                cmd_print_chain()?;
            }
            Commands::CreateWallet => {
                println!("address: {}", cmd_create_wallet()?);
            }
            Commands::ListAddresses => {
                cmd_list_address()?;
            }
            Commands::Reindex => {
                let count = cmd_reindex()?;
                println!("Done! There are {} transactions in the UTXO set.", count);
            }
            Commands::StartNode { port } => {
                println!("Start node...");
                let bc = Blockchain::new()?;
                let utxo_set = UTXOSet { blockchain: bc };
                let server = Server::new(port, "", utxo_set)?;
                server.start_server()?;
            }
            Commands::StartMiner { port, address } => {
                println!("Start miner node...");
                let bc = Blockchain::new()?;
                let utxo_set = UTXOSet { blockchain: bc };
                let server = Server::new(port, address, utxo_set)?;
                server.start_server()?;
            }
            Commands::GetBalance { address } => {
                let balance = cmd_get_balance(address)?;
                println!("Balance: {}\n", balance);
            }
            Commands::CreateBlockchain { address } => {
                cmd_create_blockchain(address)?;
            }
            Commands::Send {
                from,
                to,
                amount,
                mine,
            } => {
                cmd_send(from, to, *amount, *mine)?;
            }
        }
        Ok(())
    }
}

fn cmd_send(from: &str, to: &str, amount: i32, mine_now: bool) -> Result<()> {
    let bc = Blockchain::new()?;
    let mut utxo_set = UTXOSet { blockchain: bc };
    let wallets = Wallets::new()?;
    let wallet = wallets.get_wallet(from).unwrap();
    let tx = Transaction::new_UTXO(wallet, to, amount, &utxo_set)?;
    if mine_now {
        let cbtx = Transaction::new_coinbase(from.to_string(), String::from("reward!"))?;
        let new_block = utxo_set.blockchain.mine_block(vec![cbtx, tx])?;

        utxo_set.update(&new_block)?;
    } else {
        Server::send_transaction(&tx, utxo_set)?;
    }

    println!("success!");
    Ok(())
}

fn cmd_create_wallet() -> Result<String> {
    let mut ws = Wallets::new()?;
    let address = ws.create_wallet();
    ws.save_all()?;
    Ok(address)
}

fn cmd_reindex() -> Result<i32> {
    let bc = Blockchain::new()?;
    let utxo_set = UTXOSet { blockchain: bc };
    utxo_set.reindex()?;
    utxo_set.count_transactions()
}

fn cmd_create_blockchain(address: &str) -> Result<()> {
    let address = String::from(address);
    let bc = Blockchain::create_blockchain(address)?;

    let utxo_set = UTXOSet { blockchain: bc };
    utxo_set.reindex()?;
    println!("create blockchain");
    Ok(())
}

fn cmd_get_balance(address: &str) -> Result<i32> {
    let pub_key_hash = Address::decode(address).unwrap().body;
    let bc = Blockchain::new()?;
    let utxo_set = UTXOSet { blockchain: bc };
    let utxos = utxo_set.find_UTXO(&pub_key_hash)?;

    let mut balance = 0;
    for out in utxos.outputs {
        balance += out.value;
    }
    Ok(balance)
}

fn cmd_print_chain() -> Result<()> {
    let bc = Blockchain::new()?;
    for b in bc.iter() {
        println!("{:#?}", b);
    }
    Ok(())
}

fn cmd_list_address() -> Result<()> {
    let ws = Wallets::new()?;
    let addresses = ws.get_all_addresses();
    println!("addresses: ");
    for ad in addresses {
        println!("{}", ad);
    }
    Ok(())
}

