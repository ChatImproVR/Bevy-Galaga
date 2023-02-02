use bevy::{
    ecs::schedule::ShouldRun, prelude::*, sprite::collide_aabb::collide, time::FixedTimestep,
};
use rand::{thread_rng, Rng};

use crate::components::{Bullet, Enemy, FromEnemy, Movable, Speed, SpriteSize};
use crate::{
    EnemyCount, EnemyState, WinSize, ENEMY_BULLET_COLOR, ENEMY_BULLET_SIZE, ENEMY_COLOR, ENEMY_MAX,
    ENEMY_SIZE,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyState::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.5))
                    .with_system(enemy_spawn),
            )
            .add_system(enemy_movement)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(enemy_fire_criteria)
                    .with_system(enemy_fire),
            )
            .add_system(enemy_collide);
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
        // spawn enemy
        let top = win_size.h / 2.0;
        let mut rng = thread_rng();
        let x = rng.gen_range(-win_size.w / 2.0..win_size.w / 2.0);
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(ENEMY_SIZE),
                    color: ENEMY_COLOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, top - ENEMY_SIZE.y / 2. - 5., 10.),
                    scale: Vec3::new(1., 1., 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(Speed { x: 0.0, y: 0.0 })
            .insert(SpriteSize(ENEMY_SIZE))
            .insert(Movable {
                despawn: false,
                player: false,
                enemy: true,
            });

        enemy_count.0 += 1;
    }
}

fn enemy_movement(mut query: Query<&mut Speed, With<Enemy>>) {
    let mut rng = thread_rng();
    for mut speed in query.iter_mut() {
        speed.x = rng.gen_range(-7.0..7.0);
        speed.y = rng.gen_range(-7.0..7.0);
    }
}

fn enemy_fire_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1. / 60.) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn enemy_fire(mut commands: Commands, enemy_query: Query<&Transform, With<Enemy>>) {
    for &tf in enemy_query.iter() {
        let (x, y) = (tf.translation.x, tf.translation.y);

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(ENEMY_BULLET_SIZE),
                    color: ENEMY_BULLET_COLOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y - ENEMY_SIZE.y / 2. - 5., 10.),
                    scale: Vec3::new(1., 1., 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Speed { x: 0.0, y: -5.0 })
            .insert(Bullet)
            .insert(FromEnemy)
            .insert(SpriteSize(ENEMY_BULLET_SIZE))
            .insert(Movable {
                despawn: true,
                player: false,
                enemy: false,
            });
    }
}

fn enemy_collide(
    mut commands: Commands,
    mut enemy_one_query: Query<(Entity, &Transform, &Speed, &SpriteSize), With<Enemy>>,
    mut enemy_two_query: Query<(Entity, &Transform, &Speed, &SpriteSize), With<Enemy>>,
) {
    for (e1, tf1, speed1, size1) in enemy_one_query.iter_mut() {
        for (e2, tf2, speed2, size2) in enemy_two_query.iter_mut() {
            if e1 == e2 {
                continue;
            }

            if collide(tf1.translation, size1.0, tf2.translation, size2.0)
                // Is ther a better option than just this? TODO
                .is_some()
            {
                commands.entity(e1).insert(Speed {
                    x: -speed1.x,
                    y: -speed1.y,
                });
                commands.entity(e2).insert(Speed {
                    x: -speed2.x,
                    y: -speed2.y,
                });
            }
        }
    }
}
