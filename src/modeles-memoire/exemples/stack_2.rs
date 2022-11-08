// tag::stack[]
/// Une pile définie récursivement.
/// 
/// Problème de compilation
/// error[E0446]: private type `Node` in public interface
/// 
#[derive(Debug,PartialEq)]
pub enum Stack {
    Empty,
    Elem(Box<Node>) // can't leak private type
                    // struct Node {
                    // ----------- `Node` declared as private

}

#[derive(Debug,PartialEq)]
struct Node { //<1>
    elem: i32,
    next: Stack,
}
// end::stack[]

pub fn empty_stack() -> Stack {
    Stack::Empty
}

pub fn top(s: &Stack) -> Option<i32> {
    match s {
        Stack::Empty => None,
        Stack::Elem(n) => Some(n.elem),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let s = Stack::Elem(Box::new(Node{elem: 1, next: Stack::Empty}));
        assert_eq!(1, top(&s).unwrap());
    }
}
