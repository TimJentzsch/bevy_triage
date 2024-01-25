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
            near: 0.0,
            far: 1000.0,
            scale: 1.0,
            ..default()
        },
        ..default()
    },));

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Hello.",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 250.0,
                color: Color::WHITE,
            },
        ),
        ..Default::default()
    });
}

fn switcher(
    mut switch: Local<bool>,
    mut projection_query: Query<&mut OrthographicProjection>,
    mut text_query: Query<&mut Text>,
) {
    let mut projection = projection_query.get_single_mut().unwrap();
    let mut text = text_query.get_single_mut().unwrap();

    if *switch {
        projection.scale = 1.0;
        text.sections[0].style.font_size = 250.0;
    } else {
        projection.scale = 0.1;
        text.sections[0].style.font_size = 25.0;
    }

    *switch = !*switch;
}
