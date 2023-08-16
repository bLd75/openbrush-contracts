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
    extensions::governor_votes::Data,
    traits::{
        errors::GovernanceError,
        governance::utils::*,
    },
};
use ink::{
    prelude::vec::Vec,
    primitives::AccountId,
};
use openbrush::traits::{
    Balance,
    Storage,
    Timestamp,
};

pub trait GovernorVotesInternal: Storage<Data> {
    fn _init_governor_votes(&mut self, token: AccountId) -> Result<(), GovernanceError> {
        self.data().token.set(&token);
        Ok(())
    }

    fn _get_votes(
        &mut self,
        account: AccountId,
        timepoint: Timestamp,
        _params: Vec<u8>,
    ) -> Result<Balance, GovernanceError> {
        let token = self.data().token.get().ok_or(GovernanceError::TokenNotSet)?;

        VotesRef::get_past_votes(&token, account, timepoint)
    }
}
