// FastFiles (internally known as XFiles) are structured as follows (all
// values native endian - that is, little endian for Windows and macOS, big
// endian for Xbox 360, PS3, and presumably Wii):
//
// ----------------------------------------------------------------------------
// | Offset    | Size | Field       | Description                             |
// ----------------------------------------------------------------------------
// | 0x0000000 | 4    | Magic       | Magic value to identify the file as an  |
// |           |      |             | XFile. Will always be ASCII "IWff".     |
// ----------------------------------------------------------------------------
// | 0x0000004 | 1    | Compression | Magic value to identify the             |
// |           |      |             | compression method used. Will always be |
// |           |      |             | ASCII '0' for Xbox 360 and PS3, and     |
// |           |      |             | *seems* to always be 'u' for Windows    |
// |           |      |             | (might be different for, e.g., modded   |
// |           |      |             | maps). Unsure for Wii, macOS is         |
// |           |      |             | presumably the same as Windows.         |
// ----------------------------------------------------------------------------
// | 0x0000005 | 3    | Unknown     | Exact meaning unknown. Maybe it was     |
// |           |      |             | supposed to represent some version info |
// |           |      |             | info? Will always be ASCII "100".       |
// ----------------------------------------------------------------------------
// | 0x0000008 | 4    | Version     | The real version. For reasons explained |
// |           |      |             | below, XFiles are neither backward- nor |
// |           |      |             | forward-compatible for deserialization  |
// |           |      |             | purposes. It is **imperative** that the |
// |           |      |             | XFile version match the version the     |
// |           |      |             | deserializer is expecting. For all      |
// |           |      |             | release builds of T5, that value is     |
// |           |      |             | 0x000001D9.                             |
// ----------------------------------------------------------------------------
// | 0x000000C | *    | Blob        | The rest of the file is a DEFLATE-      |
// |           |      |             | compressed blob. To get the "real"      |
// |           |      |             | contents of the file, it must be        |
// |           |      |             | inflated.                               |
// ----------------------------------------------------------------------------
//
// XFiles don't contain an easy way to detect the platform they're compiled
// for. The endianness of the Version field can serve as a simple sanity
// check (i.e., if the expected platform is Windows but Version is
// big-endian, then the platform is obviously wrong), but since both
// little- and big-endian have multiple potential platforms, the correct
// platform can't be derived for certain, and even if the endianness matches
// the expected platform, that's no guarantee the expected platform is correct.
//
// (In theory, one could probably use structure differences between platforms
// or other known values that differ between platforms to verify the correct
// platform, but someone else can do that.)
//
// The inflated blob is structured as follows:
//
// ----------------------------------------------------------------------------
// | Offset    | Size | Field       | Description                             |
// ----------------------------------------------------------------------------
// | 0x0000000 | 36   | XFile       | See the [`XFile`] struct below.         |
// ----------------------------------------------------------------------------
// | 0x0000024 | 16   | XAssetList  | See the [`XAssetList`] struct below.    |
// ----------------------------------------------------------------------------
// | 0x0000034 | *    | XAssets     | The XAssets.                            |
// ----------------------------------------------------------------------------
//
// The XAssetList essentially contains two fat pointers: first, to a string
// array, then an asset array. And herein comes the first major annoyance
// with XFiles - the assets are essentially just the structs used by the engine
// serialzed into a file. Any pointers in said structs become offsets in the
// file. Occasionally the offsets are NULL or a "real" value, but most of the
// time they're 0xFFFFFFFF or 0xFFFFFFFE, which indicates that, instead of being at a
// specific offset, they come immediately after the current struct. This means
// basically nothing in the file is relocatable.
//
// In addition, if the structures' sizes or alignments don't match exactly what
// the serializer used, or if new structures are added, the file is basically
// un-parseable (this is why, as mentioned above, the versions must match
// exactly). Pulling out only assets of a specific type or by name is also impossible,
// because you can't know where a given asset is at in the file until you pull
// out everything before it too. For this reason, you're more or less forced
// into deserializng everything at once and then grabbing the assets you need
// afterwards. Which, in fairness, makes sense in the context of a game engine (you're
// never going to need to load *half*, or some other fraction, of a level), but it
// *really* doesn't make writing a deserializer fun.

#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::missing_transmute_annotations)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::from_over_into)]
#![allow(clippy::needless_borrows_for_generic_args)]
//#![no_std]

extern crate alloc;

pub mod deserializer;
pub use deserializer::*;

pub mod serializer;
pub use serializer::*;

pub mod clipmap;
pub mod com_world;
pub mod common;
pub mod ddl;
pub mod destructible;
pub mod font;
pub mod fx;
pub mod gameworld;
pub mod gfx_world;
pub mod light;
pub mod menu;
pub mod misc;
pub mod sound;
pub mod techset;
pub mod util;
pub mod weapon;
pub mod xanim;
pub mod xasset;
pub mod xmodel;

