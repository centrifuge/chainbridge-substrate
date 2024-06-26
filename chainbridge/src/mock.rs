// Copyright 2021 Centrifuge Foundation (centrifuge.io).
// This file is part of Centrifuge chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! Mocking runtime for testing the Substrate/Ethereum chains bridging pallet.
//!
//! The main components implemented in this mock module is a mock runtime
//! and some helper functions.

// ----------------------------------------------------------------------------
// Module imports and re-exports
// ----------------------------------------------------------------------------

// Import crate types, traits and constants
use crate::{
    self as pallet_chainbridge, constants::DEFAULT_RELAYER_VOTE_THRESHOLD, ChainId,
    Config as ChainBridgePalletConfig, ResourceId, WeightInfo,
};
use sp_runtime::BuildStorage;
use sp_std::convert::{TryFrom, TryInto};

// Import Substrate primitives and components
use frame_support::{
    assert_ok, parameter_types,
    traits::{Everything, SortedMembers},
    weights::Weight,
    PalletId,
};

use frame_system::mocking::MockBlock;

use sp_core::H256;

use sp_io::TestExternalities;

use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

// ----------------------------------------------------------------------------
// Types and constants declaration
// ----------------------------------------------------------------------------

type Balance = u64;

// Runtime mocking types definition
type Block = MockBlock<MockRuntime>;

pub type SystemCall = frame_system::Call<MockRuntime>;

// Implement testing extrinsic weights for the pallet
pub struct MockWeightInfo;
impl WeightInfo for MockWeightInfo {
    fn set_threshold() -> Weight {
        Weight::from_parts(0, 0)
    }

    fn set_resource() -> Weight {
        Weight::from_parts(0, 0)
    }

    fn remove_resource() -> Weight {
        Weight::from_parts(0, 0)
    }

    fn whitelist_chain() -> Weight {
        Weight::from_parts(0, 0)
    }

    fn add_relayer() -> Weight {
        Weight::from_parts(0, 0)
    }

    fn remove_relayer() -> Weight {
        Weight::from_parts(0, 0)
    }

    fn acknowledge_proposal(_: Weight) -> Weight {
        Weight::from_parts(0, 0)
    }

    fn reject_proposal() -> Weight {
        Weight::from_parts(0, 0)
    }

    fn eval_vote_state(_: Weight) -> Weight {
        Weight::from_parts(0, 0)
    }
}

// Constants definition
pub(crate) const RELAYER_A: u64 = 0x2;
pub(crate) const RELAYER_B: u64 = 0x3;
pub(crate) const RELAYER_C: u64 = 0x4;
pub(crate) const ENDOWED_BALANCE: u64 = 100_000_000;
pub(crate) const TEST_RELAYER_VOTE_THRESHOLD: u32 = 2;

// ----------------------------------------------------------------------------
// Mock runtime configuration
// ----------------------------------------------------------------------------

// Build mock runtime
frame_support::construct_runtime!(
    pub enum MockRuntime
    {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        ChainBridge: pallet_chainbridge::{Pallet, Call, Storage, Event<T>},
    }
);

// Parameterize default test user identifier (with id 1)
parameter_types! {
    pub const TestUserId: u64 = 1;
}

impl SortedMembers<u64> for TestUserId {
    fn sorted_members() -> Vec<u64> {
        vec![1]
    }
}

// Parameterize FRAME system pallet
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = Weight::from_parts(1024, 0);
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
    pub const MaxLocks: u32 = 100;
}

// Implement FRAME system pallet configuration trait for the mock runtime
impl frame_system::Config for MockRuntime {
    type BaseCallFilter = Everything;
    type RuntimeTask = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type PalletInfo = PalletInfo;
    type BlockWeights = ();
    type BlockLength = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type Nonce = u64;
    type Block = Block;
}

// Parameterize FRAME balances pallet
parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}

// Implement FRAME balances pallet configuration trait for the mock runtime
impl pallet_balances::Config for MockRuntime {
    type Balance = Balance;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
    type WeightInfo = ();
    type MaxFreezes = ();
    type FreezeIdentifier = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
}

