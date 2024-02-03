use std::marker::PhantomData;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    alloy_sol_types::{sol, SolError},
    evm, msg,
    prelude::*,
};

// ERC721 に任意に渡せるパラメータ
pub trait ERC721Params {
    const NAME: &'static str;
    const SYMBOL: &'static str;
    const BASE_URI: &'static str;
}

sol_storage! {
    pub struct ERC721<T> {
        PhantomData<T> custom_params;
        mapping(uint256 => address) owners;
        mapping(address => uint256) balances;
        mapping(uint256 => address) token_approvals;
        mapping(address => mapping(address => bool)) operator_approvals;
    }
}

sol! {
    event Transfer(address indexed _from, address indexed _to, uint256 indexed _tokenId);
    event Approval(address indexed _owner, address indexed _approved, uint256 indexed _tokenId);
    event ApprovalForAll(address indexed _owner, address indexed _operator, bool _approved);

    error ERC721InvalidOwner(address owner);
    error ERC721NonexistentToken(uint256 tokenId);
    error ERC721IncorrectOwner(address sender, uint256 tokenId, address owner);
    error ERC721InvalidSender(address sender);
    error ERC721InvalidReceiver(address receiver);
    error ERC721InsufficientApproval(address operator, uint256 tokenId);
    error ERC721InvalidApprover(address approver);
    error ERC721InvalidOperator(address operator);
}

enum ERC721Error {
    ERC721InvalidOwner(ERC721InvalidOwner),
    ERC721NonexistentToken(ERC721NonexistentToken),
    ERC721IncorrectOwner(ERC721IncorrectOwner),
    ERC721InvalidSender(ERC721InvalidSender),
    ERC721InvalidReceiver(ERC721InvalidReceiver),
    ERC721InsufficientApproval(ERC721InsufficientApproval),
    ERC721InvalidApprover(ERC721InvalidApprover),
    ERC721InvalidOperator(ERC721InvalidOperator),
}

impl From<ERC721Error> for Vec<u8> {
    fn from(err: ERC721Error) -> Vec<u8> {
        match err {
            ERC721Error::ERC721InvalidOwner(e) => e.encode(),
            ERC721Error::ERC721NonexistentToken(e) => e.encode(),
            ERC721Error::ERC721IncorrectOwner(e) => e.encode(),
            ERC721Error::ERC721InvalidSender(e) => e.encode(),
            ERC721Error::ERC721InvalidReceiver(e) => e.encode(),
            ERC721Error::ERC721InsufficientApproval(e) => e.encode(),
            ERC721Error::ERC721InvalidApprover(e) => e.encode(),
            ERC721Error::ERC721InvalidOperator(e) => e.encode(),
        }
    }
}

type ERC721Result<T> = Result<T, ERC721Error>;

impl<T: ERC721Params> ERC721<T> {
    fn _require_owned(&self, token_id: U256) -> ERC721Result<Address> {
        let owner = self.owner_of(token_id)?;

        if owner == Address::ZERO {
            return Err(ERC721Error::ERC721NonexistentToken(
                ERC721NonexistentToken { tokenId: token_id },
            ));
        }

        Ok(owner)
    }

    fn _is_approved_for_all(&self, owner: Address, operator: Address) -> ERC721Result<bool> {
        Ok(self.operator_approvals.get(owner).get(operator))
    }

    fn _approve_real(
        &mut self,
        to: Address,
        token_id: U256,
        auth: Address,
        emit_event: bool,
    ) -> ERC721Result<()> {
        if emit_event || auth != Address::ZERO {
            let owner = self._require_owned(token_id)?;

            if auth != Address::ZERO && owner != auth && !self._is_approved_for_all(owner, auth)? {
                return Err(ERC721Error::ERC721InvalidOperator(ERC721InvalidOperator {
                    operator: auth,
                }));
            }

            if emit_event {
                evm::log(Approval {
                    _owner: owner,
                    _approved: to,
                    _tokenId: token_id,
                });
            }
        }

        self.token_approvals.setter(token_id).set(to);
        Ok(())
    }

    fn _approve(&mut self, to: Address, token_id: U256, address: Address) -> ERC721Result<()> {
        self._approve_real(to, token_id, address, true)
    }

    fn _get_approved(&self, token_id: U256) -> ERC721Result<Address> {
        Ok(self.token_approvals.get(token_id))
    }

    fn _set_approval_for_all(
        &mut self,
        owner: Address,
        operator: Address,
        approved: bool,
    ) -> ERC721Result<()> {
        if operator == Address::ZERO {
            return Err(ERC721Error::ERC721InvalidOperator(ERC721InvalidOperator {
                operator,
            }));
        }

        self.operator_approvals
            .setter(owner)
            .setter(operator)
            .set(approved);

        evm::log(ApprovalForAll {
            _owner: owner,
            _operator: operator,
            _approved: approved,
        });

        Ok(())
    }
}

#[external]
impl<T: ERC721Params> ERC721<T> {
    fn balance_of(&self, owner: Address) -> ERC721Result<U256> {
        Ok(self.balances.get(owner))
    }

    fn owner_of(&self, token_id: U256) -> ERC721Result<Address> {
        Ok(self.owners.get(token_id))
    }

    fn name(&self) -> ERC721Result<String> {
        Ok(T::NAME.into())
    }

    fn symbol(&self) -> ERC721Result<String> {
        Ok(T::NAME.into())
    }

    fn token_uri(&self, token_id: U256) -> ERC721Result<String> {
        Ok(T::BASE_URI.to_string() + &token_id.to_string())
    }

    fn approve(&mut self, to: Address, token_id: U256) -> ERC721Result<()> {
        self._approve(to, token_id, msg::sender())
    }

    fn get_approved(&self, token_id: U256) -> ERC721Result<Address> {
        self._require_owned(token_id)?;

        self._get_approved(token_id)
    }

    fn set_approval_for_all(&mut self, operator: Address, approved: bool) -> ERC721Result<()> {
        self._set_approval_for_all(msg::sender(), operator, approved)
    }
}
