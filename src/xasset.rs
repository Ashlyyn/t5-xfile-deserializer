use alloc::boxed::Box;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::{
    assert_size,
    clipmap::{ClipMap, ClipMapRaw},
    com_world::{ComWorld, ComWorldRaw},
    ddl::{DdlRoot, DdlRootRaw},
    destructible::{DestructibleDef, DestructibleDefRaw},
    file_line_col,
    font::{Font, FontRaw},
    fx::{FxEffectDef, FxEffectDefRaw, FxImpactTable, FxImpactTableRaw},
    gameworld::{GameWorldMp, GameWorldMpRaw, GameWorldSp, GameWorldSpRaw},
    gfx_world::{GfxWorld, GfxWorldRaw},
    light::{GfxLightDef, GfxLightDefRaw},
    menu::{MenuDef, MenuDefRaw, MenuList, MenuListRaw},
    sound::{SndBank, SndBankRaw, SndDriverGlobals, SndDriverGlobalsRaw, SndPatch, SndPatchRaw},
    techset::{
        GfxImage, GfxImageRaw, Material, MaterialRaw, MaterialTechniqueSet, MaterialTechniqueSetRaw,
    },
    weapon::{WeaponVariantDef, WeaponVariantDefRaw},
    xanim::{XAnimParts, XAnimPartsRaw},
    xmodel::{PhysConstraints, PhysConstraintsRaw, PhysPreset, PhysPresetRaw, XModel, XModelRaw},
    EmblemSet, EmblemSetRaw, Error, ErrorKind, FatPointerCountFirstU32, Glasses, GlassesRaw,
    LocalizeEntry, LocalizeEntryRaw, MapEnts, MapEntsRaw, PackIndex, PackIndexRaw, Ptr32, RawFile,
    RawFileRaw, Result, StringTable, StringTableRaw, T5XFileDeserializer, T5XFileSerializer,
    XFileDeserializeInto, XFilePlatform, XFileSerializeInto, XGlobals, XGlobalsRaw, XString,
};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum XAsset {
    PC(XAssetGeneric<1>),
    Console(XAssetGeneric<4>),
}

