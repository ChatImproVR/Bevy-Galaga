use bevy::{ecs::schedule::ShouldRun, prelude::*, time::FixedTimestep};
use rand::{thread_rng, Rng};

use std::f32::consts::PI;

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

            // .add_system(enemy_movement_input)
            // .add_system_set(
            //     SystemSet::new()
            //         .with_run_criteria(enemy_fire_criteria)
            //         .with_system(enemy_fire),
            // )
            ;
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut enemy_status: ResMut<EnemyState>,
    mut enemy_count: ResMut<EnemyCount>,
    time: Res<Time>,
    win_size: Res<WinSize>,
) {
    let now = time.elapsed_seconds_f64();
    let last_shot = enemy_status.last_shot;

    if enemy_count.0 < ENEMY_MAX{
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

        // enemy_status.spawned();
        enemy_count.0 += 1;
    }
}

fn enemy_movement(mut query: Query<&mut Speed, With<Enemy>>) {
    let mut rng = thread_rng();
    if let Ok(mut speed) = query.get_single_mut() {
        speed.x = rng.gen_range(-5.0..5.0);
        speed.y = rng.gen_range(-5.0..5.0);
    }
}
