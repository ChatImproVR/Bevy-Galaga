use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
    time::FixedTimestep,
};

const TIME_STEP: f32 = 1.0 / 60.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0); // Black

const WALL_THICKNESS: f32 = 10.0;

const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;

const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;
const WALL_COLOR: Color = Color::rgb(0.0, 0.0, 0.0); // Black

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_BULLET_SPEED: f32 = 500.0;
const PLAYER_SIZE: Vec3 = Vec3::new(100.0, 100.0, 1.0); // Sqaure for now
const PLAYER_STARING_POSITION: Vec3 = Vec3::new(0.0, -300.0, 0.0);
const PLAYER_COLOR: Color = Color::rgb(0.0, 0.0, 1.0); // Blue
const PLAYER_BULLET_COLOR: Color = Color::rgb(0.0, 1.0, 0.0); // Green

const ENEMY_SPEED: f32 = 200.0;
const ENEMY_BULLET_SPEED: f32 = 500.0;
const ENEMY_SIZE: Vec3 = Vec3::new(100.0, 100.0, 1.0); // Sqaure for now
const ENEMY_COLOR: Color = Color::rgb(1.0, 0.0, 0.0); // Red
const ENEMY_BULLET_COLOR: Color = Color::rgb(1.0, 1.0, 0.0); // Yellow

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement_system)
                .with_system(player_bullet_movement_system)
                .with_system(enemy_spawn)
                .with_system(enemy_movement_system)
                .with_system(enemy_bullet_movement_system)
                .with_system(collision_system)
                .with_system(scoreboard_system),
        )
        .add_system(update_scoreboard)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct PlayerBullet;

#[derive(Component)]
struct EnemyBullet;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct CollisionEvent;

#[derive(Component)]
struct Scoreboard {
    score: usize,
}

#[derive(Component)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

enum WallLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.0),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.0),
            WallLocation::Top => Vec2::new(0.0, TOP_WALL),
            WallLocation::Bottom => Vec2::new(0.0, BOTTOM_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left => Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS),
            WallLocation::Right => Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS),
            WallLocation::Top => Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS),
            WallLocation::Bottom => Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS),
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    // Player
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: PLAYER_STARING_POSITION,
                scale: PLAYER_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            ..default()
        },
        Player,
        Collider,
    ));

    // Player Bullets
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -1000.0, 0.0),
                scale: Vec3::new(10.0, 10.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_BULLET_COLOR,
                ..default()
            },
            ..default()
        },
        PlayerBullet,
        Collider,
    ));

    // Enemy
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 300.0, 0.0),
                scale: ENEMY_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: ENEMY_COLOR,
                ..default()
            },
            ..default()
        },
        Enemy,
        Collider,
    ));

    // Enemy Bullets
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -1000.0, 0.0),
                scale: Vec3::new(10.0, 10.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: ENEMY_BULLET_COLOR,
                ..default()
            },
            ..default()
        },
        EnemyBullet,
        Collider,
    ));

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
}
