use bevy::prelude::*;

pub fn setup_lighting(mut commands: Commands) {
    // Directional light (sun)
    commands.spawn((
        DirectionalLight {
            illuminance: 1000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_4)),
        Name::new("Sun"),
    ));
    
    // Ambient light so shadows aren't pitch black
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.5, 0.5, 0.6),
        brightness: 200.0,
    });
    
    println!("[LIGHT] Sun and ambient light added");
}
