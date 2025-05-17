#![allow(missing_docs)]

use std::{fs, vec::Vec};

use cpio_reader::Mode;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct ExpectedEntryInfo {
    name: String,
    file: Vec<u8>,
    mode: Mode,
    uid: u32,
    gid: u32,
    ino: u32,
    mtime: u64,
    nlink: u32,
    dev: Option<u32>,
    devmajor: Option<u32>,
    devminor: Option<u32>,
    rdev: Option<u32>,
    rdevmajor: Option<u32>,
    rdevminor: Option<u32>,
}

#[test]
fn read_bin() {
    general_test("tests/bin.cpio", test_files_for_bin_and_odc());
}

#[test]
fn read_odc() {
    general_test("tests/odc.cpio", test_files_for_bin_and_odc());
}

#[test]
fn read_newc() {
    general_test("tests/newc.cpio", test_files_for_newc());
}

#[test]
fn read_crc() {
    general_test("tests/crc.cpio", test_files_for_crc());
}

// https://github.com/toku-sa-n/cpio_reader/pull/8
#[test]
fn file_and_entry_live_as_long_as_underlying_data() {
    let bin = fs::read("tests/crc.cpio").unwrap();

    let file_finder = |name_to_find: &str| {
        for entry in cpio_reader::iter_files(&bin) {
            if entry.name() == name_to_find {
                return Some((entry.name().to_string(), entry.file().to_vec()));
            }
        }
        None
    };

    assert_eq!(
        file_finder("magics/derich"),
        Some(("magics/derich".to_string(), "King\n".as_bytes().to_vec()))
    );
}

fn general_test(cpio_filename: &'static str, expected_entries_vec: Vec<ExpectedEntryInfo>) {
    let bin_cpio = fs::read(cpio_filename).unwrap();

    let collected_entries_vec = cpio_reader::iter_files(&bin_cpio)
        .map(|entry| ExpectedEntryInfo {
            name: entry.name().to_string(),
            file: entry.file().to_vec(),
            mode: entry.mode(),
            uid: entry.uid(),
            gid: entry.gid(),
            ino: entry.ino(),
            mtime: entry.mtime(),
            nlink: entry.nlink(),
            dev: entry.dev(),
            devmajor: entry.devmajor(),
            devminor: entry.devminor(),
            rdev: entry.rdev(),
            rdevmajor: entry.rdevmajor(),
            rdevminor: entry.rdevminor(),
        })
        .collect::<Vec<_>>();

    assert_eq!(
        collected_entries_vec.len(),
        expected_entries_vec.len(),
        "Number of entries mismatch for {}",
        cpio_filename
    );

    for (idx, (collected, expected)) in collected_entries_vec
        .iter()
        .zip(expected_entries_vec.iter())
        .enumerate()
    {
        assert_eq!(
            collected, expected,
            "Mismatch at entry index {} for file {}",
            idx, cpio_filename
        );
    }
}

