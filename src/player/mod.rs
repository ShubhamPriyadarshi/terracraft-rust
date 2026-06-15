use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
    #[allow(dead_code)]
    pub health: u32,
    #[allow(dead_code)]
    pub max_health: u32,
    pub is_on_ground: bool,
    pub velocity: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 12.0,
            jump_force: 12.0,
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
    let player_entity = commands.spawn((
        Player::default(),
        Transform::from_xyz(0.0, 4.0, 0.0),
        Name::new("Player"),
    )).id();

    let camera_entity = commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            is_active: true,
            ..default()
        },
        Transform::from_xyz(0.0, 1.6, 0.0),
        Name::new("PlayerCamera"),
    )).id();

    commands.entity(player_entity).add_child(camera_entity);

    println!("[PLAYER] Player spawned at (0, 4, 0)");
    println!("[PLAYER] Camera attached as child with local offset (0, 1.6, 0)");
}

pub fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in player_query.iter_mut() {
        let forward = Vec3::new(
            -player.yaw.sin(),
            0.0,
            -player.yaw.cos(),
        );
        let right = Vec3::new(
            -player.yaw.cos(),
            0.0,
            player.yaw.sin(),
        );

        let mut input_dir = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            input_dir += forward;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            input_dir -= forward;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            input_dir -= right;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            input_dir += right;
        }

        if input_dir.length() > 0.0 {
            input_dir = input_dir.normalize();
        }

        player.velocity.y -= 25.0 * time.delta_secs();

        if keyboard.pressed(KeyCode::Space) && player.is_on_ground {
            player.velocity.y = player.jump_force;
            player.is_on_ground = false;
        }

        player.velocity.x = input_dir.x * player.speed;
        player.velocity.z = input_dir.z * player.speed;

        transform.translation += player.velocity * time.delta_secs();

        if transform.translation.y < 1.0 {
            transform.translation.y = 1.0;
            player.velocity.y = 0.0;
            player.is_on_ground = true;
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
        }

        transform.rotation = Quat::from_euler(EulerRot::YXZ, player.yaw, player.pitch, 0.0);
    }
}
