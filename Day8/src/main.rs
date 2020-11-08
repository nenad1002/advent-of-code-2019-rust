use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn is_same_tree(left: Option<&Rc<RefCell<TreeNode>>>, right: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    if let (Some(p), Some(q)) = (&left, &right) {
        if p.borrow().val != q.borrow().val {
            return false;
        }
        return is_same_tree(p.borrow().left.as_ref(), q.borrow().right.as_ref()) &&
        is_same_tree(q.borrow().left.as_ref(), p.borrow().right.as_ref());
    } else if left.is_none() && right.is_none() {
        return true;
    }
    false
}

pub fn is_sims(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(root) = root {
        return is_same_tree(root.borrow().left.as_ref(), root.borrow().right.as_ref());
    }
    true
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.clone().unwrap().borrow_mut().left = left;
    root.clone().unwrap().borrow_mut().right = right;
    println!("{}", is_sims(root));
}
