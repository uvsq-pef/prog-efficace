// tag::stack[]
/// Une pile définie récursivement.
/// 
/// Problème à la compilation
/// error[E0072]: recursive type `Stack` has infinite size
/// 
pub enum Stack { //<1>
    Empty,
    Elem(i32, Stack) // recursive without indirection
                     // help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `Stack` representable
                     //     Elem(i32, Box<Stack>)
}
// end::stack[]

pub fn empty_stack() -> Stack {
    Stack::Empty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_an_empty_stack() {
        let s = empty_stack();
        assert_eq!(Stack::Empty, s);
    }
}
