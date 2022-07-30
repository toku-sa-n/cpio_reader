#![doc = include_str!("../README.md")]
#![no_std]
#![deny(unsafe_code)]

use {
    bitflags::bitflags,
    core::{convert::TryInto, str},
};

/// Returns an iterator that iterates over each content of the given cpio file.
///
/// The iterator checks if the header of an entry is correct. If it is corrupt (e.g., wrong magic
/// value), the iterator stops iterating.
pub fn iter_files<'a>(cpio_binary: &'a [u8]) -> impl Iterator<Item = Entry<'a>> {
    Iter::new(cpio_binary)
}

/// An entry of a cpio file.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entry<'a> {
    dev: Option<u32>,
    devmajor: Option<u32>,
    devminor: Option<u32>,
    ino: u32,
    mode: Mode,
    uid: u32,
    gid: u32,
    nlink: u32,
    rdev: Option<u32>,
    rdevmajor: Option<u32>,
    rdevminor: Option<u32>,
    mtime: u64,
    name: &'a str,
    file: &'a [u8],
}
impl<'a> Entry<'a> {
    /// Returns the device number of the device which contained the file.
    ///
    /// This method returns [`None`] if the cpio file format is either New ASCII Format or New CRC
    /// Format. For these formats, use [`Entry::devmajor`] and [`Entry::devminor`].
    #[must_use]
    pub fn dev(&self) -> Option<u32> {
        self.dev
    }

    /// Returns the major number of the device which contained the file.
    ///
    /// This method returns [`None`] if the entry format is either Old Binary Format or Portable
    /// ASCII Format. For these formats, use [`Entry::dev`].
    #[must_use]
    pub fn devmajor(&self) -> Option<u32> {
        self.devmajor
    }

    /// Returns the minor number of the device which contained the file.
    ///
    /// This method returns [`None`] if the entry format is either Old Binary Format or Portable
    /// ASCII Format. For these formats, use [`Entry::dev`].
    #[must_use]
    pub fn devminor(&self) -> Option<u32> {
        self.devminor
    }

    /// Returns the inode number of the file.
    #[must_use]
    pub fn ino(&self) -> u32 {
        self.ino
    }

    /// Returns the [`Mode`] value of the file, which contains the file's permission information
    /// and file type.
    #[must_use]
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Returns the user id of the owner of the file.
    #[must_use]
    pub fn uid(&self) -> u32 {
        self.uid
    }

    /// Returns the group id of the owner of the file.
    #[must_use]
    pub fn gid(&self) -> u32 {
        self.gid
    }

    /// Returns the number of links to this file.
    #[must_use]
    pub fn nlink(&self) -> u32 {
        self.nlink
    }

    /// Returns the associated device number if the entry is block special device or character
    /// special device. For the other types of entries, the caller should not use this method.
    ///
    /// This method returns [`None`] if the entry format is either New ASCII Format or New CRC
    /// Format. For these formats, use [`Entry::rdevmajor`] and [`Entry::rdevminor`].
    #[must_use]
    pub fn rdev(&self) -> Option<u32> {
        self.rdev
    }

    /// Returns the associated device major number if the entry is block special device or
    /// character special device. For the other types of entries, the caller should not use this
    /// method.
    ///
    /// This method returns [`None`] if the entry format is either Old Binary Format or Portable
    /// ASCII Format. For these formats, use [`Entry::rdev`].
    #[must_use]
    pub fn rdevmajor(&self) -> Option<u32> {
        self.rdevmajor
    }

    /// Returns the associated device minor number if the entry is block special device or
    /// character special device. For the other types of entries, the caller should not use this
    /// method.
    ///
    /// This method returns [`None`] if the entry format is either Old Binary Format or Portable
    /// ASCII Format. For these formats, use [`Entry::rdev`].
    #[must_use]
    pub fn rdevminor(&self) -> Option<u32> {
        self.rdevminor
    }

