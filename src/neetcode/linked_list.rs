#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut current) = (None, head);

    while let Some(mut node) = current {
        current = node.next;
        node.next = prev;
        prev = Some(node);
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list() {
        // let mut head = Box::new(ListNode::new(1));
        // let mut node1 = Box::new(ListNode::new(5));
        // head.next = Some(node1);
        // let node2 = Box::new(ListNode::new(10));
        // node1.next = Some(node2);
        //
        // let reversed = reverse_list(Some(head));
        //
        // assert_eq!(reversed.unwrap().val, 10);
    }
}
