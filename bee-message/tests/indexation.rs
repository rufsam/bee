// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_common::packable::Packable;
use bee_message::prelude::*;

#[test]
fn index_length_0_new() {
    assert!(matches!(
        Indexation::new("".to_string(), &[0x42, 0xff, 0x84, 0xa2, 0x42, 0xff, 0x84, 0xa2]),
        Err(Error::InvalidIndexationLength(0))
    ));
}

#[test]
fn index_length_0_unpack() {
    assert!(matches!(
        Indexation::unpack(&mut vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00].as_slice()),
        Err(Error::InvalidIndexationLength(0))
    ));
}

#[test]
fn index_length_32_new() {
    assert!(Indexation::new(".".repeat(32), &[0x42, 0xff, 0x84, 0xa2, 0x42, 0xff, 0x84, 0xa2]).is_ok());
}

#[test]
fn index_length_32_unpack() {
    assert!(Indexation::unpack(
        &mut vec![
            0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00
        ]
        .as_slice()
    )
    .is_ok(),);
}

#[test]
fn index_length_65_new() {
    assert!(matches!(
        Indexation::new(".".repeat(65), &[0x42, 0xff, 0x84, 0xa2, 0x42, 0xff, 0x84, 0xa2]),
        Err(Error::InvalidIndexationLength(65))
    ));
}

#[test]
fn index_length_65_unpack() {
    assert!(matches!(
        Indexation::unpack(
            &mut vec![
                0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00
            ]
            .as_slice()
        ),
        Err(Error::InvalidIndexationLength(65))
    ));
}

#[test]
fn packed_len() {
    let indexation = Indexation::new(
        "indexation".to_string(),
        &[0x42, 0xff, 0x84, 0xa2, 0x42, 0xff, 0x84, 0xa2],
    )
    .unwrap();
    assert_eq!(indexation.packed_len(), 24);
}

#[test]
fn pack_unpack() {
    let indexation_1 = Indexation::new(
        "indexation".to_string(),
        &[0x42, 0xff, 0x84, 0xa2, 0x42, 0xff, 0x84, 0xa2],
    )
    .unwrap();
    let bytes = indexation_1.pack_new();
    let indexation_2 = Indexation::unpack(&mut bytes.as_slice()).unwrap();

    assert_eq!(indexation_1.packed_len(), bytes.len());
    assert_eq!(indexation_1.index(), indexation_2.index());
    assert_eq!(indexation_1.data(), indexation_2.data());
    assert_eq!(indexation_1.hash(), indexation_2.hash())
}

// #[test]
// fn unpack_invalid_index_len() {
//     let indexation = Indexation::new(
//         "indexation".to_string(),
//         &[0x42, 0xff, 0x84, 0xa2, 0x42, 0xff, 0x84, 0xa2],
//     )
//     .unwrap();
//     let mut bytes = indexation.pack_new();
//     bytes[0..2].copy_from_slice(&1000u16.to_le_bytes());
//
//     assert!(matches!(
//         Indexation::unpack(&mut bytes.as_slice()),
//         Err(Error::Io { .. })
//     ));
// }
//
// #[test]
// fn unpack_invalid_data_len() {
//     let indexation = Indexation::new(
//         "indexation".to_string(),
//         &[0x42, 0xff, 0x84, 0xa2, 0x42, 0xff, 0x84, 0xa2],
//     )
//     .unwrap();
//     let mut bytes = indexation.pack_new();
//     bytes[14..18].copy_from_slice(&1000u32.to_le_bytes());
//
//     assert!(matches!(
//         Indexation::unpack(&mut bytes.as_slice()),
//         Err(Error::Io { .. })
//     ));
// }

#[test]
fn unpack_non_utf8_index() {
    let indexation = Indexation::new(
        unsafe { String::from_utf8_unchecked(vec![0, 159, 146, 150]) },
        &[0x42, 0xff, 0x84, 0xa2, 0x42, 0xff, 0x84, 0xa2],
    )
    .unwrap();
    let bytes = indexation.pack_new();

    assert!(matches!(
        Indexation::unpack(&mut bytes.as_slice()),
        Err(Error::Utf8String(std::string::FromUtf8Error { .. }))
    ));
}
