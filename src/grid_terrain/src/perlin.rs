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
    pub fn height(&self, x: f64, y: f64) -> Option<f64> {

        // Bilinear interpolation

        let find_x = find(&(self.x), x);
        let find_y = find(&(self.y), y);

        let x_ind: usize;
        let y_ind: usize;

        match find_x {
            Some(x) => x_ind = x,
            None => {
                return None
            },
        }
        match find_y {
            Some(y) => y_ind = y,
            None => {
                return None
            },
        }

        // Section copied from Chris who copied from ChatGPT
        let q11 = self.z[x_ind][y_ind];
        let q12 = self.z[x_ind][y_ind + 1];
        let q21 = self.z[x_ind + 1][y_ind];
        let q22 = self.z[x_ind + 1][y_ind + 1];

        let x1 = self.x[x_ind];
        let x2 = self.x[x_ind + 1];
        let y1 = self.y[y_ind];
        let y2 = self.y[y_ind + 1];

        let r1 = ((x2 - x) / (x2 - x1)) * q11 + ((x - x1) / (x2 - x1)) * q21;
        let r2 = ((x2 - x) / (x2 - x1)) * q12 + ((x - x1) / (x2 - x1)) * q22;

        return Some(((y2 - y) / (y2 - y1)) * r1 + ((y - y1) / (y2 - y1)) * r2);
    }
}


// Assumption: array is bigger than size 1
fn find(array: &[f64], target: f64) -> Option<usize> {

    // O(logn) Binary search 
    // -- for number that is floored (rounded down)
    // ex 8.8 -> 8

    // BROKEN FIX THIS


    // let mut low: usize = 0;
    // let mut high: usize = 128;//array.len() - 1 - 1;

    // if target < array[low] {
    //     return None;
    // }
    // if target > array[high] {
    //     return None;
    // }

    // while low != high {
    //     let mid = (low + high) / 2 as usize;
    //     if mid == low {
    //         if array[high] > target {
    //             return Some(low);
    //         }
    //         else {
    //             return Some(high);
    //         }
    //     }
    //     else if array[mid] <= target {
    //         high = mid;
    //     }
    //     else {
    //         low = mid;
    //     }
    // }
    // return Some(low);


    // O(n) dumb search

    let mut temp: usize = 0;
    for &num in array {
        if num < target {
            temp += 1;
        }
        if num > target {
            return Some(temp - 1);
        }
    }
    return None;

}


pub struct Perlin {
    pub size: [f64; 2],
    pub subdivisions: u32,
    pub heightmap: HeightMap, // 2D lookup table of z height vs x and y
}

impl GridElement for Perlin {
    fn interference(&self, point: Vector) -> Option<Interference> {


        let ground_height: f64;
        let find_ground_height = self.heightmap.height(point.x, point.y);


        match find_ground_height {
            Some(x) => ground_height = x,
            None => {
                return None
            }
        }

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

        // let fbm = Fbm::<PerlinNoise>::new(2348956); // FIX hard coded seed

        // let perlin_noise = PlaneMapBuilder::<_, 2>::new(&fbm)
        //     .set_size((self.subdivisions + 2) as usize, (self.subdivisions + 2) as usize)
        //     .set_x_bounds(-1.0, 1.0)
        //    .set_y_bounds(-1.0, 1.0)
        //    .build();

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


                // let yi = perlin_noise.get_value(x as usize, z as usize);
                // let y_pos = yi - 0.5; // ???

                // // Edge on origin
                // // new_x, new_z = [0, size]
                // let x_pos = xi * self.size[0];
                // let z_pos = zi * self.size[1];


                // // Centered around origin
                // // new_x, new_z = [- size/2, + size/2]
                // //let x_pos = (xi - 0.5) * self.size[0];
                // //let z_pos = (zi - 0.5) * self.size[1];


                let x_pos = self.heightmap.x[x as usize];
                let z_pos = self.heightmap.y[z as usize];
                let y_pos = self.heightmap.z[x as usize][z as usize];
                // Build vertices/positions via set of squares
                // zs and xs flipped to flip on screen since normals were facing down
                positions.push([x_pos as f32, z_pos as f32, y_pos as f32]);

                // Build normals
                // Per vertex - Up vector
                // FIX THIS -- should consider surrounding points to be a not up vector

                normals.push([0.0, 0.0, 1.0]);
                
                // One idea to fix edge cases

                // if x == x_vertices - 1  && z == z_vertices - 1 {
                //     normals.push([0.0, 0.0, 1.0]);
                // }
                // else if x == x_vertices - 1 {
                //     normals.push([0.0, 0.0, 1.0]);
                // }
                // else if z == z_vertices - 1 {
                //     normals.push([0.0, 0.0, 1.0]);
                // }
                // else {
                //     normals.push([0.0, 0.0, 1.0]);
                // }


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
                
                // counter-clockwise

                // Triangle 1 xz 00-10-11
                indices.push((bl) as u32);
                indices.push((br) as u32);
                indices.push((tr) as u32);

                // Triangle 2 xz 00-11-01
                indices.push((bl) as u32);
                indices.push((tr) as u32);
                indices.push((tl) as u32);
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
