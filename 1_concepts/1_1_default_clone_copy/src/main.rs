use std::fmt;

#[derive(Copy, Clone, Default)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Polyline {
    points: Option<Vec<Point>>,
}

impl Polyline {
    pub fn new() -> Self {
        Polyline { points: None }
    }

    pub fn from_vec(points: Vec<Point>) -> Self {
        Polyline {
            points: Some(points),
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point (x: {}, y: {})", self.x, self.y)
    }
}

impl fmt::Display for Polyline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s: Vec<String> = vec!["Polyline (".to_owned()];

        if let Some(pts) = &self.points {
            for point in pts {
                s.push(format!("[{};{}]", point.x, point.y));
            }
        }

        s.push(")".to_owned());

        write!(f, "{}", s.join(" "))
    }
}

fn main() {
    let point1 = Point::default();
    let point2 = Point {
        x: 40,
        ..Point::default()
    };
    let copy_point2 = point2;

    println!("Point 1 with default values: {}", point1);
    println!("Point 2 with y default value: {}", point1);
    println!("Point 3 is copy of Point 2: {}", copy_point2);

    let points1 = Polyline::new();
    let points2 = Polyline::from_vec(vec![point1, point2, copy_point2]);
    let points3 = points2.clone();

    println!("{}", points1);
    println!("{}", points2);
    println!("Clone of points 2: {}", points3);
}
