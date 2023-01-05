use bevy::{
    prelude::*,
    time::FixedTimestep,
};

const TIME_STEP: f32 = 1.0 / 60.0;

pub struct IsoCameraPlugin;

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct MovementBindings {
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
}

#[derive(Component)]
pub struct IsoCamera {
    pub focus: Vec3,
    pub radius: f32,
}

#[derive(Component)]
pub struct Player {
    pub facing: Direction,
    pub movement_bindings: MovementBindings,
}

impl Default for IsoCamera {
    fn default() -> Self {
        IsoCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            facing: Direction::North,
            movement_bindings: MovementBindings {
                up: KeyCode::W,
                down: KeyCode::S,
                left: KeyCode::A,
                right: KeyCode::D,
            },
        }
    }
}

impl Plugin for IsoCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            .with_system(move_camera)
            .with_system(move_player.before(move_camera))
        );
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let (player, mut transform) = query.single_mut();
    let mut translation = Vec3::new(0.0, 0.0, 0.0);

    if keyboard_input.just_pressed(player.movement_bindings.up) {
        translation.z -= 1.0;
    }

    if keyboard_input.just_pressed(player.movement_bindings.down) {
        translation.z += 1.0;
    }

    if keyboard_input.just_pressed(player.movement_bindings.left) {
        translation.x -= 1.0;
    }

    if keyboard_input.just_pressed(player.movement_bindings.right) {
        translation.x += 1.0;
    }

    transform.translation += translation;
}

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<(&mut IsoCamera, &mut Transform), Without<Player>>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
) {

    let (mut player, player_transform) = player_query.single_mut();

    for (mut camera, mut transform) in camera_query.iter_mut() {
        
        let mut yaw: f32 = 0.0;
    
        if keyboard_input.just_pressed(KeyCode::Left) {
            yaw += 2.0 * std::f32::consts::PI / 4.0;
            (player.facing, player.movement_bindings) = change_player_facing(&player.facing, true);

            match player.facing {
                Direction::North => println!("N"),
                Direction::East => println!("E"),
                Direction::South => println!("S"),
                Direction::West => println!("W"),
            };
        }
    
        if keyboard_input.just_pressed(KeyCode::Right) {
            yaw -= 2.0 * std::f32::consts::PI / 4.0;
            (player.facing, player.movement_bindings) = change_player_facing(&player.facing, false);
        
            match player.facing {
                Direction::North => println!("N"),
                Direction::East => println!("E"),
                Direction::South => println!("S"),
                Direction::West => println!("W"),
            };
        }
        
        transform.rotation =  Quat::from_rotation_y(yaw) * transform.rotation;

        camera.focus = player_transform.translation;

        let rot_matrix = Mat3::from_quat(transform.rotation);
        transform.translation = camera.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, camera.radius));
    }
}

fn change_player_facing(direction: &Direction, left: bool) -> (Direction, MovementBindings) {

    let new_direction;

    if left {
        new_direction = match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    } else {
        new_direction = match direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
    }

    let new_binding = match new_direction {
        Direction::North => MovementBindings {
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
        },
        Direction::East => MovementBindings {
            up: KeyCode::D,
            down: KeyCode::A,
            left: KeyCode::W,
            right: KeyCode::S,
        },
        Direction::South => MovementBindings {
            up: KeyCode::S,
            down: KeyCode::W,
            left: KeyCode::D,
            right: KeyCode::A,
        },
        Direction::West => MovementBindings {
            up: KeyCode::D,
            down: KeyCode::A,
            left: KeyCode::W,
            right: KeyCode::S,
        },
    };

    (new_direction, new_binding)
}