use bevy::{
    prelude::{Mesh, Vec3},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    math,
};
use rigid_body::sva::Vector;

//use bevy::render::mesh::Indices;
//use bevy::render::render_resource::PrimitiveTopology;

use noise::{Fbm, Perlin as PerlinNoise};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use std::io::{stdout, Write};
use image::{GenericImageView, DynamicImage, Luma};
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
                return None;
            }
        }

        match find_y {
            Some(y) => y_ind = y,
            None => {
                return None;
            }
        }

        // Adjust indices to match the original array
        let x_ind_next = x_ind + 1;
        let y_ind_next = y_ind + 1;

        let q11 = self.z[x_ind][y_ind];
        let q12 = self.z[x_ind][y_ind_next];
        let q21 = self.z[x_ind_next][y_ind];
        let q22 = self.z[x_ind_next][y_ind_next];

        let x1 = self.x[x_ind];
        let x2 = self.x[x_ind_next];
        let y1 = self.y[y_ind];
        let y2 = self.y[y_ind_next];

        let r1 = ((x2 - x) / (x2 - x1)) * q11 + ((x - x1) / (x2 - x1)) * q21;
        let r2 = ((x2 - x) / (x2 - x1)) * q12 + ((x - x1) / (x2 - x1)) * q22;

        Some(((y2 - y) / (y2 - y1)) * r1 + ((y - y1) / (y2 - y1)) * r2)
    }
}


// Assumption: array is bigger than size 1
fn find(array: &[f64], target: f64) -> Option<usize> {

    // O(logn) Binary search 
    // -- for number that is floored (rounded down)
    // ex 8.8 -> 8

    let mut low = 0;
    let mut high = array.len() - 1;
    
    while low < high 
    {
        let mid = (low + high) / 2;

        if mid == low && array[mid] < target 
        {
            return Some(mid);
        }
        else if mid == low && array[mid] >= target
        {
            return Some(high);
        }
        else if array[mid] < target 
        {
            low = mid;
        } 
        else 
        {
            high = mid;
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

        let x_vertices = self.subdivisions + 2;
        let y_vertices = self.subdivisions + 2;
        let tot_vertices = (x_vertices * y_vertices) as usize;
        let tot_indices = ((x_vertices - 1) * (y_vertices - 1) * 6) as usize;

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(tot_vertices);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(tot_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(tot_vertices);
        let mut indices: Vec<u32> = Vec::with_capacity(tot_indices);


        for x in 0..x_vertices {
            for y in 0..y_vertices {

                let xi = x as f64 / (x_vertices - 1) as f64;
                let yi = y as f64 / (y_vertices - 1) as f64;

                let x_pos = self.heightmap.x[x as usize];
                let y_pos = self.heightmap.y[y as usize];
                let z_pos = self.heightmap.z[x as usize][y as usize];

                // Build vertices/positions via set of squares
                positions.push([x_pos as f32, y_pos as f32, z_pos as f32]);

                // Build normals
                // Per vertex - Up vector
                // FIX THIS, edge cases not yet covered
                if x == x_vertices - 1  && y == y_vertices - 1 {
                    normals.push([0.0, 0.0, -1.0]);
                }
                else if x == x_vertices - 1 {
                    normals.push([0.0, 0.0, -1.0]);
                }
                else if y == y_vertices - 1 {
                    normals.push([0.0, 0.0, -1.0]);
                }
                else {
                    let p1 = Vec3{x: x_pos as f32, y: y_pos as f32, z: z_pos as f32};
                    let p2 = Vec3{x: self.heightmap.x[(x + 1) as usize] as f32, y: y_pos as f32, z: self.heightmap.z[(x + 1) as usize][y as usize] as f32};
                    let p3 = Vec3{x: x_pos as f32, y: self.heightmap.y[(y + 1) as usize] as f32, z: self.heightmap.z[x as usize][(y + 1) as usize] as f32};
                
                    let v = p3 - p1;
                    let u = p2 - p1;

                    let n1 = u[1] * v[2] - u[2] * v[1];
                    let n2 = u[2] * v[0] - u[0] * v[2];
                    let n3 = u[0] * v[1] - u[1] * v[0];

                    normals.push([n1, n2, n3]);
                }


                // Build uvs
                // FIX THIS -- it is wrong maybe, but no textures are being used so should be fine
                uvs.push([xi as f32, yi as f32]);
            }
        }

        for x in 0..x_vertices-1 {
            for y in 0..y_vertices-1 {

                // build indices
                let bl = (x * y_vertices) + y;
                let tl = bl + 1;
                let br = bl + y_vertices;
                let tr = br + 1;
                
                // counter-clockwise

                // Triangle 1 xy 00-10-11
                indices.push((bl) as u32);
                indices.push((br) as u32);
                indices.push((tr) as u32);

                // Triangle 2 xy 00-11-01
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
