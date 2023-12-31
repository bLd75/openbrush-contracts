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

pub use crate::{
    psp22_pallet,
    psp22_pallet::extensions::metadata,
    traits::psp22::{
        extensions::metadata::*,
        *,
    },
};
pub use ink::env::DefaultEnvironment;
use openbrush::traits::Storage;
pub use openbrush::traits::String;
pub use pallet_assets_chain_extension::traits::{
    Error,
    Origin,
    PalletAssets,
};
pub use psp22_pallet::{
    Internal as _,
    InternalImpl as _,
    PSP22PalletImpl,
};

pub trait PSP22PalletMetadataImpl: Storage<psp22_pallet::Data> {
    fn token_name(&self) -> Option<String> {
        let self_ = self.data();
        let name = self_
            .pallet_assets
            .get_or_default()
            .metadata_name(self_.asset_id.get_or_default());

        if name.is_empty() {
            None
        } else {
            Some(String::from_utf8(name).expect("Invalid UTF-8 string for token"))
        }
    }

    fn token_symbol(&self) -> Option<String> {
        let self_ = self.data();
        let symbol = self_
            .pallet_assets
            .get_or_default()
            .metadata_symbol(self_.asset_id.get_or_default());

        if symbol.is_empty() {
            None
        } else {
            Some(String::from_utf8(symbol).expect("Invalid UTF-8 string for token"))
        }
    }

    fn token_decimals(&self) -> u8 {
        let self_ = self.data();
        self_
            .pallet_assets
            .get_or_default()
            .metadata_decimals(self_.asset_id.get_or_default())
    }
}