    /// Returns the modification time of this file.
    #[must_use]
    pub fn mtime(&self) -> u64 {
        self.mtime
    }

    /// Returns the filename.
    #[must_use]
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// Returns the content of this file.
    ///
    /// This method returns the path to the original file if the file is a symbolic link. For the
    /// New ASCII Format and New CRC Format, this method returns an empty slice if the file is a
    /// hard link and is not the last entry of the multiple duplicate files.
    #[must_use]
    pub fn file(&self) -> &'a [u8] {
        self.file
    }

    fn interpret_as_old_binary(binary: &'a [u8]) -> Option<(Self, &'a [u8])> {
        const MAGIC: u16 = 0o070_707;

        let mut byte_array = ByteArray::new(binary);

        let magic = [byte_array.proceed_byte()?, byte_array.proceed_byte()?];

        let endianness = if u16::from_be_bytes(magic) == MAGIC {
            Endianness::Big
        } else if u16::from_le_bytes(magic) == MAGIC {
            Endianness::Little
        } else {
            return None;
        };

        let dev = byte_array.proceed_u16(endianness)?;
        let ino = byte_array.proceed_u16(endianness)?;
        let mode = byte_array.proceed_u16(endianness)?;
        let u_id = byte_array.proceed_u16(endianness)?;
        let g_id = byte_array.proceed_u16(endianness)?;
        let nlink = byte_array.proceed_u16(endianness)?;
        let r_dev = byte_array.proceed_u16(endianness)?;
        let mtime_most: u64 = byte_array.proceed_u16(endianness)?.into();
        let mtime_least: u64 = byte_array.proceed_u16(endianness)?.into();
        let namesize = byte_array.proceed_u16(endianness)?;
        let filesize_most_byte: u32 = byte_array.proceed_u16(endianness)?.into();
        let filesize_least_byte: u32 = byte_array.proceed_u16(endianness)?.into();

        let filesize = (filesize_most_byte << 16) | filesize_least_byte;

        if namesize == 0 {
            return None;
        }

        let name = byte_array.proceed_str((namesize - 1).into())?;

        byte_array.skip_bytes((namesize % 2 + 1).into()); // +1 for the terminating null character.

        let file = byte_array.proceed_bytes(filesize.try_into().unwrap())?;

        let mode = Mode::from_bits(mode.into())?;

        let old_binary = Self {
            dev: Some(dev.into()),
            devmajor: None,
            devminor: None,
            ino: ino.into(),
            mode,
            uid: u_id.into(),
            gid: g_id.into(),
            nlink: nlink.into(),
            rdev: Some(r_dev.into()),
            rdevmajor: None,
            rdevminor: None,
            mtime: (mtime_most << 16) | mtime_least,
            name,
            file,
        };

        byte_array.skip_bytes((filesize % 2).try_into().unwrap());

        Some((old_binary, byte_array.into_inner()))
    }

    fn interpret_as_portable_ascii(binary: &'a [u8]) -> Option<(Self, &'a [u8])> {
        const MAGIC: &str = "070707";

        let mut byte_array = ByteArray::new(binary);

        let magic = byte_array.proceed_str(6)?;

        if magic != MAGIC {
            return None;
        }

        let dev = byte_array.proceed_str_into_octal_u32(6)?;
        let ino = byte_array.proceed_str_into_octal_u32(6)?;
        let mode = byte_array.proceed_str_into_octal_u32(6)?;
        let u_id = byte_array.proceed_str_into_octal_u32(6)?;
        let g_id = byte_array.proceed_str_into_octal_u32(6)?;
        let nlink = byte_array.proceed_str_into_octal_u32(6)?;
        let r_dev = byte_array.proceed_str_into_octal_u32(6)?;
        let mtime = byte_array.proceed_str_into_octal_u64(11)?;
        let namesize = byte_array.proceed_str_into_octal_u32(6)?;
        let filesize = byte_array.proceed_str_into_octal_u64(11)?;

        if namesize == 0 {
            return None;
        }

        let name = byte_array.proceed_str((namesize - 1).try_into().unwrap())?;

        byte_array.skip_bytes(1); // For the terminating '\0'.

        let file = byte_array.proceed_bytes(filesize.try_into().unwrap())?;

        let mode = Mode::from_bits(mode)?;

        let portable_ascii = Self {
            dev: Some(dev),
            devmajor: None,
            devminor: None,
            ino,
            mode,
            uid: u_id,
            gid: g_id,
            nlink,
            rdev: Some(r_dev),
            rdevmajor: None,
            rdevminor: None,
            mtime,
            name,
            file,
        };

        Some((portable_ascii, byte_array.into_inner()))
    }

    fn interpret_as_new_ascii_or_crc(binary: &'a [u8]) -> Option<(Self, &'a [u8])> {
        const MAGIC_NEW_ASCII: &str = "070701";
        const MAGIC_CRC: &str = "070702";

        let mut byte_array = ByteArray::new(binary);

        let is_crc = match byte_array.proceed_str(6)? {
            MAGIC_CRC => true,
            MAGIC_NEW_ASCII => false,
            _ => return None,
        };

        let ino = byte_array.proceed_str_into_hex()?;
        let mode = byte_array.proceed_str_into_hex()?;
        let u_id = byte_array.proceed_str_into_hex()?;
        let g_id = byte_array.proceed_str_into_hex()?;
        let nlink = byte_array.proceed_str_into_hex()?;
        let mtime: u64 = byte_array.proceed_str_into_hex()?.into();
        let filesize = byte_array.proceed_str_into_hex()?;
        let devmajor = byte_array.proceed_str_into_hex()?;
        let devminor = byte_array.proceed_str_into_hex()?;
        let r_devmajor = byte_array.proceed_str_into_hex()?;
        let r_devminor = byte_array.proceed_str_into_hex()?;
        let namesize = byte_array.proceed_str_into_hex()?;
        let check = byte_array.proceed_str_into_hex()?;

        if namesize == 0 {
            return None;
        }

        let name = byte_array.proceed_str((namesize - 1).try_into().unwrap())?;

        // For the terminating `\0`.
        byte_array.skip_bytes(1);

        byte_array.skip_to_next_multiple_of_four();

        let file = byte_array.proceed_bytes(filesize.try_into().unwrap())?;

        let mode = Mode::from_bits(mode)?;

        let checksum = file
            .iter()
            .fold(0_u32, |acc, &x| acc.wrapping_add(x.into()));

        // Refer to line 1277, copyin.c, GNU cpio 2.13. It does not check the checksum of the
        // symbolic files.
        if is_crc && !mode.contains(Mode::SYMBOLIK_LINK) && (checksum != check) {
            return None;
        }

        let new_ascii = Self {
            ino,
            mode,
            uid: u_id,
            gid: g_id,
            nlink,
            mtime,
            dev: None,
            devmajor: Some(devmajor),
            devminor: Some(devminor),
            rdev: None,
            rdevmajor: Some(r_devmajor),
            rdevminor: Some(r_devminor),
            name,
            file,
        };

        byte_array.skip_to_next_multiple_of_four();

        Some((new_ascii, byte_array.into_inner()))
    }

    fn new(binary: &'a [u8]) -> Option<(Self, &'a [u8])> {
        Self::interpret_as_old_binary(binary)
            .or_else(|| Self::interpret_as_portable_ascii(binary))
            .or_else(|| Self::interpret_as_new_ascii_or_crc(binary))
            .filter(|(entry, _)| entry.name() != "TRAILER!!!")
    }
}