impl XAsset {
    pub(crate) fn try_get(
        de: &mut T5XFileDeserializer,
        xasset_raw: XAssetRaw,
        platform: XFilePlatform,
    ) -> Result<Self> {
        let asset = if platform.is_pc() {
            Self::PC(xasset_raw.xfile_deserialize_into(de, ())?)
        } else {
            Self::Console(xasset_raw.xfile_deserialize_into(de, ())?)
        };
        Ok(asset)
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            Self::PC(a) => a.name(),
            Self::Console(a) => a.name(),
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            Self::PC(a) => a.is_some(),
            Self::Console(a) => a.is_some(),
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn is_pc(&self) -> bool {
        match self {
            Self::PC(_) => true,
            _ => false,
        }
    }

    pub fn is_console(&self) -> bool {
        !self.is_pc()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum XAssetGeneric<const MAX_LOCAL_CLIENTS: usize = 1> {
    PhysPreset(Option<Box<PhysPreset>>),
    PhysConstraints(Option<Box<PhysConstraints>>),
    DestructibleDef(Option<Box<DestructibleDef>>),
    XAnimParts(Option<Box<XAnimParts>>),
    XModel(Option<Box<XModel>>),
    Material(Option<Box<Material>>),
    TechniqueSet(Option<Box<MaterialTechniqueSet>>),
    Image(Option<Box<GfxImage>>),
    Sound(Option<Box<SndBank>>),
    SoundPatch(Option<Box<SndPatch>>),
    ClipMap(Option<Box<ClipMap>>),
    ClipMapPVS(Option<Box<ClipMap>>),
    ComWorld(Option<Box<ComWorld>>),
    GameWorldSp(Option<Box<GameWorldSp>>),
    GameWorldMp(Option<Box<GameWorldMp>>),
    MapEnts(Option<Box<MapEnts>>),
    GfxWorld(Option<Box<GfxWorld<MAX_LOCAL_CLIENTS>>>),
    LightDef(Option<Box<GfxLightDef>>),
    Font(Option<Box<Font>>),
    MenuList(Option<Box<MenuList<MAX_LOCAL_CLIENTS>>>),
    Menu(Option<Box<MenuDef<MAX_LOCAL_CLIENTS>>>),
    LocalizeEntry(Option<Box<LocalizeEntry>>),
    Weapon(Option<Box<WeaponVariantDef>>),
    SndDriverGlobals(Option<Box<SndDriverGlobals>>),
    Fx(Option<Box<FxEffectDef>>),
    ImpactFx(Option<Box<FxImpactTable>>),
    RawFile(Option<Box<RawFile>>),
    StringTable(Option<Box<StringTable>>),
    PackIndex(Option<Box<PackIndex>>),
    XGlobals(Option<Box<XGlobals>>),
    Ddl(Option<Box<DdlRoot>>),
    Glasses(Option<Box<Glasses>>),
    EmblemSet(Option<Box<EmblemSet>>),
}

impl<const MAX_LOCAL_CLIENTS: usize> XAssetGeneric<MAX_LOCAL_CLIENTS> {
    pub fn is_some(&self) -> bool {
        match self {
            Self::PhysPreset(p) => p.is_some(),
            Self::PhysConstraints(p) => p.is_some(),
            Self::DestructibleDef(p) => p.is_some(),
            Self::XAnimParts(p) => p.is_some(),
            Self::XModel(p) => p.is_some(),
            Self::Material(p) => p.is_some(),
            Self::TechniqueSet(p) => p.is_some(),
            Self::Image(p) => p.is_some(),
            Self::Sound(p) => p.is_some(),
            Self::SoundPatch(p) => p.is_some(),
            Self::ClipMap(p) => p.is_some(),
            Self::ClipMapPVS(p) => p.is_some(),
            Self::ComWorld(p) => p.is_some(),
            Self::GameWorldSp(p) => p.is_some(),
            Self::GameWorldMp(p) => p.is_some(),
            Self::MapEnts(p) => p.is_some(),
            Self::GfxWorld(p) => p.is_some(),
            Self::LightDef(p) => p.is_some(),
            Self::Font(p) => p.is_some(),
            Self::MenuList(p) => p.is_some(),
            Self::Menu(p) => p.is_some(),
            Self::LocalizeEntry(p) => p.is_some(),
            Self::Weapon(p) => p.is_some(),
            Self::SndDriverGlobals(p) => p.is_some(),
            Self::Fx(p) => p.is_some(),
            Self::ImpactFx(p) => p.is_some(),
            Self::RawFile(p) => p.is_some(),
            Self::StringTable(p) => p.is_some(),
            Self::PackIndex(p) => p.is_some(),
            Self::XGlobals(p) => p.is_some(),
            Self::Ddl(p) => p.is_some(),
            Self::Glasses(p) => p.is_some(),
            Self::EmblemSet(p) => p.is_some(),
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            Self::PhysPreset(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::PhysConstraints(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::DestructibleDef(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::XAnimParts(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::XModel(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Material(p) => p.as_ref().map(|p| p.info.name.as_str()),
            Self::TechniqueSet(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Image(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Sound(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::SoundPatch(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::ClipMap(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::ClipMapPVS(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::ComWorld(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::GameWorldSp(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::GameWorldMp(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::MapEnts(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::GfxWorld(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::LightDef(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Font(p) => p.as_ref().map(|p| p.font_name.as_str()),
            Self::MenuList(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Menu(p) => p.as_ref().map(|p| p.window.name.as_str()),
            Self::LocalizeEntry(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Weapon(p) => p.as_ref().map(|p| p.internal_name.as_str()),
            Self::SndDriverGlobals(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Fx(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::ImpactFx(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::RawFile(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::StringTable(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::PackIndex(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::XGlobals(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Ddl(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Glasses(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::EmblemSet(_) => Some("emblemset"),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XAssetListRaw<'a> {
    pub strings: FatPointerCountFirstU32<'a, XString<'a>>,
    pub assets: FatPointerCountFirstU32<'a, XAssetRaw<'a>>,
}
assert_size!(XAssetListRaw, 16);

#[derive(Clone, Debug, Default)]
pub(crate) struct XAssetList {
    pub _strings: Vec<String>,
    pub assets: Vec<XAsset>,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XAssetRaw<'a> {
    pub asset_type: u32,
    pub asset_data: Ptr32<'a, ()>,
}
assert_size!(XAssetRaw, 8);

impl<'a> XFileSerializeInto<XAssetListRaw<'a>, ()> for XAssetList {
    fn xfile_serialize_into(
        &self,
        _ser: &mut T5XFileSerializer,
        _data: (),
    ) -> Result<XAssetListRaw<'a>> {
        todo!()
    }
}

/// T5 doesn't actually use all of these.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug, FromPrimitive)]
#[repr(u32)]
pub enum XAssetType {
    #[default]
    XMODELPIECES = 0x00,
    PHYSPRESET = 0x01,
    PHYSCONSTRAINTS = 0x02,
    DESTRUCTIBLEDEF = 0x03,
    XANIMPARTS = 0x04,
    XMODEL = 0x05,
    MATERIAL = 0x06,
    TECHNIQUE_SET = 0x07,
    IMAGE = 0x08,
    SOUND = 0x09,
    SOUND_PATCH = 0x0A,
    CLIPMAP = 0x0B,
    CLIPMAP_PVS = 0x0C,
    COMWORLD = 0x0D,
    GAMEWORLD_SP = 0x0E,
    GAMEWORLD_MP = 0x0F,
    MAP_ENTS = 0x10,
    GFXWORLD = 0x11,
    LIGHT_DEF = 0x12,
    UI_MAP = 0x13,
    FONT = 0x14,
    MENULIST = 0x15,
    MENU = 0x16,
    LOCALIZE_ENTRY = 0x17,
    WEAPON = 0x18,
    WEAPONDEF = 0x19,
    WEAPON_VARIANT = 0x1A,
    SNDDRIVER_GLOBALS = 0x1B,
    FX = 0x1C,
    IMPACT_FX = 0x1D,
    AITYPE = 0x1E,
    MPTYPE = 0x1F,
    MPBODY = 0x20,
    MPHEAD = 0x21,
    CHARACTER = 0x22,
    XMODELALIAS = 0x23,
    RAWFILE = 0x24,
    STRINGTABLE = 0x25,
    PACKINDEX = 0x26,
    XGLOBALS = 0x27,
    DDL = 0x28,
    GLASSES = 0x29,
    EMBLEMSET = 0x2A,
    STRING = 0x2B,
    ASSETLIST = 0x2C,
}

impl<'a, const MAX_LOCAL_CLIENTS: usize> XFileDeserializeInto<XAssetGeneric<MAX_LOCAL_CLIENTS>, ()>
    for XAssetRaw<'a>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut T5XFileDeserializer,
        _data: (),
    ) -> Result<XAssetGeneric<MAX_LOCAL_CLIENTS>> {
        //dbg!(de.stream_pos()?);
        let asset_type = num::FromPrimitive::from_u32(self.asset_type).ok_or(Error::new(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::InvalidXAssetType(self.asset_type),
        ))?;
        //println!("type={:?} ({})", asset_type, self.asset_type);
        Ok(match asset_type {
            XAssetType::PHYSPRESET => XAssetGeneric::PhysPreset(
                self.asset_data
                    .cast::<PhysPresetRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::PHYSCONSTRAINTS => XAssetGeneric::PhysConstraints(
                self.asset_data
                    .cast::<PhysConstraintsRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::DESTRUCTIBLEDEF => XAssetGeneric::DestructibleDef(
                self.asset_data
                    .cast::<DestructibleDefRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::XANIMPARTS => XAssetGeneric::XAnimParts(
                self.asset_data
                    .cast::<XAnimPartsRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::XMODEL => XAssetGeneric::XModel(
                self.asset_data
                    .cast::<XModelRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::MATERIAL => XAssetGeneric::Material(
                self.asset_data
                    .cast::<MaterialRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::TECHNIQUE_SET => XAssetGeneric::TechniqueSet(
                self.asset_data
                    .cast::<MaterialTechniqueSetRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::IMAGE => XAssetGeneric::Image(
                self.asset_data
                    .cast::<GfxImageRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::SOUND => XAssetGeneric::Sound(
                self.asset_data
                    .cast::<SndBankRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::SOUND_PATCH => XAssetGeneric::SoundPatch(
                self.asset_data
                    .cast::<SndPatchRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::CLIPMAP => XAssetGeneric::ClipMap(
                self.asset_data
                    .cast::<ClipMapRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::CLIPMAP_PVS => XAssetGeneric::ClipMapPVS(
                self.asset_data
                    .cast::<ClipMapRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::COMWORLD => XAssetGeneric::ComWorld(
                self.asset_data
                    .cast::<ComWorldRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::GAMEWORLD_SP => XAssetGeneric::GameWorldSp(
                self.asset_data
                    .cast::<GameWorldSpRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::GAMEWORLD_MP => XAssetGeneric::GameWorldMp(
                self.asset_data
                    .cast::<GameWorldMpRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::MAP_ENTS => XAssetGeneric::MapEnts(
                self.asset_data
                    .cast::<MapEntsRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::GFXWORLD => XAssetGeneric::GfxWorld(
                self.asset_data
                    .cast::<GfxWorldRaw<MAX_LOCAL_CLIENTS>>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::LIGHT_DEF => XAssetGeneric::LightDef(
                self.asset_data
                    .cast::<GfxLightDefRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::FONT => XAssetGeneric::Font(
                self.asset_data
                    .cast::<FontRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::MENULIST => XAssetGeneric::MenuList(
                self.asset_data
                    .cast::<MenuListRaw<MAX_LOCAL_CLIENTS>>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::MENU => XAssetGeneric::Menu(
                self.asset_data
                    .cast::<MenuDefRaw<MAX_LOCAL_CLIENTS>>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::LOCALIZE_ENTRY => XAssetGeneric::LocalizeEntry(
                self.asset_data
                    .cast::<LocalizeEntryRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::WEAPON => XAssetGeneric::Weapon(
                self.asset_data
                    .cast::<WeaponVariantDefRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::SNDDRIVER_GLOBALS => XAssetGeneric::SndDriverGlobals(
                self.asset_data
                    .cast::<SndDriverGlobalsRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::FX => XAssetGeneric::Fx(
                self.asset_data
                    .cast::<FxEffectDefRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::IMPACT_FX => XAssetGeneric::ImpactFx(
                self.asset_data
                    .cast::<FxImpactTableRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::RAWFILE => XAssetGeneric::RawFile(
                self.asset_data
                    .cast::<RawFileRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::STRINGTABLE => XAssetGeneric::StringTable(
                self.asset_data
                    .cast::<StringTableRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::PACKINDEX => XAssetGeneric::PackIndex(
                self.asset_data
                    .cast::<PackIndexRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::XGLOBALS => XAssetGeneric::XGlobals(
                self.asset_data
                    .cast::<XGlobalsRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::DDL => XAssetGeneric::Ddl(
                self.asset_data
                    .cast::<DdlRootRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::GLASSES => XAssetGeneric::Glasses(
                self.asset_data
                    .cast::<GlassesRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::EMBLEMSET => XAssetGeneric::EmblemSet(
                self.asset_data
                    .cast::<EmblemSetRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            _ => {
                //dbg!(asset_type);
                return Err(Error::new(
                    file_line_col!(),
                    de.stream_pos()? as _,
                    ErrorKind::UnusedXAssetType(asset_type),
                ));
            }
        })
    }
}
