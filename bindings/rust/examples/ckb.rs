// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use blst::*;

// Benchmark min_pk
use blst::min_pk::*;

fn main() {
    let ikm: [u8; 32] = [
        0x93, 0xad, 0x7e, 0x65, 0xde, 0xad, 0x05, 0x2a, 0x08, 0x3a, 0x91, 0x0c, 0x8b, 0x72, 0x85,
        0x91, 0x46, 0x4c, 0xca, 0x56, 0x60, 0x5b, 0xb0, 0x56, 0xed, 0xfe, 0x2b, 0x60, 0xa6, 0x3c,
        0x48, 0x99,
    ];

    let sk = SecretKey::key_gen(&ikm, &[]).unwrap();
    let pk = sk.sk_to_pk();

    let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
    let msg = b"hello foo";
    let sig = sk.sign(msg, dst, &[]);

    let err = sig.verify(true, msg, dst, &[], &pk, true);
    assert_eq!(err, BLST_ERROR::BLST_SUCCESS);
}
