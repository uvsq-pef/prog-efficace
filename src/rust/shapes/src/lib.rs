pub mod point;
pub mod circle;
pub mod movable;

#[cfg(test)]
mod tests {
    use crate::point::Point;

    #[test]
    fn instanciate_a_point() {
        let point = Point::from(6.0, 9.0);
        assert_eq!(point.abscissa(), 6.0);
        assert_eq!(point.ordinate(), 9.0);
    }
}
