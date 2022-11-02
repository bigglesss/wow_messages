mod impls;

pub use impls::*;

use crate::helper::update_mask_common;
use crate::helper::update_mask_common::{
    update_item, update_mask, update_mask_size, CONTAINER, CORPSE, DYNAMICOBJECT, GAMEOBJECT, ITEM,
    PLAYER, UNIT,
};
use std::collections::BTreeMap;
use std::io;
use std::io::Read;

update_item!(UpdateItem, UpdateItemBuilder, ITEM);
update_item!(UpdateContainer, UpdateContainerBuilder, ITEM | CONTAINER);
update_item!(UpdateUnit, UpdateUnitBuilder, UNIT);
update_item!(UpdatePlayer, UpdatePlayerBuilder, UNIT | PLAYER);
update_item!(UpdateGameObject, UpdateGameObjectBuilder, GAMEOBJECT);
update_item!(
    UpdateDynamicObject,
    UpdateDynamicObjectBuilder,
    DYNAMICOBJECT
);
update_item!(UpdateCorpse, UpdateCorpseBuilder, CORPSE);

update_mask!();
