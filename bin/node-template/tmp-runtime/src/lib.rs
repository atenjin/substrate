#![cfg_attr(not(feature = "std"), no_std)]
use sp_api::impl_runtime_apis;
use sp_version::RuntimeVersion;
use sp_runtime::traits::Block as BlockT;

mod primitives {
	use sp_runtime::{generic, MultiSignature};
	use sp_runtime::traits::{BlakeTwo256, IdentifyAccount, Verify};

	pub type Signature = MultiSignature;
	pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
	pub type BlockNumber = u32;
	/// The address format for describing accounts.
	pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
	/// Block header type as expected by this runtime.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Block type as expected by this runtime.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// The SignedExtension to the basic transaction logic.
	pub type SignedExtra = (
		// frame_system::CheckSpecVersion<Runtime>,
		// frame_system::CheckTxVersion<Runtime>,
		// frame_system::CheckGenesis<Runtime>,
		// frame_system::CheckEra<Runtime>,
		// frame_system::CheckNonce<Runtime>,
		// frame_system::CheckWeight<Runtime>,
		// pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
	);
	pub type Call = String;
	/// Unchecked extrinsic type as expected by this runtime.
	pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
}

pub struct Runtime;
pub use primitives::Block;

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			// VERSION
			todo!()
		}

		fn execute_block(block: Block) {
			// Executive::execute_block(block);
			todo!()
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			// Executive::initialize_block(header)
			todo!()
		}
	}
}
