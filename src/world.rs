#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Particle {
    Air,
    Dirt,
}

pub struct World {
    width: u32,
    height: u32,
    scale: u32,
    particles: Vec<Particle>,
    particles_old: Vec<Particle>,
}

impl World {
    pub fn new((width, height): (u32, u32), scale: u32) -> Self {
        let particles = vec![Particle::Air; (width * height) as usize];
        let particles_old = particles.clone();
        let mut world = Self {
            width,
            height,
            scale,
            particles,
            particles_old,
        };
        for x in 0..width {
            for y in 0..height {
                if rand::random() {
                    world.set((x, y), Particle::Dirt);
                }
            }
        }
        world
    }

    fn get(&self, (x, y): (u32, u32)) -> Option<Particle> {
        if x >= self.width {
            None
        } else if y >= self.height {
            None
        } else {
            Some(self.particles[(x + y * self.width) as usize])
        }
    }

    fn get_old(&self, (x, y): (u32, u32)) -> Option<Particle> {
        if x >= self.width {
            None
        } else if y >= self.height {
            None
        } else {
            Some(self.particles_old[(x + y * self.width) as usize])
        }
    }

    fn set(&mut self, (x, y): (u32, u32), particle: Particle) {
        if x >= self.width {
            panic!("{} exceeds the width ({})", x, self.width)
        }
        if y >= self.height {
            panic!("{} exceeds the height ({})", y, self.height)
        }
        self.particles[(x + y * self.width) as usize] = particle
    }

    pub fn setat(&mut self, (x, y): (u32, u32), particle: Particle) {
        let (x, y) = (x / self.scale, y / self.scale);
        self.set((x, y), particle)
    }

    pub fn update(&mut self) {
        std::mem::swap(&mut self.particles, &mut self.particles_old);
        self.particles.fill(Particle::Air);

        for y in 0..self.height {
            for x in 0..self.width {
                let particle = self.get_old((x, y));
                if particle.is_none() {
                    continue;
                }
                let particle = particle.unwrap();

                match particle {
                    Particle::Dirt => {
                        if self.get_old((x, y + 1)) == Some(Particle::Air) {
                            self.set((x, y + 1), particle);
                        } else {
                            self.set((x, y), particle);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % (self.width * self.scale) as usize) as u32;
            let y = (i / (self.width * self.scale) as usize) as u32;
            let (x, y) = (x / self.scale, y / self.scale);

            let color = match self.get((x, y)).unwrap_or(Particle::Air) {
                Particle::Air => [0xff, 0xff, 0xff, 0xff],
                Particle::Dirt => [0xef, 0xb4, 0x59, 0xff],
            };
            pixel.copy_from_slice(&color);
        }
    }
}
