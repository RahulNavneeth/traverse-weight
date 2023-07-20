use std::collections::{HashMap, HashSet};
use std::{env, io::BufRead};

type Graph = HashMap<usize, HashMap<usize, usize>>;

fn traverse(graph: &Graph, source: usize) {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();

    stack.push((source, 0));
    visited.insert(source);

    while let Some((current_node, total_weight)) = stack.pop() {
        println!("Node: {}, Total Weight: {}", current_node, total_weight);
        for (&neighbor, &weight) in &graph[&current_node] {
            if !visited.contains(&neighbor) {
                let new_total_weight = total_weight + weight;
                stack.push((neighbor, new_total_weight));
                visited.insert(neighbor);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut graph: Graph = HashMap::new();
    let args = env::args().collect::<Vec<String>>();
    let fs = std::fs::OpenOptions::new()
        .read(true)
        .open(args.get(1).map_or("./data.txt", |arg| &*arg))?;
    let r = std::io::BufReader::new(fs);
    r.lines().enumerate().for_each(|(idx, res)| match res {
        Ok(value) => {
            let c = value
                .split_whitespace()
                .filter_map(|x| match x.parse::<usize>() {
                    Ok(value) => Some(value),
                    Err(_) => None,
                })
                .collect::<Vec<usize>>();

            c.iter().enumerate().for_each(|(key, &value)| {
                if value != 0 {
                    if !graph.contains_key(&idx) {
                        graph.insert(idx, HashMap::from([(key, value)]));
                    } else {
                        if let Some(i) = graph.get_mut(&idx) {
                            i.insert(key, value);
                        }
                    }
                }
            });
        }
        Err(e) => println!("ERROR {:?}", e),
    });
    traverse(&mut graph, 0);
    Ok(())
}