bitflags! {
    /// File information.
    pub struct Mode: u32 {
        /// User executable bit.
        const USER_EXECUTABLE = 0o000_001;
        /// User writable bit.
        const USER_WRITABLE = 0o000_002;
        /// User readable bit.
        const USER_READABLE = 0o000_004;

        /// Group executable bit.
        const GROUP_EXECUTABLE = 0o000_010;
        /// Group writable bit.
        const GROUP_WRITABLE = 0o000_020;
        /// Group readable bit.
        const GROUP_READABLE = 0o000_040;

        /// Executable bit for the other groups.
        const WORLD_EXECUTABLE = 0o000_100;
        /// Writable bit for the other groups.
        const WORLD_WRITABLE = 0o000_200;
        /// Readable bit for the other groups.
        const WORLD_READABLE = 0o000_400;

        /// Sticky bit.
        const STICKY = 0o001_000;
        /// SGID bit.
        const SGID = 0o002_000;
        /// SUID bit.
        const SUID = 0o004_000;

        /// Named pipe or FIFO.
        const NAMED_PIPE_FIFO = 0o010_000;
        /// Character special device.
        const CHARACTER_SPECIAL_DEVICE = 0o020_000;
        /// Directory.
        const DIRECTORY = 0o040_000;
        /// Block special device.
        const BLOCK_SPECIAL_DEVICE = 0o060_000;

        /// Regular file.
        const REGULAR_FILE = 0o100_000;
        /// Symbolik link.
        const SYMBOLIK_LINK = 0o120_000;
        /// Socket.
        const SOCKET = 0o140_000;
    }
}

