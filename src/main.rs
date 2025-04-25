use bevy::{prelude::*, window::PrimaryWindow};

const SPRITE_SIZE: Vec2 = Vec2::new(256.0, 256.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_sprite)
        .run();
}

#[derive(Component)]
struct Mover
{
    direction: Vec2,
    speed: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            image: asset_server.load("bevy_bird_dark.png"),
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(SPRITE_SIZE),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mover
        {
            direction: Vec2::new(1.0, 1.0),
            speed: 300.0,
        },
    ));
}

fn move_sprite(
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &mut Mover), With<Mover>>,
) {
    for (mut transform, mut mover) in &mut query {
        let translation_change = mover.direction * mover.speed * time.delta_secs();
        transform.translation.x += translation_change.x;
        transform.translation.y += translation_change.y;
        if let Ok(window) = primary_window.single() {
            let collision_width = (window.width() - SPRITE_SIZE.x) / 2.0;
            if transform.translation.x < -collision_width {
                mover.direction.x = -mover.direction.x;
                transform.translation.x = -collision_width;
            }
            else if transform.translation.x > collision_width {
                mover.direction.x = -mover.direction.x;
                transform.translation.x = collision_width;
            }

            let collision_height = (window.height() - SPRITE_SIZE.y) / 2.0;
            if transform.translation.y < -collision_height {
                mover.direction.y = -mover.direction.y;
                transform.translation.y = -collision_height;
            }
            else if transform.translation.y > collision_height {
                mover.direction.y = -mover.direction.y;
                transform.translation.y = collision_height;
            }
        }
    }
}
