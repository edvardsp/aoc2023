#[derive(Debug)]
pub struct Input {
    hails: Vec<Hail>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let hails = s.lines().map(Hail::from).collect();
        Self { hails }
    }
}

#[derive(Copy, Clone, Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl From<&str> for Vector {
    fn from(value: &str) -> Self {
        let mut values = value.split(',').map(|n| n.trim().parse().unwrap());
        let x = values.next().unwrap();
        let y = values.next().unwrap();
        let z = values.next().unwrap();
        Self { x, y, z }
    }
}

impl Vector {
    fn bounding_box_xy(&self, min: f64, max: f64) -> bool {
        min <= self.x && self.x <= max && min <= self.y && self.y <= max
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Hail {
    pos: Vector,
    vel: Vector,
}

impl From<&str> for Hail {
    fn from(value: &str) -> Self {
        let (pos, vel) = value.split_once(" @ ").unwrap();
        let pos = Vector::from(pos);
        let vel = Vector::from(vel);
        Self { pos, vel }
    }
}

impl Hail {
    fn at(&self, t: f64) -> Vector {
        let x = self.pos.x + self.vel.x * t;
        let y = self.pos.y + self.vel.y * t;
        let z = self.pos.z + self.vel.z * t;
        Vector { x, y, z }
    }

    fn transform(&self, vel: Vector) -> Self {
        let vel = self.vel - vel;
        Self { pos: self.pos, vel }
    }

    fn is_parallel(&self, other: &Self) -> bool {
        (self.vel.x * other.vel.y - self.vel.y * other.vel.x) == 0.0
    }

    fn intersects_xy(&self, other: &Self) -> Option<f64> {
        // From equations, solve for t0:
        //  vx0 * t0 + px0 = vx1 * t1 + px1
        //  vy0 * t0 + py0 = vy1 * t1 + py1
        let den = self.vel.x * other.vel.y - self.vel.y * other.vel.x;
        if den == 0.0 {
            // parallel
            return None;
        }

        let num =
            other.vel.x * (self.pos.y - other.pos.y) + other.vel.y * (other.pos.x - self.pos.x);
        let t = num / den;
        if t < 0.0 {
            // in the past
            return None;
        }

        Some(t)
    }

    fn intersects_at_xy(&self, other: &Self) -> Option<Vector> {
        self.intersects_xy(other).map(|t| self.at(t))
    }

    fn collides_xy(&self, other: &Self) -> bool {
        self.intersects_xy(other)
            .and(other.intersects_xy(self))
            .is_some()
    }
}

fn combinations(hails: &[Hail]) -> impl Iterator<Item = (&Hail, &Hail)> {
    let num = hails.len();
    (0..num - 1)
        .flat_map(move |i| (i + 1..num).map(move |j| (i, j)))
        .map(|(i, j)| (&hails[i], &hails[j]))
}

fn colliding_hail_within_xy(hails: &[Hail], min: f64, max: f64) -> usize {
    combinations(hails)
        .filter_map(|(lhs, rhs)| lhs.intersects_at_xy(rhs).and(rhs.intersects_at_xy(lhs)))
        .filter(|pos| pos.bounding_box_xy(min, max))
        .count()
}

fn transform(hails: &[Hail], reference_xy: Vector) -> Option<Vector> {
    let lhs = hails[0].transform(reference_xy);
    let rhs = hails[1].transform(reference_xy);

    let t0 = lhs.intersects_xy(&rhs)?;
    let t1 = rhs.intersects_xy(&lhs)?;

    // From equation, solve for rz:
    // t0 * (vz0 - rz) + z0 = t1 * (vz1 - rz) + z1
    let rz = (t1 * rhs.vel.z - t0 * lhs.vel.z + rhs.pos.z - lhs.pos.z) / (t1 - t0);

    let mut reference_xyz = reference_xy;
    reference_xyz.z = rz;

    let rock_pos = hails[0].transform(reference_xyz).at(t0);
    if rock_pos.z < 0.0 {
        // below ground
        return None;
    }

    // Cherck first that all hail transformed to rock's reference frame either intersect
    // or are parallel towards a common point on the XY plane.
    for (hail0, hail1) in combinations(hails) {
        let hail0 = hail0.transform(reference_xyz);
        let hail1 = hail1.transform(reference_xyz);
        if !hail0.collides_xy(&hail1) && !hail0.is_parallel(&hail1) {
            return None;
        }
    }

    Some(rock_pos)
}

pub fn part1(input: &Input) -> usize {
    const MIN: f64 = 200000000000000.0;
    const MAX: f64 = 400000000000000.0;

    colliding_hail_within_xy(&input.hails, MIN, MAX)
}

pub fn part2(input: &Input) -> usize {
    let rock_pos = (-512..512)
        .flat_map(|x| (-512..512).map(move |y| (x as f64, y as f64)))
        .map(|(x, y)| Vector { x, y, z: 0.0 })
        .filter_map(|rock_velocity_xy| transform(&input.hails, rock_velocity_xy))
        .next()
        .unwrap();

    (rock_pos.x + rock_pos.y + rock_pos.z) as usize
}
