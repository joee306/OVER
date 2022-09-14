use bevy::{prelude::*,pbr::wireframe::{WireframePlugin, Wireframe}};
use rand::Rng;
use bevy_flycam::{PlayerPlugin, MovementSettings};

const CHUNK_DIMENSIONS : (usize,usize,usize) = (16,128,16); //xyz

#[derive(Clone,Copy,Debug)]
struct LayerSide {
    XM : bool,
    XP : bool,
    YM : bool,
    YP : bool,
    ZM : bool,
    ZP : bool,
}

#[derive(Clone,Copy,Debug)]
struct Block {
    render_side  : LayerSide,
    material : u32,
}

impl Block {
    pub fn new(id : u32) -> Self {
        Self {
            render_side : LayerSide { XM : false , XP : false , YM : false , YP : false , ZM : false , ZP : false },
            material : id,
        }
    }
}

#[derive(Component)]
struct Chunk {
    coords : (i64, i64, i64),
    data : [[[Block ; CHUNK_DIMENSIONS.0] ; CHUNK_DIMENSIONS.2] ; CHUNK_DIMENSIONS.1],
    loaded : bool,
}

impl Chunk {
    pub fn new(cx : i64,cy : i64,cz : i64) -> Self {
        Self {
            coords : (cx,0,cz),
            data :
            [[[ Block {
                render_side : LayerSide { XM : false , XP : false , YM : false , YP : false , ZM : false , ZP : false },
                material : 0,
            }; CHUNK_DIMENSIONS.0] ; CHUNK_DIMENSIONS.2] ; CHUNK_DIMENSIONS.1],
            loaded : false,
        }
    }
    pub fn get(&self, x_index : usize , y_index : usize, z_index : usize) -> bool {
        if x_index <= self.coords.0 as usize + 0 || x_index >= self.coords.0 as usize + CHUNK_DIMENSIONS.0 -1 ||
           y_index <= self.coords.1 as usize + 0 || y_index >= self.coords.1 as usize + CHUNK_DIMENSIONS.1 -1 || 
           z_index <= self.coords.2 as usize + 0 || z_index >= self.coords.2 as usize + CHUNK_DIMENSIONS.2 -1  {
            return true;
        } else {
            false
        }
    }
}

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
    commands.spawn().insert(Chunk::new(16,0,0));
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
            for x in chunk.coords.0 as usize..chunk.coords.0 as usize + chunk.data[0].len() {
                for y in chunk.coords.1 as usize..chunk.coords.1 as usize + chunk.data.len() {
                    for z in chunk.coords.2 as usize..chunk.coords.2 as usize + chunk.data[0][0].len() {
                        if chunk.get(x,y,z) {
                            let mut rng = rand::thread_rng();
                            //println!("{:?}",chunk.data[x - 1][y - 1][z - 1].render_side);
                            commands.spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                                //material: materials.add(Color::rgb(0.33, 0.49, 0.27).into()),
                                material: materials.add(Color::rgb(rng.gen::<f32>(),rng.gen::<f32>(),rng.gen::<f32>()).into()),
                                transform: Transform::from_xyz(x as f32 ,y as f32 ,z as f32),
                                ..default()
                            });
                        }
                    }
                }
            }
            println!("chunk loaded with <{:?}> ", chunk.coords);
        }
        chunk.loaded = true;
    }
}
