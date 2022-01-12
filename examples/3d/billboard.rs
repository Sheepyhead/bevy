use bevy::{
    pbr::{Billboard, BillboardBundle},
    prelude::*,
};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate_camera)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // billboard
    commands.spawn_bundle(BillboardBundle {
        billboard: Billboard {
            bounds: Rect::<f32> {
                left: 100.0,
                right: 10.0,
                top: 20.0,
                bottom: 40.0,
            },
        },
        transform: Transform::from_xyz(1.0, 1.0, 1.0),
        ..BillboardBundle::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

// rotate the camera to demonstrate billboardness
fn rotate_camera(
    windows: Res<Windows>,
    mut camera: Query<(&mut Transform, &PerspectiveProjection), With<Camera>>,
) {
    let (mut transform, projection) = camera.single_mut();

    let mut pan = Vec2::new(1.0, 0.0);
    let rotation_move = Vec2::new(1.0, 0.0);

    let window = get_primary_window_size(&windows);
    let delta_x = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
    let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
    let yaw = Quat::from_rotation_y(-delta_x);
    let pitch = Quat::from_rotation_x(-delta_y);
    transform.rotation *= yaw; // rotate around global y axis
    transform.rotation *= pitch; // rotate around local x axis

    // make panning distance independent of resolution and FOV,
    let window = get_primary_window_size(&windows);
    pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
    // translate by local axes
    let right = transform.rotation * Vec3::X * -pan.x;
    let up = transform.rotation * Vec3::Y * pan.y;
    // make panning proportional to distance away from focus point
    let translation = (right + up) * 5.0;

    let rot_matrix = Mat3::from_quat(transform.rotation);
    transform.translation = translation + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, 5.0));
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();

    Vec2::new(window.width() as f32, window.height() as f32)
}
