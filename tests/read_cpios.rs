use std::{collections::HashSet, fs, str};

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
    general_test("tests/newc.cpio", test_files_for_newc_and_crc());
}

#[test]
fn read_crc() {
    general_test("tests/crc.cpio", test_files_for_newc_and_crc());
}

fn general_test(name: &'static str, files: HashSet<FileInfo>) {
    let bin_cpio = fs::read(name).unwrap();

    let v = cpio_reader::iter_files(&bin_cpio)
        .into_iter()
        .map(|file| FileInfo {
            name: file.name().into(),
            contents: str::from_utf8(file.file()).unwrap().into(),
        })
        .collect::<HashSet<_>>();

    assert_eq!(v, files);
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct FileInfo {
    name: String,
    contents: String,
}

fn test_files_for_bin_and_odc() -> HashSet<FileInfo> {
    test_files(&[
        ("magics", ""),
        ("magics/rosemary", "Mother green\n"),
        ("magics/derich", "King\n"),
        ("skills", ""),
        ("skills/derich", "King\n"),
        ("derich", "skills/derich"),
    ])
}

fn test_files_for_newc_and_crc() -> HashSet<FileInfo> {
    test_files(&[
        ("magics", ""),
        ("magics/rosemary", "Mother green\n"),
        ("magics/derich", "King\n"),
        ("skills", ""),
        ("skills/derich", ""),
        ("derich", "skills/derich"),
    ])
}

fn test_files(name_and_contents: &[(&str, &str)]) -> HashSet<FileInfo> {
    let mut h = HashSet::new();

    for (name, contents) in name_and_contents {
        h.insert(FileInfo {
            name: name.to_string(),
            contents: contents.to_string(),
        });
    }

    h
}
