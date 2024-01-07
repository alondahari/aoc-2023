static INPUT_FILE: &str = include_str!("./input.txt");
static MIN_LIMIT: i64 = 200_000_000_000_000;
static MAX_LIMIT: i64 = 400_000_000_000_000;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct HailStoneVector {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl HailStoneVector {
    fn new(s: &str) -> Self {
        let parts = s.split_once('@').unwrap();
        let (x, y) = string_parts(parts.0);
        let (vx, vy) = string_parts(parts.1);
        Self { x, y, vx, vy }
    }

    fn intersection_point_with(&self, v: &HailStoneVector) -> Option<Point> {
        let dx = v.x - self.x;
        let dy = v.y - self.y;
        let det = v.vx * self.vy - v.vy * self.vx;
        // line don't intersect
        if det == 0 {
            return None;
        };

        let t = (dy * v.vx - dx * v.vy) / det;
        let u = (dy * self.vx - dx * self.vy) / det;

        // lines intersect in the past
        if t < 0 || u < 0 {
            None
        } else {
            Some(Point::new(self.x + self.vx * t, self.y + self.vy * t))
        }
    }

    fn intersects_in_limits(&self, v: &HailStoneVector) -> bool {
        match self.intersection_point_with(v) {
            Some(point) => in_limits(point.x) && in_limits(point.y),
            _ => false,
        }
    }
}

fn string_parts(s: &str) -> (i64, i64) {
    let current_coordinates: Vec<i64> = s.split(',').map(|m| m.trim().parse().unwrap()).collect();
    let x = *current_coordinates.first().unwrap();
    let y = *current_coordinates[1..].first().unwrap();
    (x, y)
}

fn in_limits(x: i64) -> bool {
    x < MAX_LIMIT && x > MIN_LIMIT
}

fn main() {
    let stones: Vec<HailStoneVector> = INPUT_FILE.lines().map(HailStoneVector::new).collect();

    let total = stones.iter().enumerate().fold(0, |acc, (i, stone)| {
        acc + stones[i..]
            .iter()
            .filter(|stone2| stone.intersects_in_limits(stone2))
            .count()
    });
    println!("{}", total);
}
