#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode {
            val,
            next,
        }
    }

    fn stringify(&self) -> String {
        match self.next {
            Some(ref n) => format!("{} -> {}", self.val, n.stringify()),
            None => format!("nil"),
        }
    }
}

pub fn make_list(nodes: Vec<i32>) -> Option<Box<ListNode>> {
    let mut curr: Option<Box<ListNode>> = None;

    for i in (0..nodes.len()).rev() {
        if curr == None {
            curr = Some(Box::new(ListNode::new(nodes[i], None)));
        } else {
            curr = Some(Box::new(ListNode::new(nodes[i], curr)));
        }
    }

    curr
}

pub fn print_list(list: &Option<Box<ListNode>>) -> String {
    match list {
        Some(n) => {
            match &n.next {
                Some(nn) => format!("{} -> {}", n.val, nn.stringify()),
                None => format!("{} -> nil", n.val),
            }
        }
        None => format!("nil"),
    }
}

#[test]
fn test_make_list() {
    let list = make_list(vec![1, 2, 3, 4, 5, 6]);
    println!("{}", print_list(&list));
}

