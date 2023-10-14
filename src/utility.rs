use crate::colors::{BOARD, TILE, TILE_PLACEHODER};
use crate::components::{
    Board, FontSpec, Points,
    Position, TileText, 
    Game, TILE_SIZE, RunState
};
use bevy::prelude::*;
use bevy_easings::*;
use itertools::Itertools;
use rand::prelude::*;
use std::{
    cmp::Ordering, collections::HashMap, convert::TryFrom,
    ops::Range,
};

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_board(mut commands: Commands) {
    let board = Board::new(4);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: BOARD,
                custom_size: Some(Vec2::new(board.physical_size, board.physical_size)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            // Move the tile at center of board to the left bottom
            for tile in (0..board.size).cartesian_product(0..board.size) {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: TILE_PLACEHODER,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(tile.0),
                        board.cell_position_to_physical(tile.1),
                        1.0,
                    ),
                    ..default()
                });
            }
        })
        .insert(board);
}

pub fn spawn_tiles(mut commands: Commands, query_board: Query<&Board>, font_spec: Res<FontSpec>) {
    let board = query_board.single();
    let mut rng = rand::thread_rng();
    let starting_tiles: Vec<(u8, u8)> = (0..board.size)
        .cartesian_product(0..board.size)
        .choose_multiple(&mut rng, 2);

    for (x, y) in starting_tiles.iter() {
        let pos = Position { x: *x, y: *y };
        spawn_tile(&mut commands, board, &font_spec, pos);
    }
}

pub fn spawn_tile(
    commands: &mut Commands,
    board: &Board,
    font_spec: &Res<FontSpec>,
    pos: Position,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: TILE,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Sprite::default()
            },
            transform: Transform::from_xyz(
                board.cell_position_to_physical(pos.x),
                board.cell_position_to_physical(pos.y),
                2.0,
            ),
            ..Default::default()
        })
        .with_children(|builder| {
            builder
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        "2",
                        TextStyle {
                            font: font_spec.family.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..Default::default()
                })
                .insert(TileText);
        })
        .insert(Points { value: 2 })
        .insert(pos);
}
pub fn render_tile_points(
    mut texts: Query<&mut Text, With<TileText>>,
    tiles: Query<(&Points, &Children)>,
) {
    for (points, children) in tiles.iter() {
        if let Some(entity) = children.first() {
            let mut text = texts.get_mut(*entity).expect("Expected Text to exist");
            let mut text_section = text
                .sections
                .first_mut()
                .expect("Expect first section to be accessible as mutable.");
            text_section.value = points.value.to_string()
        }
    }
}

//----------------------------------------------------------------
pub enum BoardShift {
    Left,
    Right,
    Up,
    Down,
}
impl BoardShift {
    fn sort(&self, a: &Position, b: &Position) -> Ordering {
        match self {
            BoardShift::Left => match Ord::cmp(&a.y, &b.y) {
                Ordering::Equal => Ord::cmp(&a.x, &b.x),
                ordering => ordering,
            },
            BoardShift::Right => match Ord::cmp(&a.y, &b.y) {
                Ordering::Equal => Ord::cmp(&b.x, &a.x),
                ordering => ordering,
            },
            BoardShift::Up => match Ord::cmp(&a.x, &b.x) {
                Ordering::Equal => Ord::cmp(&b.y, &a.y),
                ordering => ordering,
            },
            BoardShift::Down => match Ord::cmp(&a.x, &b.x) {
                Ordering::Equal => Ord::cmp(&a.y, &b.y),
                ordering => ordering,
            },
        }
    }
    fn set_column_position(&self, board_size: u8, position: &mut Mut<Position>, index: u8) {
        match self {
            BoardShift::Left => position.x = index,
            BoardShift::Right => position.x = board_size - index - 1,
            BoardShift::Up => position.y = board_size - index - 1,
            BoardShift::Down => position.y = index,
        }
    }
    fn get_row_position(&self, position: &Position) -> u8 {
        match self {
            BoardShift::Left | BoardShift::Right => position.y,
            BoardShift::Up | BoardShift::Down => position.x,
        }
    }
}
impl TryFrom<&KeyCode> for BoardShift {
    type Error = &'static str;

