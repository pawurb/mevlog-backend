use tokio::process::Command;

use alloy::rpc::types::BlockTransactions;
use alloy::{
    eips::BlockNumberOrTag,
    providers::{Provider, ProviderBuilder},
};
use eyre::Result;
use mevlog_backend::config::{middleware, schedule::get_schedule};
use mevlog_backend::misc::utils::{measure_end, measure_start, uptime_ping};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    match run().await {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(e)
        }
    }
}

async fn run() -> Result<()> {
    middleware::init_logs("scheduler.log");
    // tokio::spawn(async move {
    //     let result = std::panic::AssertUnwindSafe(populate_revm_cache())
    //         .catch_unwind()
    //         .await;

    //     match result {
    //         Ok(Ok(_)) => panic!("Cache task finished cleanly (which it never should)"),
    //         Ok(Err(e)) => error!("Cache task errored: {:?}", e),
    //         Err(e) => error!("Cache task panicked: {:?}", e),
    //     }
    // });

    let sched = get_schedule().await?;
    sched.start().await?;

    tokio::signal::ctrl_c().await?;

    info!("Scheduler ending");

    Ok(())
}

async fn _populate_revm_cache() -> Result<()> {
    let rpc_url = std::env::var("ETH_RPC_URL_VAL").expect("Missing ETH_RPC_URL_VAL env var");
    let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);
    tracing::info!("Scheduler connected to HTTP provider");

    let mut current_block_number = provider.get_block_number().await?;
    loop {
        let new_block_number = provider.get_block_number().await?;
        if new_block_number == current_block_number {
            tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
            info!("No new blocks, sleeping: {}", current_block_number);
            continue;
        }

        current_block_number = new_block_number;
        let block_tag = BlockNumberOrTag::Number(new_block_number);
        let block = provider.get_block_by_number(block_tag).await?;

        let block = match block {
            Some(block) => block,
            None => continue,
        };

        let txs = match block.transactions {
            BlockTransactions::Hashes(hashes) => hashes,
            _ => continue,
        };

        if txs.is_empty() {
            continue;
        }

        let last_tx = txs[txs.len() - 1];
        let start = measure_start("cast run last");
        let _resp = match Command::new("cast")
            .arg("run")
            .arg(last_tx.to_string())
            .arg("--rpc-url")
            .arg(std::env::var("ETH_RPC_URL_VAL").unwrap())
            .output()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                error!("Failed to run cast: {}", &e);
                continue;
            }
        };

        if new_block_number % 10 == 0 {
            let uptime_url = std::env::var("UPTIME_URL_REVM_CACHE")
                .expect("Missing UPTIME_URL_REVM_CACHE env var");
            info!("Revm cache uptime ping");

            match uptime_ping(&uptime_url).await {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to uptime ping: {}", &e);
                }
            }
        }

        measure_end(start);
    }

    #[allow(unreachable_code)]
    Ok(())
}
