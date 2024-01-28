use bevy::{
    prelude::{Mesh, Vec3},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use rigid_body::sva::Vector;



//use bevy::render::mesh::Indices;
//use bevy::render::render_resource::PrimitiveTopology;

//Ezra Code Start
use noise::{Fbm, Perlin as PerlinNoise};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use std::io::{stdout, Write};
use image::{GenericImageView, DynamicImage, Luma};
//Ezra Code End


use crate::{GridElement, Interference};

pub struct HeightMap {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<Vec<f64>>,
}

impl HeightMap {
    pub fn height(&self, x: f64, y: f64) -> f64 {
        // implement bilinear interpolation
        0.0
    }
}

pub struct Perlin {
    pub size: [f64; 2],
    pub subdivisions: u32,
    pub heightmap: HeightMap, // 2D lookup table of z height vs x and y
}

impl GridElement for Perlin {
    fn interference(&self, point: Vector) -> Option<Interference> {

        let ground_height = self.heightmap.height(point.x, point.y);

        if point.z < ground_height {
            return Some(Interference {
                magnitude: ground_height - point.z,
                position: Vector::new(point.x, point.y, ground_height),
                normal: Vector::z(), // FIX THIS to real normal
            });
        } else {
            return None;
        }
    }

    fn mesh(&self) -> Mesh {  

        let fbm = Fbm::<PerlinNoise>::new(2348956); // FIX hard coded seed

        let perlin_noise = PlaneMapBuilder::<_, 2>::new(&fbm)
            .set_size((self.subdivisions + 2) as usize, (self.subdivisions + 2) as usize)
            .set_x_bounds(-1.0, 1.0)
            .set_y_bounds(-1.0, 1.0)
            .build();

        let x_vertices = self.subdivisions + 2;
        let z_vertices = self.subdivisions + 2;
        let tot_vertices = (x_vertices * z_vertices) as usize;
        let tot_indices = ((x_vertices - 1) * (z_vertices - 1) * 6) as usize;

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(tot_vertices);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(tot_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(tot_vertices);
        let mut indices: Vec<u32> = Vec::with_capacity(tot_indices);


        for x in 0..x_vertices {
            for z in 0..z_vertices {

                let xi = x as f64 / (x_vertices - 1) as f64;
                let zi = z as f64 / (z_vertices - 1) as f64;
                let yi = perlin_noise.get_value(x as usize, z as usize);

                // Edge on origin
                // new_x, new_z = [0, size]
                let x_pos = xi * self.size[0];
                let z_pos = zi * self.size[1];


                // Centered around origin
                // new_x, new_z = [- size/2, + size/2]
                //let x_pos = (xi - 0.5) * self.size[0];
                //let z_pos = (zi - 0.5) * self.size[1];
                let y_pos = (yi - 0.5); // ???

                // Build vertices/positions via set of squares
                // zs and xs flipped to flip on screen since normals were facing down
                positions.push([z_pos as f32, x_pos as f32, y_pos as f32]);

                // Build normals
                // Per vertex - Up vector
                // FIX THIS -- should consider surrounding points to be a not up vector
                normals.push([0.0, 0.0, 1.0]);

                // Build uvs
                // FIX THIS -- it is wrong maybe, but no textures are being used so should be fine
                uvs.push([xi as f32, zi as f32]);
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
}






fn create_plane_mesh(size: f32, subdivisions: i32) -> Mesh {

    let fbm = Fbm::<PerlinNoise>::new(2348956);

    PlaneMapBuilder::<_, 2>::new(&fbm)
        .set_size(130, 130)
        .set_x_bounds(-1.0, 1.0)
        .set_y_bounds(-1.0, 1.0)
        .build()
        .write_to_file("fbm.png");



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
            positions.push([z_pos, x_pos, y_pos]);

            // build normals
            /*
             * If want actual perface normals on non-flat plane...
             *  do cross product of any 2 edge of the triangle
             * https://stackoverflow.com/questions/19350792/calculate-normal-of-a-single-triangle-in-3d-space
             */

            // Per vertex
            // Up vector
            normals.push([0.0, 0.0, 1.0]);

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
