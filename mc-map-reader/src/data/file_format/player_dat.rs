use jbe::Builder;

use crate::data::entity::Entity;

// TODO https://minecraft.fandom.com/wiki/Player.dat_format
#[derive(Debug, Builder)]
pub struct Player {
    entity: Entity
}