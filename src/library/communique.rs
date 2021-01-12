pub struct Communique<T> {
    id: Option<usize>,
    load: T,
}
impl<T> Communique<T> {
    pub fn new(id:usize, data:T) -> Communique<T> {
        Communique {
            id: Some(id),
            load: data,
        }
    }
    pub fn new_no_id(data:T) -> Communique<T> {
        Communique {
            id: None,
            load: data,
        }
    }
    pub fn id(&self) -> Option<usize> {
        self.id
    }
    pub fn open(&self) -> &T {
        &self.load
    }
}
impl<T: std::fmt::Display> Communique<T> {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("id:{:?} load:{}", self.id, self.load))
    }
}
impl<T: std::fmt::Display> std::fmt::Display for Communique<T> {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result { self.fmt(f) }
}
impl<T: std::fmt::Display> std::fmt::Debug for Communique<T> {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result { self.fmt(f) }
}