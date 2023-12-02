
use bevy::prelude::*;
use bevy::render::mesh::Indices;
//use bevy::render::mesh::VertexAttributeValues;
use bevy::render::render_resource::PrimitiveTopology;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}


fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {

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


    // if x_vertices = 2;  ----- do 0, 1
    for x in 0..x_vertices {
        for z in 0..z_vertices {

            // xi, zi = [0, 1]
            let xi = x as f32 / (x_vertices - 1) as f32;
            let zi = z as f32 / (z_vertices - 1) as f32;
            let yi = 0.0 as f32;

            // Edge on origin
            // new_x, new_z = [0, size]
            //let new_x = xi * size;
            //let new_z = zi * size;

            // Centered around origin
            // new_x, new_z = [- size/2, + size/2]
            let x_pos = (xi - 0.5) * size;
            let z_pos = (zi - 0.5) * size;
            
            // Setting y's
            /* 
                // Flat Plane
                // let y_pos = 0.0;      

                // x^2 curve
                // let y_pos = x_pos * x_pos / 2.0;     
            */

            // PERLIN NOISEEEEE
            let y_pos = perlin_noise_3d(x_pos, yi, z_pos);

            // build vertices/positions via set of squares
            //positions.push([xi * size, 0.0, zi * size]);
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
        /* 
            // Triangle 1 xz 00-01-10
            indices.push((x * z + z) as u32);
            indices.push((x * z + (z + 1)) as u32);
            indices.push(((x * z + 1) + z) as u32);

            // Triangle 2 xz 10-01-11
            indices.push(((x * z + 1) + z) as u32);
            indices.push((x * z + (z + 1)) as u32);
            indices.push(((x * z + 1) + (z + 1)) as u32);
        */
        }
    }


/* Bevy V 1.12.0
    #[rustfmt::skip]
    let plane_mesh = Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            positions,
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            uvs,
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normals,
        )
        .with_indices(Some(Indices::U32(indices)));
*/
        
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


// perlin noise heavily based on https://adrianb.io/2014/08/09/perlinnoise.html
fn perlin_noise_3d (x: f32, y: f32, z: f32) -> f32 {

    // 0-255 inclusive + unique + random
    // Is this the seed???
    // stolen from https://adrianb.io/2014/08/09/perlinnoise.html
    let permutation = [ 151,160,137,91,90,15,
        131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,
        190, 6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,
        88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,
        77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,
        102,143,54, 65,25,63,161, 1,216,80,73,209,76,132,187,208, 89,18,169,200,196,
        135,130,116,188,159,86,164,100,109,198,173,186, 3,64,52,217,226,250,124,123,
        5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,
        223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167, 43,172,9,
        129,22,39,253, 19,98,108,110,79,113,224,232,178,185, 112,104,218,246,97,228,
        251,34,242,193,238,210,144,12,191,179,162,241, 81,51,145,235,249,14,239,107,
        49,192,214, 31,181,199,106,157,184, 84,204,176,115,121,50,45,127, 4,150,254,
        138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180
    ];
    
    // array of 0's 
    let mut p: [i32; 512] = [0; 512];

    for x in 0..512-1 {
        p[x] = permutation[x % 255];
    }

    // Which square (repeats after 255)
    // get whole int [0, 255]
    let xi = x as i32 & 255;
    let yi = y as i32 & 255;
    let zi = z as i32 & 255;
    // Where in the square
    // get decimals
    let xf = (x - (xi ) as f32);
    let yf = (y - (yi ) as f32);
    let zf = (z - (zi ) as f32);

    0.0
    //(xi) as f32
    //x * x
}