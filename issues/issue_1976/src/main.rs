use bevy::prelude::*;

#[derive(Component)]
struct Foo;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "../../assets".to_string(),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Yellow rectangle at the bottom
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(-25.0, -25.0, 0.0),
        sprite: Sprite {
            color: Color::YELLOW,
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        ..default()
    });

    // Red rectangle in the middle, but with a parent without transform
    commands.spawn(Foo).with_children(|child| {
        child.spawn(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            ..default()
        });
    });

    // Blue rectangle at the top
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(25.0, 25.0, 20.0),
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        ..default()
    });
}
