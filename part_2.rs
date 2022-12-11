use rand::Rng;
use std::fs::File;
use std::io::BufRead;

// The goal is to find the degree distribution in my graph. 
// I need to compute the degree of of a random selection of 3000 nodes in the graph and then compute the distribution of these nodes.
// Since that data set I have is very long, I will choose a random subset of 3000 vertices and look at the degree at these 500 vertices 

struct Graph {
    adj_list: Vec<Vec<usize>>,
   
}

impl Graph {
    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).expect("Unable to open file");
        let buf_reader = std::io::BufReader::new(file).lines();
        let mut n: usize = 0;
        let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n];
        for (_i, line) in buf_reader.enumerate() {
            let line_str = line.expect("Error reading");
            if _i == 0 {
                n = line_str.parse::<usize>().unwrap();
                adj_list = vec![vec![];n];
            }
            else {
                let v: Vec<&str> = line_str.trim().split_whitespace().collect();
                let x = v[0].parse::<usize>().unwrap();
                let y = v[1].parse::<usize>().unwrap();
                adj_list[x].push(y);
            }
        }
        return Graph {
            adj_list,
        };
    }

    // calculating the degree of a node or vertex
    fn calculate_degree_of_node(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let random_node: usize = rng.gen_range(0..self.adj_list.len());
        let mut counter = 0;
        for _ in self.adj_list[random_node as usize].iter() {
            counter +=1
    }
        //println!("The degree of the node is {}", counter);
        return counter;
} 
   
    
    fn calculate_degreee_of_node_at_distance_2(&self) -> f32 {
        let mut rng = rand::thread_rng();
        let mut vector_of_distances:Vec<usize> = Vec::new();
        let random_node: usize = rng.gen_range(0..self.adj_list.len());
        for neighbors_distance_1 in self.adj_list[random_node].iter() {
            for neighbors_distance_2 in self.adj_list[*neighbors_distance_1].iter() {
                vector_of_distances.push(self.adj_list[*neighbors_distance_2].len());
            }
        } 
        let avg_distance: f32 = (vector_of_distances.iter().sum::<usize>() as f32)/ (vector_of_distances.len() as f32);
        return avg_distance;

    }

}


pub fn main() {
    let data_file = Graph::new("amazon.txt");
    let mut vector_of_nodes:Vec<i32> = vec![];
    for _ in 1..= 2000 {
        let degree_of_node:i32 = Graph::calculate_degree_of_node(&data_file); 
        //println!("{}", degree_of_node);
        vector_of_nodes.push(degree_of_node);
    }
    println!("Below is the vector containing the degree of 3000 nodes that were randomly chosen from my graph");
    println!();
    println!();
    println!("{:?}", vector_of_nodes);
    let sum: i32 =  vector_of_nodes.iter().sum();
    println!();
    let average_degree_of_vertex: f32 = sum as f32/ vector_of_nodes.len() as f32;
    println!("The average degree of a node in this graph is {}", average_degree_of_vertex);
    //println!("The degree of nodes placed inside a vector for random 3000 nodes are {:?}", vector_of_nodes);
    //println!("The average degree of a vertex is {}", average_degree_of_vertex);
    for _ in 1..= 3000 {
        //println!("The average degree at neighbor distance 2 is {}", Graph::calculate_degreee_of_node_at_distance_2(&data_file));
    }
}
// test cases for both functions