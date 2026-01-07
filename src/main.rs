mod voxel;
use voxel::chunk::Chunk;
use bevy::prelude::*;
use bevy::math::primitives::Plane3d;
use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy::color::palettes::basic::SILVER;
use bevy::math::primitives::Cuboid;
use bevy::color::Srgba;
use bevy::pbr::MeshMaterial3d;
use crate::voxel::chunk::update_visible_chunks;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, test_chunk_system)
        .add_systems(Update,player_movement)
        .add_systems(Update,mouse_look)
        .add_systems(Update,cursor_grab_system)
        .add_systems(Update,get_player_pos)
        .add_systems(Update,update_visible_chunks)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: Query<(&mut Window, &mut CursorOptions), With<PrimaryWindow>>
) {
    // Kamera + Player
    commands
        .spawn((
            Player,
            Transform::from_xyz(0.0, 1.5, 5.0),
            GlobalTransform::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 0.0, 0.0)
                    .looking_at(Vec3::ZERO, Vec3::Y),
            ));
        });

    // Licht
    commands.spawn((
                     PointLight {
                         intensity: 3000.0,
                         shadows_enabled: true,
                         ..default()
                     },
                     Transform::from_xyz(10.0, 10.0, 10.0),
    ))// 2. Fügen Sie die sichtbare Kugel als Kind hinzu.
        .with_children(|parent| {
            // Hängen Sie die Kugel als Kind an, indem Sie EINZELNE Komponenten hinzufügen
            parent.spawn((
                Mesh3d(meshes.add(Sphere::new(0.3).mesh().uv(32, 18))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Srgba::rgb(1.0, 1.0, 0.0).into(),
                    unlit: true,
                    ..default()
                })),
                Transform::default(),
            ));
        });

    // Boden
    commands.spawn((Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),

                   MeshMaterial3d(materials.add(StandardMaterial {
                       base_color: SILVER.into(),
                       metallic: 0.5,
                       perceptual_roughness: 0.5,
                       ..default()
                   }))
    ));



    // Ein paar Blöcke
    let block_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0).mesh());
    let block_material = materials.add(StandardMaterial {
        // Verwenden Sie Color::rgb direkt im Struct (kein .into())
        base_color: Srgba::rgb(0.2, 0.8, 0.2).into(),
        ..default()
    });
    for x in -5..5 {
        for z in -5..5 {
            commands.spawn((Mesh3d(block_mesh.clone()),
                           MeshMaterial3d(block_material.clone()),
                           Transform::from_xyz(x as f32 * 2.0, 0.5, z as f32 * 2.0),
            ));
        }
    }

    // Cursor initial „locked“ und versteckt
    if let Ok((mut primary_window, mut cursor_options)) = windows.single_mut() {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }
}

// Bewegung mit WASD
fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut p_transform) = query.single_mut() {
        let speed = 5.0;
        let delta = time.delta().as_secs_f32();

        // Berechnen Sie die lokalen Richtungen manuell basierend auf der Rotation
        // In Bevy 0.17 zeigt "forward" standardmäßig in die negative Z-Achse (-Z)
        let forward_vector = p_transform.rotation * Vec3::NEG_Z;
        let right_vector = p_transform.rotation * Vec3::X; // "Right" ist die positive X-Achse (+X)

        if keys.pressed(KeyCode::KeyW) {
            p_transform.translation += forward_vector * speed * delta;
        }
        if keys.pressed(KeyCode::KeyS) {
            p_transform.translation -= forward_vector * speed * delta;
        }
        if keys.pressed(KeyCode::KeyA) {
            p_transform.translation -= right_vector * speed * delta;
        }
        if keys.pressed(KeyCode::KeyD) {
            p_transform.translation += right_vector * speed * delta;
        }
    }
}
// Maus‑Look
fn mouse_look(
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    // Packen Sie das Resultat der single_mut() Abfrage aus
    if let Ok(mut transform) = query.single_mut() {
        for ev in mouse_motion.read() {
            let sensitivity = 0.002;
            let yaw = Quat::from_rotation_y(-ev.delta.x * sensitivity);

            // Jetzt, da 'transform' ein Mut<Transform> ist, funktioniert der Zugriff
            transform.rotation = yaw * transform.rotation;
            // Pitch nach oben/unten optional auf Kamera anwenden
        }
    }
}

// Cursor bei Esc wieder freigeben
fn cursor_grab_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut windows: Query<(&mut Window, &mut CursorOptions), With<PrimaryWindow>>,
) {
    // 3. Query-Ergebnis sicher auspacken
    if let Ok((mut window, mut cursor_options)) = windows.single_mut() {

        if keys.just_pressed(KeyCode::Escape) {
            // 4. Auf die CursorOptions-Komponente zugreifen
            cursor_options.visible = true;
            cursor_options.grab_mode = CursorGrabMode::None;
        }

        // Optional: Wenn Sie eine Taste drücken, um den Cursor wieder zu "graben"
        if keys.just_pressed(KeyCode::Space) { // Beispiel: Leertaste
            cursor_options.visible = false;
            cursor_options.grab_mode = CursorGrabMode::Locked;
        }
    }
}



fn test_chunk_system() {

    
    let mut chunk = Chunk::new(0, 0);
    chunk.fill_test_terrain();
    chunk.generate_chunk();

    println!(
        "Block (0, 0, 0): {}",
        chunk.get_block(0, 0, 0)
    );
    println!(
        "Block (0, 20, 0): {}",
        chunk.get_block(0, 20, 0)
    );

}



//Similar to fn get_player_pos(query: Query<&GlobalTransform, With<Player>>) -> Vec<Vec3> {
//     query.iter().map(|gt| gt.translation()).collect()
// }
fn get_player_pos(query: Query<&GlobalTransform, With<Player>>, mut player_positions: ResMut<PlayerPositions>){
    player_positions.positions.clear();
    for i in &query {
        player_positions.positions.push(i.translation());
    }

}

#[derive(Resource, Default)]
pub struct PlayerPositions {
    pub positions: Vec<Vec3>
}







// use bevy::prelude::*;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Update, testhello)
//         .run();
//
// }

// fn testhello(){
//     println!("Bevy is working !");
//     testrun();
// }

// fn testrun(){
//
//     //Ownership Testrun
//     let x =5;
//     let y = x;
//     println!("x = {}, y = {}", x, y);
//     let z = String::from("hello");
//     println!("z = {}", z);
//     testrun2(z);
//     println!("z = {z}");
//
//
//     //Borrowing Testrun
//     let mut y = 6;
//
//     let p = &mut y;
//
//
//     *p = 7;
//     println!("p is {p}");
//
// }

// fn testrun2(string: String) {
//     println!("string is {}", string);
//
// }