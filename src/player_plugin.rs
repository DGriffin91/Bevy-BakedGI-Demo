use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::{MouseMotion, MouseWheel};

use bevy::prelude::*;
use bevy::render::camera::CameraProjection;
use bevy::render::primitives::Frustum;

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

/// Mouse sensitivity and movement speed
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
    pub run_multiplier: f32,
    pub lock_y: bool,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.02,
            speed: 12.,
            run_multiplier: 3.0,
            lock_y: false,
        }
    }
}

/// Used in queries when you want flycams and not other cameras
#[derive(Component)]
pub struct FlyCam;

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

/// Spawns the `Camera3dBundle` to be controlled
fn setup_player(mut commands: Commands) {
    //let orthographic_projection = OrthographicProjection {
    //    scaling_mode: ScalingMode::FixedVertical,
    //    depth_calculation: DepthCalculation::Distance,
    //    scale: 30.0,
    //    ..Default::default()
    //};
    //let view_projection = orthographic_projection.get_projection_matrix();
    //let frustum = Frustum::from_view_projection(
    //    &view_projection,
    //    &Vec3::ZERO,
    //    &Vec3::Z,
    //    orthographic_projection.far(),
    //);
    //let ortho = OrthographicCameraBundle {
    //    camera: Camera {
    //        name: Some(CameraPlugin::CAMERA_3D.to_string()),
    //        near: orthographic_projection.near,
    //        far: orthographic_projection.far,
    //        ..Default::default()
    //    },
    //    orthographic_projection,
    //    visible_entities: VisibleEntities::default(),
    //    frustum,
    //    transform: Transform::from_xyz(-20.0, 5.0, 0.0)
    //        .looking_at(Vec3::ZERO + Vec3::new(0.0, 0.0, 5.0), Vec3::Y),
    //    global_transform: Default::default(),
    //};
    let perspective_projection = PerspectiveProjection::default();
    let view_projection = perspective_projection.get_projection_matrix();
    let frustum = Frustum::from_view_projection(
        &view_projection,
        &Vec3::ZERO,
        &Vec3::Z,
        100000.0, //perspective_projection.far()
    );
    let camera = commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-20.0, 5.0, 0.0)
                .looking_at(Vec3::ZERO + Vec3::new(0.0, 0.0, 5.0), Vec3::Y),
            perspective_projection,
            frustum,
            ..Default::default()
        })
        .insert(FlyCam)
        .id();

    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(4.0), Val::Px(4.0)),
            // center button
            margin: Rect::all(Val::Auto),
            ..Default::default()
        },
        color: Color::rgb(1.0, 1.0, 1.0).into(),
        ..Default::default()
    });
}

/// Handles keyboard input and movement
fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    settings: Res<MovementSettings>,
    mut query: Query<(&FlyCam, &mut Transform)>,
) {
    let window = windows.get_primary().unwrap();
    if window.is_focused() && window.cursor_locked() {
        for (_camera, mut transform) in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, local_z.y, local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);
            let mut run = 1.0;

            for key in keys.get_pressed() {
                match key {
                    KeyCode::LShift => run = 3.0,
                    _ => (),
                }
            }

            for key in keys.get_pressed() {
                match key {
                    KeyCode::W => velocity += forward,
                    KeyCode::S => velocity -= forward,
                    KeyCode::A => velocity -= right,
                    KeyCode::D => velocity += right,
                    _ => (),
                }
            }

            velocity = velocity.normalize_or_zero();

            transform.translation += run * velocity * time.delta_seconds() * settings.speed
        }
    }
}

fn player_change_speed(
    mut settings: ResMut<MovementSettings>,
    windows: Res<Windows>,
    mut mouse_wheel: EventReader<MouseWheel>,
) {
    let window = windows.get_primary().unwrap();
    if window.is_focused() && window.cursor_locked() {
        for ev in mouse_wheel.iter() {
            settings.speed = (ev.y + settings.speed).max(0.0);
        }
    }
}

/// Handles looking around if cursor is locked
fn player_look(
    settings: Res<MovementSettings>,
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<(&FlyCam, &mut Transform)>,
) {
    let window = windows.get_primary().unwrap();
    let mut pitch = state.pitch;
    let mut yaw = state.yaw;
    for (_camera, mut transform) in query.iter_mut() {
        for ev in state.reader_motion.iter(&motion) {
            if window.is_focused() && window.cursor_locked() {
                // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                // let window_scale = window.height().min(window.width());

                pitch -= (settings.sensitivity * ev.delta.y).to_radians(); //* window_scale
                yaw -= (settings.sensitivity * ev.delta.x).to_radians(); //* window_scale
            }

            pitch = pitch.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            transform.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }
    state.pitch = pitch;
    state.yaw = yaw;
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape) {
        toggle_grab_cursor(window);
    }
}

/// Contains everything needed to add first-person fly camera behavior to your game
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_startup_system(setup_player)
            .add_system(player_move)
            .add_system(player_look)
            .add_system(cursor_grab)
            .add_system(player_change_speed);
    }
}
