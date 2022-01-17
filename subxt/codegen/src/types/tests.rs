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

use super::*;
use pretty_assertions::assert_eq;
use scale_info::{
    meta_type,
    Registry,
    TypeInfo,
};

const MOD_PATH: &[&str] = &["subxt_codegen", "types", "tests"];

fn get_mod<'a>(module: &'a Module, path_segs: &[&'static str]) -> Option<&'a Module<'a>> {
    let (mod_name, rest) = path_segs.split_first()?;
    let mod_ident = Ident::new(mod_name, Span::call_site());
    let module = module.children.get(&mod_ident)?;
    if rest.is_empty() {
        Some(module)
    } else {
        get_mod(module, rest)
    }
}

#[test]
fn generate_struct_with_primitives() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: bool,
        b: u32,
        c: char,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<S>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;

                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct S {
                    pub a: ::core::primitive::bool,
                    pub b: ::core::primitive::u32,
                    pub c: ::core::primitive::char,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn generate_struct_with_a_struct_field() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Parent {
        a: bool,
        b: Child,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Child {
        a: i32,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<Parent>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;

                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct Child {
                    pub a: ::core::primitive::i32,
                }

                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct Parent {
                    pub a: ::core::primitive::bool,
                    pub b: root::subxt_codegen::types::tests::Child,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn generate_tuple_struct() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Parent(bool, Child);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Child(i32);

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<Parent>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;

                    #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                    pub struct Child(pub ::core::primitive::i32,);

                    #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                    pub struct Parent(pub ::core::primitive::bool, pub root::subxt_codegen::types::tests::Child,);
                }
            }
                .to_string()
        )
}

#[test]
fn derive_compact_as_for_uint_wrapper_structs() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su8 {
        a: u8,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu8(u8);
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su16 {
        a: u16,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu16(u16);
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su32 {
        a: u32,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu32(u32);
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su64 {
        a: u64,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu64(u64);
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su128 {
        a: u128,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu128(u128);

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<Su8>());
    registry.register_type(&meta_type::<TSu8>());
    registry.register_type(&meta_type::<Su16>());
    registry.register_type(&meta_type::<TSu16>());
    registry.register_type(&meta_type::<Su32>());
    registry.register_type(&meta_type::<TSu32>());
    registry.register_type(&meta_type::<Su64>());
    registry.register_type(&meta_type::<TSu64>());
    registry.register_type(&meta_type::<Su128>());
    registry.register_type(&meta_type::<TSu128>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;

                #[derive(::subxt::codec::CompactAs)]
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct Su128 { pub a: ::core::primitive::u128, }

                #[derive(::subxt::codec::CompactAs)]
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct Su16 { pub a: ::core::primitive::u16, }

                #[derive(::subxt::codec::CompactAs)]
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct Su32 { pub a: ::core::primitive::u32, }

                #[derive(::subxt::codec::CompactAs)]
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct Su64 { pub a: ::core::primitive::u64, }

                #[derive(::subxt::codec::CompactAs)]
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct Su8 { pub a: ::core::primitive::u8, }

                #[derive(::subxt::codec::CompactAs)]
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct TSu128(pub ::core::primitive::u128,);

                #[derive(::subxt::codec::CompactAs)]
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct TSu16(pub ::core::primitive::u16,);

                #[derive(::subxt::codec::CompactAs)]
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct TSu32(pub ::core::primitive::u32,);

                #[derive(::subxt::codec::CompactAs)]
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct TSu64(pub ::core::primitive::u64,);

                #[derive(::subxt::codec::CompactAs)]
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct TSu8(pub ::core::primitive::u8,);
            }
        }
        .to_string()
    )
}

#[test]
fn generate_enum() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    enum E {
        A,
        B(bool),
        C { a: u32 },
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<E>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub enum E {
                    # [codec (index = 0)]
                    A,
                    # [codec (index = 1)]
                    B (::core::primitive::bool,),
                    # [codec (index = 2)]
                    C { a: ::core::primitive::u32, },
                }
            }
        }
        .to_string()
    )
}

#[test]
fn compact_fields() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        #[codec(compact)]
        a: u32,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TupleStruct(#[codec(compact)] u32);

    #[allow(unused)]
    #[derive(TypeInfo)]
    enum E {
        A {
            #[codec(compact)]
            a: u32,
        },
        B(#[codec(compact)] u32),
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<S>());
    registry.register_type(&meta_type::<TupleStruct>());
    registry.register_type(&meta_type::<E>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub enum E {
                    # [codec (index = 0)]
                    A {
                        #[codec(compact)]
                        a: ::core::primitive::u32,
                    },
                    # [codec (index = 1)]
                    B( #[codec(compact)] ::core::primitive::u32,),
                }

                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct S {
                    #[codec(compact)] pub a: ::core::primitive::u32,
                }

                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct TupleStruct(#[codec(compact)] pub ::core::primitive::u32,);
            }
        }
        .to_string()
    )
}

#[test]
fn generate_array_field() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: [u8; 32],
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<S>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct S {
                    pub a: [::core::primitive::u8; 32usize],
                }
            }
        }
        .to_string()
    )
}

