use std::any::{Any, AnyRefExt};

///Managing the state of component requiering it

pub struct Container{
    pub id: uint,//id part of the node
    pub childs: Vec<Container>,//child nodes
    pub state: Option<Box<Any + 'static>>,//the state of the component
}

impl Container{
    pub fn new(id: uint) -> Container{
        Container{
            id: id,
            childs: Vec::new(),
            state: None
        }
    }
    ///get the state of a component
    #[inline(always)]
    pub fn get<T: Any>(&self) -> Option<&T>{
        match self.state{
            Some(ref e) => e.downcast_ref::<T>(),
            None => None
        }
    }
    ///set the state of a component
    #[inline(always)]
    pub fn set(&mut self, v: &'static Any){
        //let v = v as &Any;
        self.state = Some(box v as Box<Any+'static>);
    }

    pub fn find(&self, v: Vec<uint>) -> Option<()>{
        unimplemented!()
        None
    }
}