    fn try_from(value: &KeyCode) -> Result<Self, Self::Error> {
        match value {
            KeyCode::Left => Ok(BoardShift::Left),
            KeyCode::Right => Ok(BoardShift::Right),
            KeyCode::Up => Ok(BoardShift::Up),
            KeyCode::Down => Ok(BoardShift::Down),
            _ => Err("Not a valid board shift key"),
        }
    }
}
pub fn board_shift(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut tiles: Query<(Entity, &mut Position, &mut Points)>,
    query_board: Query<&Board>,
    mut tile_writer: EventWriter<NewTileEvent>,
    mut game: ResMut<Game>,
) {
    let board = query_board.single();
    let shift_direction = input
        .get_just_pressed()
        .find_map(|key_code| BoardShift::try_from(key_code).ok());
    if let Some(board_shift) = shift_direction {
        let mut iter = tiles
            .iter_mut()
            .sorted_by(|a, b| board_shift.sort(&a.1, &b.1))
            .peekable();
        let mut column: u8 = 0;
        while let Some(mut tile) = iter.next() {
            board_shift.set_column_position(board.size, &mut tile.1, column);
            if let Some(tile_next) = iter.peek() {
                if board_shift.get_row_position(&tile.1)
                    != board_shift.get_row_position(&tile_next.1)
                {
                    column = 0;
                } else if tile.2.value != tile_next.2.value {
                    // Different values, dont merge!
                    column = column + 1;
                } else {
                    let real_next_tile = iter
                        .next()
                        .expect("A peeked tile should always exist when we .next here");
                    tile.2.value = tile.2.value + real_next_tile.2.value;
                    game.score += tile.2.value;
                    commands.entity(real_next_tile.0).despawn_recursive();
                    if let Some(future) = iter.peek() {
                        if board_shift.get_row_position(&tile.1)
                            != board_shift.get_row_position(&future.1)
                        {
                            column = 0;
                        } else {
                            column = column + 1;
                        }
                    }
                }
            }
        }
        tile_writer.send(NewTileEvent);
        if game.best_score < game.score {
            game.best_score = game.score;
        }
    }
}

//----------------------------------------------------------------
pub fn render_tiles(
    mut commands: Commands,
    mut tiles: Query<(Entity, &mut Transform, &Position), Changed<Position>>,
    query_board: Query<&Board>,
) {
    let board = query_board.single();
    for (entity, transform, pos) in tiles.iter_mut() {
        let x = board.cell_position_to_physical(pos.x);
        let y = board.cell_position_to_physical(pos.y);

        commands.entity(entity).insert(transform.ease_to(
            Transform::from_xyz(
                x,
                y,
                transform.translation.z,
            ),
            EaseFunction::QuadraticInOut,
            EasingType::Once {
                duration: std::time::Duration::from_millis(
                    100,
                ),
            },
        ));
    }
}

#[derive(Event)]
pub struct NewTileEvent;

pub fn new_tile_handler(
    mut tile_reader: EventReader<NewTileEvent>,
    mut commands: Commands,
    query_board: Query<&Board>,
    tiles: Query<&Position>,
    font_spec: Res<FontSpec>,
) {
    let board = query_board.single();
    for _event in tile_reader.iter() {
        let mut rng = rand::thread_rng();
        let possible_pos: Option<Position> = (0..board.size)
            .cartesian_product(0..board.size)
            .filter_map(|tile_pos| {
                let new_position = Position {
                    x: tile_pos.0,
                    y: tile_pos.1,
                };
                match tiles.iter().find(|&&pos| pos == new_position) {
                    Some(_) => None,
                    None => Some(new_position),
                }
            })
            .choose(&mut rng);
        if let Some(pos) = possible_pos {
            spawn_tile(&mut commands, board, &font_spec, pos);
        }
    }
}

pub fn end_game(
    tiles: Query<(&Position, &Points)>,
    query_board: Query<&Board>,
    mut run_state: ResMut<NextState<RunState>>,
) {
    let board = query_board.single();
    if tiles.iter().len() == 16 {
        let neighbor_points = [(-1,0), (1,0), (0,1), (0,-1)];
        let map: HashMap<&Position, &Points> = tiles.iter().collect();
        let board_range: Range<i8> = 0..(board.size as i8);

        let has_move = tiles.iter().any(
            |(Position {x,y}, value)| {
                neighbor_points
                    .iter()
                    .filter_map(|(x2,y2)| {
                        let new_x = *x as i8 - x2;
                        let new_y = *y as i8 - y2;

                        if !board_range.contains(&new_x) 
                        || !board_range.contains(&new_y) {
                            return None;
                        }
                        map.get(&Position {
                            x: new_x.try_into().unwrap(),
                            y: new_y.try_into().unwrap()
                        })
                    })
                    .any(|&v| v == value)
            },
        );
        if !has_move {
            dbg!("Game over");
            run_state.set(RunState::GameOver);
        }
    }
}

pub fn game_reset(
    mut commands: Commands,
    tiles: Query<Entity, With<Position>>,
    mut game: ResMut<Game>,
) {
    for entity in tiles.iter() {
        commands.entity(entity).despawn_recursive();
    }
    game.score = 0;
}