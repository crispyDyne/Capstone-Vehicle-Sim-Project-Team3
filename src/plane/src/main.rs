use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

//Ezra Code Start
use noise::{Fbm, Perlin};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use std::io::{stdout, Write};
use image::{GenericImageView, DynamicImage, Luma};
//Ezra Code End

fn main() 
{
    //Ezra Code Start
    // Print to terminal
    let mut lock = stdout().lock();
    writeln!(lock, "hello world").unwrap();
    
    // Generate height map
    let fbm = Fbm::<Perlin>::new(2348956);
    /*
    PlaneMapBuilder::<_, 2>::new(&fbm)
    .set_size(100, 100)
    .set_x_bounds(-10000.0, 10000.0)
    .set_y_bounds(-10000.0, 10000.0)
    .build()
    .write_to_file("fbm.png");
    */
    PlaneMapBuilder::<_, 2>::new(&fbm)
            .set_size(130, 130)
            .set_x_bounds(-1.0, 1.0)
            .set_y_bounds(-1.0, 1.0)
            .build()
            .write_to_file("fbm.png");

    //Ezra Code End

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}



fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,) 
    
    {
    let red_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 0.0, 0.0, 0.5),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let plane_size = 4.0;
    let subdivisions = 128;

    let plane_mesh_handle: Handle<Mesh> = meshes.add(create_plane_mesh(plane_size, subdivisions));

    commands.spawn((
        PbrBundle {
            mesh: plane_mesh_handle,
            material: red_material_handle,
            //transform: Transform::from_xyz(-plane_size/2.0, 0.0, 0.0),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.1, 2.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn create_plane_mesh(size: f32, subdivisions: i32) -> Mesh {

    let x_vertices = subdivisions + 2;
    let z_vertices = subdivisions + 2;
    let tot_vertices = (x_vertices * z_vertices) as usize;
    let tot_indices = ((subdivisions + 1) * (subdivisions + 1) * 6) as usize;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(tot_vertices);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(tot_vertices);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(tot_vertices);
    let mut indices: Vec<u32> = Vec::with_capacity(tot_indices);


    // Ezra Code Start
    // Load the image of hightmap (FIX THIS!!!!!)
    let img_path = r"C:\rustbevy\Capstone-Vehicle-Sim-Project-Team3\src\example_images\fbm.png";

    let img = image::open(img_path).expect("Failed to open image");
        
    // Extract pixel values as f32 numbers
    let perlin_values: Vec<f32> = img
        .pixels()
        .map(|(_, _, pixel)| pixel[0] as f32 / 255.0)
        .collect();

    // Keep track of position in vector
    let mut index = 0;
    // Ezra Code End

    // if x_vertices = 2;  ----- do 0, 1
    for x in 0..x_vertices {
        for z in 0..z_vertices {

            // Ezra Code Start
            let current_value = perlin_values[index];
            // Ezra Code End

            // xi, zi = [0, 1]
            let xi = x as f32 / (x_vertices - 1) as f32;
            let zi = z as f32 / (z_vertices - 1) as f32;
            //let yi = 0.0 as f32;
            // Ezra Code Start
            let yi = current_value as f32;
            // Ezra Code End


            // Edge on origin
            // new_x, new_z = [0, size]
            //let new_x = xi * size;
            //let new_z = zi * size;

            // Centered around origin
            // new_x, new_z = [- size/2, + size/2]
            let x_pos = (xi - 0.5) * size;
            let z_pos = (zi - 0.5) * size;

            // Ezra Code Start
            //y_pos needs to be a pretty small number to not spike wild style
            let y_pos = (yi - 0.5);
            //Ezra Code End

            // build vertices/positions via set of squares
            // positions.push([xi * size, 0.0, zi * size]);
            positions.push([x_pos, y_pos, z_pos]);

            // build normals
            /*
             * If want actual perface normals on non-flat plane...
             *  do cross product of any 2 edge of the triangle
             * https://stackoverflow.com/questions/19350792/calculate-normal-of-a-single-triangle-in-3d-space
             */

            // Per vertex
            // Up vector
            normals.push([0.0, 1.0, 0.0]);

            // build uvs
            uvs.push([xi, zi]);

            // Ezra Code
            // Increment position in the index
            index = (index + 1) % perlin_values.len();
            // Ezra Code
        }
    }

    for x in 0..x_vertices-1 {
        for z in 0..z_vertices-1 {

            // build indices
            let bl = (x * z_vertices) + z;
            let tl = bl + 1;
            let br = bl + z_vertices;
            let tr = br + 1;
            
            // Triangle 1 xz 00-11-10
            indices.push((bl) as u32);
            indices.push((tr) as u32);
            indices.push((br) as u32);

            // Triangle 2 xz 00-01-11
            indices.push((bl) as u32);
            indices.push((tl) as u32);
            indices.push((tr) as u32);
        }
    }
        
    let mut plane_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    plane_mesh.set_indices(Some(Indices::U32(indices)));

    plane_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION, 
        positions);

    plane_mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL, 
        normals);

    plane_mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0, 
        uvs);

    plane_mesh
}