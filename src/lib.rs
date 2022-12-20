mod pb;

use pb::basicexample;

use substreams::{log};
use substreams_ethereum::{pb as ethpb};

#[substreams::handlers::map]
fn map_basic_eth(block: ethpb::eth::v2::Block) -> Result<basicexample::BasicExampleProtoData, substreams::errors::Error> {
    // Extract data from the Ethereum Block and log to the console.
    // The data available in the Block directly represents the related protobuf.
    // The full data model for an Ethereum Block is available at the following link.
    // https://github.com/streamingfast/firehose-ethereum/blob/develop/proto/sf/ethereum/type/v2/type.proto
    log::info!("block.ver: {:#?}", block.ver);
    log::info!("block.number: {:#?}", block.number);

    // Copy the data in the Block's version field and return it to caller.
    // Substreams developers will typically pass extracted data through a custom
    // protobuf to a store module.
    Ok(basicexample::BasicExampleProtoData {version: block.ver})
}