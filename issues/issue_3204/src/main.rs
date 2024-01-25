use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "../../assets".to_string(),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, fader)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Hello.",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 250.0,
                color: Color::RED,
            },
        ),
        ..Default::default()
    });
}

fn fader(mut increase: Local<bool>, mut text_query: Query<&mut Text>, time: Res<Time>) {
    let speed = 0.5;
    let mut text = text_query.get_single_mut().unwrap();
    let color = &mut text.sections[0].style.color;

    if *increase {
        color.set_a(color.a() + time.delta_seconds() * speed);
    } else {
        color.set_a(color.a() - time.delta_seconds() * speed);
    }

    if color.a() >= 1.0 {
        *increase = false;
    } else if color.a() <= 0.0 {
        *increase = true;
    }

    text.sections[0].value = format!("alpha {:.2}", color.a());
}
