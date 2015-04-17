use ID;
use std::mem::{forget, transmute};
use std::raw::TraitObject;
use std::any::Any;
use std::default::Default;
use std::collections::BTreeMap;

/// this struct is responsible for managing all the state of the gui
#[derive(Debug)]
pub struct State{
    pub state: BTreeMap<ID, Box<Any>>,
    pub focused: ID,
    pub hovered: ID,
	//TODO: add heshset to keep track of widget types
}

impl State{
    pub fn new() -> State{
        Default::default()
    }
	/// get a mutable reference to a component state
	/// if state is not already set, it will be inserted and the default will be returned
	pub fn get_mut<T>(&mut self, id: &ID) -> &mut T where T: Any + Default + 'static{
		if !self.state.contains_key(id){
			let d: Box<T> = Box::new(Default::default());
			self.state.insert(*id, d);
		}
		unsafe{ self.state.get_mut(id).unwrap().downcast_mut_unchecked::<T>() }
	}

	/// get a immutable ref to the state of the component, if the state is not jet set
	/// the default value will be returned
	pub fn get<T>(&mut self, id: &ID) -> &T where T: Any + Default + 'static{
		if !self.state.contains_key(id){
			let d: Box<T> = Box::new(Default::default());
			self.state.insert(*id, d);
		}
		unsafe{ self.state.get_mut(id).unwrap().downcast_ref_unchecked::<T>() }
	}

	/// remove one state
	pub fn remove(&mut self, id: &ID){
		self.state.remove(id);
	}
	/// remove state and child states
	pub fn remove_c(&mut self, id: &ID){
		use std::collections::Bound::Included;
		let mut keys = vec![];

		for (&k, _) in self.state.range(Included(id),Included(&max_id(id))){
			//println!("delete: {:?}", k);
			keys.push(k);
		}
		for k in keys.iter(){
			self.state.remove(k);
		}
	}
}

impl Default for State{
	fn default() -> State{
		State{
            focused: [0; 12],
            hovered: [0; 12],
            state: BTreeMap::new(),
        }
	}
}

fn max_id(id: &ID) -> ID{
	let mut e: ID = [65535;12];
	let mut i = 0;
	for &v in id.iter(){
		if v == 0{
			break;
		}
		e[i] = v;
		//println!("i:{}, v: {}",i,v);
		i+=1;
	};
	e
}
#[test]
fn test_max_id(){
	assert_eq!(max_id(&[1,0,0,0,0,0,0,0,0,0,0,0]), [1,65535,65535,65535,65535,65535,65535,65535,65535,65535,65535,65535]);
}

#[test]
fn test_state(){
	let mut s = State::new();
	{let _: &u8 = s.get(&[1;12]);                   }
	{let _: &u8 = s.get(&[1,0,0,0,0,0,0,0,0,0,0,0]);}
	{let _: &u8 = s.get(&[1,1,0,0,0,0,0,0,0,0,0,0]);}
	{let _: &u8 = s.get(&[1,1,1,0,0,0,0,0,0,0,0,0]);}
	{let _: &u8 = s.get(&[1,1,2,0,0,0,0,0,0,0,0,0]);}
	{let _: &u8 = s.get(&[1,3,1,0,0,0,0,0,0,0,0,0]);}
	{let _: &u8 = s.get(&[1,1,1,1,0,0,0,0,0,0,0,0]);}
	{let _: &u8 = s.get(&[1,2,1,0,0,0,0,0,0,0,0,0]);}
	{let _: &u8 = s.get(&[1,1,1,2,0,0,0,0,0,0,0,0]);}

	assert_eq!(s.state.len(), 9);
	s.remove(&[1,1,1,2,0,0,0,0,0,0,0,0]);
	assert_eq!(s.state.len(), 8);
	s.remove_c(&[1,1,0,0,0,0,0,0,0,0,0,0]);
	assert_eq!(s.state.len(), 3);
}

/// An extension of `AnyRefExt` allowing unchecked downcasting of trait objects to `&T`.
pub trait UncheckedAnyRefExt<'a> {
    /// Returns a reference to the boxed value, assuming that it is of type `T`. This should only be
    /// called if you are ABSOLUTELY CERTAIN of `T` as you will get really wacky output if it’s not.
    unsafe fn downcast_ref_unchecked<T: 'static>(self) -> &'a T;
}

impl<'a> UncheckedAnyRefExt<'a> for &'a Any {
    #[inline]
    unsafe fn downcast_ref_unchecked<T: 'static>(self) -> &'a T {
        // Get the raw representation of the trait object
        let to: TraitObject = transmute(self);

        // Extract the data pointer
        transmute(to.data)
    }
}

/// An extension of `AnyMutRefExt` allowing unchecked downcasting of trait objects to `&mut T`.
pub trait UncheckedAnyMutRefExt<'a> {
    /// Returns a reference to the boxed value, assuming that it is of type `T`. This should only be
    /// called if you are ABSOLUTELY CERTAIN of `T` as you will get really wacky output if it’s not.
    unsafe fn downcast_mut_unchecked<T: 'static>(self) -> &'a mut T;
}

impl<'a> UncheckedAnyMutRefExt<'a> for &'a mut Any {
    #[inline]
    unsafe fn downcast_mut_unchecked<T: 'static>(self) -> &'a mut T {
        // Get the raw representation of the trait object
        let to: TraitObject = transmute(self);

        // Extract the data pointer
        transmute(to.data)
    }
}

/// An extension of `BoxAny` allowing unchecked downcasting of trait objects to `Box<T>`.
pub trait UncheckedBoxAny {
    /// Returns the boxed value, assuming that it is of type `T`. This should only be called if you
    /// are ABSOLUTELY CERTAIN of `T` as you will get really wacky output if it’s not.
    unsafe fn downcast_unchecked<T: 'static>(self) -> Box<T>;
}

impl UncheckedBoxAny for Box<Any + 'static> {
    #[inline]
    unsafe fn downcast_unchecked<T: 'static>(self) -> Box<T> {
        // Get the raw representation of the trait object
        let to: TraitObject = *transmute::<&Box<Any>, &TraitObject>(&self);

        // Prevent destructor on self being run
        forget(self);

        // Extract the data pointer
        transmute(to.data)
    }
}

//TODO: make test for state
/*
#[test]
fn test_state(){
    let mut w = Window::new("",10,10);
    w.set_state([1,0,0,0,0,0,0,0,0,0,0,0], Box::new(34u8));
    assert!(w.find_state([1,0,0,0,0,0,0,0,0,0,0,0]) == Some(&34u8));

    w.set_state([1,1,2,1,1,1,1,1,4,0,0,0], Box::new(34u64));
    assert!(w.find_state([1,1,2,1,1,1,1,1,4,0,0,0]) == Some(&34u64));
}*/
