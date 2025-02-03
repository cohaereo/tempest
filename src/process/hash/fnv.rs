#![allow(dead_code)]

use paste::paste;
use ruint::{uint, Uint};
use std::ops::BitXor;

macro_rules! fnv_impl {
    (
        bits: $bits:expr,
        primitive: $primitive:ty,
        base: $base:expr,
        prime: $prime:expr,
    ) => {
        paste! {
            pub fn [< fnv0 _ $bits >](data: &[u8]) -> $primitive {
                let prime = $prime;
                let mut hash: $primitive = $primitive::from(0u8);
                for &byte in data {
                    hash = hash.wrapping_mul(prime).bitxor($primitive::from(byte));
                }
                hash
            }

            pub fn [< fnv1 _ $bits >](data: &[u8]) -> $primitive {
                let prime = $prime;
                let mut hash: $primitive = $base;
                for &byte in data {
                    hash = hash.wrapping_mul(prime).bitxor($primitive::from(byte));
                }
                hash
            }

            pub fn [< fnv1a _ $bits >](data: &[u8]) -> $primitive {
                let prime = $prime;
                let mut hash: $primitive = $base;
                for &byte in data {
                    hash = hash.bitxor($primitive::from(byte)).wrapping_mul(prime);
                }
                hash
            }
        }
    };
}

// FNV 32-bit
fnv_impl! {
    bits: 32,
    primitive: u32,
    base: 0x811c9dc5,
    prime: 0x01000193,
}

// FNV 64-bit
fnv_impl! {
    bits: 64,
    primitive: u64,
    base: 0xcbf29ce484222325,
    prime: 0x00000100000001b3,
}

// FNV 128-bit
fnv_impl! {
    bits: 128,
    primitive: u128,
    base: 0x6c62272e07bb014262b821756295c58d,
    prime: 0x0000000001000000000000000000013b,
}

// FNV 256-bit
fnv_impl! {
    bits: 256,
    primitive: Uint::<256, 4>,
    base: uint!(0xdd268dbcaac550362d98c384c4e576ccc8b1536847b6bbb31023b4c8caee0535_U256),
    prime: uint!(0x0000000000000000000001000000000000000000000000000000000000000163_U256),
}

// FNV 512-bit
fnv_impl! {
    bits: 512,
    primitive: Uint::<512, 8>,
    base: uint!(0xb86db0b1171f4416dca1e50f309990acac87d059c90000000000000000000d21e948f68a34c192f62ea79bc942dbe7ce182036415f56e34bac982aac4afe9fd9_U512),
    prime: uint!(0x00000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000157_U512),
}

// FNV 1024-bit
fnv_impl! {
    bits: 1024,
    primitive: Uint::<1024, 16>,
    base: uint!(0x0000000000000000005f7a76758ecc4d32e56d5a591028b74b29fc4223fdada16c3bf34eda3674da9a21d9000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004c6d7eb6e73802734510a555f256cc005ae556bde8cc9c6a93b21aff4b16c71ee90b3_U1024),
    prime: uint!(0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000018d_U1024),
}
