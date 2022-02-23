use glam::Vec2;
use hsluv::hsluv_to_rgb;
use image::Rgb;
use rand::{random, thread_rng};
use rand_distr::{Distribution, Normal, Uniform};

use crate::params::Params;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub typ: u8,
}

// impl Particle {
//     pub fn distance_sq(&self, q: &Self, wrap: bool) -> f32 {

//     }
// }

pub struct ParticleTypes {
    colors: Vec<Rgb<u8>>,
    attraction: Vec<Vec<f32>>,
    min_radius: Vec<Vec<f32>>,
    max_radius: Vec<Vec<f32>>,
}

pub const DIAMETER: f32 = 1.0;

impl ParticleTypes {
    pub fn new(types: u8, params: &Params) -> Self {
        let mut pt = Self {
            colors: Vec::new(),
            attraction: Vec::new(),
            min_radius: Vec::new(),
            max_radius: Vec::new(),
        };
        pt.resize(types);
        pt.randomize(params);
        pt
    }

    pub fn resize(&mut self, types: u8) {
        let types = types as usize;
        self.colors.resize(types, Rgb([0, 0, 0]));
        let hue_offset = random::<f64>();
        self.colors.iter_mut().enumerate().for_each(|(i, c)| {
            let h = f64::fract(i as f64 / types as f64 + hue_offset);
            let (r, g, b) = hsluv_to_rgb((h * 360., 100., 50.));
            c.0 = [(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8];
        });
        println!("{:?}", self.colors[0]);
        self.attraction.resize_with(types, || vec![0.; types]);
        self.min_radius.resize_with(types, || vec![0.; types]);
        self.max_radius.resize_with(types, || vec![0.; types]);
    }

    pub fn size(&self) -> usize {
        self.colors.len()
    }

    pub fn randomize(&mut self, params: &Params) {
        let mut rng = thread_rng();
        let normal = Normal::new(params.mean_attraction, params.std_attraction).unwrap();
        let rand_min = Uniform::new(params.min_radius_lower, params.min_radius_upper);
        let rand_max = Uniform::new(params.max_radius_lower, params.max_radius_upper);
        for i in 0..self.size() {
            for j in 0..self.size() {
                if i == j {
                    self.attraction[i][j] = f32::abs(normal.sample(&mut rng));
                    self.min_radius[i][j] = DIAMETER;
                } else {
                    self.attraction[i][j] = normal.sample(&mut rng);
                    self.min_radius[i][j] = f32::max(DIAMETER, rand_min.sample(&mut rng));
                }
                self.max_radius[i][j] = f32::max(rand_max.sample(&mut rng), self.min_radius[i][j]);

                // enforce radii symmetry
                self.min_radius[j][i] = self.min_radius[i][j];
                self.max_radius[j][i] = self.max_radius[i][j];
            }
        }
    }

    pub fn get_radii(&self, p_type: u8, q_type: u8) -> (f32, f32) {
        let i = p_type as usize;
        let j = q_type as usize;
        (self.min_radius[i][j], self.max_radius[i][j])
    }

    pub fn get_attraction(&self, p_type: u8, q_type: u8) -> f32 {
        self.attraction[p_type as usize][q_type as usize]
    }

    pub fn get_color(&self, p_type: u8) -> Rgb<u8> {
        self.colors[p_type as usize]
    }

    pub fn get_max_radius(&self) -> f32 {
        let mut max = f32::NEG_INFINITY;
        for i in 0..self.size() {
            for j in i..self.size() {
                max = max.max(self.max_radius[i][j]);
            }
        }
        max
    }
}
