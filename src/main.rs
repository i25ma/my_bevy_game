use bevy::prelude::*;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, add_peple)
    .add_systems(Update, (hello_world, greet_people))
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

// your first Components 
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_peple(mut commands: Commands) {
    commands.spawn((Person, Name("zhangsan".to_string())));
    commands.spawn((Person, Name("lisi".to_string())));
    commands.spawn((Person, Name("wangwu".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}