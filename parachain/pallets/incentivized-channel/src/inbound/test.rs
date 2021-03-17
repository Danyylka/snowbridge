
use super::*;

use sp_core::{H160, H256};
use frame_support::{
	assert_ok, assert_noop,
	parameter_types,
	dispatch::DispatchError
};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup, IdentifyAccount, Verify}, testing::Header, MultiSignature
};
use sp_keyring::AccountKeyring as Keyring;
use sp_std::convert::From;

use artemis_core::{MessageDispatch, Message, Proof, rewards::InstantRewards};
use artemis_ethereum::Log;

use hex_literal::hex;

use crate::inbound::Error;

use crate::inbound as incentivized_inbound_channel;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Storage, Event<T>},
		Balances: pallet_balances::{Module, Call, Storage, Event<T>},
		IncentivizedInboundChannel: incentivized_inbound_channel::{Module, Call, Storage, Event},
	}
);

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Balance = u128;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}


parameter_types! {
	pub const ExistentialDeposit: u128 = 1;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
	/// The ubiquitous event type.
	type Event = Event;
	type MaxLocks = MaxLocks;
	/// The type for recording an account's balance.
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

// Mock verifier
pub struct MockVerifier;

impl Verifier for MockVerifier {
	fn verify(message: &Message) -> Result<Log, DispatchError> {
		let log: Log = rlp::decode(&message.data).unwrap();
		Ok(log)
	}
}

// Mock Dispatch
pub struct MockMessageDispatch;

impl MessageDispatch<MessageId> for MockMessageDispatch {
	fn dispatch(_: H160, _: MessageId, _: &[u8]) {}
}

parameter_types! {
	pub RewardsAccount: AccountId = Keyring::Eve.into();
}


impl incentivized_inbound_channel::Config for Test {
	type Event = Event;
	type Verifier = MockVerifier;
	type MessageDispatch = MockMessageDispatch;
	type RewardsAccount = RewardsAccount;
	type InboundMessageFee = Balance;
	type RewardRelayer = InstantRewards<Self, Balances>;
}

pub fn new_tester(source_channel: H160) -> sp_io::TestExternalities {
	new_tester_with_config(incentivized_inbound_channel::GenesisConfig {
		source_channel,
	})
}

pub fn new_tester_with_config(config: incentivized_inbound_channel::GenesisConfig) -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	config.assimilate_storage(&mut storage).unwrap();

	let mut ext: sp_io::TestExternalities = storage.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}


// The originating channel address for the messages below
const SOURCE_CHANNEL_ADDR: [u8; 20] = hex!["2d02f2234d0B6e35D8d8fD77705f535ACe681327"];

// Ethereum Log:
//   address: 0xe4ab635d0bdc5668b3fcb4eaee1dec587998f4af (outbound channel contract)
//   topics: ...
//   data:
//     source: 0x8f5acf5f15d4c3d654a759b96bb674a236c8c0f3  (ETH bank contract)
//     nonce: 1
//     payload ...
const MESSAGE_DATA_0: [u8; 284] = hex!("
	f90119942d02f2234d0b6e35d8d8fd77705f535ace681327e1a0779b38144a38
	cfc4351816442048b17fe24ba2b0e0c63446b576e8281160b15bb8e000000000
	00000000000000000a42cba2b7960a0ce216ade5d6a82574257023d800000000
	0000000000000000000000000000000000000000000000000000000100000000
	0000000000000000000000000000000000000000000000000000006000000000
	000000000000000000000000000000000000000000000000000000570c018213
	dae5f9c236beab905c8305cb159c5fa1aae500d43593c715fdd31c61141abd04
	a99fd6822c8558854ccde39a5684e7a56da27d0000d9e9ac2d78030000000000
	00000000000000000000000000000000000000000000000000000000
");

// Ethereum Log:
//   address: 0xe4ab635d0bdc5668b3fcb4eaee1dec587998f4af (outbound channel contract)
//   topics: ...
//   data:
//     source: 0x8f5acf5f15d4c3d654a759b96bb674a236c8c0f3  (ETH bank contract)
//     nonce: 1
//     payload ...
const MESSAGE_DATA_1: [u8; 284] = hex!("
	f90119942d02f2234d0b6e35d8d8fd77705f535ace681327e1a0779b38144a38
	cfc4351816442048b17fe24ba2b0e0c63446b576e8281160b15bb8e000000000
	00000000000000000a42cba2b7960a0ce216ade5d6a82574257023d800000000
	0000000000000000000000000000000000000000000000000000000200000000
	0000000000000000000000000000000000000000000000000000006000000000
	000000000000000000000000000000000000000000000000000000570c018213
	dae5f9c236beab905c8305cb159c5fa1aae500d43593c715fdd31c61141abd04
	a99fd6822c8558854ccde39a5684e7a56da27d0000d9e9ac2d78030000000000
	00000000000000000000000000000000000000000000000000000000
");

#[test]
fn test_submit_with_invalid_source_channel() {
	new_tester(H160::zero()).execute_with(|| {
		let relayer: AccountId = Keyring::Bob.into();
		let origin = Origin::signed(relayer);

		// Submit message
		let message = Message {
			data: MESSAGE_DATA_0.into(),
			proof: Proof {
				block_hash: Default::default(),
				tx_index: Default::default(),
				data: Default::default()
			},
		};
		assert_noop!(
			IncentivizedInboundChannel::submit(origin.clone(), message.clone()),
			Error::<Test>::InvalidSourceChannel
		);
	});
}

#[test]
fn test_submit() {
	new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
		let relayer: AccountId = Keyring::Bob.into();
		let origin = Origin::signed(relayer);

		// Submit message 1
		let message_1 = Message {
			data: MESSAGE_DATA_0.into(),
			proof: Proof {
				block_hash: Default::default(),
				tx_index: Default::default(),
				data: Default::default()
			},
		};
		assert_ok!(IncentivizedInboundChannel::submit(origin.clone(), message_1));
		let nonce: u64 = Nonce::get();
		assert_eq!(nonce, 1);

		// Submit message 2
		let message_2 = Message {
			data: MESSAGE_DATA_1.into(),
			proof: Proof {
				block_hash: Default::default(),
				tx_index: Default::default(),
				data: Default::default()
			},
		};
		assert_ok!(IncentivizedInboundChannel::submit(origin.clone(), message_2));
		let nonce: u64 = Nonce::get();
		assert_eq!(nonce, 2);
	});
}

#[test]
fn test_submit_with_invalid_nonce() {
	new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
		let relayer: AccountId = Keyring::Bob.into();
		let origin = Origin::signed(relayer);

		// Submit message
		let message = Message {
			data: MESSAGE_DATA_0.into(),
			proof: Proof {
				block_hash: Default::default(),
				tx_index: Default::default(),
				data: Default::default()
			},
		};
		assert_ok!(IncentivizedInboundChannel::submit(origin.clone(), message.clone()));
		let nonce: u64 = Nonce::get();
		assert_eq!(nonce, 1);

		// Submit the same again
		assert_noop!(
			IncentivizedInboundChannel::submit(origin.clone(), message.clone()),
			Error::<Test>::InvalidNonce
		);
	});
}