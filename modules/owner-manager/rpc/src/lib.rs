//! RPC interface for the transaction payment module.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
// use owner_manager_runtime_api::SumStorageApi as SumStorageRuntimeApi;
use owner_manager_runtime_api::OwnerManagerApi as OwnerManagerRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::H160;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

#[rpc]
pub trait OwnerManagerApi<BlockHash> {
    #[rpc(name = "clover_get_owner_address")]
    fn get_owner_address(
        &self,
        contract_address: Option<H160>,
        at: Option<BlockHash>,
    ) -> Result<Option<H160>>;
}

pub struct OwnerManager<C, M> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}
impl<C, M> OwnerManager<C, M> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block> OwnerManagerApi<<Block as BlockT>::Hash> for OwnerManager<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: OwnerManagerRuntimeApi<Block>,
{
    fn get_owner_address(
        &self,
        contract_address: Option<H160>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Option<H160>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_owner_address(&at, contract_address);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876),
            message: "Not find the owner address".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
}
