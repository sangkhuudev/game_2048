use bevy::prelude::*;
use bevy_easings::*;

use boxes::utility::{setup, spawn_board, 
    spawn_tiles,render_tile_points, 
    board_shift, render_tiles, 
    new_tile_handler, NewTileEvent, 
    end_game, game_reset
};
use boxes::components::{Game, FontSpec, RunState};
use boxes::ui::GameUiPlugin;

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
        .add_plugins(GameUiPlugin)
        .add_plugins(EasingsPlugin)
        .init_resource::<Game>()
        .init_resource::<FontSpec>()
        // .init_resource::<State<RunState>>()
        .add_event::<NewTileEvent>()
        .add_state::<RunState>()
        .add_systems(
            Startup,
            (setup, spawn_board, apply_deferred)
            .chain(),
        )
        .add_systems(Update, 
            (
                render_tile_points, board_shift, 
                render_tiles,  new_tile_handler,
                end_game
            )
            .run_if(in_state(RunState::Playing))
        )
        .add_systems(OnEnter(RunState::Playing),
            (game_reset, spawn_tiles)
        )
        .run()
}
