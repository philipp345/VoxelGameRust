use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, testhello)
        .run();

}

fn testhello(){
    println!("Bevy is working !");
    testrun();
}

fn testrun(){

    //Ownership Testrun
    let x =5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    let z = String::from("hello");
    println!("z = {}", z);
    testrun2(z);
    println!("z = {z}");


    //Borrowing Testrun
    let mut y = 6;

    let p = &mut y;


    *p = 7;
    println!("p is {p}");

}

fn testrun2(string: String) {
    println!("string is {}", string);

}