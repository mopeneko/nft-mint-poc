use core::marker::PhantomData;

// ERC721 に任意に渡せるパラメータ
pub trait ERC721Params {
}

sol_storage! {
    pub struct ERC721<T> {
        params: PhantomData<T>;
    }
}

sol! {
    event Transfer(address indexed _from, address indexed _to, uint256 indexed _tokenId);
    event Approval(address indexed _owner, address indexed _approved, uint256 indexed _tokenId);
    event ApprovalForAll(address indexed _owner, address indexed _operator, bool _approved);
}

impl<T: ERC21Params> ERC721<T> {
    // TODO: ここに内部処理(Mintとか)を書く
}

#[external]
impl<T: ERC21Params> ERC721<T> {
    // ここにABIに露出させる処理を書く
}
