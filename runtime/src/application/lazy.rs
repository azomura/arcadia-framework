use std::sync::Arc;

use super::Container;

pub struct Lazy<T> {
    container: Container,
    marker: std::marker::PhantomData<fn() -> T>,
}

impl<T> Clone for Lazy<T> {
    fn clone(&self) -> Self {
        Self { container: self.container.clone(), marker: std::marker::PhantomData }
    }
}

impl<T: Send + Sync + 'static> Lazy<T> {
    pub fn get(&self) -> Arc<T> {
        self.container.resolve::<T>()
    }
}

impl Container {
    pub fn lazy<T: Send + Sync + 'static>(&self) -> Lazy<T> {
        Lazy { container: self.clone(), marker: std::marker::PhantomData }
    }
}
