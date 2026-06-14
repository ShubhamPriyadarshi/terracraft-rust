use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
    pub health: u32,
    pub max_health: u32,
    pub is_on_ground: bool,
    pub velocity: Vec3,
    yaw: f32,
    pitch: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 8.0,
            jump_force: 8.0,
            health: 100,
            max_health: 100,
            is_on_ground: false,
            velocity: Vec3::ZERO,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

pub fn spawn_player(mut commands: Commands) {
    println!("[PLAYER] Spawning player at (0, 30, 0)...");
    commands.spawn((
        Player::default(),
        Transform::from_xyz(0.0, 30.0, 0.0),
        Name::new("Player"),
    ));
    println!("[PLAYER] Player spawned successfully");
}

pub fn player_movement(
    _time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction += Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction -= Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction -= Vec3::X;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += Vec3::X;
        }
        
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        
        player.velocity.y -= 20.0 * _time.delta_secs();
        
        if keyboard_input.pressed(KeyCode::Space) && player.is_on_ground {
            player.velocity.y = player.jump_force;
            player.is_on_ground = false;
        }
        
        player.velocity.x = direction.x * player.speed;
        player.velocity.z = direction.z * player.speed;
        
        transform.translation += player.velocity * _time.delta_secs();
    }
}

pub fn camera_control(
    mut mouse_motion: EventReader<MouseMotion>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in player_query.iter_mut() {
        for event in mouse_motion.read() {
            player.yaw += event.delta.x * 0.002;
            player.pitch -= event.delta.y * 0.002;
            player.pitch = player.pitch.clamp(-1.5, 1.5);
        }
        
        transform.rotation = Quat::from_euler(EulerRot::YXZ, player.yaw, player.pitch, 0.0);
    }
}
