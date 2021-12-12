use std::collections::{HashMap, HashSet};

type Node<'a> = &'a str;
type Routes<'a> = HashSet<Node<'a>>;
type Map<'a> = HashMap<Node<'a>, Routes<'a>>;

fn parse_input(input: &str) -> Map {
    let mut map = HashMap::new();

    for line in input.split_whitespace() {
        let mut split = line.split('-');
        let from = split.next().unwrap();
        let to = split.next().unwrap();

        let from_entry = map.entry(from).or_insert_with(HashSet::new);
        from_entry.insert(to);

        let to_entry = map.entry(to).or_insert_with(HashSet::new);
        to_entry.insert(from);
    }

    map
}

fn part1_traverse(map: &Map, node: &str, visited: HashSet<&str>) -> usize {
    if node == "end" {
        return 1;
    }

    let routes = map.get(node);

    if routes.is_none() {
        return 0;
    }

    let mut path_count = 0;

    for &route in routes.unwrap() {
        let is_lowercase = route.to_lowercase() == route;
        if is_lowercase && visited.contains(route) {
            continue;
        }

        let mut inner_visited = visited.clone();
        inner_visited.insert(node);

        path_count += part1_traverse(map, route, inner_visited);
    }

    path_count
}

fn part2_traverse(
    map: &Map,
    node: &str,
    visited: HashMap<&str, usize>,
    twice_cave: Option<&str>,
) -> usize {
    if node == "end" {
        return 1;
    }

    let is_small_cave = node.to_lowercase() == node;

    let mut visited = visited.clone();
    let visited_count = visited.entry(node).or_default();

    let mut twice_cave = twice_cave;

    let visit_limit = {
        if node == "start" {
            1
        } else if is_small_cave {
            if let Some(twice_cave) = twice_cave {
                if twice_cave == node {
                    2
                } else {
                    1
                }
            } else if *visited_count == 1 {
                twice_cave = Some(node);
                2
            } else {
                1
            }
        } else {
            usize::MAX
        }
    };

    if *visited_count >= visit_limit {
        return 0;
    }

    *visited_count += 1;

    let mut path_count = 0;
    for &route in map.get(node).unwrap() {
        path_count += part2_traverse(map, route, visited.clone(), twice_cave);
    }
    path_count
}

pub fn part1(input: &str) -> usize {
    part1_traverse(&parse_input(input), "start", HashSet::new())
}

pub fn part2(input: &str) -> usize {
    part2_traverse(&parse_input(input), "start", HashMap::new(), None)
}
