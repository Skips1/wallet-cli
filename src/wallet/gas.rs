// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! This library contains the primitive related to the gas used for transaction
//! in the Dusk Network.

use crate::currency::Lux;

/// The minimum gas limit
pub const MIN_LIMIT: u64 = 350_000_000;

/// The default gas limit
pub const DEFAULT_LIMIT: u64 = 500_000_000;

/// The default gas price
pub const DEFAULT_PRICE: Lux = 1;

#[derive(Debug)]
/// Gas price and limit for any transaction
pub struct Gas {
    /// The gas price in [Lux]
    pub price: Lux,
    /// The gas limit
    pub limit: u64,
}

impl Gas {
    /// Default gas price and limit
    pub fn new() -> Self {
        Gas {
            price: DEFAULT_PRICE,
            limit: DEFAULT_LIMIT,
        }
    }

    /// Returns `true` if the gas is equal or greater than the minimum limit
    pub fn is_enough(&self) -> bool {
        self.limit >= MIN_LIMIT
    }

    /// Set the price
    pub fn set_price<T>(&mut self, price: T)
    where
        T: Into<Option<Lux>>,
    {
        self.price = price.into().unwrap_or(DEFAULT_PRICE);
    }

    /// Set the limit
    pub fn set_limit<T>(&mut self, limit: T)
    where
        T: Into<Option<u64>>,
    {
        self.limit = limit.into().unwrap_or(DEFAULT_LIMIT)
    }
}

impl Default for Gas {
    fn default() -> Self {
        Self::new()
    }
}
