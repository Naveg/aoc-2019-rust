use std::collections::{HashMap, HashSet};
use std::io;

fn get_wire_points(wire_desc: &String) -> HashMap<(i32, i32), i32> {
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();
    let mut pos = (0, 0);
    let mut total_distance = 0;
    for cmd in wire_desc.trim().split(',') {
        let distance: i32 = cmd.trim_matches(char::is_alphabetic).parse().unwrap();
        let dir = cmd.chars().nth(0).unwrap();
        let dest = match dir {
            'R' => (pos.0 + distance, pos.1),
            'L' => (pos.0 - distance, pos.1),
            'U' => (pos.0, pos.1 + distance),
            'D' => (pos.0, pos.1 - distance),
            _ => pos,
        };
        match cmd.chars().nth(0).unwrap() {
            'R' => (pos.0..pos.0 + distance).for_each(|x| {
                points
                    .entry((x, pos.1))
                    .or_insert(total_distance + x - pos.0);
                return;
            }),
            'L' => (pos.0 - distance..pos.0).for_each(|x| {
                points
                    .entry((x, pos.1))
                    .or_insert(total_distance + pos.0 - x);
                return;
            }),
            'U' => (pos.1..pos.1 + distance).for_each(|y| {
                points
                    .entry((pos.0, y))
                    .or_insert(total_distance + y - pos.1);
                return;
            }),
            'D' => (pos.1 - distance..pos.1).for_each(|y| {
                points
                    .entry((pos.0, y))
                    .or_insert(total_distance + pos.1 - y);
                return;
            }),
            _ => (),
        };
        pos = dest;
        total_distance += distance;
    }
    points
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let wire1points = get_wire_points(&input);
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let wire2points = get_wire_points(&input);

    let min_dist = wire1points
        .keys()
        .filter(|p| wire2points.contains_key(p))
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap();
    let min_total_dist = wire1points
        .iter()
        .filter(|(k, v)| wire2points.contains_key(k))
        .map(|(k, v)| v + wire2points.get(k).unwrap())
        .min()
        .unwrap();
    println!("Part 1: {}", min_dist);
    println!("Part 2: {}", min_total_dist);
}
