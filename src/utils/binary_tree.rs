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

pub fn make_binary_tree(mut nodes: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if nodes.is_empty() || nodes[0] == None {
        return None;
    }

    let mut q :Vec<Rc<RefCell<TreeNode>>> = vec![];
    let root = Rc::new(RefCell::new(TreeNode::new(nodes[0].unwrap())));
    q.push(Rc::clone(&root));
    nodes = nodes.split_off(1);

    while nodes.len() >= 2 {
        let node = Rc::clone(&q[0]);
        q = q.split_off(1);

        if nodes[0] != None {
            node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(nodes[0].unwrap()))));
        }

        if nodes[1] != None {
            node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(nodes[1].unwrap()))))
        }

        nodes = nodes.split_off(2);
    }

    if nodes.len() > 0 {
        if nodes[0] != None {
            q[0].borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(nodes[0].unwrap()))));
        }
    }

    Some(root)
}

#[test]
fn test_make_binary_tree() {
    let root = make_binary_tree(vec![Some(1), Some(2), Some(3)]);
    println!("{:?}", root);
}