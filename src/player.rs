use bevy::{prelude::*, time::FixedTimestep};

use crate::components::{Bullet, FromPlayer, Movable, Player, Speed, SpriteSize};
use crate::{
    PlayerState, WinSize, PLAYER_BULLET_COLOR, PLAYER_BULLET_SIZE, PLAYER_COLOR,
    PLAYER_RESPAWN_TIME, PLAYER_SIZE,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.5))
                    .with_system(player_spawn),
            )
            .add_system(player_movement_input)
            .add_system(player_fire_input);
    }
}

fn player_spawn(
    mut commands: Commands,
    mut player_status: ResMut<PlayerState>,
    time: Res<Time>,
    win_size: Res<WinSize>,
) {
    let now = time.elapsed_seconds_f64();
    let last_shot = player_status.last_shot;

    if !player_status.on && (last_shot == -1. || now > last_shot + PLAYER_RESPAWN_TIME) {
        // spawn player
        let bottom = -win_size.h / 2.0;
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(PLAYER_SIZE),
                    color: PLAYER_COLOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0., bottom + PLAYER_SIZE.y / 2. + 5., 10.),
                    scale: Vec3::new(1., 1., 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player)
            .insert(Speed { x: 0.0, y: 0.0 })
            .insert(SpriteSize(PLAYER_SIZE))
            .insert(Movable {
                despawn: false,
                player: true,
                enemy: false,
            });

        player_status.spawned();
    }
}

fn player_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Speed, With<Player>>,
) {
    if let Ok(mut speed) = query.get_single_mut() {
        speed.x = if keyboard_input.pressed(KeyCode::Left) {
            -5.
        } else if keyboard_input.pressed(KeyCode::Right) {
            5.
        } else {
            0.
        };
        speed.y = if keyboard_input.pressed(KeyCode::Up) {
            5.
        } else if keyboard_input.pressed(KeyCode::Down) {
            -5.
        } else {
            0.
        };
    }
}

fn player_fire_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            let x_offset = PLAYER_SIZE.x / 2.0 - 5.;

            let mut spawn_bullet = |x_offset: f32| {
                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(PLAYER_BULLET_SIZE),
                            color: PLAYER_BULLET_COLOR,
                            ..Default::default()
                        },
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y + PLAYER_SIZE.y / 2. + 5., 10.),
                            scale: Vec3::new(1., 1., 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Bullet)
                    .insert(FromPlayer)
                    .insert(SpriteSize(PLAYER_BULLET_SIZE))
                    .insert(Speed { x: 0.0, y: 10.0 })
                    .insert(Movable {
                        despawn: true,
                        player: false,
                        enemy: false,
                    });
            };

            spawn_bullet(x_offset);
            spawn_bullet(-x_offset);
        }
    }
}