// Parameterize chainbridge pallet
parameter_types! {
    pub const MockChainId: ChainId = 5;
    pub const ChainBridgePalletId: PalletId = PalletId(*b"cb/bridg");
    pub const ProposalLifetime: u64 = 10;
    pub const RelayerVoteThreshold: u32 = DEFAULT_RELAYER_VOTE_THRESHOLD;
}

// Implement chainbridge pallet configuration trait for the mock runtime
impl ChainBridgePalletConfig for MockRuntime {
    type RuntimeEvent = RuntimeEvent;
    type Proposal = RuntimeCall;
    type ChainId = MockChainId;
    type PalletId = ChainBridgePalletId;
    type AdminOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type ProposalLifetime = ProposalLifetime;
    type RelayerVoteThreshold = RelayerVoteThreshold;
    type WeightInfo = MockWeightInfo;
}

// ----------------------------------------------------------------------------
// Test externalities
// ----------------------------------------------------------------------------

// Test externalities builder type declaraction.
//
// This type is mainly used for mocking storage in tests. It is the type alias
// for an in-memory, hashmap-based externalities implementation.
pub struct TestExternalitiesBuilder {}

// Default trait implementation for test externalities builder
impl Default for TestExternalitiesBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl TestExternalitiesBuilder {
    // Build a genesis storage key/value store
    pub(crate) fn build(self) -> TestExternalities {
        let bridge_id = ChainBridge::account_id();

        let mut storage = frame_system::GenesisConfig::<MockRuntime>::default()
            .build_storage()
            .unwrap();

        // pre-fill balances
        pallet_balances::GenesisConfig::<MockRuntime> {
            balances: vec![(bridge_id, ENDOWED_BALANCE)],
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        let mut externalities = TestExternalities::new(storage);
        externalities.execute_with(|| System::set_block_number(1));
        externalities
    }

    // Build a genesis storage with a pre-configured chainbridge
    pub(crate) fn build_with(
        self,
        src_id: ChainId,
        r_id: ResourceId,
        resource: Vec<u8>,
    ) -> TestExternalities {
        let mut externalities = Self::build(self);

        externalities.execute_with(|| {
            // Set and check threshold
            assert_ok!(ChainBridge::set_threshold(
                RuntimeOrigin::root(),
                TEST_RELAYER_VOTE_THRESHOLD
            ));
            assert_eq!(ChainBridge::get_threshold(), TEST_RELAYER_VOTE_THRESHOLD);
            // Add relayers
            assert_ok!(ChainBridge::add_relayer(RuntimeOrigin::root(), RELAYER_A));
            assert_ok!(ChainBridge::add_relayer(RuntimeOrigin::root(), RELAYER_B));
            assert_ok!(ChainBridge::add_relayer(RuntimeOrigin::root(), RELAYER_C));
            // Whitelist chain
            assert_ok!(ChainBridge::whitelist_chain(RuntimeOrigin::root(), src_id));
            // Set and check resource ID mapped to some junk data
            assert_ok!(ChainBridge::set_resource(
                RuntimeOrigin::root(),
                r_id,
                resource
            ));
            assert_eq!(ChainBridge::resource_exists(r_id), true);
        });

        externalities
    }
}

// ----------------------------------------------------------------------------
// Helper functions
// ----------------------------------------------------------------------------

pub mod helpers {

    use super::{MockRuntime, RuntimeEvent};

    // Checks events against the latest. A contiguous set of events must be provided. They must
    // include the most recent event, but do not have to include every past event.
    pub fn assert_events(mut expected: Vec<RuntimeEvent>) {
        let mut actual: Vec<RuntimeEvent> = frame_system::Pallet::<MockRuntime>::events()
            .iter()
            .map(|e| e.event.clone())
            .collect();

        expected.reverse();

        for evt in expected {
            let next = actual.pop().expect("event expected");
            assert_eq!(next, evt.into(), "Events don't match (actual,expected)");
        }
    }
} // end of 'helpers' inner module
