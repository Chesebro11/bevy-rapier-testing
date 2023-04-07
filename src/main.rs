use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component)]
pub struct MoveSpeed(pub f32);

#[derive(Component)]
pub struct Player;

fn main () {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_startup_system(player_stuff)
    .add_startup_system(camera)
    .add_system(follow_player)
    .add_system(move_player)
    .add_system(track_player_movement)
    
    .run();
}

fn camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn track_player_movement(positions: Query<&Transform, With<Player>>) {
    for transform in positions.iter() {
        println!("Player Position: {}, {}", transform.translation.y, transform.translation.x)
    }
}

fn follow_player(
    mut transforms: Query<&mut Transform>,
    player: Query<Entity, With<Player>>,
    camera: Query<Entity, With<Camera>>,
    time: Res<Time>,
) {
    let player_transform = *transforms.get(player.single()).unwrap();
    let mut camera_transform = transforms.get_mut(camera.single()).unwrap();

    camera_transform.translation = camera_transform
        .translation
        .lerp(player_transform.translation, time.delta_seconds() * 20.);
}

fn player_stuff(mut commands: Commands) {
    // Player Stuff!
    // If I want to give the player an actual sprite add an asset server system parameter and in place of the color and size for sprite load the asset
    let mut player = commands.spawn(
        SpriteBundle{
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(30.0, 30.0,)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.,)),
            ..default()
        });

        player.insert((
            Player,
            Collider::cuboid(20., 20.),
            GravityScale(0.),
            MoveSpeed (25.)
        ));
        // Is Noah using this for giving stats or buffs?
        // attribute::insert_common(&mut player);
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &MoveSpeed), With<Player>>,
) {
    let (mut transform, speed) = query.single_mut();

    transform.translation.x -= keyboard_input.pressed(KeyCode::A) as i32 as f32 * speed.0;
    transform.translation.x += keyboard_input.pressed(KeyCode::D) as i32 as f32 * speed.0;
    transform.translation.y += keyboard_input.pressed(KeyCode::W) as i32 as f32 * speed.0;
    transform.translation.y -= keyboard_input.pressed(KeyCode::S) as i32 as f32 * speed.0;
}
