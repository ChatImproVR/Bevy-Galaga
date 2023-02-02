#![allow(unused)]

use std::collections::HashSet;

use bevy::{
    math::{f64, Vec3Swizzles},
    prelude::*,
    sprite::collide_aabb::collide,
};

use components::{Bullet, Enemy, FromEnemy, FromPlayer, Movable, Player, Speed, SpriteSize};

use enemy::EnemyPlugin;
use player::PlayerPlugin;

mod components;
mod enemy;
mod player;

const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 30.0); // Sqaure for now
const PLAYER_COLOR: Color = Color::rgb(0.0, 0.0, 1.0); // Blue
const PLAYER_BULLET_COLOR: Color = Color::rgb(0.0, 1.0, 0.0); // Green
const PLAYER_BULLET_SIZE: Vec2 = Vec2::new(3.0, 3.0);
const PLAYER_RESPAWN_TIME: f64 = 2.0;

const ENEMY_SIZE: Vec2 = Vec2::new(30.0, 30.0); // Sqaure for now
const ENEMY_COLOR: Color = Color::rgb(1.0, 0.0, 0.0); // Red
const ENEMY_BULLET_COLOR: Color = Color::rgb(1.0, 1.0, 0.0); // Yellow
const ENEMY_BULLET_SIZE: Vec2 = Vec2::new(3.0, 3.0);
const ENEMY_MAX: u32 = 3;

const SCORE_COLOR: Color = Color::rgb(0.37, 0.37, 0.37); // Gray

const TIME_STEP: f32 = 1. / 60.0;
const BASE_SPEED: f32 = 100.0;

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct PlayerState {
    on: bool,
    last_shot: f64,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            last_shot: -1.0,
        }
    }
}

impl PlayerState {
    pub fn shot(&mut self, time: f64) {
        self.last_shot = time;
        self.on = false;
    }
    pub fn spawned(&mut self) {
        self.on = true;
        self.last_shot = -1.0;
    }
}

#[derive(Resource, Default)]
pub struct EnemyState;

#[derive(Resource)]
struct EnemyCount(u32);

#[derive(Resource)]
struct Score(u32);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Galaga".to_string(),
                width: 400.0,
                height: 1000.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_startup_system(setup_system)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_system(movement)
        .add_system(enemy_hit_player)
        .add_system(player_hit_enemy)
        .add_system(update_score)
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
) {
    // camera

    commands.spawn(Camera2dBundle::default());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let window_size = Vec2::new(window.width(), window.height());

    // add WinSize resource
    let win_size = WinSize {
        w: window_size.x,
        h: window_size.y,
    };

    // score board

    commands.spawn(TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "0".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 150.0,
                    color: SCORE_COLOR,
                },
            }],
            alignment: Default::default(),
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(win_size.w / 2. - 50.),
                top: Val::Px(win_size.h / 2. - 50.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    commands.insert_resource(win_size);
    commands.insert_resource(EnemyCount(0));
    commands.insert_resource(Score(0));
}

fn movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Speed, &mut Transform, &Movable)>,
) {
    for (entity, speed, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += speed.x * TIME_STEP * BASE_SPEED;
        translation.y += speed.y * TIME_STEP * BASE_SPEED;

        if movable.despawn {
            const MARGIN: f32 = 10.0;
            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        } else if movable.player {
            if translation.x < -win_size.w / 2. + PLAYER_SIZE.y / 2. + 5. {
                translation.x = -win_size.w / 2. + PLAYER_SIZE.y / 2. + 5.;
            } else if translation.x > win_size.w / 2. - PLAYER_SIZE.y / 2. - 5. {
                translation.x = win_size.w / 2. - PLAYER_SIZE.y / 2. - 5.;
            }
            if translation.y < -win_size.h / 2. + PLAYER_SIZE.y / 2. + 5. {
                translation.y = -win_size.h / 2. + PLAYER_SIZE.y / 2. + 5.;
            } else if translation.y > -win_size.h / 4. {
                translation.y = -win_size.h / 4.;
            }
        } else if movable.enemy {
            if translation.x < -win_size.w / 2. + ENEMY_SIZE.y / 2. + 5. {
                translation.x = -win_size.w / 2. + ENEMY_SIZE.y / 2. + 5.;
            } else if translation.x > win_size.w / 2. - ENEMY_SIZE.y / 2. - 5. {
                translation.x = win_size.w / 2. - ENEMY_SIZE.y / 2. - 5.;
            }
            if translation.y < -win_size.h / 2. + ENEMY_SIZE.y / 2. + 5. {
                translation.y = -win_size.h / 2. + ENEMY_SIZE.y / 2. + 5.;
            } else if translation.y > win_size.h / 2. - ENEMY_SIZE.y / 2. - 5. {
                translation.y = win_size.h / 2. - ENEMY_SIZE.y / 2. - 5.;
            }
        }
    }
}

fn player_hit_enemy(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    mut score: ResMut<Score>,
    bullet_query: Query<(Entity, &Transform, &SpriteSize), (With<Bullet>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    for (bullet_entity, bullet_transform, bullet_size) in bullet_query.iter() {
        if despawned_entities.contains(&bullet_entity) {
            continue;
        }

        let bullet_scale = Vec2::from(bullet_transform.scale.xy());

        for (enemy_entity, enemy_transform, enemy_size) in enemy_query.iter() {
            if despawned_entities.contains(&enemy_entity)
                || despawned_entities.contains(&bullet_entity)
            {
                continue;
            }

            let enemy_scale = Vec2::from(enemy_transform.scale.xy());

            let collision = collide(
                bullet_transform.translation,
                bullet_size.0 * bullet_scale,
                enemy_transform.translation,
                enemy_size.0 * enemy_scale,
            );

            if let Some(_) = collision {
                commands.entity(enemy_entity).despawn();
                despawned_entities.insert(enemy_entity);
                enemy_count.0 -= 1;
                score.0 += 1;

                commands.entity(bullet_entity).despawn();
                despawned_entities.insert(bullet_entity);
            }
        }
    }
}

fn enemy_hit_player(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    mut score: ResMut<Score>,
    time: Res<Time>,
    enemy_bullet_query: Query<(Entity, &Transform, &SpriteSize), (With<Bullet>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
    player_bullet_query: Query<(Entity, &Transform, &SpriteSize), (With<Bullet>, With<FromPlayer>)>,
) {
    if let Ok((player_entity, player_transform, player_size)) = player_query.get_single() {
        let player_scale = Vec2::from(player_transform.scale.xy());

        for (enemy_bullet_entity, enemy_bullet_transform, enemy_bullet_size) in
            enemy_bullet_query.iter()
        {
            let bullet_scale = Vec2::from(enemy_bullet_transform.scale.xy());

            let collision = collide(
                enemy_bullet_transform.translation,
                enemy_bullet_size.0 * bullet_scale,
                player_transform.translation,
                player_size.0 * player_scale,
            );

            if let Some(_) = collision {
                commands.entity(player_entity).despawn();
                player_state.shot(time.elapsed_seconds_f64());
                score.0 = 0;

                commands.entity(enemy_bullet_entity).despawn();

                for (player_bullet_entity, _, _) in player_bullet_query.iter() {
                    commands.entity(player_bullet_entity).despawn();
                }

                break;
            }
        }
    }
}

fn update_score(score_value: Res<Score>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[0].value = score_value.0.to_string();
}
