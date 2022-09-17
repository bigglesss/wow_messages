use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::tbc::Area;
use crate::world::tbc::CharacterGear;
use crate::world::tbc::Class;
use crate::world::tbc::Gender;
use crate::world::tbc::Map;
use crate::world::tbc::Race;
use crate::world::tbc::Vector3d;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_enum_2_4_3.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_enum_2_4_3.wowm#L8):
/// ```text
/// struct Character {
///     Guid guid;
///     CString name;
///     Race race;
///     Class class;
///     Gender gender;
///     u8 skin;
///     u8 face;
///     u8 hair_style;
///     u8 hair_color;
///     u8 facial_hair;
///     u8 level;
///     Area area;
///     Map map;
///     Vector3d position;
///     u32 guild_id;
///     u32 flags;
///     Bool first_login;
///     u32 pet_display_id;
///     u32 pet_level;
///     u32 pet_family;
///     CharacterGear[20] equipment;
/// }
/// ```
pub struct Character {
    pub guid: Guid,
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub gender: Gender,
    pub skin: u8,
    pub face: u8,
    pub hair_style: u8,
    pub hair_color: u8,
    pub facial_hair: u8,
    pub level: u8,
    pub area: Area,
    pub map: Map,
    pub position: Vector3d,
    pub guild_id: u32,
    pub flags: u32,
    pub first_login: bool,
    pub pet_display_id: u32,
    pub pet_level: u32,
    pub pet_family: u32,
    pub equipment: [CharacterGear; 20],
}

impl Character {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        w.write_all(&(self.race.as_int() as u8).to_le_bytes())?;

        // class: Class
        w.write_all(&(self.class.as_int() as u8).to_le_bytes())?;

        // gender: Gender
        w.write_all(&(self.gender.as_int() as u8).to_le_bytes())?;

        // skin: u8
        w.write_all(&self.skin.to_le_bytes())?;

        // face: u8
        w.write_all(&self.face.to_le_bytes())?;

        // hair_style: u8
        w.write_all(&self.hair_style.to_le_bytes())?;

        // hair_color: u8
        w.write_all(&self.hair_color.to_le_bytes())?;

        // facial_hair: u8
        w.write_all(&self.facial_hair.to_le_bytes())?;

        // level: u8
        w.write_all(&self.level.to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        // guild_id: u32
        w.write_all(&self.guild_id.to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // first_login: Bool
        w.write_all(if self.first_login { &[1] } else { &[0] })?;

        // pet_display_id: u32
        w.write_all(&self.pet_display_id.to_le_bytes())?;

        // pet_level: u32
        w.write_all(&self.pet_level.to_le_bytes())?;

        // pet_family: u32
        w.write_all(&self.pet_family.to_le_bytes())?;

        // equipment: CharacterGear[20]
        for i in self.equipment.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
}

impl Character {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // race: Race
        let race: Race = crate::util::read_u8_le(r)?.try_into()?;

        // class: Class
        let class: Class = crate::util::read_u8_le(r)?.try_into()?;

        // gender: Gender
        let gender: Gender = crate::util::read_u8_le(r)?.try_into()?;

        // skin: u8
        let skin = crate::util::read_u8_le(r)?;

        // face: u8
        let face = crate::util::read_u8_le(r)?;

        // hair_style: u8
        let hair_style = crate::util::read_u8_le(r)?;

        // hair_color: u8
        let hair_color = crate::util::read_u8_le(r)?;

        // facial_hair: u8
        let facial_hair = crate::util::read_u8_le(r)?;

        // level: u8
        let level = crate::util::read_u8_le(r)?;

        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        // guild_id: u32
        let guild_id = crate::util::read_u32_le(r)?;

        // flags: u32
        let flags = crate::util::read_u32_le(r)?;

        // first_login: Bool
        let first_login = crate::util::read_u8_le(r)? != 0;
        // pet_display_id: u32
        let pet_display_id = crate::util::read_u32_le(r)?;

        // pet_level: u32
        let pet_level = crate::util::read_u32_le(r)?;

        // pet_family: u32
        let pet_family = crate::util::read_u32_le(r)?;

        // equipment: CharacterGear[20]
        let mut equipment = [CharacterGear::default(); 20];
        for i in equipment.iter_mut() {
            *i = CharacterGear::read(r)?;
        }

        Ok(Self {
            guid,
            name,
            race,
            class,
            gender,
            skin,
            face,
            hair_style,
            hair_color,
            facial_hair,
            level,
            area,
            map,
            position,
            guild_id,
            flags,
            first_login,
            pet_display_id,
            pet_level,
            pet_family,
            equipment,
        })
    }

}

impl Character {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.name.len() + 1 // name: CString
        + 1 // race: Race
        + 1 // class: Class
        + 1 // gender: Gender
        + 1 // skin: u8
        + 1 // face: u8
        + 1 // hair_style: u8
        + 1 // hair_color: u8
        + 1 // facial_hair: u8
        + 1 // level: u8
        + 4 // area: Area
        + 4 // map: Map
        + 12 // position: Vector3d
        + 4 // guild_id: u32
        + 4 // flags: u32
        + 1 // first_login: Bool
        + 4 // pet_display_id: u32
        + 4 // pet_level: u32
        + 4 // pet_family: u32
        + 20 * 9 // equipment: CharacterGear[20]
    }
}

