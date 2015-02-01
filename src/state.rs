use Window;
use std::any::TypeId;
use std::mem::forget;
use std::collections::hash_map;
use std::hash::{Hasher, Writer};
use std::collections::hash_state::HashState;
use std::mem::transmute;
use std::raw::TraitObject;
use std::any::Any;

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

#[test]
fn test_state(){
    let mut w = Window::new("",10,10);
    w.set_state([1,0,0,0,0,0,0,0,0,0,0,0], Box::new(34u8));
    assert!(w.find_state([1,0,0,0,0,0,0,0,0,0,0,0]) == Some(&34u8));

    w.set_state([1,1,2,1,1,1,1,1,4,0,0,0], Box::new(34u64));
    assert!(w.find_state([1,1,2,1,1,1,1,1,4,0,0,0]) == Some(&34u64));
}
