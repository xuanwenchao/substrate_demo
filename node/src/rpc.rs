//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use jsonrpsee::RpcModule;
use node_template_runtime::{opaque::Block, AccountId, Balance, Index};

//for babe
// use node_template_runtime::{
// 	opaque::Block, AccountId, Balance,
// 	Index, Hash, BlockNumber,
// };
// use jsonrpsee::RpcModule;
// use node_primitives::{AccountId, Balance, Block, BlockNumber, Hash, Index};
// use sc_client_api::AuxStore;
// use sc_consensus_babe::{BabeConfiguration, Epoch};
// use sc_consensus_epochs::SharedEpochChanges;
// use sc_consensus_grandpa::{
// 	FinalityProofProvider, GrandpaJustificationStream, SharedAuthoritySet, SharedVoterState,
// };
// use sc_rpc::SubscriptionTaskExecutor;

pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
// use sp_consensus::SelectChain;
// use sp_consensus_babe::BabeApi;
// use sp_keystore::SyncCryptoStorePtr;

/// Extra dependencies for BABE.
// pub struct BabeDeps {
// 	/// BABE protocol config.
// 	pub babe_config: BabeConfiguration,
// 	/// BABE pending epoch changes.
// 	pub shared_epoch_changes: SharedEpochChanges<Block, Epoch>,
// 	/// The keystore that manages the keys of the node.
// 	pub keystore: SyncCryptoStorePtr,
// }

// /// Extra dependencies for GRANDPA
// pub struct GrandpaDeps<B> {
// 	/// Voting round info.
// 	pub shared_voter_state: SharedVoterState,
// 	/// Authority set info.
// 	pub shared_authority_set: SharedAuthoritySet<Hash, BlockNumber>,
// 	/// Receives notifications about justification events from Grandpa.
// 	pub justification_stream: GrandpaJustificationStream<Block>,
// 	/// Executor to drive the subscription manager in the Grandpa RPC handler.
// 	pub subscription_executor: SubscriptionTaskExecutor,
// 	/// Finality proof provider.
// 	pub finality_provider: Arc<FinalityProofProvider<B, Block>>,
// }

/// Full client dependencies.
pub struct FullDeps<C, P> {
//pub struct FullDeps<C, P, SC, B> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// The SelectChain Strategy
	// pub select_chain: SC,
	/// A copy of the chain spec.
	// pub chain_spec: Box<dyn sc_chain_spec::ChainSpec>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	// /// BABE specific dependencies.
	// pub babe: BabeDeps,
	// /// GRANDPA specific dependencies.
	// pub grandpa: GrandpaDeps<B>,
}

/// Instantiate all Full RPC extensions.

// pub fn create_full<C, P, SC, B>(
// 	deps: FullDeps<C, P, SC, B>,
// ) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
// where
pub fn create_full<C, P>(
	deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool + 'static,
	// C::Api: BabeApi<Block>,
	// SC: SelectChain<Block> + 'static,
	// B: sc_client_api::Backend<Block> + Send + Sync + 'static,
	// B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashFor<Block>>,
{
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	let FullDeps { client, pool, deny_unsafe } = deps;

	module.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client).into_rpc())?;

	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `module.merge(YourRpcTrait::into_rpc(YourRpcStruct::new(ReferenceToClient, ...)))?;`

	Ok(module)
}
