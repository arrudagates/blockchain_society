// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use proc_macro_error::abort;
use std::collections::HashMap;
use syn::{
    spanned::Spanned as _,
    token,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ItemMod {
    // attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    mod_token: token::Mod,
    pub ident: syn::Ident,
    brace: token::Brace,
    items: Vec<Item>,
}

impl From<syn::ItemMod> for ItemMod {
    fn from(module: syn::ItemMod) -> Self {
        let (brace, items) = match module.content {
            Some((brace, items)) => (brace, items),
            None => {
                abort!(module, "out-of-line subxt modules are not supported",)
            }
        };
        let items = items
            .into_iter()
            .map(<Item as From<syn::Item>>::from)
            .collect::<Vec<_>>();
        Self {
            vis: module.vis,
            mod_token: module.mod_token,
            ident: module.ident,
            brace,
            items,
        }
    }
}

impl ItemMod {
    pub fn type_substitutes(&self) -> HashMap<String, syn::TypePath> {
        self.items
            .iter()
            .filter_map(|item| {
                if let Item::Subxt(SubxtItem::TypeSubstitute {
                    generated_type_path,
                    substitute_with: substitute_type,
                }) = item
                {
                    Some((generated_type_path.clone(), substitute_type.clone()))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq)]
pub enum Item {
    Rust(syn::Item),
    Subxt(SubxtItem),
}

impl From<syn::Item> for Item {
    fn from(item: syn::Item) -> Self {
        if let syn::Item::Use(ref use_) = item {
            let substitute_attrs = use_
                .attrs
                .iter()
                .map(|attr| {
                    let meta = attr.parse_meta().unwrap_or_else(|e| {
                        abort!(attr.span(), "Error parsing attribute: {}", e)
                    });
                    <attrs::Subxt as darling::FromMeta>::from_meta(&meta).unwrap_or_else(
                        |e| abort!(attr.span(), "Error parsing attribute meta: {}", e),
                    )
                })
                .collect::<Vec<_>>();
            if substitute_attrs.len() > 1 {
                abort!(
                    use_.attrs[0].span(),
                    "Duplicate `substitute_type` attributes"
                )
            }
            if let Some(attr) = substitute_attrs.get(0) {
                let use_path = &use_.tree;
                let substitute_with: syn::TypePath = syn::parse_quote!( #use_path );
                let type_substitute = SubxtItem::TypeSubstitute {
                    generated_type_path: attr.substitute_type(),
                    substitute_with,
                };
                Self::Subxt(type_substitute)
            } else {
                Self::Rust(item)
            }
        } else {
            Self::Rust(item)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SubxtItem {
    TypeSubstitute {
        generated_type_path: String,
        substitute_with: syn::TypePath,
    },
}

mod attrs {
    use darling::FromMeta;

    #[derive(Debug, FromMeta)]
    #[darling(rename_all = "snake_case")]
    pub enum Subxt {
        SubstituteType(String),
    }

    impl Subxt {
        pub fn substitute_type(&self) -> String {
            match self {
                Self::SubstituteType(path) => path.clone(),
            }
        }
    }
}
