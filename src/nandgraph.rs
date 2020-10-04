
#[derive(Debug)]
pub struct Graph {
    adj : Vec<Vec<(bool, usize)>>
}

impl Graph {
    pub fn add_edge(&mut self, u: usize, v:usize, p:bool) {
        self.adj[u].push((p, v));
    }
    pub fn new_with_n(n :usize) -> Self {
        Graph { adj : vec![Vec::new(); n] }
    }
    pub fn new() -> Self {
        Graph { adj : Vec::new() }
    }

    pub fn create_vertex(&mut self) -> usize {
        let r = self.adj.len();
        self.adj.push(Vec::new());
        r
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn it_works() {
        let mut g = Graph::new();
        let u = g.create_vertex();
        let v = g.create_vertex();
        g.add_edge(u, v, true);
    }
}

