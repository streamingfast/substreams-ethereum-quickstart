mod pb;

use pb::basicexample;

use substreams::{log, Hex};
use substreams_ethereum::{pb as ethpb};

#[substreams::handlers::map]
fn map_basic_eth(block: ethpb::eth::v2::Block) -> Result<basicexample::BasicExampleProtoData, substreams::errors::Error> {
    
    let header = block.header.as_ref().unwrap();
    
    log::info!("block.number: {:#?}", block.number);
    log::info!("block.hash: {:#?}",  Hex(&block.hash).to_string());
    log::info!("block.header.parent_hash: {:#?}", Hex(&header.parent_hash).to_string());
    log::info!("block.header.timestamp: {:#?}", header.timestamp.as_ref().unwrap().to_string());
    
    Ok(basicexample::BasicExampleProtoData {number: block.number, hash: Hex(&block.hash).to_string(), parent_hash: Hex(&header.parent_hash).to_string(), timestamp: header.timestamp.as_ref().unwrap().to_string()})
}