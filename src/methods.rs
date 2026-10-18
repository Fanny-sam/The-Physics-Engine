use crate::constants::*;
use rayon::prelude::*;
fn has_hardware_vulkan_compute() -> bool {
    let entry = ash::Entry::linked()?;
    let instance = create_instance(&entry)?;
    let devices = unsafe { instance.enumerate_physical_devices()? };
    
    for device in devices {
        let props = unsafe { instance.get_physical_device_properties(device) };
        // Filter out CPU drivers explicitly
        if props.device_type == ash::vk::PhysicalDeviceType::CPU {
            continue;
        }
        // Check for compute queue family
        let queue_families = unsafe { instance.get_physical_device_queue_family_properties(device) };
        if queue_families.iter().any(|q| q.queue_flags.contains(vk::QueueFlags::COMPUTE)) {
            return true; // Found real GPU with compute
        }
    }
    false
}
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
        let rot = Matrix2 {col1: Vector2 { x: cos,  y: sin },col2: Vector2 { x: -sin, y: cos },};
        for (&original, transformed) in self.nonwarp_collision.iter()
            .zip(self.collision.iter_mut()) 
        {*transformed = rot * original;};
    };
    self.prevapos = self.obj.apos;
}
pub fn correction(&mut self, c: CorrectionData) {
    match c.index {
        Pos => {self.obj.pos = Vector2 {x: c.x,y: c.y,};}
        Vel => {self.obj.vel = Vector2 {x: c.x,y: c.y,};}
        Acc => {self.obj.acc = Vector2 {x: c.x,y: c.y,};}
        Apos => {self.obj.apos = c.value;}
        Avel => {self.obj.avel = c.value;}
        Aacc => {self.obj.aacc = c.value;}
        Grav => {self.obj.grav = Vector2 {x: c.x,y: c.y,};}
        Layer => {self.layer = c.uvalue;}
        Prevapos => {self.prevapos = c.value;}
    }
}
/*---------------------------------------
       First Layer Functions
---------------------------------------*/

//-----------lift one--------------------------
pub fn sort_by_keys<T>(objects: Vec<T>, keys: Vec<f32>) -> Vec<T> {
    let mut pairs: Vec<(f32, T)> = keys.into_iter().zip(objects).collect();
    pairs.sort_by(|a, b| a.0.total_cmp(&b.0));
    pairs.into_iter().map(|(_, obj)| obj).collect()
}
pub fn get_angle(vector: Vector2) -> f32 {
    vector.y.atan2(vector.x)//.to_degrees()
}
#[inline(always)]
fn polar_cmp(a: Vector2, b: Vector2) -> bool {
    let ha = a.y < 0.0 || (a.y == 0.0 && a.x < 0.0);
    let hb = b.y < 0.0 || (b.y == 0.0 && b.x < 0.0);
    let cross = a.x * b.y - a.y * b.x;
    if ha ^ hb { ha } else { cross < 0.0 }
}
pub fn sort_mesh(mesh: Mesh, centroid: Vector2) -> Mesh {
    let mut keys = Vec![]
    for vector in mesh.vectors.iter() {
        keys.push(get_angle(vector - centroid));
    }
    let mut output = Mesh::default();
    output.vectors = sort_by_keys(mesh.vectors, keys);
}
//-----------lift one--------------------------
#[inline(always)]
pub fn dot(a: Vector2, b: Vector2) -> f32 {
    a.x*b.x + a.y*b.y
}
pub fn squareddistance(vector: Vector2) -> f32 {
    vector.x*vector.x+vector.y*vector.y
}
impl std::ops::Add<Vector2> for Vector2 {
    type Output = Vector2
    
    pub fn add(self, rhs: Vector2) -> Vector2 {
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
impl std::ops::Sub<Vector2> for Vector2 {
    type Output = Vector2
    
    pub fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl std::ops::Div<f32> for Vector2 {
    type Output = Vector2
    
    pub fn div(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x/rhs,
            y: self.y/rhs,
        }
    }
}
impl std::ops::Mul<Vector2> for Matrix2 {
    type Output = Vector2;

    pub fn mul(self, v: Vector2) -> Vector2 {
        Vector2 {
            x: self.col1.x * v.x + self.col2.x * v.y,
            y: self.col1.y * v.x + self.col2.y * v.y,
        }
    }
}

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
impl Vector2 {
    // Borrows self and returns Point2 (cheap, copies two floats)
    #[inline(always)]
    pub fn to_point2(&self) -> Point2<f32> {
        Point2::new(self.x, self.y)
    }
}
impl Archi4l {
    pub fn add(&mut self, object: Island3l) {
        self.islids.push(Island3l.islid)
    }
}
impl Island3l {
    pub fn add(&mut self, object: Object2l) {
        self.objids.push(Object2l.id)
    }
}

/*---------------------------------------
       Second Layer Functions
---------------------------------------*/
pub fn sum(v: &[Vector2]) -> Vector2 {
    let mut acc = [Vector2::default(); 4];
    let mut chunks = v.chunks_exact(4);
    for c in &mut chunks {
        acc[0] +=c[0];
        acc[1] +=c[1];
        acc[2] +=c[2];
        acc[3] +=c[3];
    }
    let mut s = (acc[0] + acc[1]) + (acc[2] + acc[3]);
    for x in chunks.remainder() { s += *x; }
    s
}
impl Mesh {
    pub fn farvec(&self, centroid: Vector2) -> Vector2 {
        let mut vectoroid = Vector2::default();
        let mut verminoid = f32::default();
        let mut vermin = f32::default();
        for &vector in self.vectors.iter() {
            vermin = squarredistance(vector-centroid);
            (vectoroid, verminoid) = if vermin > verminoid {(vector, vermin)} else {(vectoroid, verminoid)}
        }
        vectoroid
    }
}
fn Mesh_Ringer(mesh: Mesh) {
    
}
fn Triangulator(object: Object2l) -> TriangleMesh {
    
}