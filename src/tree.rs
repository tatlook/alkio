
pub struct NamedNode<T: ?Sized> {
    name: String,
    thing: Box<T>,
}

impl<T: Sized> NamedNode<T> {
        
    pub fn new(name: String, thing: T) -> Self {
        NamedNode { name: name, thing: Box::<T>::new(thing) }
    }
}

impl<T: ?Sized> NamedNode<T> {


    pub fn get_box(&self) -> &Box<T>{
        &self.thing
    }
}
