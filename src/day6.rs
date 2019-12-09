use std::collections::HashMap;

fn transfer_path<'a>(
    mut from: &'a str,
    to: &str,
    parent_map: &'a HashMap<String, String>,
) -> Vec<&'a str> {
    let mut path = Vec::new();
    while let Some(parent) = parent_map.get(from) {
        path.push(parent.as_str());

        if parent == to {
            break;
        } else {
            from = parent;
        }
    }
    path
}

fn main() {
    let input = include_str!("day6.txt");

    let orbit_items: Vec<(String, String)> = input.lines()
        .map(|line| {
            let mut parts = line.split(')');
            let ref_obj = parts.next().unwrap().to_string();
            let orbit_obj = parts.next().unwrap().to_string();

            (ref_obj, orbit_obj)
        })
        .collect();

    let mut parent_map = HashMap::new();
    let mut orbit_map = HashMap::new();
    for (ref_obj, orbit_obj) in orbit_items {
        let orbit_list = orbit_map.entry(ref_obj.clone())
            .or_insert_with(Vec::new);

        orbit_list.push(orbit_obj.clone());

        orbit_map.entry(orbit_obj.clone()).or_insert_with(Vec::new);
        parent_map.insert(orbit_obj, ref_obj);
    }

    let total: usize = orbit_map.keys()
        .map(|leaf| transfer_path(leaf, "COM", &parent_map).len())
        .sum();
    println!("total count: {}", total);

    let my_path = transfer_path("YOU", "COM", &parent_map);
    let santa_path = transfer_path("SAN", "COM", &parent_map);

    let intersection = *my_path.iter()
        .filter(|obj| santa_path.contains(*obj))
        .next().unwrap();

    println!("transfer to Santa via {}", intersection);
    let dist_to_intersection = transfer_path("YOU", intersection, &parent_map).len() - 1;
    let dist_to_santa = transfer_path("SAN", intersection, &parent_map).len() - 1;
    println!("total transfers to reach Santa: {}", dist_to_intersection + dist_to_santa);
}