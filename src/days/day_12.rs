use std::{fmt::Display, rc::Rc, cell::RefCell};

#[derive(Debug)]
struct Node {
    coordinates: (usize, usize),
    cost: u64,
    previous: Option<Rc<RefCell<Node>>>,
    is_end: bool,
    reachable_nodes: Vec<Rc<RefCell<Node>>>
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
    }
}

impl Eq for Node {}

pub fn a(input: &str) -> Box<dyn Display> {
    let height_map = input.lines().map(|l| l.chars().map(|c| if c == 'S' {'a'} else if c == 'E' {'z'} else {c}).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut graph = Vec::new();
    for (x, line) in input.lines().enumerate() {
        let mut line_nodes = Vec::new();
        for (y, node) in line.chars().enumerate() {
            if node == 'S' {
                line_nodes.push(Rc::new(RefCell::new(Node {
                    coordinates: (x,y),
                    cost: 0,
                    previous: None,
                    is_end: false,
                    reachable_nodes: Vec::new(),
                })));
            } else if node == 'E' {
                line_nodes.push(Rc::new(RefCell::new(Node {
                    coordinates: (x,y),
                    cost: u32::MAX as u64,
                    previous: None,
                    is_end: true,
                    reachable_nodes: Vec::new(),
                })));
            } else {
                line_nodes.push(Rc::new(RefCell::new(Node {
                    coordinates: (x,y),
                    cost: u32::MAX as u64,
                    previous: None,
                    is_end: false,
                    reachable_nodes: Vec::new(),
                })));
            }
        }
        graph.push(line_nodes);
    }

    for (x, line) in height_map.iter().enumerate() {
        for (y, node) in line.iter().enumerate() {
            let mut current_node = graph[x][y].borrow_mut();

            if y != 0 {
                if height_map[x][y-1] as u8 <= *node as u8 + 1 {
                    current_node.reachable_nodes.push(graph[x][y-1].clone());
                }
            }
            if x != 0 {
                if height_map[x-1][y] as u8 <= *node as u8 + 1 {
                    current_node.reachable_nodes.push(graph[x-1][y].clone());
                }
            }
            if y != height_map[0].len()-1 {
                if height_map[x][y+1] as u8 <= *node as u8 + 1 {
                    current_node.reachable_nodes.push(graph[x][y+1].clone());
                }
            }
            if x != height_map.len()-1 {
                if height_map[x+1][y] as u8 <= *node as u8 + 1 {
                    current_node.reachable_nodes.push(graph[x+1][y].clone());
                }
            }
            
         
        }
    }

    let mut q = graph.iter().flatten().cloned().collect::<Vec<Rc<RefCell<Node>>>>();

    while q.len() != 0 {
        q.sort_by_key(|n| n.borrow().cost);
        let u = q.swap_remove(0);
        for neighbor in u.borrow().reachable_nodes.iter() {
            if q.contains(neighbor) {
                if u.borrow().cost + 1 < neighbor.borrow().cost {
                    neighbor.borrow_mut().cost = u.borrow().cost + 1;
                    neighbor.borrow_mut().previous = Some(u.clone());
                }
            }
        }
    }

    let mut counter = 0;
    let mut flat_graph = graph.iter().flatten();
    let mut current_node = flat_graph.find(|n| n.borrow().is_end).unwrap().clone();
    while let Some(prev) = current_node.clone().borrow().previous.clone() {
        counter += 1;
        current_node = prev;
    }

    Box::new(counter)
}

pub fn b(input: &str) -> Box<dyn Display> {
    let height_map = input.lines().map(|l| l.chars().map(|c| if c == 'S' {'a'} else if c == 'E' {'z'} else {c}).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut graph = Vec::new();
    for (x, line) in input.lines().enumerate() {
        let mut line_nodes = Vec::new();
        for (y, node) in line.chars().enumerate() {
            if node == 'S' {
                line_nodes.push(Rc::new(RefCell::new(Node {
                    coordinates: (x,y),
                    cost: 0,
                    previous: None,
                    is_end: false,
                    reachable_nodes: Vec::new(),
                })));
            } else if node == 'a' {
                line_nodes.push(Rc::new(RefCell::new(Node {
                    coordinates: (x,y),
                    cost: 0,
                    previous: None,
                    is_end: false,
                    reachable_nodes: Vec::new(),
                })));
            } else if node == 'E' {
                line_nodes.push(Rc::new(RefCell::new(Node {
                    coordinates: (x,y),
                    cost: u32::MAX as u64,
                    previous: None,
                    is_end: true,
                    reachable_nodes: Vec::new(),
                })));
            } else {
                line_nodes.push(Rc::new(RefCell::new(Node {
                    coordinates: (x,y),
                    cost: u32::MAX as u64,
                    previous: None,
                    is_end: false,
                    reachable_nodes: Vec::new(),
                })));
            }
        }
        graph.push(line_nodes);
    }

    for (x, line) in height_map.iter().enumerate() {
        for (y, node) in line.iter().enumerate() {
            let mut current_node = graph[x][y].borrow_mut();

            if y != 0 {
                if height_map[x][y-1] as u8 <= *node as u8 + 1 {
                    current_node.reachable_nodes.push(graph[x][y-1].clone());
                }
            }
            if x != 0 {
                if height_map[x-1][y] as u8 <= *node as u8 + 1 {
                    current_node.reachable_nodes.push(graph[x-1][y].clone());
                }
            }
            if y != height_map[0].len()-1 {
                if height_map[x][y+1] as u8 <= *node as u8 + 1 {
                    current_node.reachable_nodes.push(graph[x][y+1].clone());
                }
            }
            if x != height_map.len()-1 {
                if height_map[x+1][y] as u8 <= *node as u8 + 1 {
                    current_node.reachable_nodes.push(graph[x+1][y].clone());
                }
            }
            
         
        }
    }

    let mut q = graph.iter().flatten().cloned().collect::<Vec<Rc<RefCell<Node>>>>();

    while q.len() != 0 {
        q.sort_by_key(|n| n.borrow().cost);
        let u = q.swap_remove(0);
        for neighbor in u.borrow().reachable_nodes.iter() {
            if q.contains(neighbor) {
                if u.borrow().cost + 1 < neighbor.borrow().cost {
                    neighbor.borrow_mut().cost = u.borrow().cost + 1;
                    neighbor.borrow_mut().previous = Some(u.clone());
                }
            }
        }
    }

    let mut counter = 0;
    let mut flat_graph = graph.iter().flatten();
    let mut current_node = flat_graph.find(|n| n.borrow().is_end).unwrap().clone();
    while let Some(prev) = current_node.clone().borrow().previous.clone() {
        let c = prev.borrow().coordinates;
        let x = c.0;
        let y = c.1;
        
        counter += 1;
        current_node = prev;

        if height_map[x][y] == 'a' {
            break;
        }
    }

    Box::new(counter)
}
