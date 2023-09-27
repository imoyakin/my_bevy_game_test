use std::f32::consts::{FRAC_PI_4, PI};

#[cfg(target_os = "macos")]
use bevy::window::CompositeAlphaMode;
use bevy::{
    input::keyboard::KeyboardInput,
    log::{Level, LogPlugin},
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    render::camera::ScalingMode,
    window::{close_on_esc, WindowResolution},
};
use bevy_input_mapper::{
    input::{events::*, mouse::MouseAxis},
    AutoBinder, InputMapper, InputMapperPlugin,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[derive(Component)]
pub struct Spin;

fn main() {
    let mut app = App::new();
    // App::new()
    // .insert_resource(AmbientLight {
    //     color: Color::WHITE,
    //     brightness: 1.0 / 5.0f32,
    // })
    // .insert_resource(DirectionalLightShadowMap { size: 4096 })
    // .add_plugins(DefaultPlugins)
    // .add_systems(Startup, setup)
    // .add_systems(Update, animate_light_direction)
    // .run();

    app.add_plugins(
        DefaultPlugins
            // .set(LogPlugin {
            //     level: Level::INFO,
            //     filter: "bevy_mod_fbx=trace,wgpu=warn".to_owned(),
            // })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    // Setting `transparent` allows the `ClearColor`'s alpha value to take effect
                    transparent: true,
                    // Disabling window decorations to make it feel more like a widget than a window
                    decorations: false,
                    #[cfg(target_os = "macos")]
                    composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                    resolution: WindowResolution::new(756., 574.),
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins(InputMapperPlugin)
    .insert_resource(DirectionalLightShadowMap { size: 4096 })
    .insert_resource(ClearColor(Color::NONE))
    // .add_system(spin_cube)
    // .add_system(close_on_esc)
    .add_plugins(PanOrbitCameraPlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, keyboard_controls);

    // .add_systems(Update, logger);

    app.run();
}

fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // cmd.spawn(Camera2dBundle::default());
    // Orthographic camera
    // cmd.spawn((
    //     Camera3dBundle {
    //         transform: Transform::from_xyz(0.7, 0.7, 1.0)
    //             .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
    //         ..default()
    //     },
    //     // EnvironmentMapLight {
    //     //     diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
    //     //     specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
    //     // },
    // ));
    // cmd.spawn(Camera3dBundle {
    //     projection: OrthographicProjection {
    //         scale: 3.0,
    //         scaling_mode: ScalingMode::FixedVertical(2.0),
    //         ..default()
    //     }
    //     .into(),
    //     transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });

    // light
    // cmd.spawn(PointLightBundle {
    //     transform: Transform::from_xyz(3.0, 8.0, 5.0),
    //     ..default()
    // });

    // cmd.spawn((
    //     Camera3dBundle {
    //         transform: Transform::from_xyz(0.7, 0.7, 1.0)
    //             .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
    //         ..default()
    //     },
    //     // EnvironmentMapLight {
    //     //     diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
    //     //     specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
    //     // },
    // ));

    // Camera
    cmd.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    cmd.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });

    // Cube
    cmd.spawn((
        SceneBundle {
            scene: asset_server.load("FlightHelmet/FlightHelmet.gltf#Scene0"),
            ..default()
        },
        // SceneBundle {
        //     scene: asset_server.load("chuxiong.gltf#Scene0"),
        //     ..default()
        // },
        Spin,
    ));
}