use alloc::{
    boxed::Box, fmt::{Debug, Display}, string::String,
};

use std::io::{Read, Write};

use bincode::{
    config::{BigEndian, FixintEncoding, LittleEndian, WithOtherEndian, WithOtherIntEncoding},
    DefaultOptions, Options,
};
use serde::{de::DeserializeOwned, Deserialize};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "d3d9")]
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;

pub use misc::*;
use util::*;
use xasset::XAssetType;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XFileHeader {
    pub magic: [u8; 8],
    pub version: u32,
}
assert_size!(XFileHeader, 12);

impl XFileHeader {
    pub fn new(platform: XFilePlatform) -> Self {
        let magic = *b"IWffu100";
        let version = XFileVersion::from_platform(platform).as_u32();

        Self {
            magic,
            version,
        }
    }

    pub fn magic_string(&self) -> String {
        self.magic.iter().map(|c| *c as char).collect()
    }

    pub fn magic_is_valid(&self) -> bool {
        self.magic[0] == b'I'
            && self.magic[1] == b'W'
            && self.magic[2] == b'f'
            && self.magic[3] == b'f'
            && (self.magic[4] == b'u' || self.magic[4] == b'0')
            && self.magic[5] == b'1'
            && self.magic[6] == b'0'
            && self.magic[7] == b'0'
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XFile {
    pub size: u32,
    pub external_size: u32,
    pub block_size: [u32; 7],
}
assert_size!(XFile, 36);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
struct ScriptString(u16);

impl ScriptString {
    pub fn to_string(self, de: &mut T5XFileDeserializer) -> Result<String> {
        de.script_strings
            .get(self.0 as usize)
            .cloned()
            .ok_or(Error::new(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadScriptString(self.0),
            ))
    }
}

const XFILE_VERSION: u32 = 0x000001D9u32;
const XFILE_VERSION_LE: u32 = XFILE_VERSION.to_le();
const XFILE_VERSION_BE: u32 = XFILE_VERSION.to_be();

#[repr(u32)]
enum XFileVersion {
    LE = XFILE_VERSION_LE,
    BE = XFILE_VERSION_BE,
}

impl XFileVersion {
    fn is_valid(version: u32, platform: XFilePlatform) -> bool {
        Self::from_u32(version)
            .map(|v| v.as_u32())
            .unwrap_or(0xFFFFFFFF) // sentinel value to make life simple
            == Self::from_platform(platform).as_u32()
    }

    fn is_other_endian(version: u32, platform: XFilePlatform) -> bool {
        if platform.is_le() {
            version == Self::BE.as_u32()
        } else {
            version == Self::LE.as_u32()
        }
    }

    fn from_u32(value: u32) -> Option<Self> {
        match value {
            XFILE_VERSION_LE => Some(Self::LE),
            XFILE_VERSION_BE => Some(Self::BE),
            _ => None,
        }
    }

    fn from_platform(platform: XFilePlatform) -> Self {
        match platform {
            XFilePlatform::Windows | XFilePlatform::macOS => XFileVersion::LE,
            XFilePlatform::Xbox360 | XFilePlatform::PS3 => XFileVersion::BE,
            XFilePlatform::Wii => unreachable!(), // safe since the deserializer rejects Wii
                                                  // before this function ever gets called
        }
    }

    fn as_u32(&self) -> u32 {
        match self {
            Self::LE => XFILE_VERSION_LE,
            Self::BE => XFILE_VERSION_BE,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum XFilePlatform {
    Windows,
    macOS,
    Xbox360,
    PS3,
    Wii,
}

impl Display for XFilePlatform {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Self::Windows => "Windows",
            Self::macOS => "macOS",
            Self::Xbox360 => "Xbox 360",
            Self::PS3 => "PS3",
            Self::Wii => "Wii",
        };
        write!(f, "{}", s)
    }
}

impl XFilePlatform {
    pub fn is_le(&self) -> bool {
        match self {
            Self::Windows | Self::macOS => true,
            Self::Xbox360 | Self::PS3 => false,
            Self::Wii => unreachable!(), // safe since the deserializer rejects Wii
                                         // before this function ever gets called
        }
    }

    pub fn is_be(&self) -> bool {
        !self.is_le()
    }

    pub fn is_console(&self) -> bool {
        match self {
            Self::Xbox360 | Self::PS3 | Self::Wii => true,
            Self::Windows | Self::macOS => false,
        }
    }

    pub fn is_pc(&self) -> bool {
        !self.is_console()
    }
}

/// A simple enum that contains all the possible errors this library can return.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Occurs when a [`std::io`] function returns an error.
    Io(std::io::Error),
    /// Occurs when `bincode` couldn't deserialize an object.
    Bincode(Box<bincode::ErrorKind>),
    /// Occurs when an XFile's blob couldn't be inflated.
    Inflate(String),
    /// Occurs when `num::FromPrimitive::from_*` return [`None`].
    BadFromPrimitive(i64),
    /// Occurs when `bitflags::from_bits` returns [`None`].
    BadBitflags(u32),
    /// Occurs when a character has invalid encoding.
    BadChar(u32),
    /// Occurs when an invariant expected by the deserializer is broken.
    /// Likely indicates the file is corrupt or some deserialization logic is wrong
    BrokenInvariant(String),
    /// Occurs when attempting to seek to an offset beyond the bounds of a file.
    InvalidSeek { off: u32, max: u32 },
    /// Occurs when an XFile's `magic` field is invalid.
    /// Likely indicates the file is corrupt or isn't an XFile.
    BadHeaderMagic(String),
    /// Occurs when an XFile's version doesn't match the expected version ([`XFILE_VERSION`]).
    WrongVersion(u32),
    /// Occurs when an XFile has the wrong endianness for the given platform.
    WrongEndiannessForPlatform(XFilePlatform),
    /// Occurs when an XFile's platform is unimplemented (currently just Wii).
    UnimplementedPlatform(XFilePlatform),
    /// Occurs when an XFile's platform is unsupported (all platforms except Windows).
    UnsupportedPlatform(XFilePlatform), 
    /// Occurs when some part of the library hasn't yet been implemented.
    Todo(String),
    /// Occurs when a [`ScriptString`] doesn't index [`T5XFileDeserializer::script_strings`].
    BadScriptString(u16),
    /// Occurs when an `XAsset`'s `asset_type` isn't a variant of [`XAssetType`].
    InvalidXAssetType(u32),
    /// Occurs when an `XAsset`'s `asset_type` *is* a variant of [`XAssetType`],
    /// but that `asset_type` isn't used by T5.
    UnusedXAssetType(XAssetType),
    /// Occurs when an error is returned by D3D9.
    #[cfg(feature = "d3d9")]
    Windows(windows::core::Error),
}

impl From<std::io::Error> for ErrorKind {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<Box<bincode::ErrorKind>> for ErrorKind {
    fn from(value: Box<bincode::ErrorKind>) -> Self {
        Self::Bincode(value)
    }
}

impl From<String> for ErrorKind {
    fn from(value: String) -> Self {
        Self::Inflate(value)
    }
}

#[cfg(feature = "d3d9")]
impl From<windows::core::Error> for ErrorKind {
    fn from(value: windows::core::Error) -> Self {
        Self::Windows(value)
    }
}

macro_rules! file_line_col {
    () => {
        alloc::format!("{}:{}:{}", file!(), line!(), column!())
    };
}

pub(crate) use file_line_col;

#[derive(Debug)]
pub struct Error {
    where_: String,
    kind: ErrorKind,
    off: u32,
}

impl Error {
    pub(crate) fn new(where_: String, off: u32, kind: ErrorKind) -> Self {
        Self { where_, kind, off }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn where_(&self) -> String {
        self.where_.clone()
    }

    pub fn off(&self) -> u32 {
        self.off
    }
}

pub type Result<T> = core::result::Result<T, Error>;

type BincodeOptionsLE =
    WithOtherIntEncoding<WithOtherEndian<DefaultOptions, LittleEndian>, FixintEncoding>;
type BincodeOptionsBE =
    WithOtherIntEncoding<WithOtherEndian<DefaultOptions, BigEndian>, FixintEncoding>;

#[derive(Clone)]
enum BincodeOptions {
    LE(BincodeOptionsLE),
    BE(BincodeOptionsBE),
}

impl BincodeOptions {
    fn new(little_endian: bool) -> Self {
        if little_endian {
            BincodeOptions::LE(
                DefaultOptions::new()
                    .with_little_endian()
                    .with_fixint_encoding(),
            )
        } else {
            BincodeOptions::BE(
                DefaultOptions::new()
                    .with_big_endian()
                    .with_fixint_encoding(),
            )
        }
    }

    fn from_platform(platform: XFilePlatform) -> Self {
        Self::new(platform.is_le())
    }

    fn deserialize_from<T: DeserializeOwned>(&self, reader: impl Read) -> bincode::Result<T> {
        match self {
            Self::LE(opts) => opts.deserialize_from(reader),
            Self::BE(opts) => opts.deserialize_from(reader),
        }
    }

    fn serialize_into<T: Serialize>(&self, writer: impl Write, t: T) -> bincode::Result<()> {
        match self {
            Self::LE(opts) => opts.serialize_into(writer, &t),
            Self::BE(opts) => opts.serialize_into(writer, &t), 
        }
    }
}
