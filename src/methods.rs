use spade::Point2;
use crate::constants::*;
impl Object2l {
    #[inline(always)]
    pub fn integrate_linear(&mut self, dt: f32) {
        self.obj.vel.x += (self.obj.grav.x + self.obj.acc.x) * dt;
        self.obj.vel.y += (self.obj.grav.y + self.obj.acc.y) * dt;

        self.obj.pos.x += self.obj.vel.x * dt;
        self.obj.pos.y += self.obj.vel.y * dt;
    }

    #[inline(always)]
    pub fn integrate_angular(&mut self, dt: f32) {
        self.obj.avel += self.obj.aacc * dt;
        self.obj.apos += self.obj.avel * dt;
    }
    #[inline(always)]
    pub fn calculate_collision_mesh(&mut self) {
        // calculate collision mesh rotation
        if self.obj.apos != self.prevapos {
            let (sin, cos) = (self.obj.apos).sin_cos();
            let rot = Matrix2 {
                col1: Vector2 { x: cos,  y: sin },
                col2: Vector2 { x: -sin, y: cos },
                };
            for (original, transformed) in self.nonwarp_collision.iter()
                .zip(self.collision.iter_mut())
            {
                *transformed = rot * *original;
                
            };
        };
        self.prevapos = self.obj.apos;
    }
    pub fn correction(&mut self, c: CorrectionData) {
        match c.index {
            Pos => {
                self.obj.pos = Vector2 {
                    x: c.x,
                    y: c.y,
                };
            }

            Vel => {
                self.obj.vel = Vector2 {
                    x: c.x,
                    y: c.y,
                };
            }

            Acc => {
                self.obj.acc = Vector2 {
                    x: c.x,
                    y: c.y,
                };
            }

            Apos => {
                self.obj.apos = c.value;
            }

            Avel => {
                self.obj.avel = c.value;
            }

            Aacc => {
                self.obj.aacc = c.value;
            }

            Grav => {
                self.obj.grav = Vector2 {
                    x: c.x,
                    y: c.y,
                };
            }

            Layer => {
                self.layer = c.uvalue;
            }

            Prevapos => {
                self.prevapos = c.value;
            }
        }
    }
}

#[inline(always)]
pub fn dot(a: Vector2, b: Vector2) -> f32 {
    a.x*b.x + a.y*b.y
}
impl std::ops::Mul<Vector2> for Matrix2 {
    type Output = Vector2;

    fn mul(self, v: Vector2) -> Vector2 {
        Vector2 {
            x: self.col1.x * v.x + self.col2.x * v.y,
            y: self.col1.y * v.x + self.col2.y * v.y,
        }
    }
}
impl Matrix2 {
    fn start(&self) -> Vector2 {
        self.col1
    }
    fn end(&self) -> Vector2 {
        self.col2
    }
}
impl Vector2 {
    // Borrows self and returns Point2 (cheap, copies two floats)
    #[inline(always)]
    pub fn to_point2(&self) -> Point2<f32> {
        Point2::new(self.x, self.y)
    }
}
