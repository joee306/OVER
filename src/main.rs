mod chunk;
mod block;

use chunk::*;
use block::*;

use bevy::{prelude::*,pbr::wireframe::{WireframePlugin, Wireframe}};
use rand::Rng;
use bevy_flycam::{PlayerPlugin, MovementSettings};


fn main() {
        
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(WireframePlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 18.0, // default: 12.0
        })
        .add_startup_system(setup)
        .add_system(render)
        .add_system(chunk_inicator)
        .run();
    
   // let mut c = Chunk::new();
   // println!("{}", c.get(4,4,4));
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
) {
    commands.spawn().insert(Chunk::new(0,0,0));
    //commands.spawn().insert(Chunk::new(16,0,0));
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(20.0, 18.0, 20.0),
        ..default()
    });
    /* camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-20.0, 25.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    */
}
fn chunk_inicator(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query : Query<&Chunk>,

) {
    for chunk in query.iter() {
        if chunk.loaded {
            return;
        }
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            //material: materials.add(Color::rgb(0.33, 0.49, 0.27).into()),
            material: materials.add(Color::rgb(1.0,0.0,0.0).into()),
            transform: Transform::from_xyz(chunk.coords.0 as f32 ,-3.0 ,chunk.coords.2 as f32),
            ..default()
        });
    }
} 

fn render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query : Query<&mut Chunk>,
) {
    for mut chunk in query.iter_mut() {
        if !chunk.loaded {
            /*chunk.set_save(0,1,0,0);
            chunk.set_save(1,1,0,0);
            chunk.set_save(1,2,0,0);
            chunk.set_save(0,2,0,0);*/
            chunk.calc_layers();
            for x in chunk.coords.0 as usize..chunk.coords.0 as usize + chunk.data.len() {
                for y in chunk.coords.1 as usize..chunk.coords.1 as usize + chunk.data[0].len() {
                    for z in chunk.coords.2 as usize..chunk.coords.2 as usize + chunk.data[0][0].len() {
                        let bcolor = chunk.get_save(x,y,z).unwrap();
                        let color : StandardMaterial = Color::rgb(0.6, 0.6, 0.6).into();
                        ////////////////////////////////////////////////////////////////////////////////////////
                        if chunk.data[x][y][z].render_side.XP {
                            commands.spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                                material: materials.add(color.clone()),
                                transform: Transform {
                                    translation : Vec3::from((x as f32 + 0.5,y as f32 ,z as f32)),
                                    rotation : Quat::from_rotation_z(std::f32::consts::PI / -2.0),
                                    ..default()

                                },
                                ..default()
                            });
                        }
                        ///////////////////////////////////////////////////////////////////////////////////////
                        if chunk.data[x][y][z].render_side.XM {
                            commands.spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                                material: materials.add(color.clone()),
                                transform: Transform {
                                    translation : Vec3::from((x as f32 - 0.5 ,y as f32 ,z as f32)),
                                    rotation : Quat::from_rotation_z(std::f32::consts::PI / 2.0 ),
                                    ..default()

                                },
                                ..default()
                            });
                        }
                        ////////////////////////////////////////////////////////////////////////////////////////
                        if chunk.data[x][y][z].render_side.YP {
                            commands.spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                                material: materials.add(color.clone()),
                                transform: Transform {
                                    translation : Vec3::from((x as f32 ,y as f32 + 0.5,z as f32)),
                                    ..default()
                                },
                                ..default()
                            });
                        }
                        
                        ////////////////////////////////////////////////////////////////////////////////////////
                        if chunk.data[x][y][z].render_side.YM {
                            commands.spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                                material: materials.add(color.clone()),
                                transform: Transform {
                                    translation : Vec3::from((x as f32 ,y as f32 - 0.5,z as f32)),
                                    rotation : Quat::from_rotation_z(std::f32::consts::PI),
                                    ..default()

                                },
                                ..default()
                            });
                        }
                        ////////////////////////////////////////////////////////////////////////////////////////
                        if chunk.data[x][y][z].render_side.ZP {
                            commands.spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                                material: materials.add(color.clone()),
                                transform: Transform {
                                    translation : Vec3::from((x as f32 ,y as f32 ,z as f32 + 0.5)),
                                    rotation : Quat::from_rotation_x(std::f32::consts::PI / 2.0 ),
                                    ..default()

                                },
                                ..default()
                            });
                        }
                        ////////////////////////////////////////////////////////////////////////////////////////
                        if chunk.data[x][y][z].render_side.ZM {
                            commands.spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                                material: materials.add(color.clone()),
                                transform: Transform {
                                    translation : Vec3::from((x as f32 ,y as f32 ,z as f32 - 0.5)),
                                    rotation : Quat::from_rotation_x(std::f32::consts::PI / -2.0 ),
                                    ..default()
                                },
                                ..default()
                            });
                        }
                    }
                }
            }
        }
        chunk.loaded = true;
    }
}