#[test]
fn option_fields() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: Option<bool>,
        b: Option<u32>,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<S>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct S {
                    pub a: ::core::option::Option<::core::primitive::bool>,
                    pub b: ::core::option::Option<::core::primitive::u32>,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn box_fields_struct() {
    use std::boxed::Box;

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: std::boxed::Box<bool>,
        b: Box<u32>,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<S>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct S {
                    pub a: ::std::boxed::Box<::core::primitive::bool>,
                    pub b: ::std::boxed::Box<::core::primitive::u32>,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn box_fields_enum() {
    use std::boxed::Box;

    #[allow(unused)]
    #[derive(TypeInfo)]
    enum E {
        A(Box<bool>),
        B { a: Box<u32> },
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<E>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub enum E {
                    # [codec (index = 0)]
                    A(::std::boxed::Box<::core::primitive::bool>,),
                    # [codec (index = 1)]
                    B { a: ::std::boxed::Box<::core::primitive::u32>, },
                }
            }
        }
        .to_string()
    )
}

#[test]
fn range_fields() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: core::ops::Range<u32>,
        b: core::ops::RangeInclusive<u32>,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<S>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct S {
                    pub a: ::core::ops::Range<::core::primitive::u32>,
                    pub b: ::core::ops::RangeInclusive<::core::primitive::u32>,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn generics() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Foo<T> {
        a: T,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Bar {
        b: Foo<u32>,
        c: Foo<u8>,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<Bar>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct Bar {
                    pub b: root::subxt_codegen::types::tests::Foo<::core::primitive::u32>,
                    pub c: root::subxt_codegen::types::tests::Foo<::core::primitive::u8>,
                }
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct Foo<_0> {
                    pub a: _0,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn generics_nested() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Foo<T, U> {
        a: T,
        b: Option<(T, U)>,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Bar<T> {
        b: Foo<T, u32>,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<Bar<bool>>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct Bar<_0> {
                    pub b: root::subxt_codegen::types::tests::Foo<_0, ::core::primitive::u32>,
                }

                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct Foo<_0, _1> {
                    pub a: _0,
                    pub b: ::core::option::Option<(_0, _1,)>,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn generate_bitvec() {
    use bitvec::{
        order::{
            Lsb0,
            Msb0,
        },
        vec::BitVec,
    };

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        lsb: BitVec<Lsb0, u8>,
        msb: BitVec<Msb0, u16>,
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<S>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                pub struct S {
                    pub lsb: ::subxt::bitvec::vec::BitVec<root::bitvec::order::Lsb0, ::core::primitive::u8>,
                    pub msb: ::subxt::bitvec::vec::BitVec<root::bitvec::order::Msb0, ::core::primitive::u16>,
                }
            }
        }
        .to_string()
    )
}

#[test]
fn generics_with_alias_adds_phantom_data_marker() {
    trait Trait {
        type Type;
    }

    impl Trait for bool {
        type Type = u32;
    }

    type Foo<T> = <T as Trait>::Type;
    type Bar<T, U> = (<T as Trait>::Type, <U as Trait>::Type);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct NamedFields<T: Trait> {
        b: Foo<T>,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct UnnamedFields<T: Trait, U: Trait>(Bar<T, U>);

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<NamedFields<bool>>());
    registry.register_type(&meta_type::<UnnamedFields<bool, bool>>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    #[derive(::subxt::codec::CompactAs)]
                    #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                    pub struct NamedFields<_0> {
                        pub b: ::core::primitive::u32,
                        #[codec(skip)] pub __subxt_unused_type_params: ::core::marker::PhantomData<_0>,
                    }
                    #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                    pub struct UnnamedFields<_0, _1> (
                        pub (::core::primitive::u32, ::core::primitive::u32,),
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _1)>,
                    );
                }
            }
                .to_string()
        )
}

#[test]
fn modules() {
    mod m {
        pub mod a {
            #[allow(unused)]
            #[derive(scale_info::TypeInfo)]
            pub struct Foo {}

            pub mod b {
                #[allow(unused)]
                #[derive(scale_info::TypeInfo)]
                pub struct Bar {
                    a: super::Foo,
                }
            }
        }

        pub mod c {
            #[allow(unused)]
            #[derive(scale_info::TypeInfo)]
            pub struct Foo {
                a: super::a::b::Bar,
            }
        }
    }

    let mut registry = Registry::new();
    registry.register_type(&meta_type::<m::c::Foo>());
    let portable_types: PortableRegistry = registry.into();

    let type_gen = TypeGenerator::new(
        &portable_types,
        "root",
        Default::default(),
        Default::default(),
    );
    let types = type_gen.generate_types_mod();
    let tests_mod = get_mod(&types, MOD_PATH).unwrap();

    assert_eq!(
        tests_mod.into_token_stream().to_string(),
        quote! {
            pub mod tests {
                use super::root;
                pub mod m {
                    use super::root;
                    pub mod a {
                        use super::root;

                        pub mod b {
                            use super::root;

                            #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                            pub struct Bar {
                                pub a: root::subxt_codegen::types::tests::m::a::Foo,
                            }
                        }

                        #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                        pub struct Foo {}
                    }

                    pub mod c {
                        use super::root;

                        #[derive(::subxt::codec::Encode, ::subxt::codec::Decode)]
                        pub struct Foo {
                            pub a: root::subxt_codegen::types::tests::m::a::b::Bar,
                        }
                    }
                }
            }
        }
        .to_string()
    )
}
