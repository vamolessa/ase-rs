use std::io::{self, Read, Seek, SeekFrom, Write};

use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use num_enum::CustomTryInto;

use crate::helpers::{read_string, write_string};

bitflags! {
    pub struct Flags: u16 {
        const Visible = 1;
        const Editable = 2;
        const LockMovement = 4;
        const Backgrount = 8;
        const PreferLinkedCels = 16;
        const DisplayCollapsed = 32;
        const ReferenceLayer = 64;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, CustomTryInto)]
#[repr(u16)]
pub enum LayerType {
    Normal = 0,
    Group = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, CustomTryInto)]
#[repr(u16)]
pub enum BlendMode {
    Normal = 0,
    Multiply = 1,
    Screen = 2,
    Overlay = 3,
    Darken = 4,
    Lighten = 5,
    ColorDodge = 6,
    ColorBurn = 7,
    HardLight = 8,
    SoftLight = 9,
    Difference = 10,
    Exclusion = 11,
    Hue = 12,
    Saturation = 13,
    Color = 14,
    Luminosity = 15,
    Addition = 16,
    Subtract = 17,
    Divide = 18,
}

#[derive(Debug)]
pub struct LayerChunk {
    pub flags: Flags,
    pub layer_type: LayerType,
    pub layer_child_level: u16,
    pub blend_mode: BlendMode,
    pub opacity: u8,
    pub layer_name: String,
}

impl LayerChunk {
    pub fn new(layer_name: &str, visible: bool) -> Self {
        let mut flags = Flags::Editable;
        if visible {
            flags |= Flags::Visible;
        }
        LayerChunk {
            flags,
            layer_type: LayerType::Normal,
            layer_child_level: 0,
            blend_mode: BlendMode::Normal,
            opacity: 255,
            layer_name: layer_name.to_owned(),
        }
    }
    pub fn from_read<R>(read: &mut R) -> io::Result<Self>
    where
        R: Read + Seek,
    {
        let flags = Flags::from_bits_truncate(read.read_u16::<LittleEndian>()?);
        let layer_type = read
            .read_u16::<LittleEndian>()?
            .try_into_LayerType()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let layer_child_level = read.read_u16::<LittleEndian>()?;
        read.seek(SeekFrom::Current(2 + 2))?;
        let blend_mode = read
            .read_u16::<LittleEndian>()?
            .try_into_BlendMode()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let opacity = read.read_u8()?;
        read.seek(SeekFrom::Current(3))?;
        let layer_name = read_string(read)?;

        Ok(Self {
            flags,
            layer_type,
            layer_child_level,
            blend_mode,
            opacity,
            layer_name,
        })
    }

    pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
    where
        W: Write + Seek,
    {
        wtr.write_u16::<LittleEndian>(self.flags.bits)?;
        wtr.write_u16::<LittleEndian>(self.layer_type as u16)?;
        wtr.write_u16::<LittleEndian>(self.layer_child_level)?;
        wtr.seek(SeekFrom::Current(2 + 2))?;
        wtr.write_u16::<LittleEndian>(self.blend_mode as u16)?;
        wtr.write_u8(self.opacity)?;

        wtr.seek(SeekFrom::Current(3))?;
        write_string(wtr, &self.layer_name)?;

        Ok(())
    }
}
