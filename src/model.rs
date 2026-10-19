#[derive(Debug)]
pub struct GraphAdj
{
    pub data: Vec<Vec<usize>>
}

impl GraphAdj 
{
    pub fn new() -> Self {
        GraphAdj {
            data: vec![]
        }
    }

    pub fn setup(&mut self, size: usize) {
        self.data.resize_with(size, Vec::new);
    }

    pub fn ensure_node(&mut self, node: usize) {
        if node <= self.data.len() { return; }
        
        self.data.resize_with(node + 1, Vec::new);
    }

    pub fn add_node(&mut self, from: usize, to: usize)
    {
        self.data[from].push(to);
    }
}