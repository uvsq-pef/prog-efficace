use shapes::movable::Movable;
use shapes::point::Point;
use shapes::circle::Circle;

#[test]
fn instanciate_a_point() {
    let point = Point::from(3.0, 5.0);
    assert_eq!(point.abscissa(), 3.0);
    assert_eq!(point.ordinate(), 5.0);
}

#[test]
fn instanciate_points_and_circles() {
    //tag::circles_and_points[]
    let point1 = Point::from(1.0, 2.0);
    assert_eq!(point1.abscissa(), 1.0);
    assert_eq!(point1.ordinate(), 2.0);

    let point2 = Point::from(1.0, 2.0);
    assert_eq!(point2.abscissa(), 1.0);
    assert_eq!(point2.ordinate(), 2.0);

    let circle1 = Circle::from(point1, 2.0); // <1>
    assert_eq!(circle1.center(), &Point::from(1.0, 2.0));
    assert_eq!(circle1.radius(), 2.0);
    //end::circles_and_points[]
}

#[test]
fn move_a_circle() {
    //tag::move_a_circle[]
    let p = Point::from(6.0, 9.0);
    let mut c = Circle::from(p, 5.0);

    c.move_by(4.0, 1.0);
    assert_eq!(c.center(), &Point::from(10.0, 10.0));
    assert_eq!(c.radius(), 5.0);
    //end::move_a_circle[]
}

#[test]
fn circle_as_movable() {
    //tag::circle_as_movable[]
    let c = Circle::from(Point::from(1.0, 1.0), 1.0);
    let c_as_movable_ref : &dyn Movable = &c;
    assert!(c_as_movable_ref.is_at(1.0, 1.0)); //<1>

    let circle_as_movable_box : Box<dyn Movable> =
        Box::new(
            Circle::from(Point::from(1.0, 1.0), 1.0)
        );
    assert!(circle_as_movable_box.is_at(1.0, 1.0)); //<1>
    //end::circle_as_movable[]
}

#[test]
fn move_shapes() {
    //tag::move_shapes[]
    let mut shapes : Vec<Box<dyn Movable>> = vec![
        Box::new(Point::from(1.0, 2.0)),
        Box::new(Circle::from(Point::from(3.0, 4.0), 5.0)),
        ];
    for shape in shapes.iter_mut() {
        shape.move_by(1.0, 2.0);
    }
    assert!(shapes[0].is_at(2.0, 4.0));
    assert!(shapes[1].is_at(4.0, 6.0));
    //end::move_shapes[]
}
