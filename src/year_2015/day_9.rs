use itertools::Itertools;
use std::collections::HashMap;

fn is_valid_path(
    path: Vec<&&&str>,
    matrix: &Vec<Vec<i32>>,
    nodes_map: &HashMap<&str, usize>,
) -> Option<usize> {
    let mut res: usize = 0;

    for i in 0..path.len() - 1 {
        let n1 = path[i];
        let n2 = path[i + 1];

        let j = nodes_map.get(**n1).unwrap();
        let k = nodes_map.get(**n2).unwrap();
        let edge = matrix[*j][*k];

        if edge <= 0 {
            return None;
        }
        res += edge as usize;
    }

    Some(res)
}

pub fn day_9(input: &str) {
    // shortest hamiltonian path problem
    // the graph looks small enough for us to just brute force it
    // as opposed to use an alg like held-karp
    // 8 nodes -> 8! -> 40320
    let input: Vec<&str> = input.trim().split("\n").collect();

    let mut nodes_map: HashMap<&str, usize> = HashMap::new();
    let mut counter: usize = 0;

    // get set of unqiue nodes in graph
    for l in input.clone().into_iter() {
        let parts: Vec<&str> = l.split(" ").collect();
        let x1 = parts[0];
        let x2 = parts[2];

        if !nodes_map.contains_key(x1) {
            nodes_map.insert(x1, counter);
            counter += 1;
        }

        if !nodes_map.contains_key(x2) {
            nodes_map.insert(x2, counter);
            counter += 1;
        }
    }
    // init adjacency matrix (note that upper and lower triangular sections mirror
    // each other on the main diagonal from top left to bottom right)
    // store empty/invalid edge with -1
    // store self to self edge as 0
    let n = nodes_map.len();
    let mut matrix = vec![vec![-1; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                matrix[i][j] = 0;
            }
        }
    }
    // add valid edges
    for l in input.into_iter() {
        let parts: Vec<&str> = l.split(" ").collect();
        let x1 = parts[0];
        let x2 = parts[2];
        let dist: i32 = parts[4].trim().parse().unwrap();

        let i = nodes_map.get(x1).unwrap();
        let j = nodes_map.get(x2).unwrap();

        matrix[*i][*j] = dist;
        matrix[*j][*i] = dist;
    }

    // find the smallest and longest valid paths through all nodes
    let mut smallest_path = 10000;
    let mut longest_path = 0;

    let keys: Vec<&&str> = nodes_map.keys().collect();
    for perm in keys.iter().permutations(keys.len()) {
        if let Some(valid_path_len) = is_valid_path(perm, &matrix, &nodes_map) {
            if valid_path_len < smallest_path {
                smallest_path = valid_path_len;
            }
            if valid_path_len > longest_path {
                longest_path = valid_path_len;
            }
        }
    }

    println!("Part 1: {}", smallest_path);
    println!("Part 2: {}", longest_path);
}