fn test_files_for_bin_and_odc() -> Vec<ExpectedEntryInfo> {
    let mut v = Vec::new();

    v.push(ExpectedEntryInfo {
        name: "derich".to_string(),
        file: "skills/derich".as_bytes().to_vec(),
        mode: Mode::SYMBOLIK_LINK | Mode::from_bits_truncate(0o777),
        uid: 1000,
        gid: 1000,
        ino: 48820,
        mtime: 1629615560,
        nlink: 1,
        dev: Some(2050),
        devmajor: None,
        devminor: None,
        rdev: Some(0),
        rdevmajor: None,
        rdevminor: None,
    });

    v.push(ExpectedEntryInfo {
        name: "skills".to_string(),
        file: vec![],
        mode: Mode::DIRECTORY | Mode::from_bits_truncate(0o755),
        uid: 1000,
        gid: 1000,
        ino: 48818,
        mtime: 1629615694,
        nlink: 2,
        dev: Some(2050),
        devmajor: None,
        devminor: None,
        rdev: Some(0),
        rdevmajor: None,
        rdevminor: None,
    });

    v.push(ExpectedEntryInfo {
        name: "skills/derich".to_string(),
        file: "King\n".as_bytes().to_vec(),
        mode: Mode::REGULAR_FILE | Mode::from_bits_truncate(0o644),
        uid: 1000,
        gid: 1000,
        ino: 48825,
        mtime: 1629615520,
        nlink: 2,
        dev: Some(2050),
        devmajor: None,
        devminor: None,
        rdev: Some(0),
        rdevmajor: None,
        rdevminor: None,
    });

    v.push(ExpectedEntryInfo {
        name: "magics".to_string(),
        file: vec![],
        mode: Mode::DIRECTORY | Mode::from_bits_truncate(0o755),
        uid: 1000,
        gid: 1000,
        ino: 48823,
        mtime: 1629615554,
        nlink: 2,
        dev: Some(2050),
        devmajor: None,
        devminor: None,
        rdev: Some(0),
        rdevmajor: None,
        rdevminor: None,
    });

    v.push(ExpectedEntryInfo {
        name: "magics/derich".to_string(),
        file: "King\n".as_bytes().to_vec(),
        mode: Mode::REGULAR_FILE | Mode::from_bits_truncate(0o644),
        uid: 1000,
        gid: 1000,
        ino: 48825,
        mtime: 1629615520,
        nlink: 2,
        dev: Some(2050),
        devmajor: None,
        devminor: None,
        rdev: Some(0),
        rdevmajor: None,
        rdevminor: None,
    });

    v.push(ExpectedEntryInfo {
        name: "magics/rosemary".to_string(),
        file: "Mother green\n".as_bytes().to_vec(),
        mode: Mode::REGULAR_FILE | Mode::from_bits_truncate(0o644),
        uid: 1000,
        gid: 1000,
        ino: 48828,
        mtime: 1629615553,
        nlink: 1,
        dev: Some(2050),
        devmajor: None,
        devminor: None,
        rdev: Some(0),
        rdevmajor: None,
        rdevminor: None,
    });

    v
}

fn test_files_for_newc() -> Vec<ExpectedEntryInfo> {
    let mut v = Vec::new();

    v.push(ExpectedEntryInfo {
        name: "derich".to_string(),
        file: "skills/derich".as_bytes().to_vec(),
        mode: Mode::SYMBOLIK_LINK | Mode::from_bits_truncate(0o777),
        uid: 1000,
        gid: 1000,
        nlink: 1,
        ino: 380,
        mtime: 1747442230,
        devmajor: Some(0),
        devminor: Some(26),
        dev: None,
        rdev: None,
        rdevmajor: Some(0),
        rdevminor: Some(0),
    });

    v.push(ExpectedEntryInfo {
        name: "skills".to_string(),
        file: vec![],
        mode: Mode::DIRECTORY | Mode::from_bits_truncate(0o755),
        uid: 1000,
        gid: 1000,
        nlink: 2,
        ino: 378,
        mtime: 1747442230,
        devmajor: Some(0),
        devminor: Some(26),
        dev: None,
        rdev: None,
        rdevmajor: Some(0),
        rdevminor: Some(0),
    });

    v.push(ExpectedEntryInfo {
        name: "magics".to_string(),
        file: vec![],
        mode: Mode::DIRECTORY | Mode::from_bits_truncate(0o755),
        uid: 1000,
        gid: 1000,
        nlink: 2,
        ino: 376,
        mtime: 1747442230,
        devmajor: Some(0),
        devminor: Some(26),
        dev: None,
        rdev: None,
        rdevmajor: Some(0),
        rdevminor: Some(0),
    });

    v.push(ExpectedEntryInfo {
        name: "skills/derich".to_string(),
        file: vec![],
        mode: Mode::REGULAR_FILE | Mode::from_bits_truncate(0o644),
        uid: 1000,
        gid: 1000,
        nlink: 2,
        ino: 379,
        mtime: 1747442230,
        devmajor: Some(0),
        devminor: Some(26),
        dev: None,
        rdev: None,
        rdevmajor: Some(0),
        rdevminor: Some(0),
    });

    v.push(ExpectedEntryInfo {
        name: "magics/derich".to_string(),
        file: "King\n".as_bytes().to_vec(),
        mode: Mode::REGULAR_FILE | Mode::from_bits_truncate(0o644),
        uid: 1000,
        gid: 1000,
        nlink: 2,
        ino: 379,
        mtime: 1747442230,
        devmajor: Some(0),
        devminor: Some(26),
        dev: None,
        rdev: None,
        rdevmajor: Some(0),
        rdevminor: Some(0),
    });

    v.push(ExpectedEntryInfo {
        name: "magics/rosemary".to_string(),
        file: "Mother green\n".as_bytes().to_vec(),
        mode: Mode::REGULAR_FILE | Mode::from_bits_truncate(0o751),
        uid: 1000,
        gid: 1000,
        nlink: 1,
        ino: 377,
        mtime: 1747442230,
        devmajor: Some(0),
        devminor: Some(26),
        dev: None,
        rdev: None,
        rdevmajor: Some(0),
        rdevminor: Some(0),
    });
    v
}

