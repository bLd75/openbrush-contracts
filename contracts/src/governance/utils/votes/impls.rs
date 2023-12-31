// Copyright (c) 2023 Brushfam
// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::{
    governance::utils::votes::*,
    nonces,
};
pub use crate::{
    governance::{
        governor::TimestampProvider,
        utils::votes,
    },
    traits::{
        errors::GovernanceError,
        governance::utils::votes::*,
        types::Signature,
    },
};
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
    Timestamp,
};
use scale::Encode;

/// Common interface for `PSP22Votes`, and other `Votes`-enabled contracts.
pub trait VotesImpl: Storage<Data> + VotesInternal + nonces::NoncesImpl + VotesEvents + TimestampProvider {
    /// The amount of votes owned by `account`.
    fn get_votes(&self, account: AccountId) -> Balance {
        self.data::<Data>()
            .delegate_checkpoints
            .get(&account)
            .unwrap_or_default()
            .latest()
    }

    /// The amount of votes delegated to `account` at the time `timestamp`.
    fn get_past_votes(&self, account: AccountId, timestamp: Timestamp) -> Result<Balance, GovernanceError> {
        let current_block_timestamp = TimestampProvider::block_timestamp(self);
        if timestamp > current_block_timestamp {
            return Err(GovernanceError::FutureLookup)
        }
        match self
            .data::<Data>()
            .delegate_checkpoints
            .get(&account)
            .unwrap_or_default()
            .upper_lookup_recent(timestamp)
        {
            Some(value) => Ok(value),
            None => Ok(0),
        }
    }

    /// The total amount of votes at the time `timestamp`.
    fn get_past_total_supply(&self, timestamp: Timestamp) -> Result<Balance, GovernanceError> {
        let current_block_timestamp = TimestampProvider::block_timestamp(self);
        if timestamp > current_block_timestamp {
            return Err(GovernanceError::FutureLookup)
        }

        let checkpoints = &self.data::<Data>().total_checkpoints.get_or_default();
        match checkpoints.upper_lookup_recent(timestamp) {
            Some(value) => Ok(value),
            None => Ok(0),
        }
    }

    /// Returns the address delegated to by `delegator`.
    fn delegates(&mut self, delegator: AccountId) -> Option<AccountId> {
        self._delegates(&Some(delegator))
    }

    /// Delegate votes from `signer` to `delegatee`.
    fn delegate(&mut self, delegatee: AccountId) -> Result<(), GovernanceError> {
        self._delegate(&Some(Self::env().caller()), &Some(delegatee))
    }

    /// Delegate votes from `signer` to `delegatee` using a signature.
    fn delegate_by_signature(
        &mut self,
        signer: AccountId,
        delegatee: AccountId,
        nonce: u64,
        expiry: Timestamp,
        signature: Signature,
    ) -> Result<(), GovernanceError> {
        if TimestampProvider::block_timestamp(self) > expiry {
            return Err(GovernanceError::ExpiredSignature)
        }

        let message = (&delegatee, &nonce, &expiry).encode();

        if !signature.verify(&message, &signer) {
            return Err(GovernanceError::InvalidSignature)
        } else {
            self._use_checked_nonce(&signer, nonce)?;
            self._delegate(&Some(signer), &Some(delegatee))
        }
    }
}
