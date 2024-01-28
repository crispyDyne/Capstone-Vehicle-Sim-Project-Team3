use std::f64::consts::PI as PI64;

use crate::{
    function::Function, mirror::Mirror, plane::Plane, rotate::Rotate, step::Step,
    step_slope::StepSlope, perlin::{Perlin, HeightMap}, GridElement,
};


use noise::{Fbm, Perlin as PerlinNoise};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};


pub fn table_top(size: f64, height: f64) -> Vec<Vec<Box<dyn GridElement + 'static>>> {
    let grid_elements: Vec<Vec<Box<dyn GridElement + 'static>>> = vec![
        vec![
            Box::new(StepSlope {
                size,
                height,
                mirror: Mirror::None,
                rotate: Rotate::Ninety,
            }),
            Box::new(Step {
                size,
                height,
                mirror: Mirror::None,
                rotate: Rotate::Ninety,
            }),
            Box::new(StepSlope {
                size,
                height,
                mirror: Mirror::YZ,
                rotate: Rotate::TwoSeventy,
            }),
        ],
        vec![
            Box::new(StepSlope {
                size,
                height,
                mirror: Mirror::YZ,
                rotate: Rotate::Ninety,
            }),
            Box::new(Step {
                size,
                height,
                mirror: Mirror::None,
                rotate: Rotate::TwoSeventy,
            }),
            Box::new(StepSlope {
                size,
                height,
                mirror: Mirror::None,
                rotate: Rotate::TwoSeventy,
            }),
        ],
    ];
    grid_elements
}

pub fn steps(size: f64, heights: Vec<f64>) -> Vec<Vec<Box<dyn GridElement + 'static>>> {
    let mut grid_elements: Vec<Vec<Box<dyn GridElement + 'static>>> = Vec::new();
    for height in heights {
        grid_elements.push(vec![
            Box::new(Step {
                size,
                height,
                ..Default::default()
            }),
            Box::new(Step {
                size,
                height,
                rotate: Rotate::OneEighty,
                ..Default::default()
            }),
            Box::new(Plane {
                size: [size, size],
                subdivisions: 1,
            }),
        ]);
    }
    grid_elements
}

const TAU64: f64 = 2. * PI64;
pub fn wave(size: f64, height: f64, wave_length: f64) -> Vec<Vec<Box<dyn GridElement + 'static>>> {
    let x_start = Box::new(move |x: f64, _y: f64| x / size);
    let x_end = Box::new(move |x: f64, _y: f64| 1.0 - x / size);
    let y_start = Box::new(move |_x: f64, y: f64| y / size);
    let y_end = Box::new(move |_x: f64, y: f64| 1.0 - y / size);

    let dx_start = Box::new(move |_x: f64, _y: f64| (1.0 / size, 0.));
    let dx_end = Box::new(move |_x: f64, _y: f64| (-1.0 / size, 0.));
    let dy_start = Box::new(move |_x: f64, _y: f64| (0., 1.0 / size));
    let dy_end = Box::new(move |_x: f64, _y: f64| (0., -1.0 / size));

    let z_fun = Box::new(move |x: f64, _y: f64| height * (TAU64 / wave_length * x).cos());
    let z_der = Box::new(move |x: f64, _y: f64| {
        (
            -height * TAU64 / wave_length * (TAU64 / wave_length * x).sin(),
            0.,
        )
    });

    let size = [size, size];

    let grid_elements: Vec<Vec<Box<dyn GridElement + 'static>>> = vec![
        // y_start
        vec![
            Box::new(Function {
                size,
                functions: vec![z_fun.clone(), x_start.clone(), y_start.clone()],
                derivatives: vec![z_der.clone(), dx_start.clone(), dy_start.clone()],
            }),
            Box::new(Function {
                size,
                functions: vec![z_fun.clone(), y_start.clone()],
                derivatives: vec![z_der.clone(), dy_start.clone()],
            }),
            Box::new(Function {
                size,
                functions: vec![z_fun.clone(), x_end.clone(), y_start.clone()],
                derivatives: vec![z_der.clone(), dx_end.clone(), dy_start.clone()],
            }),
        ],
        // y_middle
        vec![
            Box::new(Function {
                size,
                functions: vec![z_fun.clone(), x_start.clone()],
                derivatives: vec![z_der.clone(), dx_start.clone()],
            }),
            Box::new(Function {
                size,
                functions: vec![z_fun.clone()],
                derivatives: vec![z_der.clone()],
            }),
            Box::new(Function {
                size,
                functions: vec![z_fun.clone(), x_end.clone()],
                derivatives: vec![z_der.clone(), dx_end.clone()],
            }),
        ],
        // y_end
        vec![
            Box::new(Function {
                size,
                functions: vec![z_fun.clone(), x_start.clone(), y_end.clone()],
                derivatives: vec![z_der.clone(), dx_start.clone(), dy_end.clone()],
            }),
            Box::new(Function {
                size,
                functions: vec![z_fun.clone(), y_end.clone()],
                derivatives: vec![z_der.clone(), dy_end.clone()],
            }),
            Box::new(Function {
                size,
                functions: vec![z_fun.clone(), x_end.clone(), y_end.clone()],
                derivatives: vec![z_der.clone(), dx_end.clone(), dy_end.clone()],
            }),
        ],
    ];

    grid_elements
}


pub fn perlin_plane(size: f64, subdivisions: f64) -> Vec<Vec<Box<dyn GridElement + 'static>>> {
    let mut grid_elements: Vec<Vec<Box<dyn GridElement + 'static>>> = Vec::new();
    
    
    let fbm = Fbm::<PerlinNoise>::new(2348956); // FIX hard coded seed

    let perlin_noise = PlaneMapBuilder::<_, 2>::new(&fbm)
        .set_size((subdivisions + 2.0) as usize, (subdivisions + 2.0) as usize)
        .set_x_bounds(-1.0, 1.0)
        .set_y_bounds(-1.0, 1.0)
        .build();

    let x_vertices = subdivisions + 2.0;
    let z_vertices = subdivisions + 2.0;

    let x_factor = size / x_vertices;
    let z_factor = size / z_vertices;

    let mut xs: Vec<f64> = vec![];
    let mut zs: Vec<f64> = vec![];
    let mut ys: Vec<Vec<f64>> = vec![];


    for x in 0..x_vertices as u32 {
        xs.push(x as f64 * x_factor);
        let mut temp: Vec<f64> = vec![];

        for z in 0..z_vertices as u32 {
            zs.push(z as f64 * z_factor);
            temp.push(perlin_noise.get_value(x as usize, z as usize) - 0.5);
        }

        ys.push(temp)
    }


    // FIX THIS
    let perlin_height_map = HeightMap {
        // Currently made to match order for perlin.rs
        // Don't know if correct or not
        x: zs, 
        y: xs, 
        z: ys,
    };

    grid_elements.push(vec![
        Box::new(Perlin {
            size: [size, size],
            subdivisions: subdivisions as u32,
            heightmap: perlin_height_map, // FIX THIS
        }),

    ]);

    grid_elements
}