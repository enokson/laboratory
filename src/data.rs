use std::ops::Deref;
use std::sync::Arc;
#[derive(Debug)]
pub struct Data<T: ?Sized>(Arc<Box<T>>);

impl<T: ?Sized> Data<T> {
    /// Create new `Data` instance.
    pub fn new(state: Box<T>) -> Data<T> {
        Data(Arc::new(state))
    }

    /// Get reference to inner app data.
    pub fn get_ref(&self) -> &T {
        self.0.as_ref()
    }

    /// Convert to the internal Arc<T>
    pub fn into_inner(self) -> Arc<Box<T>> {
        self.0.clone()
    }
}

impl<T: ?Sized> Deref for Data<T> {
    type Target = Arc<Box<T>>;

    fn deref(&self) -> &Arc<Box<T>> {
        &self.0
    }
}

impl<T: ?Sized> Clone for Data<T> {
    fn clone(&self) -> Data<T> {
        Data(self.0.clone())
    }
}

impl<T: ?Sized> From<Arc<Box<T>>> for Data<T> {
    fn from(arc: Arc<Box<T>>) -> Self {
        Data(arc)
    }
}

pub trait DataTrait<T: ?Sized>: Deref + Clone + From<Arc<Box<T>>> {

}

impl<T: ?Sized> DataTrait<T> for Data<T> {

}