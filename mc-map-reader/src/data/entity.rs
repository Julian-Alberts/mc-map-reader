use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use jbe::Builder;

use crate::nbt::{Array, List, Tag};

use super::item::Item;

///<a href="https://minecraft.fandom.com/wiki/Entity_format#Entity_Format">minecraft wiki</a>
#[derive(Debug, Builder, Getters, CopyGetters, Clone, PartialEq)]
pub struct Entity {
    pub air: Option<i16>,
    pub custom_name: Option<String>,
    pub custom_name_visible: Option<bool>,
    pub fall_distance: Option<f32>,
    #[get_copy = "pub"]
    #[builder({default 0})]
    pub fire: i16,
    #[get_copy = "pub"]
    #[builder({default false})]
    pub glowing: bool,
    #[get_copy = "pub"]
    #[builder({default false})]
    pub has_visual_fire: bool,
    pub id: Option<String>,
    #[get_copy = "pub"]
    #[builder({default false})]
    pub invulnerable: bool,
    pub motion: Option<List<f64>>,
    #[get_copy = "pub"]
    #[builder({default false})]
    pub no_gravity: bool,
    #[get_copy = "pub"]
    #[builder({default true})]
    pub on_ground: bool,
    pub passengers: Option<List<Entity>>,
    #[get_copy = "pub"]
    #[builder({default 0})]
    pub portal_colldown: i32,
    pub pos: Option<List<f32>>,
    pub rotation: Option<List<f32>>,
    #[get_copy = "pub"]
    #[builder({default false})]
    pub silent: bool,
    pub tags: Option<HashMap<String, Tag>>,
    pub ticks_frozen: Option<i32>,
    pub uuid: Option<Array<i32>>,
}

#[derive(Debug, Builder)]
pub struct Mob {
    pub absorption_amount: Option<f32>,
    pub active_effects: Option<List<ActiveEffect>>,
    pub armor_drop_chances: Option<List<f32>>,
    pub armor_items: Option<List<Item>>,
    pub entity: Entity,
    pub attributes: Option<List<HashMap<String, Tag>>>,
    pub brain: Option<HashMap<String, Tag>>,
    pub can_pick_up_loot: Option<bool>,
    pub death_loot_table: Option<String>,
    pub death_loot_table_seed: Option<i64>,
    pub death_time: Option<i16>,
    pub fall_flying: Option<bool>,
    pub health: Option<f32>,
    pub hurt_by_timestamp: Option<i32>,
    pub hurt_time: Option<i16>,
    pub hand_drop_chances: Option<List<f32>>,
    pub hand_items: Option<List<Item>>,
    pub leash: Option<Leash>,
    pub left_handed: Option<bool>,
    pub no_ai: Option<bool>,
    pub persistence_required: Option<bool>,
    pub sleeping_x: Option<i32>,
    pub sleeping_y: Option<i32>,
    pub sleeping_z: Option<i32>,
    pub team: Option<String>,
}

#[derive(Debug)]
pub enum Leash {
    Entity(Array<i32>),
    Position { x: i32, y: i32, z: i32 },
}
#[derive(Debug, Builder, PartialEq)]
pub struct ActiveEffect {
    pub ambient: bool,
    pub amplifier: i8,
    pub duration: i32,
    pub id: i32,
    pub show_icon: bool,
    pub show_particles: bool,
}

impl Entity {
    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn tags(&self) -> Option<&HashMap<String, Tag>> {
        self.tags.as_ref()
    }

    pub fn ticks_frozen(&self) -> Option<i32> {
        self.ticks_frozen
    }

    pub fn air(&self) -> Option<i16> {
        self.air
    }

    pub fn custom_name_visible(&self) -> Option<bool> {
        self.custom_name_visible
    }

    pub fn fall_distance(&self) -> Option<f32> {
        self.fall_distance
    }

    pub fn motion(&self) -> Option<&List<f64>> {
        self.motion.as_ref()
    }

    pub fn passengers(&self) -> Option<&List<Entity>> {
        self.passengers.as_ref()
    }

    pub fn pos(&self) -> Option<&List<f32>> {
        self.pos.as_ref()
    }

    pub fn rotation(&self) -> Option<&List<f32>> {
        self.rotation.as_ref()
    }

    pub fn uuid(&self) -> Option<&Array<i32>> {
        self.uuid.as_ref()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn entity_getter_none() {
        let entity = EntityBuilder::default().try_build();
        assert!(entity.is_ok());
    }

    #[test]
    fn entity_getter_some() {
        let mut entity = EntityBuilder::default();
        entity
            .with_air(1)
            .with_custom_name("test".to_string())
            .with_custom_name_visible(true)
            .with_fall_distance(10.)
            .with_fire(1)
            .with_glowing(true)
            .with_has_visual_fire(true)
            .with_id("test".to_string())
            .with_invulnerable(true)
            .with_motion(List::from(vec![1., 2., 3.]))
            .with_no_gravity(true)
            .with_on_ground(false)
            .with_passengers(List::from(vec![EntityBuilder::default().try_build().unwrap()]))
            .with_portal_colldown(10)
            .with_pos(List::from(vec![1., 2., 3.]))
            .with_rotation(List::from(vec![1., 2., 3.]))
            .with_silent(true)
            .with_tags(HashMap::new())
            .with_ticks_frozen(20)
            .set_uuid(Array::from(vec![1, 2, 3, 4]));
        let entity = entity.try_build();
        assert!(entity.is_ok());
        let entity = entity.unwrap();
        assert_eq!(entity.air(), Some(1));
        assert_eq!(entity.custom_name(), Some(&"test".to_string()));
        assert_eq!(entity.custom_name_visible(), Some(true));
        assert_eq!(entity.fall_distance(), Some(10.));
        assert_eq!(entity.fire(), 1);
        assert_eq!(entity.glowing(), true);
        assert_eq!(entity.has_visual_fire(), true);
        assert_eq!(entity.id(), Some(&"test".to_string()));
        assert_eq!(entity.invulnerable(), true);
        assert_eq!(entity.motion(), Some(&List::from(vec![1., 2., 3.])));
        assert_eq!(entity.no_gravity(), true);
        assert_eq!(entity.on_ground(), false);
        assert_eq!(
            entity.passengers(),
            Some(&List::from(vec![EntityBuilder::default().build()]))
        );
        assert_eq!(entity.portal_colldown(), 10);
        assert_eq!(entity.pos(), Some(&List::from(vec![1., 2., 3.])));
        assert_eq!(entity.rotation(), Some(&List::from(vec![1., 2., 3.])));
        assert_eq!(entity.silent(), true);
        assert_eq!(entity.tags(), Some(&HashMap::new()));
        assert_eq!(entity.ticks_frozen(), Some(20));
        assert_eq!(entity.uuid(), Some(&Array::from(vec![1, 2, 3, 4])));
    }

}