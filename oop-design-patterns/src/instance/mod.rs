use std::any::{Any, TypeId};

pub trait InstanceOf
where
    Self: Any,
{
    fn instance_of<U: ?Sized + Any>(&self) -> bool {
        TypeId::of::<Self>() == TypeId::of::<U>()
    }
}


impl<T: ?Sized + Any> InstanceOf for T {}