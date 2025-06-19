use geo::{LineString, Line};
use wkt::TryFromWkt;

fn main() {
    let json = std::fs::read_to_string("input.json").unwrap();
    let raw_input: Vec<String> = serde_json::from_str(&json).unwrap();
    let lines: Vec<Line> = raw_input.into_iter().map(parse_line).collect();
    println!("Got {} lines", lines.len());

    let _ = geo::sweep::Intersections::<_>::from_iter(lines);
}

// TODO Producing a wkt LINE works, but reading it doesn't?!
fn parse_line(x: String) -> Line {
    let y = LineString::try_from_wkt_str(&x.replace("LINE", "LINESTRING")).unwrap();
    y.lines().next().unwrap()
}
