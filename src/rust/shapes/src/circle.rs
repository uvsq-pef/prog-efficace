use crate::movable::Movable;
use crate::point::Point;

pub struct Circle {
    center: Point,
    radius: f64,
}

/// Définit une structure pour un cercle en 2D.
/// # Examples
/// 
/// ```
/// use shapes::point::Point;
/// use shapes::circle::Circle;
/// 
/// let c = Circle::from(Point::from(6.0, 9.0), 5.0);
/// 
/// assert_eq!(c.center(), &Point::from(6.0, 9.0));
/// assert_eq!(c.radius(), 5.0);
/// ```
/// 
impl Circle {
    /// Initialise un cercle à partir de son centre et de son rayon.
    pub fn from(center: Point, radius: f64) -> Circle {
        Circle {
            center,
            radius,
        }
    }

    /// Retourne le centre du cercle.
    pub fn center(&self) -> &Point {
        &self.center
    }

    //tag::circle_radius[]
    /// Retourne le rayon du cercle.
    pub fn radius(&self) -> f64 {
        self.radius
    }
    //end::circle_radius[]
}

impl Movable for Circle {
    /// Déplace le cercle.
    /// # Examples
    /// 
    /// ```
    /// use shapes::movable::Movable;
    /// use shapes::point::Point;
    /// use shapes::circle::Circle;
    /// let mut p = Point::from(6.0, 9.0);
    /// let mut c = Circle::from(p, 5.0);
    /// 
    /// c.move_by(4.0, 1.0);
    /// assert_eq!(c.center(), &Point::from(10.0, 10.0));
    /// assert_eq!(c.radius(), 5.0);
    /// ```
    fn move_by(&mut self, dx: f64, dy: f64) {
        self.center.move_by(dx, dy);
    }

    /// Retourne true si le centre du cercle est à la position en paramètre.
    fn is_at(&self, x: f64, y: f64) -> bool {
        self.center.abscissa() == x && self.center.ordinate() == y
    }
}
