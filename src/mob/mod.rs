use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Mob {
    pub health: u32,
    pub max_health: u32,
    pub damage: u32,
    pub speed: f32,
    pub target: Option<Entity>,
    pub state: MobState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MobState {
    Idle,
    Chasing,
}

pub fn mob_spawner(
    time: Res<Time>,
    mut timer: Local<f32>,
    mut commands: Commands,
) {
    *timer += time.delta_secs();
    
    if *timer > 30.0 {
        *timer = 0.0;
        
        let mut rng = rand::thread_rng();
        
        for _ in 0..3 {
            let x = rng.gen_range(-64.0..64.0);
            let z = rng.gen_range(-64.0..64.0);
            let y = 30.0;
            
            commands.spawn((
                Mob {
                    health: 20,
                    max_health: 20,
                    damage: 3,
                    speed: 3.0,
                    target: None,
                    state: MobState::Idle,
                },
                Transform::from_xyz(x, y, z),
                Name::new("Mob"),
            ));
        }
    }
}

pub fn mob_ai(
    time: Res<Time>,
    mut mob_query: Query<(&mut Transform, &mut Mob), Without<super::player::Player>>,
    player_query: Query<&Transform, With<super::player::Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut transform, mut mob) in mob_query.iter_mut() {
            let player_pos = player_transform.translation;
            let mob_pos = transform.translation;
            let distance = mob_pos.distance(player_pos);
            
            if distance < 32.0 {
                mob.state = MobState::Chasing;
                let direction = (player_pos - mob_pos).normalize();
                transform.translation += direction * mob.speed * time.delta_secs();
            } else {
                mob.state = MobState::Idle;
                transform.translation.x += (rand::random::<f32>() - 0.5) * 0.1;
                transform.translation.z += (rand::random::<f32>() - 0.5) * 0.1;
            }
            
            transform.translation.y -= 10.0 * time.delta_secs();
            if transform.translation.y < 20.0 {
                transform.translation.y = 20.0;
            }
        }
    }
}