fn test_files_for_crc() -> Vec<ExpectedEntryInfo> {
    let mut v = Vec::new();

    v.push(ExpectedEntryInfo {
        name: "derich".to_string(),
        file: "skills/derich".as_bytes().to_vec(),
        mode: Mode::SYMBOLIK_LINK | Mode::from_bits_truncate(0o777),
        uid: 1000,
        gid: 1000,
        nlink: 1,
        ino: 388,
        mtime: 1747442236,
        devmajor: Some(0),
        devminor: Some(26),
        dev: None,
        rdev: None,
        rdevmajor: Some(0),
        rdevminor: Some(0),
    });

    v.push(ExpectedEntryInfo {
        name: "skills".to_string(),
        file: vec![],
        mode: Mode::DIRECTORY | Mode::from_bits_truncate(0o755),
        uid: 1000,
        gid: 1000,
        nlink: 2,
        ino: 386,
        mtime: 1747442236,
        devmajor: Some(0),
        devminor: Some(26),
        dev: None,
        rdev: None,
        rdevmajor: Some(0),
        rdevminor: Some(0),
    });

    v.push(ExpectedEntryInfo {
        name: "magics".to_string(),
        file: vec![],
        mode: Mode::DIRECTORY | Mode::from_bits_truncate(0o755),
        uid: 1000,
        gid: 1000,
        nlink: 2,
        ino: 384,
        mtime: 1747442236,
        devmajor: Some(0),
        devminor: Some(26),
        dev: None,
        rdev: None,
        rdevmajor: Some(0),
        rdevminor: Some(0),
    });

    v.push(ExpectedEntryInfo {
        name: "skills/derich".to_string(),
        file: vec![],
        mode: Mode::REGULAR_FILE | Mode::from_bits_truncate(0o644),
        uid: 1000,
        gid: 1000,
        nlink: 2,
        ino: 387,
        mtime: 1747442236,
        devmajor: Some(0),
        devminor: Some(26),
        dev: None,
        rdev: None,
        rdevmajor: Some(0),
        rdevminor: Some(0),
    });

    v.push(ExpectedEntryInfo {
        name: "magics/derich".to_string(),
        file: "King\n".as_bytes().to_vec(),
        mode: Mode::REGULAR_FILE | Mode::from_bits_truncate(0o644),
        uid: 1000,
        gid: 1000,
        nlink: 2,
        ino: 387,
        mtime: 1747442236,
        devmajor: Some(0),
        devminor: Some(26),
        dev: None,
        rdev: None,
        rdevmajor: Some(0),
        rdevminor: Some(0),
    });

    v.push(ExpectedEntryInfo {
        name: "magics/rosemary".to_string(),
        file: "Mother green\n".as_bytes().to_vec(),
        mode: Mode::REGULAR_FILE | Mode::from_bits_truncate(0o751),
        uid: 1000,
        gid: 1000,
        nlink: 1,
        ino: 385,
        mtime: 1747442236,
        devmajor: Some(0),
        devminor: Some(26),
        dev: None,
        rdev: None,
        rdevmajor: Some(0),
        rdevminor: Some(0),
    });
    v
}
