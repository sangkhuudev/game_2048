use bevy::prelude::*;
use crate::components::FontSpec;
use crate::colors;
use crate::styles::score_container_style;
#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;
pub struct GameUiPlugin;
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
    }
}

pub fn setup_ui(mut commands: Commands, font_spec: Res<FontSpec>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content:
                    JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(50.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "2048",
                TextStyle {
                    font: font_spec.family.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                }
            ));


            // div 
            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content:
                            JustifyContent::Center,
                        width: Val::Auto,
                        height: Val::Auto,
                        row_gap: Val::Px(20.0),
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Score box
                    parent
                        .spawn(NodeBundle {
                            style: score_container_style(),
                            background_color:
                                BackgroundColor(
                                    colors::SCORE_BOX,
                                ),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Score",
                                    TextStyle {
                                        font: font_spec
                                            .family
                                            .clone(),
                                        font_size: 15.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_text_alignment(
                                    TextAlignment::Center,
                                ),
                            );
                            parent.spawn((
                                TextBundle::from_section(
                                    "<score>",
                                    TextStyle {
                                        font: font_spec
                                            .family
                                            .clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_text_alignment(
                                    TextAlignment::Center,
                                ),
                                ScoreDisplay,
                            ));
                        });
                    // end of score box
                    
                    // Best score box
                    parent
                        .spawn(NodeBundle {
                            style: score_container_style(),
                            background_color:
                                BackgroundColor(
                                    colors::SCORE_BOX,
                                ),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Best",
                                    TextStyle {
                                        font: font_spec
                                            .family
                                            .clone(),
                                        font_size: 15.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_text_alignment(
                                    TextAlignment::Center,
                                ),
                            );
                            parent.spawn((
                                TextBundle::from_section(
                                    "<score>",
                                    TextStyle {
                                        font: font_spec
                                            .family
                                            .clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_text_alignment(
                                    TextAlignment::Center,
                                ),
                                BestScoreDisplay,
                            ));
                        });
                    // end of best score box
                });
            // end of div
            // Button    
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(130.0),
                        height: Val::Px(50.0),
                        justify_content:
                            JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Button",
                            TextStyle {
                                font: font_spec
                                    .family
                                    .clone(),
                                font_size: 20.0,
                                color: Color::BLACK
                            },
                        ),
                        ..default()
                    });
            });    
        });
}
