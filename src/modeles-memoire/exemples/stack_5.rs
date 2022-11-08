// tag::stack[]
/// Une pile définie récursivement.
#[derive(Debug,PartialEq)]
pub struct Stack {
    head: Option<Box<Node>>, //<1>
}

#[derive(Debug,PartialEq)]
struct Node {
    elem: i32,
    next: Option<Box<Node>>,
}
// end::stack[]

pub fn empty_stack() -> Stack {
    Stack {head: None}
}

pub fn top(s: &Stack) -> Option<i32> {
    match &s.head {
        None => None,
        Some(n) => Some(n.elem),
    }
}

// tag::stackop[]
/// Empile un élément
pub fn push(s: &mut Stack, elem: i32) {
    let next = std::mem::take(&mut s.head); //<1>
                                            // OU plus simple let next = s.head.take();
    let newtop = Box::new(Node{elem, next});
    s.head = Some(newtop);
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
}
