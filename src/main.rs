use std::time::Duration;

use bevy::{
    app::PluginGroupBuilder,
    core_pipeline::clear_color::ClearColorConfig,
    ecs::query::QuerySingleError, prelude::*,
    render::camera::ScalingMode,
};

#[derive(Component, Debug)]
pub struct Player {
    pub speed: f32,
}

#[derive(Resource)]
pub struct Money(pub f32);

#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer,
}

fn main() {
    App::new()
        .add_plugins(custom_plugins())
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .add_systems(Update, spawn_pig)
        .add_systems(Update, pig_lifetime)
        .insert_resource(Money(100.0))
        .run();
}

fn custom_plugins() -> PluginGroupBuilder {
    DefaultPlugins.set(ImagePlugin::default_nearest()).set(
        WindowPlugin {
            primary_window: Some(Window {
                resolution: (640.0, 460.0).into(),
                title: "App".into(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        },
    )
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut camera = Camera2dBundle::default();

    camera.camera_2d.clear_color =
        ClearColorConfig::Custom(Color::WHITE);
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };
    commands.spawn(camera);

    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                // custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            texture,
            ..Default::default()
        },
        Player { speed: 300.0 },
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        if input.pressed(KeyCode::W) {
            transform.translation.y +=
                player.speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -=
                player.speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x +=
                player.speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -=
                player.speed * time.delta_seconds();
        }
    }
}

fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    players: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    let player_transform = match players.get_single() {
        Ok(player) => player,
        Err(QuerySingleError::NoEntities(_)) => {
            error!("Error: There is no player!");
            return;
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            error!("Error: There is more than one player!");
            return;
        }
    };

    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!(
            "Spent $10 on a pig, remainging money: ${:?}",
            money.0
        );
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: player_transform.translation,
                    ..Default::default()
                },
                texture: asset_server.load("pig.png"),
                ..Default::default()
            },
            Pig {
                lifetime: Timer::from_seconds(
                    2.0,
                    TimerMode::Once,
                ),
            },
        ));
    } else {
        info!(
                "Sorry not enough balance to buy a pig, remaining balance: ${:?}",
                money.0
            );
    }
}

fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut money: ResMut<Money>,
    mut pigs: Query<(Entity, &mut Pig)>,
) {
    for (pig_entity, mut pig) in &mut pigs {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            money.0 += 15.0;

            commands.entity(pig_entity).despawn();

            info!(
                "Pig sold for $15! Current Balance: ${:?}",
                money.0
            );
        }
    }
}
