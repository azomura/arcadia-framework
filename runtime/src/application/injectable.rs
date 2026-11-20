use super::Container;

pub trait Injectable: Sized {
    fn create(container: &Container) -> Self;
}