fn keyboard_controls(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    for (mut pan_orbit, mut transform) in pan_orbit_query.iter_mut() {
        if key_input.pressed(KeyCode::ControlLeft) {
            // Jump focus point 1m using Ctrl+Shift + Arrows
            if key_input.pressed(KeyCode::ShiftLeft) {
                if key_input.just_pressed(KeyCode::Right) {
                    pan_orbit.target_focus += Vec3::X;
                }
                if key_input.just_pressed(KeyCode::Left) {
                    pan_orbit.target_focus -= Vec3::X;
                }
                if key_input.just_pressed(KeyCode::Up) {
                    pan_orbit.target_focus += Vec3::Y;
                }
                if key_input.just_pressed(KeyCode::Down) {
                    pan_orbit.target_focus -= Vec3::Y;
                }
            } else {
                // Jump by 45 degrees using Left Ctrl + Arrows
                if key_input.just_pressed(KeyCode::Right) {
                    pan_orbit.target_alpha += 45f32.to_radians();
                }
                if key_input.just_pressed(KeyCode::Left) {
                    pan_orbit.target_alpha -= 45f32.to_radians();
                }
                if key_input.just_pressed(KeyCode::Up) {
                    pan_orbit.target_beta += 45f32.to_radians();
                }
                if key_input.just_pressed(KeyCode::Down) {
                    pan_orbit.target_beta -= 45f32.to_radians();
                }
            }
        }
        // Pan using Left Shift + Arrows
        else if key_input.pressed(KeyCode::ShiftLeft) {
            let mut delta_translation = Vec3::ZERO;
            if key_input.pressed(KeyCode::Right) {
                delta_translation += transform.rotation * Vec3::X * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Left) {
                delta_translation += transform.rotation * Vec3::NEG_X * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Up) {
                delta_translation += transform.rotation * Vec3::Y * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Down) {
                delta_translation += transform.rotation * Vec3::NEG_Y * time.delta_seconds();
            }
            transform.translation += delta_translation;
            pan_orbit.target_focus += delta_translation;
        }
        // Smooth rotation using arrow keys without modifier
        else {
            if key_input.pressed(KeyCode::Right) {
                pan_orbit.target_alpha += 50f32.to_radians() * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Left) {
                pan_orbit.target_alpha -= 50f32.to_radians() * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Up) {
                pan_orbit.target_beta += 50f32.to_radians() * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Down) {
                pan_orbit.target_beta -= 50f32.to_radians() * time.delta_seconds();
            }

            // Zoom with Z and X
            if key_input.pressed(KeyCode::Z) {
                pan_orbit.radius = pan_orbit
                    .radius
                    .map(|radius| radius - 5.0 * time.delta_seconds());
            }
            if key_input.pressed(KeyCode::X) {
                pan_orbit.radius = pan_orbit
                    .radius
                    .map(|radius| radius + 5.0 * time.delta_seconds());
            }
        }

        // Force camera to update its transform
        pan_orbit.force_update = true;
    }
}

// fn bind_keys(
//     mut im: ResMut<InputMapper>
// ) {
//     im.keyboard_binding.bind(KeyCode::Space, "jump".to_string());
//     im.mouse_button_binding.bind(MouseButton::Left, "fire".to_string());
//     im.mouse_axis_binding.bind(MouseAxis::PositiveX, "look_right".to_string());
//     im.mouse_axis_binding.bind(MouseAxis::NegativeY, "look_up".to_string());
//     im.mouse_axis_binding.bind(MouseAxis::NegativeX, "look_left".to_string());
//     im.mouse_axis_binding.bind(MouseAxis::PositiveY, "look_down".to_string());
// }

// fn logger(
//     mut action_active: EventReader<InputActionActive>,
//     mut action_started: EventReader<InputActionStarted>,
//     mut action_continuing: EventReader<InputActionContinuing>,
//     mut action_finished: EventReader<InputActionFinished>,
// ) {
//     for ev in action_active.iter() {
//         info!("Action Active: {}, {}", ev.0, ev.1);
//     }
//     for ev in action_started.iter() {
//         info!("Action Started: {}, {}", ev.0, ev.1);
//     }
//     for ev in action_continuing.iter() {
//         info!("Action Continuing: {}, {}", ev.0, ev.1);
//     }
//     for ev in action_finished.iter() {
//         info!("Action Finished: {}", ev.0);
//     }
// }

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}
