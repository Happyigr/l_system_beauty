use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

pub fn move_camera(
    mut camera_q: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut c_pos = camera_q.single_mut();

    let mut move_dir = Vec2::new(0., 0.);

    if input.pressed(KeyCode::KeyW) {
        move_dir.y += 1.;
    }
    if input.pressed(KeyCode::KeyS) {
        move_dir.y -= 1.;
    }
    if input.pressed(KeyCode::KeyA) {
        move_dir.x -= 1.;
    }
    if input.pressed(KeyCode::KeyD) {
        move_dir.x += 1.;
    }

    c_pos.translation += (move_dir * 300. * time.delta_seconds()).extend(0.);
}

pub fn zoom_camera(
    mut camera_q: Query<&mut OrthographicProjection, With<MainCamera>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut projection = camera_q.get_single_mut().unwrap();

    if input.pressed(KeyCode::KeyI) {
        projection.scale -= 1. * time.delta_seconds();
    }

    if input.pressed(KeyCode::KeyO) {
        projection.scale += 1. * time.delta_seconds();
    }
}
