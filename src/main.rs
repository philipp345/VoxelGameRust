use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, testhello)
        .run();
}

fn testhello(){
    println!("Bevy is working !");
}
