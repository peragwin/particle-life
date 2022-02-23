use glam::Vec2;
use rand::thread_rng;
use rand_distr::{Distribution, Normal, Uniform};
use rayon::prelude::*;

use crate::params::Params;
use crate::particles::{Particle, ParticleTypes, DIAMETER};

pub struct Universe {
    width: f32,
    height: f32,
}

const R_SMOOTH: f32 = 2.0;
const GRID_SIZE: usize = 32;

struct ParticleBuckets {
    buckets: Vec<Vec<Particle>>,
    grid_size: usize,
    world_scale: Vec2,
}

impl ParticleBuckets {
    fn new(particles: &Vec<Particle>, world_size: Vec2, grid_size: usize) -> Self {
        let world_scale = 1. / world_size * grid_size as f32;
        let mut buckets: Vec<Vec<Particle>> = vec![vec![]; grid_size * grid_size];

        for &p in particles {
            let pos = p.pos * world_scale;
            let x = pos.x.floor() as usize;
            let y = pos.y.floor() as usize;
            buckets[x + y * grid_size].push(p)
        }

        Self {
            buckets,
            grid_size,
            world_scale,
        }
    }

    fn get_neighbors(&self, pos: Vec2, radius: f32) -> Vec<Particle> {
        let grid_pos = pos * self.world_scale;
        let grid_radius = radius * self.world_scale;
        let (min_cell, max_cell) = (grid_pos - grid_radius, grid_pos + grid_radius);

        let mut particles = vec![];
        for y in min_cell.y.floor() as isize..max_cell.y.floor() as isize {
            for x in min_cell.x.floor() as isize..max_cell.x.floor() as isize {
                let x = if x < 0 {
                    x + self.grid_size as isize
                } else if x > (self.grid_size as isize - 1) {
                    x - self.grid_size as isize
                } else {
                    x
                } as usize;
                let y = if y < 0 {
                    y + self.grid_size as isize
                } else if y > (self.grid_size as isize - 1) {
                    y - self.grid_size as isize
                } else {
                    y
                } as usize;
                particles.extend(&self.buckets[x + y * self.grid_size]);
            }
        }

        particles
    }
}

impl Universe {
    pub fn new(width: f32, height: f32) -> Universe {
        Universe { width, height }
    }

    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    pub fn create_particles(&self, particle_types: &ParticleTypes, n: usize) -> Vec<Particle> {
        let mut rng = thread_rng();
        let rand_vel = Normal::new(0.0f32, 0.2f32).unwrap();
        let rand_pos = Uniform::new(0.0f32, 1.0f32);
        let rand_type = Uniform::new(0, particle_types.size() as u8);
        (0..n)
            .map(|_| Particle {
                vel: Vec2::new(rand_vel.sample(&mut rng), rand_vel.sample(&mut rng)),
                pos: Vec2::new(
                    rand_pos.sample(&mut rng) * self.width as f32,
                    rand_pos.sample(&mut rng) * self.height as f32,
                ),
                typ: rand_type.sample(&mut rng),
            })
            .collect()
    }

    pub fn step(
        &self,
        particle_types: &ParticleTypes,
        params: &Params,
        particles: &Vec<Particle>,
    ) -> Vec<Particle> {
        let width = self.width as f32;
        let height = self.height as f32;

        let particle_buckets =
            ParticleBuckets::new(&particles, Vec2::from((width, height)), GRID_SIZE);
        let max_radius = particle_types.get_max_radius();

        let particles: Vec<Particle> = particles
            .par_iter()
            .map(|p| {
                let mut vel = p.vel;

                for q in particle_buckets.get_neighbors(p.pos, max_radius) {
                    let mut dx = q.pos - p.pos;
                    if params.wrap {
                        if dx.x > width * 0.5 {
                            dx.x -= width;
                        } else if dx.x < -width * 0.5 {
                            dx.x += width;
                        }
                        if dx.y > height * 0.5 {
                            dx.y -= height;
                        } else if dx.y < -height {
                            dx.y += height;
                        }
                    }

                    let r2 = dx.x * dx.x + dx.y * dx.y;
                    let (min_radius, max_radius) = particle_types.get_radii(p.typ, q.typ);

                    if r2 > max_radius * max_radius || r2 < 0.01 {
                        continue;
                    }

                    let r = f32::sqrt(r2);
                    let f = dx / r
                        * (if r > min_radius {
                            let numer = 2. * f32::abs(r - 0.5 * (max_radius + min_radius));
                            let demon = max_radius - min_radius;
                            particle_types.get_attraction(p.typ, q.typ) * (1.0 - numer / demon)
                        } else {
                            R_SMOOTH
                                * min_radius
                                * (1. / (min_radius + R_SMOOTH) - 1. / (r + R_SMOOTH))
                        });

                    vel += f;
                }

                Particle { vel, ..*p }
            })
            .collect();

        let particles: Vec<Particle> = particles
            .iter()
            .map(|p| {
                let mut pos = p.pos + p.vel;
                let mut vel = p.vel * (1. - params.friction);
                let width = self.width as f32;
                let height = self.height as f32;

                if params.wrap {
                    pos.x %= width;
                    if pos.x < 0. {
                        pos.x += width;
                    }
                    pos.y %= height;
                    if pos.y < 0. {
                        pos.y += height;
                    }
                } else {
                    if pos.x <= DIAMETER {
                        vel.x = -vel.x;
                        pos.x = DIAMETER;
                    } else if pos.x >= width - DIAMETER {
                        vel.x = -vel.x;
                        pos.x = width - DIAMETER;
                    }
                    if pos.y <= DIAMETER {
                        vel.y = -vel.y;
                        pos.y = DIAMETER;
                    } else if pos.y >= height - DIAMETER {
                        vel.y = -vel.y;
                        pos.y = height - DIAMETER;
                    }
                }

                Particle { pos, vel, ..*p }
            })
            .collect();

        particles
    }
}
