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

//! Extrinsincs weight information for example ERC721 pallet.
//!
//! Note that the following weights are used only for development.
//! In fact, weights shoudl be calculated using runtime benchmarking.

// ----------------------------------------------------------------------------
// Module imports and re-exports
// ----------------------------------------------------------------------------

use frame_support::weights::Weight;

use crate::traits::WeightInfo;

impl WeightInfo for () {
    fn mint() -> Weight {
        Weight::from_parts(195_000_000, 0)
    }

    fn transfer() -> Weight {
        Weight::from_parts(195_000_000, 0)
    }

    fn burn() -> Weight {
        Weight::from_parts(195_000_000, 0)
    }
}
