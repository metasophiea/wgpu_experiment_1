use std::fmt;

use super::intersect_result_types::{
    LineSide,
    PolySide,
};

use super::bounding_box::BoundingBox;
use super::line::Line;
use super::simple_polygon::SimplePolygon;
use super::complex_polygon::ComplexPolygon;




//struct
    #[derive(Copy, Clone)]
    pub struct Point {
        x: f32,
        y: f32,
    }
//new
    impl Point {
        pub fn new(x:f32, y:f32) -> Point {
            Point {
                x,
                y,
            }
        }
        pub fn new_optional(x:Option<f32>, y:Option<f32>) -> Point {
            Point::new(
                match x { Some(num) => num, None => 0.0 },
                match y { Some(num) => num, None => 0.0 },
            )
        }
    }
//getters
    impl Point {
        pub fn get_x(&self) -> f32 {
            self.x
        }
        pub fn get_ref_x(&self) -> &f32 {
            &self.x
        }
        pub fn get_y(&self) -> f32 {
            self.y
        }
        pub fn get_ref_y(&self) -> &f32 {
            &self.y
        }
        pub fn to_tuple(&self) -> (f32,f32) {
            (self.x,self.y)
        }
    }
//setters
    impl Point {
        pub fn set_x(&mut self, new:f32) {
            self.x = new;
        }
        pub fn set_y(&mut self, new:f32) {
            self.y = new;
        }
        pub fn set(&mut self, new_x:f32, new_y:f32) {
            self.x = new_x;
            self.y = new_y;
        }
        pub fn set_from_point_ref(&mut self, other_point:&Point) {
            self.x = other_point.x;
            self.y = other_point.y;
        }
    }
//printing
    impl Point {
        fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
            write!(
                f, "{{x:{},y:{}}}",
                self.x,
                self.y,
            )
        }
    }
    impl fmt::Display for Point {
        fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { self.fmt(f) }
    }
    impl fmt::Debug for Point {
        fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { self.fmt(f) }
    }
//comparison
    impl PartialEq for Point {
        fn eq(&self, other:&Self) -> bool {
            if 
                self.x.is_sign_positive() ^ other.x.is_sign_positive() ||
                self.y.is_sign_positive() ^ other.y.is_sign_positive()
            {
                return false;
            }

            let allow = 0.000_1; //f32
            // let allow = 0.000_000_000_000_1; //f64
            (self.x.abs() - other.x.abs()).abs() < allow &&
            (self.y.abs() - other.y.abs()).abs() < allow
        }
    }
//intersect
    impl Point {
        //point // can compare with point directly, see the comparison section above
        pub fn intersect_with_bounding_box(&self, other:&BoundingBox) -> bool {
            other.intersect_with_point(self)
        }
        pub fn intersect_with_line(&self, line:&Line) -> LineSide {
            line.intersect_with_point(self)
        }
        pub fn intersect_with_simple_polygon(&self, simple_polygon:&SimplePolygon) -> PolySide {
            simple_polygon.intersect_with_point(self)
        }
        pub fn intersect_with_complex_polygon(&self, complex_polygon:&ComplexPolygon) -> PolySide {
            complex_polygon.intersect_with_point(self)
        }
    }