/* Layer 1: Foundational functions.
 * branch 1: get angle, radial sort, angle comparison
 * branch 2: Dot products, Distance, and Distance Squared
 * branch 3: Vector x Vector operations, and Vector x Float operations. atomic_handler.rs 
 * branch 4: Matrixes and operations to choose what part of the matrix
 * branch 5: Adding IDs to Architectural layers
 * layer 2: Farthest points, Bounded Triangulation, and Rings
 * branch 1: Triangulation, Farthest Vector, Windmill Theorem implementation
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 */
// You can ignore these, also, why tf do i have to import enums like they are a crate
use std::ops::{Add, Sub, Div, Mul, AddAssign};
use crate::constants::*;
use crate::CorrectionKeyVectors::*;
use crate::CorrectionKey32::*;
impl Object2l {
				pub fn correction_vector(&mut self, c: CorrectionKeyVectors) {
				    match c {
				        Pos(pos) => {self.obj.pos = pos}
				        Vel(vel) => {self.obj.vel = vel}
				        Acc(acc) => {self.obj.acc = acc}
				        Grav(grav) => {self.obj.grav = grav}
				    }
				}
				pub fn correction_f32(&mut self, c: CorrectionKey32) {
				    match c {
				        Apos(apos) => {self.obj.apos = apos}
				        Avel(avel) => {self.obj.avel = avel}
				        Aacc(aacc) => {self.obj.aacc = aacc}
				        Layer(layer) => {self.layer = layer}
				    }
				}
}
/*---------------------------------------
       First Layer Functions
---------------------------------------*/

//-----------branch one--------------------------
pub fn sort_by_keys<T>(objects: Vec<T>, keys: Vec<f32>) -> Vec<T> {
    let mut pairs: Vec<(f32, T)> = keys.into_iter().zip(objects).collect();
    pairs.sort_by(|a, b| a.0.total_cmp(&b.0));
    pairs.into_iter().map(|(_, obj)| obj).collect()
}
pub fn get_angle(vector: Vector2) -> f32 {
    vector.y.atan2(vector.x)//.to_degrees()
}
fn polar_cmp(a: Vector2, b: Vector2) -> bool {
    let ha = a.y < 0.0 || (a.y == 0.0 && a.x < 0.0);
    let hb = b.y < 0.0 || (b.y == 0.0 && b.x < 0.0);
    let cross = a.x * b.y - a.y * b.x;
    if ha ^ hb { ha } else { cross < 0.0 }
}
pub fn sort_mesh(mesh: Mesh, centroid: Vector2) -> Mesh {
    let mut keys = vec![];
    for vector in mesh.vectors.iter() {
        keys.push(get_angle(*vector - centroid));
    }
    let mut output = Mesh::default();
    output.vectors = sort_by_keys(mesh.vectors, keys);
    output
}
//-----------branch two--------------------------
#[inline(always)]
pub fn dot(a: Vector2, b: Vector2) -> f32 {
    a.x*b.x + a.y*b.y
}
pub fn squareddistance(vector: Vector2) -> f32 {
    vector.x*vector.x+vector.y*vector.y
}
pub fn distance(vector: Vector2) -> f32 {
    squareddistance(vector).sqrt()
}
//-----------branch three------------------------
impl Add<Vector2> for Vector2 {
    type Output = Vector2;
    
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub<Vector2> for Vector2 {
    type Output = Vector2;
    
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Div<f32> for Vector2 {
    type Output = Vector2;
    
    fn div(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x/rhs,
            y: self.y/rhs,
        }
    }
}
impl Mul<f32> for Vector2 {
    type Output = Vector2;
    
    fn mul(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x*rhs,
            y: self.y*rhs,
        }
    }
}
impl Mul<Vector2> for f32 { // bruh why do i have to do this, multiplication is associative!!!
    type Output = Vector2;
    
    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: rhs.x*self,
            y: rhs.y*self,
        }
    }
}
impl Mul<Vector2> for Matrix2 {
    type Output = Vector2;

    fn mul(self, v: Vector2) -> Vector2 {
        Vector2 {
            x: self.col1.x * v.x + self.col2.x * v.y,
            y: self.col1.y * v.x + self.col2.y * v.y,
        }
    }
}
//-----------branch four-------------------------
impl Matrix2 {
    pub fn start(&self) -> Vector2 {
        self.col1
    }
    pub fn end(&self) -> Vector2 {
        self.col2
    }
}
impl MatrixMatrix2 {
    pub fn c1r1(&self) -> Matrix2 {
        self.m1
    }
    pub fn c1r2(&self) -> Matrix2 {
        self.m2
    }
    pub fn c2r1(&self) -> Matrix2 {
        self.m3
    }
    pub fn c2r2(&self) -> Matrix2 {
        self.m4
    }
}
//-----------branch five-------------------------
pub fn sum(v: &[Vector2]) -> Vector2 {
    let mut acc = [Vector2::default(); 4];
    let chunks = v.chunks_exact(4);
    for c in chunks.clone() {for i in 0..4 {acc[i] += c[i];}}
    let mut s = (acc[0] + acc[1]) + (acc[2] + acc[3]);
    for x in chunks.remainder() {s += *x;}
    s
}
/*---------------------------------------
       Second Layer Functions
---------------------------------------*/
//-----------branch one--------------------------
impl Mesh {
    pub fn farvec(&self, centroid: Vector2) -> Vector2 {
        let mut vectoroid = Vector2::default();
        let mut verminoid = f32::default();
        let mut vermin = f32::default();
        for &vector in self.vectors.iter() {
            vermin = squareddistance(vector-centroid);
            (vectoroid, verminoid) = if vermin > verminoid {(vector, vermin)} else {(vectoroid, verminoid)}
        }
        vectoroid
    }
}
fn mesh_ringer(_mesh: Mesh) {
    todo!()
}
fn Triangulator(_object: Object2l) -> TriangleMesh {
    todo!()
}