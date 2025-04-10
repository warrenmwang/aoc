use std::{
    collections::{HashMap, HashSet},
    env,
    fmt::{self},
    fs::{self},
    process::exit,
};

#[derive(Clone)]
pub struct Node {
    id: String,
    num: Option<u16>,
    operator: Option<String>,
    pub dependencies: Option<Vec<String>>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ id: {}, num: {:?}, op: {:?}, deps: {:?} }}",
            self.id, self.num, self.operator, self.dependencies
        )
    }
}

fn build_graph(lines: Vec<&str>) -> HashMap<String, Node> {
    let mut graph: HashMap<String, Node> = HashMap::new();
    for line in lines.iter() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let output = parts[parts.len() - 1].to_string();
        let dependencies = parts[..parts.len() - 2].to_vec();

        // println!("output: {:?}", output);
        // println!("deps: {:?}", dependencies);

        let new_node = match dependencies.len() {
            // Handle case where X -> Y
            1 => {
                if let Ok(num_input) = dependencies[0].parse::<u16>() {
                    // X -> Y
                    // X is a number
                    Node {
                        id: output.clone(),
                        num: Some(num_input),
                        operator: None,
                        dependencies: None,
                    }
                } else {
                    // X -> Y
                    // X is another wire
                    let other_id = dependencies[0].to_string();
                    Node {
                        id: output.clone(),
                        num: None,
                        operator: None,
                        dependencies: Some(vec![other_id]),
                    }
                }
            }
            // Handle NOT X -> Y
            2 => {
                if dependencies[0] == "NOT" {
                    let dep_id = dependencies[1].to_string();
                    Node {
                        id: output.clone(),
                        num: None,
                        operator: Some(String::from("NOT")),
                        dependencies: Some(vec![dep_id]),
                    }
                } else {
                    panic!("Unsupported 2 part dependencies detected.");
                }
            }
            // Handle 3 deps
            3 => {
                // NUM VERB X
                if let Ok(num_input) = dependencies[0].parse::<u16>() {
                    let other_id = dependencies[2].to_string();
                    Node {
                        id: output.clone(),
                        num: Some(num_input),
                        operator: Some(dependencies[1].to_string()),
                        dependencies: Some(vec![other_id]),
                    }
                } else if let Ok(num_input) = dependencies[2].parse::<u16>() {
                    // X VERB NUM
                    let other_id = dependencies[0].to_string();
                    Node {
                        id: output.clone(),
                        num: Some(num_input),
                        operator: Some(dependencies[1].to_string()),
                        dependencies: Some(vec![other_id]),
                    }
                } else {
                    // X VERB Y
                    Node {
                        id: output.clone(),
                        num: None,
                        operator: Some(dependencies[1].to_string()),
                        dependencies: Some(vec![
                            dependencies[0].to_string(),
                            dependencies[2].to_string(),
                        ]),
                    }
                }
            }
            _ => panic!("Unhandled line: {line}"),
        };

        // println!("> {}", new_node);
        graph.insert(output.clone(), new_node);
    }
    graph
}

fn topological_sort(graph: &HashMap<String, Node>, starting_node_id: String) -> Vec<Node> {
    fn helper(
        graph: &HashMap<String, Node>,
        node_id: String,
        visited_node_ids: &mut HashSet<String>,
        result: &mut Vec<Node>,
    ) {
        visited_node_ids.insert(node_id.clone());

        if let Some(neighbors) = &graph.get(&node_id.clone()).unwrap().dependencies {
            for neighbor in neighbors {
                if !visited_node_ids.contains(&neighbor.to_string()) {
                    helper(graph, neighbor.to_string(), visited_node_ids, result);
                }
            }
        }

        let node = graph.get(&node_id).unwrap().clone();
        result.push(node);
    }

    let mut topo_order: Vec<Node> = Vec::new();
    let mut discovered: HashSet<String> = HashSet::new();

    if !graph.contains_key(&starting_node_id) {
        panic!("Requested wire: {} not found", starting_node_id);
    }

    helper(graph, starting_node_id, &mut discovered, &mut topo_order);

    // topo_order.reverse(); // the actual topo order reverses it, but for our purposes we don't want to

    topo_order
}

