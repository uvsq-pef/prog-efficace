use crate::movable::Movable;

/// Définit une structure pour un point en 2D.
/// # Examples
/// 
/// ```
/// use shapes::point::Point;
/// let point = Point::from(6.0, 9.0);
/// 
/// assert_eq!(point.abscissa(), 6.0);
/// assert_eq!(point.ordinate(), 9.0);
/// ```
/// 
#[derive(Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    //tag::point_from[]
    /// Initialise un point à partir de coordonnées cartésiennes.
    pub fn from(x: f64, y: f64) -> Point {
        Point {
            x,
            y,
        }
    }
    //end::point_from[]

    /// Retourne l'abscisse du point.
    pub fn abscissa(&self) -> f64 {
        self.x
    }

    /// Retourne l'ordonnée du point.
    pub fn ordinate(&self) -> f64 {
        self.y
    }
}

//tag::point_movable_impl[]
impl Movable for Point {
    /// Déplace le point.
    /// # Examples
    /// 
    /// ```
    /// use shapes::movable::Movable;
    /// use shapes::point::Point;
    /// let mut point = Point::from(6.0, 9.0);
    /// 
    /// point.move_by(4.0, 1.0);
    /// assert_eq!(point.abscissa(), 10.0);
    /// assert_eq!(point.ordinate(), 10.0);
    /// ```
    fn move_by(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    /// Retourne true si le point est à la position en paramètre.
    fn is_at(&self, x: f64, y: f64) -> bool {
        self.x == x && self.y == y
    }
}
//end::point_movable_impl[]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instanciate_a_point() {
        let point = Point {
            x: 1.0,
            y: 2.0,
        };
        assert_eq!(point.abscissa(), 1.0);
        assert_eq!(point.ordinate(), 2.0);
    }

    #[test]
    fn move_a_point() {
        let mut point = Point {
            x: 1.0,
            y: 2.0,
        };
        point.move_by(1.0, 2.0);
        assert_eq!(point.abscissa(), 2.0);
        assert_eq!(point.ordinate(), 4.0);
    }
}
