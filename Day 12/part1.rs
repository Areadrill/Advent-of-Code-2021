use std::fs;
use std::collections::HashMap;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let line_separation = input.split('\n').collect::<Vec<&str>>();

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    line_separation.iter()
        .take(line_separation.len() - 1)
        .for_each(|it| {
            let edge = it.split('-').collect::<Vec<&str>>();    

            if graph.get(&edge[0 as usize].to_string()) == None{
                graph.insert(edge[0 as usize].to_string(), Vec::new());
            }
            
            graph.get_mut(&edge[0 as usize].to_string()).unwrap().push(edge[1 as usize].to_string());

            if graph.get(&edge[1 as usize].to_string()) == None{
                graph.insert(edge[1 as usize].to_string(), Vec::new());
            }
            
            graph.get_mut(&edge[1 as usize].to_string()).unwrap().push(edge[0 as usize].to_string());
        
        });

    println!("{:?}", dfs(&"start".to_string(), &graph, &mut Vec::new()));  
}

fn dfs(node: &String, graph: &HashMap<String, Vec<String>>, visited: &mut Vec<String>) -> u32 {
    let mut node_visited = visited.clone();
    node_visited.push(node.to_string());

    let mut cnt = 0;

    for neighbor in graph.get(node).unwrap() {
        if *neighbor == "end".to_string() {
            cnt += 1;
        } else if !visited.contains(neighbor) || neighbor.chars().collect::<Vec<char>>()[0].is_uppercase()  {
            //println!("In node {}, next up is {}", node, neighbor);
            cnt += dfs(&neighbor, graph, &mut node_visited);
        }
    }

    return cnt;

    //println!("end of the line {:?}", visited);
}
