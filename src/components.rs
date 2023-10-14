use bevy::prelude::*;

pub const TILE_SIZE: f32 = 40.0;
pub const TILE_SPACER: f32 = 10.0;

#[derive(Component)]
pub struct Board {
    pub size: u8,
    pub physical_size: f32,
}
impl Board {
    pub fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;

        Board {
            size,
            physical_size,
        }
    }

    pub fn cell_position_to_physical(&self, pos: u8) -> f32 {
        let offset = 0.5 * (-self.physical_size + TILE_SIZE);
        offset + f32::from(pos) * TILE_SIZE + f32::from(pos + 1) * TILE_SPACER
    }
}

#[derive(
    Component, Debug,
    PartialEq, Clone, 
    Copy,
)]
pub struct Points {
    pub value: u32,
}

#[derive(Default, Resource)]
pub struct Game {
    pub score: u32,
    pub score_best: u32,
}
#[derive(
    Component, Debug,
    PartialEq, Clone, 
    Copy, Hash, Eq
)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Component)]
pub struct TileText;

#[derive(Resource)]
pub struct FontSpec {
    pub family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        FontSpec {
            family: asset_server
                .load("fonts/FiraSans-Bold.ttf"),
        }
    }
}

#[derive(
    Default, Debug, PartialEq, 
    Clone, Hash, Eq, States,
)]
pub enum RunState {
    #[default]
    Playing,
    GameOver,
}