use substreams::prelude::*;
use substreams::errors::Error;
use crate::pb;
use crate::generated::substreams::{Substreams, SubstreamsTrait};


#[no_mangle]
pub extern "C" fn eth_basic_mapper(
    block_ptr: *mut u8,
    block_len: usize,
) {
    substreams::register_panic_hook();
    let func = ||-> Result<pb::eth_basicexample_v1::BasicExampleProtoData, Error>{
        
        let block: substreams_ethereum::pb::eth::v2::Block = substreams::proto::decode_ptr(block_ptr, block_len).unwrap();

        Substreams::eth_basic_mapper(block,
            
        )
    };
    let result = func();
    if result.is_err() {
        panic!("{:?}", &result.err().unwrap());
    }
    substreams::output(result.unwrap());
}