fn calculate_from_topo(mut graph: HashMap<String, Node>, mut topo_order: Vec<Node>) -> u16 {
    for (i, curr_node) in topo_order.iter_mut().enumerate() {
        // utilize verb and nums to compute values, updating nodes until we reach the final
        // desired node and get its value.

        // println!("before {}", curr_node);

        if i == 0 {
            assert_ne!(curr_node.num, None);
            assert_eq!(curr_node.dependencies, None);
            assert_eq!(curr_node.operator, None);
        }

        if let Some(deps) = curr_node.dependencies.as_ref() {
            if let Some(op) = curr_node.operator.as_ref() {
                match op.as_str() {
                    "NOT" => {
                        assert!(deps.len() == 1);
                        let other_node = graph.get(&deps[0]).unwrap();
                        curr_node.num = Some(!other_node.num.unwrap());
                    }
                    "AND" => match deps.len() {
                        1 => {
                            let other_node = graph.get(&deps[0]).unwrap();
                            let op1 = curr_node.num.unwrap();
                            let op2 = other_node.num.unwrap();
                            curr_node.num = Some(op1 & op2);
                        }
                        2 => {
                            let op1 = graph.get(&deps[0]).unwrap().num.unwrap();
                            let op2 = graph.get(&deps[1]).unwrap().num.unwrap();
                            curr_node.num = Some(op1 & op2);
                        }
                        _ => panic!(
                            "something went wrong. AND op should only have at most 2 deps for node: {}",
                            curr_node.id
                        ),
                    },
                    "OR" => match deps.len() {
                        1 => {
                            let other_node = graph.get(&deps[0]).unwrap();
                            let op1 = curr_node.num.unwrap();
                            let op2 = other_node.num.unwrap();
                            curr_node.num = Some(op1 | op2);
                        }
                        2 => {
                            let op1 = graph.get(&deps[0]).unwrap().num.unwrap();
                            let op2 = graph.get(&deps[1]).unwrap().num.unwrap();
                            curr_node.num = Some(op1 | op2);
                        }
                        _ => panic!(
                            "something went wrong. AND op should only have at most 2 deps for node: {}",
                            curr_node.id
                        ),
                    },
                    "LSHIFT" => {
                        let other_node = graph.get(&deps[0]).unwrap();
                        let offset = curr_node.num.unwrap();
                        curr_node.num = Some(other_node.num.unwrap() << offset);
                    }
                    "RSHIFT" => {
                        let other_node = graph.get(&deps[0]).unwrap();
                        let offset = curr_node.num.unwrap();
                        curr_node.num = Some(other_node.num.unwrap() >> offset);
                    }
                    _ => {}
                }
            } else {
                assert!(deps.len() == 1);
                // X = Y
                let other_node = graph.get(&deps[0]).unwrap();
                curr_node.num = other_node.num;
            }
        }

        graph.insert(curr_node.id.clone(), curr_node.clone());
        // println!("after {}", curr_node);
    }

    topo_order[topo_order.len() - 1].num.unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: [binary] [filename] [desired_wire]");
        exit(-1);
    }
    let filename = args[1].to_string();
    let desired_wire = args[2].to_string();

    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<_> = input.lines().collect();

    // println!("====== GRAPH ====== ");
    let mut graph = build_graph(lines);
    // println!();

    // println!("======= TOPO ======= ");
    let mut topo_order = topological_sort(&graph, desired_wire.clone());
    // println!();

    // create a topological sort of a subgraph of the entire
    // graph in order to resolve the necessary dependencies for just
    // the desired wire (no need to explore entire graph unless necessary
    // for the wire we want to resolve).
    // println!("====== RESOLVE ====== ");
    let a_wire_val = calculate_from_topo(graph.clone(), topo_order.clone());

    // part 1 result
    println!("PART 1: {} = {}", desired_wire, a_wire_val);

    // part 2 result
    // override b to be part 1 'a' wire value, then just
    // recompute the DAG for 'a' again, this is a separate graph
    // so that wires are already "reset"
    // unfort, our current setup has two copies of the DAG
    // and we need to update in both...
    //
    // one in graph
    let b_wire = graph.get_mut(&String::from("b")).unwrap();
    b_wire.num = Some(a_wire_val);

    // another in topo order vec
    for n in topo_order.iter_mut() {
        if n.id == String::from("b") {
            n.num = Some(a_wire_val);
        }
    }

    // finally do part 2
    let a_wire_val = calculate_from_topo(graph, topo_order);
    println!("PART 2: {} = {}", desired_wire, a_wire_val);
}
