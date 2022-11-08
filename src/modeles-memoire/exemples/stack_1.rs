// tag::stack[]
/// Une pile définie récursivement.
/// 
/// Problème
/// Le premier élément se trouve sur la pile, les autres sur le tas
/// => les manipulations seront plus complexes
/// 
#[derive(Debug,PartialEq)]
pub enum Stack {
    Empty,
    Elem(i32, Box<Stack>)
}
// end::stack[]

// tag::stackop[]
pub fn empty_stack() -> Stack {
    Stack::Empty
}

pub fn top(s: &Stack) -> Option<i32> {
    match s {
        Stack::Empty => None,
        Stack::Elem(n, _) => Some(*n),
    }
}
// end::stackop[]

#[cfg(test)]
mod tests {
    use super::*;

    // tag::stacktst[]
    #[test]
    fn should_create_an_empty_stack() {
        let s = empty_stack();
        assert_eq!(Stack::Empty, s);
    }

    #[test]
    fn should_return_the_top_of_an_empty_stack() {
        let s = empty_stack();
        assert_eq!(Option::None, top(&s));
    }

    #[test]
    fn should_return_the_top_of_the_stack() {
        let s = Stack::Elem(1, Box::new(Stack::Empty));
        assert_eq!(1, top(&s).unwrap());
    }
    // end::stacktst[]
}
