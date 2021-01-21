//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]
use std::sync::Arc;

use primitives::{Block, BlockNumber, AccountId, CurrencyId, Index, Balance, Hash, Rate, Share};
use sc_client_api::{
	backend::{Backend, StateBackend,},
};
pub use sc_rpc::SubscriptionTaskExecutor;
pub use sc_rpc_api::DenyUnsafe;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderMetadata, HeaderBackend};
use sp_runtime::traits::BlakeTwo256;
use sp_transaction_pool::TransactionPool;
use sc_transaction_graph::{ChainApi, Pool};
use sc_network::NetworkService;
use jsonrpc_pubsub::manager::SubscriptionManager;

/// Full client dependencies.
pub struct FullDeps<C, P, A: ChainApi> {
  /// The client instance to use.
  pub client: Arc<C>,
  /// Transaction pool instance.
  pub pool: Arc<P>,
  /// Graph pool instance.
  pub graph: Arc<Pool<A>>,
  /// Whether to deny unsafe calls
  pub deny_unsafe: DenyUnsafe,
  /// The Node authority flag
  pub is_authority: bool,
  /// Network service
  pub network: Arc<NetworkService<Block, Hash>>,
}

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, B, A>(
  deps: FullDeps<C, P, A>,
  subscription_task_executor: SubscriptionTaskExecutor
) -> jsonrpc_core::IoHandler<sc_rpc_api::Metadata> where
  C: ProvideRuntimeApi<Block> + sc_client_api::backend::StorageProvider<Block, B> + sc_client_api::AuxStore,
  C: sc_client_api::client::BlockchainEvents<Block>,
  C: HeaderBackend<Block> + HeaderMetadata<Block, Error=BlockChainError> + 'static,
  C: Send + Sync + 'static,
  C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
  C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber>,
  C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
  C::Api: clover_rpc::balance::CurrencyBalanceRuntimeApi<Block, AccountId, CurrencyId, Balance>,
  C::Api: clover_rpc::pair::CurrencyPairRuntimeApi<Block>,
  C::Api: clover_rpc::incentive_pool::IncentivePoolRuntimeApi<Block, AccountId, CurrencyId, Share, Balance>,
  C::Api: clover_rpc::exchange::CurrencyExchangeRuntimeApi<Block, AccountId, CurrencyId, Balance, Rate, Share>,
  C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
  C::Api: BlockBuilder<Block>,
  P: TransactionPool<Block=Block> + 'static,
  B: Backend<Block> + 'static,
  B::State: StateBackend<BlakeTwo256>,
  A: ChainApi<Block = Block> + 'static,
{
  use fc_rpc::{
    EthApi, EthApiServer, NetApi, NetApiServer, EthPubSubApi, EthPubSubApiServer,
    Web3Api, Web3ApiServer, EthDevSigner, EthSigner, HexEncodedIdProvider,
  };
  use substrate_frame_rpc_system::{FullSystem, SystemApi};
  use pallet_contracts_rpc::{Contracts, ContractsApi};
  use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};

  let mut io = jsonrpc_core::IoHandler::default();
  let FullDeps {
    client,
    pool,
    graph: _,
    deny_unsafe,
    is_authority,
    network,
  } = deps;

  io.extend_with(
    SystemApi::to_delegate(FullSystem::new(client.clone(), pool.clone(), deny_unsafe))
  );
  io.extend_with(
    TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone()))
  );
  io.extend_with(ContractsApi::to_delegate(Contracts::new(client.clone())));

  io.extend_with(clover_rpc::balance::CurrencyBalanceRpc::to_delegate(
    clover_rpc::balance::CurrencyBalance::new(client.clone()),
  ));

  io.extend_with(clover_rpc::currency::CurrencyRpc::to_delegate(
        clover_rpc::currency::Currency {},
    ));

  io.extend_with(clover_rpc::pair::CurrencyPairRpc::to_delegate(
    clover_rpc::pair::CurrencyPair::new(client.clone()),
  ));

  io.extend_with(clover_rpc::exchange::CurrencyExchangeRpc::to_delegate(
    clover_rpc::exchange::CurrencyExchange::new(client.clone()),
  ));

  io.extend_with(clover_rpc::incentive_pool::IncentivePoolRpc::to_delegate(
    clover_rpc::incentive_pool::IncentivePool::new(client.clone()),
  ));

  let mut signers = Vec::new();
  signers.push(Box::new(EthDevSigner::new()) as Box<dyn EthSigner>);
  io.extend_with(EthApiServer::to_delegate(EthApi::new(
    client.clone(),
    pool.clone(),
    clover_runtime::TransactionConverter,
    network.clone(),
    signers,
    is_authority,
  )));

  io.extend_with(
    NetApiServer::to_delegate(NetApi::new(
      client.clone(),
      network.clone(),
    ))
  );

  io.extend_with(
    Web3ApiServer::to_delegate(Web3Api::new(
      client.clone(),
    ))
  );

  io.extend_with(
    EthPubSubApiServer::to_delegate(EthPubSubApi::new(
      pool.clone(),
      client.clone(),
      network.clone(),
      SubscriptionManager::<HexEncodedIdProvider>::with_id_provider(
        HexEncodedIdProvider::default(),
        Arc::new(subscription_task_executor)
      ),
    ))
  );

  io
}
