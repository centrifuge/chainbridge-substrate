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

//! Extrinsincs weight information for Substrate/Ethereum chains bridging pallet.
//! 
//! Note that the following weights are used only for development.
//! In fact, weights shoudl be calculated using runtime benchmarking.

// ----------------------------------------------------------------------------
// Module imports and re-exports
// ----------------------------------------------------------------------------

use frame_support::weights::Weight;

use crate::traits::WeightInfo;


impl WeightInfo for () {
    fn set_threshold() -> Weight {
        195_000_000 as Weight
    }

    fn set_resource() -> Weight {
        195_000_000 as Weight
    }
    
    fn remove_resource() -> Weight {
        195_000_000 as Weight
    }

    fn whitelist_chain() -> Weight {
        195_000_000 as Weight    
    }

    fn add_relayer() -> Weight {
        195_000_000 as Weight
    }

    fn remove_relayer() -> Weight {
        195_000_000 as Weight
    }

    // #[weight = (call.get_dispatch_info().weight + 195_000_000, call.get_dispatch_info().class, Pays::Yes)]
    fn acknowledge_proposal() -> Weight {
        195_000_000 as Weight
    }

    fn reject_proposal() -> Weight {
        195_000_000 as Weight
    }

    // #[weight = (prop.get_dispatch_info().weight + 195_000_000, prop.get_dispatch_info().class, Pays::Yes)]
    fn eval_vote_state() -> Weight {
        195_000_000 as Weight
    }
}