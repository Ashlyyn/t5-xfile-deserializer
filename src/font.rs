use crate::*;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FontRaw<'a> {
    pub font_name: XString<'a>,
    pub pixel_height: i32,
    pub glyph_count: i32,
    pub material: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub glow_material: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub glyphs: Ptr32<'a, Glyph>,
}
assert_size!(FontRaw, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Font {
    pub font_name: String,
    pub pixel_height: i32,
    pub material: Option<Box<techset::Material>>,
    pub glow_material: Option<Box<techset::Material>>,
    pub glyphs: Vec<Glyph>,
}

impl<'a> XFileInto<Font, ()> for FontRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<Font> {
        Ok(Font {
            font_name: self.font_name.xfile_into(de, ())?,
            pixel_height: self.pixel_height,
            material: self.material.xfile_into(de, ())?,
            glow_material: self.glow_material.xfile_into(de, ())?,
            glyphs: self.glyphs.to_array(self.glyph_count as _).to_vec(de)?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Glyph {
    pub letter: u16,
    pub x0: i8,
    pub y0: i8,
    pub dx: u8,
    pub pixel_width: u8,
    pub pixel_height: u8,
    pad: [u8; 1],
    pub s0: f32,
    pub to: f32,
    pub s1: f32,
    pub t1: f32,
}
assert_size!(Glyph, 24);
