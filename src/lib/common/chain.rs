#[derive(Debug)]
pub enum Element<T> {
    Data(T),
    Link(Link<T>)
}

#[derive(Debug)]
pub struct Link<T> {
    pub prev: std::boxed::Box<Element<T>>,
    pub current: T
}

