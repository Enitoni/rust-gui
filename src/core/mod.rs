mod component;
mod element;


struct Node {
    element: Box<dyn ErasedComponent>,
    
}


struct Tree {
    nodes: Vec<Node>,
}