struct Iter<'a>(&'a [u8]);
impl<'a> Iter<'a> {
    fn new(binary: &'a [u8]) -> Self {
        Self(binary)
    }
}
impl<'a> Iterator for Iter<'a> {
    type Item = Entry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            let (entry, remaining) = Entry::new(self.0)?;

            self.0 = remaining;

            Some(entry)
        }
    }
}

struct ByteArray<'a> {
    binary: &'a [u8],
    current: usize,
}
impl<'a> ByteArray<'a> {
    fn new(binary: &'a [u8]) -> Self {
        Self { binary, current: 0 }
    }

    fn into_inner(self) -> &'a [u8] {
        self.binary
    }

    fn proceed_byte(&mut self) -> Option<u8> {
        let byte = self.binary.first().copied()?;

        self.skip_bytes(1);

        Some(byte)
    }

    fn proceed_bytes(&mut self, n: usize) -> Option<&'a [u8]> {
        let bytes = self.binary.get(..n)?;

        self.skip_bytes(n);

        Some(bytes)
    }

    fn proceed_str_into_octal_u32(&mut self, n: usize) -> Option<u32> {
        self.proceed_str(n)
            .and_then(|s| u32::from_str_radix(s, 8).ok())
    }

    fn proceed_str_into_octal_u64(&mut self, n: usize) -> Option<u64> {
        self.proceed_str(n)
            .and_then(|s| u64::from_str_radix(s, 8).ok())
    }

    fn proceed_str_into_hex(&mut self) -> Option<u32> {
        self.proceed_str(8)
            .and_then(|s| u32::from_str_radix(s, 16).ok())
    }

    fn proceed_str(&mut self, n: usize) -> Option<&'a str> {
        self.proceed_bytes(n)
            .and_then(|bytes| str::from_utf8(bytes).ok())
    }

    fn proceed_u16(&mut self, endianness: Endianness) -> Option<u16> {
        Some(endianness.u8_array_to_u16([self.proceed_byte()?, self.proceed_byte()?]))
    }

    fn skip_to_next_multiple_of_four(&mut self) {
        self.skip_bytes((4 - self.current % 4) % 4);
    }

    fn skip_bytes(&mut self, n: usize) {
        self.binary = self.binary.get(n..).unwrap_or_default();
        self.current += n;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Endianness {
    Big,
    Little,
}
impl Endianness {
    fn u8_array_to_u16(self, bytes: [u8; 2]) -> u16 {
        match self {
            Self::Big => u16::from_be_bytes(bytes),
            Self::Little => u16::from_le_bytes(bytes),
        }
    }
}
