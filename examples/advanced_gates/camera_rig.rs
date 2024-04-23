use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// Marks the root entity in a camera rig hierarchy.
#[derive(Component, Debug, Reflect)]
pub struct CameraRig {
    pub camera_entity: Entity,
    pub joint_entity: Entity,
    pub target: Option<Entity>,
}

/// Parent of the camera entity, allowing camera zoom and rotation.
#[derive(Component, Debug, Default, Reflect)]
pub struct CameraJoint;

/// Marks an entity as the primary camera.
#[derive(Component, Debug, Default, Reflect)]
pub struct PrimaryCamera;

/// Spawns a camera rig hierarchy
///
/// ```
/// +-- CameraRig (translation and rotation)
/// |  +-- CameraJoint (zoom)
/// |  |  +-- PrimaryCamera
/// ```
pub fn spawn_camera_rig(mut commands: Commands) {
    let mut camera_entity: Entity = Entity::PLACEHOLDER;
    let mut joint_entity: Entity = Entity::PLACEHOLDER;
    commands
        .spawn((
            InputManagerBundle::with_map(
                InputMap::new([
                    (CameraAction::Move, UserInput::from(VirtualDPad::wasd())),
                    (
                        CameraAction::Zoom,
                        UserInput::from(VirtualAxis::vertical_arrow_keys().inverted()),
                    ),
                    (
                        CameraAction::ZoomQuantized,
                        UserInput::from(VirtualAxis {
                            positive: InputKind::MouseWheel(MouseWheelDirection::Up),
                            negative: InputKind::MouseWheel(MouseWheelDirection::Down),
                        }),
                    ),
                ])
            ),
            SpatialBundle::default(),
        ))
        .with_children(|rig| {
            joint_entity = rig
                .spawn((
                    CameraJoint,
                    SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 10.0)),
                ))
                .with_children(|joint| {
                    camera_entity = joint
                        .spawn((
                            PrimaryCamera,
                            Camera3dBundle {
                                // projection: Projection::Perspective(PerspectiveProjection {
                                //     fov: (50.5_f32).to_radians(),
                                //     ..default()
                                // }),
                                ..default()
                            },
                        ))
                        .id();
                })
                .id();
        })
        .insert(CameraRig {
            camera_entity,
            joint_entity,
            target: None,
        });
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CameraAction {
    Move,
    Zoom,
    ZoomQuantized,
}

impl CameraAction {
    pub const ZOOM_SPEED: f32 = 20.0;
    pub const ZOOM_MIN: f32 = 1.0;
    pub const ZOOM_MAX: f32 = 100.0;
    pub const ZOOM_LEN: f32 = Self::ZOOM_MAX - Self::ZOOM_MIN;

    pub fn clamp_zoom(z: f32) -> f32 {
        z.clamp(Self::ZOOM_MIN, Self::ZOOM_MAX)
    }

    pub fn current_zoom(z: f32) -> f32 {
        (z - Self::ZOOM_MIN) / Self::ZOOM_LEN
    }
}

pub type CameraActionState = ActionState<CameraAction>;

pub fn control_camera_rig(
    mut query_rig: Query<(&CameraActionState, &CameraRig, &mut Transform)>,
    mut query_joint: Query<&mut Transform, (With<CameraJoint>, Without<CameraRig>)>,
    time: Res<Time>
) {
    for (action_state, rig, mut transform) in query_rig.iter_mut() {
        // translation applied to the root entity
        if action_state.pressed(&CameraAction::Move) {
            if let Some(clamped_axis) = action_state.clamped_axis_pair(&CameraAction::Move) {
                let translation = Vec3::new(clamped_axis.x(), clamped_axis.y(), 0.0);
                transform.translation += translation * time.delta_seconds();
            }
        }

        // zoom applied to the joint entity
        if action_state.pressed(&CameraAction::Zoom) {
            let clamped_value = action_state.value(&CameraAction::Zoom);
            if let Ok(mut joint_transform) = query_joint.get_mut(rig.joint_entity) {
                let z = joint_transform.translation.z;
                let current_zoom = CameraAction::current_zoom(z);

                let speed =
                    CameraAction::ZOOM_SPEED + CameraAction::ZOOM_SPEED * (current_zoom * 8.0);
                let zoom = clamped_value * speed * time.delta_seconds();

                {
                    joint_transform.translation.z += zoom;
                    joint_transform.translation.z = CameraAction::clamp_zoom(
                        joint_transform.translation.z
                    );
                }
            }
        }
    }
}

pub struct CameraRigPlugin;

impl Plugin for CameraRigPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraAction>()
            .register_type::<CameraActionState>()
            .register_type::<CameraRig>()
            .register_type::<CameraJoint>()
            .register_type::<PrimaryCamera>();

        app.add_plugins(InputManagerPlugin::<CameraAction>::default())
            .add_systems(Startup, spawn_camera_rig)
            .add_systems(Update, control_camera_rig);
    }
}
