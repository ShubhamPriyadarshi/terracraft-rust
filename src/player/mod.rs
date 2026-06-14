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
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 10.0,
            jump_force: 10.0,
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
    println!("[PLAYER] Spawning player at (0, 4, 0)...");
    commands.spawn((
        Player::default(),
        Transform::from_xyz(0.0, 4.0, 0.0),
        Name::new("Player"),
    ));
    println!("[PLAYER] Player spawned at (0, 4, 0)");
}

pub fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;
        
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            direction += Vec3::Z;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            direction -= Vec3::Z;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction -= Vec3::X;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            direction += Vec3::X;
        }
        
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        
        // Gravity
        player.velocity.y -= 20.0 * time.delta_secs();
        
        // Jump
        if keyboard.pressed(KeyCode::Space) && player.is_on_ground {
            player.velocity.y = player.jump_force;
            player.is_on_ground = false;
            println!("[PLAYER] Jump!");
        }
        
        // Move
        player.velocity.x = direction.x * player.speed;
        player.velocity.z = direction.z * player.speed;
        
        transform.translation += player.velocity * time.delta_secs();
        
        // Ground collision
        if transform.translation.y < 1.0 {
            transform.translation.y = 1.0;
            player.velocity.y = 0.0;
            player.is_on_ground = true;
        }
        
        if time.elapsed_secs().fract() < 0.05 {
            println!("[PLAYER] Pos: ({:.1}, {:.1}, {:.1})", 
                transform.translation.x, 
                transform.translation.y, 
                transform.translation.z);
        }
    }
}

pub fn camera_control(
    mut mouse_motion: EventReader<MouseMotion>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in player_query.iter_mut() {
        for event in mouse_motion.read() {
            player.yaw += event.delta.x * 0.003;
            player.pitch -= event.delta.y * 0.003;
            player.pitch = player.pitch.clamp(-1.4, 1.4);
            println!("[CAMERA] delta=({}, {}) yaw={} pitch={}", 
                event.delta.x, event.delta.y, player.yaw, player.pitch);
        }
        
        transform.rotation = Quat::from_euler(EulerRot::YXZ, player.yaw, player.pitch, 0.0);
    }
}
