use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Paint {
    color: String,
    area: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Pint {
    beer: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Pt {
    Paint(Paint),
    Point(Point),
    Pint(Pint),
}

fn main() -> Result<(), serde_yaml::Error> {
    let yaml = "
- Point:
    x: 1.0
    y: 2.0
- Pint:
    beer: Guiness
- Paint:
    color: Red
    area: 10
";
    let pts: Vec<Pt> = serde_yaml::from_str(yaml)?;
    println!("{:?}", pts);

    Ok(())
}
