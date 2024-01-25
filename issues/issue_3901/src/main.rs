use std::time::Duration;

use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;
use bevy::time::common_conditions::on_timer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "../../assets".to_string(),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, switcher.run_if(on_timer(Duration::from_secs(1))))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle {
        projection: OrthographicProjection {
            scale: 2.0,
            ..default()
        },
        ..default()
    },));

    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|root| {
            root.spawn(NodeBundle {
                background_color: Color::WHITE.into(),
                style: Style {
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|child| {
                child.spawn(NodeBundle {
                    background_color: Color::RED.into(),
                    style: Style {
                        width: Val::Px(100.0),
                        height: Val::Px(100.0),
                        ..default()
                    },
                    ..default()
                });
            });

            root.spawn(NodeBundle {
                background_color: Color::GRAY.into(),
                style: Style {
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|child| {
                child.spawn(TextBundle {
                    text: Text::from_section(
                        "Projection Scale",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 100.0,
                            color: Color::RED,
                        },
                    ),
                    ..default()
                });
            });
        });
}

fn switcher(
    mut switch: Local<bool>,
    mut projection_query: Query<&mut OrthographicProjection>,
    mut text_query: Query<&mut Text>,
    mut ui_scale: ResMut<UiScale>,
) {
    let mut projection = projection_query.get_single_mut().unwrap();
    let mut text = text_query.get_single_mut().unwrap();

    if *switch {
        projection.scale = 2.0;
        ui_scale.0 = 1.0;
        text.sections[0].value = "Projection Scale".to_string();
    } else {
        projection.scale = 1.0;
        ui_scale.0 = 2.0;
        text.sections[0].value = "UI Scale".to_string();
    }

    *switch = !*switch;
}
