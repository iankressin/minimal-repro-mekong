use alloy::{
    eips::BlockNumberOrTag,
    primitives::b256,
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::BlockTransactionsKind,
    transports::http::{Client, Http},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = ProviderBuilder::new().on_http("https://rpc.mekong.ethpandaops.io".try_into()?);
    fetch_7702_tx(&provider).await?;
    fetch_block_with_7702_tx(&provider).await?;
    Ok(())
}

async fn fetch_7702_tx(
    provider: &RootProvider<Http<Client>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let transaction = provider
        .get_transaction_by_hash(b256!(
            "adc3f24d05f05f1065debccb1c4b033eaa35917b69b343d88d9062cdf8ecad83"
        ))
        .await?;

    println!("{:?}", transaction);

    Ok(())
}

async fn fetch_block_with_7702_tx(
    provider: &RootProvider<Http<Client>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let block = provider
        .get_block_by_number(BlockNumberOrTag::Number(45090), BlockTransactionsKind::Full)
        .await?;

    println!("{:?}", block);

    Ok(())
}
