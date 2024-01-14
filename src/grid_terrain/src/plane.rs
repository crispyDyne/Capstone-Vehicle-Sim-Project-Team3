use bevy::{
    prelude::{Mesh, Vec3},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use rigid_body::sva::Vector;



use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

//Ezra Code Start
use noise::{Fbm, Perlin};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use std::io::{stdout, Write};
use image::{GenericImageView, DynamicImage, Luma};
//Ezra Code End


use crate::{GridElement, Interference};

pub struct Plane {
    pub size: [f64; 2],
    pub subdivisions: u32,
}

impl GridElement for Plane {
    fn interference(&self, point: Vector) -> Option<Interference> {
        if point.z < 0. {
            return Some(Interference {
                magnitude: -point.z,
                position: Vector::new(point.x, point.y, 0.),
                normal: Vector::z(),
            });
        } else {
            return None;
        }
    }

    fn mesh(&self) -> Mesh {
        let y_vertex_count = self.subdivisions + 2;
        let x_vertex_count = self.subdivisions + 2;
        let num_vertices = (y_vertex_count * x_vertex_count) as usize;
        let num_indices = ((y_vertex_count - 1) * (x_vertex_count - 1) * 6) as usize;
        let up = Vec3::Z.to_array();

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
        let mut indices: Vec<u32> = Vec::with_capacity(num_indices);

        for y in 0..y_vertex_count {
            for x in 0..x_vertex_count {
                let tx = x as f32 / (x_vertex_count - 1) as f32;
                let ty = y as f32 / (y_vertex_count - 1) as f32;
                positions.push([tx * self.size[0] as f32, ty * self.size[1] as f32, 0.0]);
                normals.push(up);
                uvs.push([tx, 1.0 - ty]);
            }
        }

        for y in 0..y_vertex_count - 1 {
            for x in 0..x_vertex_count - 1 {
                let quad = y * x_vertex_count + x;
                indices.push(quad);
                indices.push(quad + 1);
                indices.push(quad + x_vertex_count);
                indices.push(quad + x_vertex_count + 1);
                indices.push(quad + x_vertex_count);
                indices.push(quad + 1);
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}






fn create_plane_mesh(size: f32, subdivisions: i32) -> Mesh {

    //Ezra Code Start
    use noise::{Fbm, Perlin};
    use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
    use std::io::{stdout, Write};
    use image::{GenericImageView, DynamicImage, Luma};
    //Ezra Code End    

    let fbm = Fbm::<Perlin>::new(2348956);

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
