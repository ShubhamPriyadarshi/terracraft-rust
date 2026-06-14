use bevy::prelude::*;

#[derive(Component)]
pub struct Mob {
    #[allow(dead_code)]
    pub health: u32,
    #[allow(dead_code)]
    pub max_health: u32,
    #[allow(dead_code)]
    pub damage: u32,
    #[allow(dead_code)]
    pub speed: f32,
    #[allow(dead_code)]
    pub target: Option<Entity>,
}

pub fn mob_spawner(
    mut commands: Commands,
    time: Res<Time>,
) {
    if time.elapsed_secs().fract() < 0.01 {
        commands.spawn((
            Mob {
                health: 50,
                max_health: 50,
                damage: 10,
                speed: 3.0,
                target: None,
            },
            Transform::from_xyz(5.0, 1.0, 5.0),
            Name::new("Mob"),
        ));
    }
}

pub fn mob_ai(
    _time: Res<Time>,
    _mob_query: Query<&Transform, With<Mob>>,
) {
}
