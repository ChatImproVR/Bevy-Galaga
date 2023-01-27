use bevy::{
    prelude::*,
    math::{Vec3Swizzles, f64},
    sprite::collide_aabb::collide
};

use components::{
    Speed,
    Movable,
    Bullet,
    SpriteSize,
    Player,
    FromPlayer,
    Enemy,
    FromEnemy,
    BulletToSpawn,
    BulletToSpawnTimer,
};

use player::PlayerPlugin;


mod components;
mod player;

const PLAYER_BULLET_SPEED: f32 = 500.0;
const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 30.0); // Sqaure for now
// const PLAYER_STARING_POSITION: Vec2 = Vec2::new(0.0, -300.0);
const PLAYER_COLOR: Color = Color::rgb(0.0, 0.0, 1.0); // Blue
const PLAYER_BULLET_COLOR: Color = Color::rgb(0.0, 1.0, 0.0); // Green
const PLAYER_BULLET_SIZE : Vec2 = Vec2::new(3.0, 3.0);
const PLAYER_RESPAWN_TIME: f64 = 2.0;


const ENEMY_BULLET_SPEED: f32 = 500.0;
const ENEMY_SIZE: Vec2 = Vec2::new(30.0, 30.0); // Sqaure for now
const ENEMY_COLOR: Color = Color::rgb(1.0, 0.0, 0.0); // Red
const ENEMY_BULLET_COLOR: Color = Color::rgb(1.0, 1.0, 0.0); // Yellow
const ENEMY_BULLET_SIZE : Vec2 = Vec2::new(3.0, 3.0);

const TIME_STEP: f32 = 1./60.0;
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

// #[derive(Resource)]
// struct GameTextures{
//     player: Handle<TextureAtlas>,
//     enemy: Handle<TextureAtlas>,
//     player_bullet: Handle<TextureAtlas>,
//     enemy_bullet: Handle<TextureAtlas>,
// }

// #[derive(Default)]
// struct Player{
//     speed: f32,
//     size: Vec2,
//     color: Color,
//     position: Vec2,
//     bullet_speed: f32,
//     bullet_color: Color,
// }

fn main(){
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor{
                title: "Galaga".to_string(),
                width: 800.0,
                height: 600.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_startup_system(setup_system)
        .add_plugin(PlayerPlugin)
        .add_system(movement)
        .run();
    
}

fn setup_system(
    mut commands:Commands,
    mut windows: ResMut<Windows>,
)   {
    
    // camera

    commands.spawn(Camera2dBundle::default());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let window_size = Vec2::new(window.width(), window.height());


    // add WinSize resource
    let win_size = WinSize {w: window_size.x, h: window_size.y};
    commands.insert_resource(win_size);

    // add Player
    // let bottom = -window_size.y / 2.0;
    //     commands
    //         .spawn(SpriteBundle{
    //             sprite: Sprite{
    //                 custom_size: Some(PLAYER_SIZE),
    //                 color: PLAYER_COLOR,
    //                 ..Default::default()
    //             },
    //             transform: Transform{
    //                 translation: Vec3::new(
    //                     0.,
    //                     bottom + PLAYER_SIZE.y / 2. +5.,
    //                     10.,

    //                 ),
    //                 scale: Vec3::new(1., 1., 1.),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         })
    //         .insert(Player)
    //         .insert(Speed{x: 0.0, y: 0.0})
    //         .insert(SpriteSize(PLAYER_SIZE))
    //         .insert(Movable {despawn: false});
    
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
            const MARGIN: f32 = 200.0;
            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}