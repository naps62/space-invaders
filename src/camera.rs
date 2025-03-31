use crate::constants::*;
use bevy::{prelude::*, render::camera::ScalingMode, window::WindowResized};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (startup, init_aspect_ratio).chain())
            .add_systems(Update, update_aspect_ratio);
    }
}

/// Fits the camera viewport to the window size while maintaining a fixed aspect ratio.
#[derive(Component)]
struct FixedAspectRatio(f32);

fn startup(mut cmds: Commands) {
    cmds.spawn((
        Camera2d,
        FixedAspectRatio(ARENA_SIZE.x / ARENA_SIZE.y),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: ARENA_SIZE.x,
                min_height: ARENA_SIZE.y,
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(ARENA_SIZE.x / 2.0, ARENA_SIZE.y / 2.0, 0.0), //.looking_at(Vec3::ZERO, Vec3::Z), //.looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn init_aspect_ratio(
    mut cameras: Query<(&mut Camera, &FixedAspectRatio)>,
    window: Single<&Window>,
) {
    let window = window.into_inner();
    dbg!("setting viewport start");
    for (mut camera, aspect_ratio) in cameras.iter_mut() {
        set_viewport(&mut camera, aspect_ratio, window);
    }
}

fn update_aspect_ratio(
    mut e: EventReader<WindowResized>,
    mut cameras: Query<(&mut Camera, &FixedAspectRatio)>,
    windows: Query<&Window>,
) {
    for event in e.read() {
        dbg!("updating viewport");
        let window = windows.get(event.window).unwrap();
        for (mut camera, aspect_ratio) in cameras.iter_mut() {
            set_viewport(&mut camera, aspect_ratio, window);
        }
    }
}

fn set_viewport(camera: &mut Camera, aspect_ratio: &FixedAspectRatio, window: &Window) {
    // Calculate the new viewport size based on the window size and aspect ratio.
    let window_width = window.physical_width() as f32;
    let window_height = window.physical_height() as f32;
    let window_aspect = window_width / window_height;
    let (width, height) = if window_aspect > aspect_ratio.0 {
        (window_height * aspect_ratio.0, window_height)
    } else {
        (window_width, window_width / aspect_ratio.0)
    };

    // Update the camera viewport.
    let viewport = camera.viewport.get_or_insert_default();
    viewport.physical_size.x = width as u32;
    viewport.physical_size.y = height as u32;
    // Center the viewport in the window.
    viewport.physical_position.x = (window_width / 2.0 - width / 2.0) as u32;
    viewport.physical_position.y = (window_height / 2.0 - height / 2.0) as u32;
}
