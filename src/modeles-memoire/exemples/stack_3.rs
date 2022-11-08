// tag::stack[]
/// Une pile définie récursivement.
#[derive(Debug,PartialEq)]
pub struct Stack {
    head: Link,
}

#[derive(Debug,PartialEq)]
enum Link {
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
/// Problème de compilation :
/// error[E0507]: cannot move out of `*next` which is behind a shared reference
/// 
pub fn push(s: &mut Stack, elem: i32) {
    let next = &mut s.head;
    let newtop = Box::new(Node{elem, next: *next});  //<1>
                                                     // move occurs because `*next` has type `Link`, which does not implement the `Copy` trait
    *s = Stack{head: Link::Elem(newtop)};
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

    // tag::stacktst[]
    #[test]
    fn should_push_an_element_on_the_stack() {
        let mut s = empty_stack();
        push(&mut s, 1);
        assert_eq!(1, top(&s).unwrap());
    }
    // end::stacktst[]
}
