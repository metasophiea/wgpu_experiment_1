pub struct Dimensions<T> {
    width: T,
    height: T,
}
impl<T> Dimensions<T> {
    pub fn new(width:T, height:T) -> Dimensions<T> {
        Dimensions {
            width: width,
            height: height,
        }
    }
}
impl<T> Dimensions<T> {
    pub fn get_width(&self) -> &T { &self.width }
    pub fn set_width(&mut self, new:T) { self.width = new; }
    pub fn get_height(&self) -> &T { &self.height }
    pub fn set_height(&mut self, new:T) { self.height = new; }
}
impl<T: std::fmt::Display> Dimensions<T> {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{{width:{},height:{}}}",
            self.width,
            self.height,
        )
    }
}
impl<T: std::fmt::Display> std::fmt::Display for Dimensions<T> {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result { self.fmt(f) }
}
impl<T: std::fmt::Display> std::fmt::Debug for Dimensions<T> {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result { self.fmt(f) }
}