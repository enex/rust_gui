pub struct Container{
    id: uint,//id part of the node
    childs: Vec<Container>,//child nodes
    state: Option<Box<Any>>,//the state of the component
}

impl Container{
    pub fn new(id: uint) -> Container{
        Container{
            id: id,
            childs: Vec::new(),
            state: None
        }
    }
}
