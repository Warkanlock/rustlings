// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // we need to create a new object because if we still use Some(p) we cannot use the variable after the usage
    // it works because we did not bind y to the anything
    match y {
        Some(Point { x, y }) => println!("Co-ordinates are {},{} ", x, y),
        _ => println!("no match"),
    }

    y; // Fix without deleting this line.
}
