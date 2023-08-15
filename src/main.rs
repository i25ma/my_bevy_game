use bevy::prelude::*;


fn main() {
    App::new()
    .add_systems(Update, hello_world)
    .run();
}


// components
#[derive(Component)]
struct Position { x: f32, y: f32 }


// systems : normal rust funcitons

fn print_position_system(query: Query<&Transform>) {
    for transform in &query {
        println!("position: {:?}", transform.translation);
    }
}

// entities : a simple type containing a unique integer

struct Entity(u64);


// your first system

fn hello_world() {
    println!("hello world");
}