use bevy::prelude::*;
use boxes::utility::{setup, spawn_board, 
    spawn_tiles,render_tile_points, 
    board_shift, render_tiles, 
    new_tile_handler, NewTileEvent};
use boxes::components::{Game, FontSpec};


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("#1f2638").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Game>()
        .init_resource::<FontSpec>()
        .add_event::<NewTileEvent>()
        .add_systems(
            Startup,
            (setup, spawn_board, apply_deferred, spawn_tiles)
            .chain(),
        )
        .add_systems(Update, (render_tile_points, board_shift, render_tiles,  new_tile_handler))
        .run()
}