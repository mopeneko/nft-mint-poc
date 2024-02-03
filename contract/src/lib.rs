#![cfg_attr(not(feature = "export-abi"), no_main)]
mod erc721;

extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use crate::erc721::{ERC721Params, ERC721};
use stylus_sdk::prelude::*;

struct NFTParams;

impl ERC721Params for NFTParams {
    const NAME: &'static str = "Test NFT";
    const SYMBOL: &'static str = "TEST";
    const BASE_URI: &'static str = "https://example.com";
}

sol_storage! {
    #[entrypoint]
    pub struct NFT {
        #[borrow]
        ERC721<NFTParams> erc721;
    }
}

#[external]
#[inherit(ERC721<NFTParams>)]
impl NFT {}
