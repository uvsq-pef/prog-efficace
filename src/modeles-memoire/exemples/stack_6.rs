/// Une pile définie récursivement.
#[derive(Debug,PartialEq)]
pub struct Stack {
    head: Option<Box<Node>>,
}

#[derive(Debug,PartialEq)]
struct Node {
    elem: i32,
    next: Option<Box<Node>>,
}

pub fn empty_stack() -> Stack {
    Stack {head: None}
}

pub fn top(s: &Stack) -> Option<i32> {
    match &s.head {
        None => None,
        Some(n) => Some(n.elem),
    }
}

/// Empile un élément
pub fn push(s: &mut Stack, elem: i32) {
    let next = std::mem::take(&mut s.head); //<1>
    let newtop = Box::new(Node{elem, next});
    s.head = Some(newtop);
}

// tag::stackop[]
/// Dépile un élément
/// 
/// Problème de compilation
/// error[E0507]: cannot move out of `n.next` which is behind a shared reference
pub fn pop(s: &mut Stack) -> Option<i32> {
    if let Some(n) = &s.head { //<1>
        let value = n.elem;
        s.head = n.next; //<2>
                         // move occurs because `n.next` has type `Option<Box<Node>>`, which does not implement the `Copy` trait
                         // help: consider borrowing the `Option`'s content: `n.next.as_ref()`
        return Some(value);
    }
    None
}
// end::stackop[]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_an_empty_stack() {
        let s = empty_stack();
        assert_eq!(Stack{head: None}, s);
    }

    #[test]
    fn should_return_the_top_of_an_empty_stack() {
        let s = empty_stack();
        assert_eq!(None, top(&s));
    }

    #[test]
    fn should_return_the_top_of_the_stack() {
        let s = Stack {head: Some(Box::new(Node{elem: 1, next: None}))};
        assert_eq!(1, top(&s).unwrap());
    }

    #[test]
    fn should_push_an_element_on_the_stack() {
        let mut s = empty_stack();
        push(&mut s, 1);
        assert_eq!(1, top(&s).unwrap());
    }

    // tag::stacktst[]
    #[test]
    fn should_pop_the_empty_stack() {
        let mut s = empty_stack();
        let top_of_stack = pop(&mut s);
        assert_eq!(None, top_of_stack);
        assert_eq!(None, top(&s));
    }

    #[test]
    fn should_pop_an_element_from_the_stack() {
        let mut s = Stack {head: Some(Box::new(Node{elem: 1, next: None}))};
        let top_of_stack = pop(&mut s);
        assert_eq!(1, top_of_stack.unwrap());
        assert_eq!(None, top(&s));
    }
    // end::stacktst[]
}
