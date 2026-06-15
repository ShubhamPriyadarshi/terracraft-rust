use bevy::prelude::*;

pub fn setup_lighting(mut commands: Commands) {
    // Directional light (sun) — negative angle points the light DOWN toward the ground
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.9),
            illuminance: 5000.0,
            ..default()
        },
        // Light direction: pointing down at a 45-degree angle
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
        Name::new("Sun"),
    ));

    // Ambient light so everything is visible even in shadow
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.4, 0.45, 0.55),
        brightness: 400.0,
    });

    println!("[LIGHT] DirectionalLight: illuminance=5000, angle=-45deg");
    println!("[LIGHT] AmbientLight: brightness=400, color=(0.4, 0.45, 0.55)");
}
