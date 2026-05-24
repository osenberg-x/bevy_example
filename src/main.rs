use bevy::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Component, Default)]
struct Velocity(Vec3);

#[derive(Component, Default)]
struct FpsCounter {
    frame_count: f64,
    current_time: f64,
    previous_time: f64,
}

// 自定义消息
#[derive(Message, Debug)]
struct CustomMessage {
    // 发出事件的实体 ID
    entity: Entity,
    // 其他信息
    some_infos: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (combo_key_system, jump_system, gravity_system))
        .add_systems(Update, (write_message, read_message))
        .add_message::<CustomMessage>()
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_empty().insert(FpsCounter::default());
    // 立方体
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Velocity::default(),
    ));
    // 灯光
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn write_message(
    mut message: MessageWriter<CustomMessage>,
    entity_and_transform: Query<Entity, With<Velocity>>,
) {
    for entiry in entity_and_transform {
        message.write(CustomMessage {
            entity: entiry,
            some_infos: 123.0,
        });
    }
}

fn read_message(mut messages: MessageReader<CustomMessage>) {
    for message in messages.read() {
        println!("message {:?}", message)
    }
}

fn combo_key_system(input: Res<ButtonInput<KeyCode>>) {
    let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let ctrl = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    if shift && ctrl && input.just_pressed(KeyCode::KeyA) {
        println!("Combo key pressed!");
    }
}

// 空格跳跃
fn jump_system(mut query: Query<&mut Velocity>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        for mut velocity in &mut query {
            velocity.0.y = 5.0;
        }
    }
}

fn gravity_system(mut query: Query<(&mut Velocity, &mut Transform)>, time: Res<Time>) {
    const GRAVITY: f32 = -9.8;
    const GROUND_Y: f32 = 0.5;

    for (mut velocity, mut transform) in &mut query {
        velocity.0.y += GRAVITY * time.delta_secs();
        transform.translation += velocity.0 * time.delta_secs();
        if transform.translation.y < GROUND_Y {
            transform.translation.y = GROUND_Y;
            velocity.0.y = 0.0;
        }
    }
}

fn asset_system(time: Res<Time>) {
    println!(
        "this how I see time: delta {:?}, elapsed: {:?}",
        time.delta(),
        time.elapsed()
    )
}

fn counter_fps(mut counters: Query<&mut FpsCounter>) {
    let mut counter = counters
        .single_mut()
        .expect("Expected exactly on FpsCounter entity.");

    if counter.previous_time == 0.0 {
        counter.previous_time = counter.current_time;
    }

    let now = SystemTime::now();
    let unix_time_f32 = now.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();

    counter.current_time = unix_time_f32;
    if counter.current_time - counter.previous_time >= 1.0 {
        println!(
            "FPS: {}",
            counter.frame_count / (counter.current_time - counter.previous_time)
        );
        counter.frame_count = 0.0;
        counter.previous_time = counter.current_time;
    } else {
        counter.frame_count += 1.0;
    }
}
