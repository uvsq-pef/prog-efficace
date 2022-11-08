// tag::stack[]
/// Une pile définie récursivement.
#[derive(Debug,PartialEq)]
pub struct Stack {
    head: Link,
}

/// Le type Link est identique à un type de la bibliothèque standard
/// Lequel ?
/// 
#[derive(Debug,PartialEq)]
enum Link { //<1>
    Empty,
    Elem(Box<Node>)
}

#[derive(Debug,PartialEq)]
struct Node {
    elem: i32,
    next: Link,
}
// end::stack[]

pub fn empty_stack() -> Stack {
    Stack {head: Link::Empty}
}

pub fn top(s: &Stack) -> Option<i32> {
    match &s.head {
        Link::Empty => None,
        Link::Elem(n) => Some(n.elem),
    }
}

// tag::stackop[]
/// Empile un élément
pub fn push(s: &mut Stack, elem: i32) {
    let next = std::mem::replace(&mut s.head, Link::Empty); //<1>
    let newtop = Box::new(Node{elem, next});
    s.head = Link::Elem(newtop);
}
// end::stackop[]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_an_empty_stack() {
        let s = empty_stack();
        assert_eq!(Stack{head: Link::Empty}, s);
    }

    #[test]
    fn should_return_the_top_of_an_empty_stack() {
        let s = empty_stack();
        assert_eq!(Option::None, top(&s));
    }

    #[test]
    fn should_return_the_top_of_the_stack() {
        let s = Stack {head: Link::Elem(Box::new(Node{elem: 1, next: Link::Empty}))};
        assert_eq!(1, top(&s).unwrap());
    }

    #[test]
    fn should_push_an_element_on_the_stack() {
        let mut s = empty_stack();
        push(&mut s, 1);
        assert_eq!(1, top(&s).unwrap());
    }
}
