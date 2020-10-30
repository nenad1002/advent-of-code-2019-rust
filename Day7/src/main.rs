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

pub fn find_m_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = head.clone();
    let mut fast = head.clone();

    while fast.is_some() && fast.clone().unwrap().next.is_some() {
        slow = slow.unwrap().next;
        fast = fast.unwrap().next.unwrap().next;
    }

    return slow;
}
fn main() {
    let third = Some(Box::new(ListNode::new(7)));
    let second = Some(Box::new(ListNode {
        val: 6,
        next: third,
    }));
    let first = Some(Box::new(ListNode {
        val: 6,
        next: second,
    }));

    println!("{}", find_m_node(first).unwrap().val);
}
