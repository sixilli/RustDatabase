pub struct BPlusTree<'a>  {
    root: Option<Box<Node<'a>>>,
    first_leaf: Option<Node<'a>>,
    m: u8,
}

struct Node<'a> {
    parent: Option<Box<Node<'a>>>,
    degree: u8,
    max_degree: u8,
    min_degree: u8,
    keys: Vec<u8>,
    children: Option<Vec<&'a mut LeafNode>>,
}

struct LeafNode {
    dict: Vec<Dictionary>
}

struct BTReeError {
    details: String
}

impl BTReeError {
    fn new(msg: &str) -> BTReeError {
        BTReeError{details: msg.to_string()}
    }
}

// Will hopefully be able to make values generic
struct Dictionary {
    key: u64,
    value: u64,
}

pub fn build_btree<'a>() -> BPlusTree<'a> {
    BPlusTree {
        root: None,
        first_leaf: None,
        m: 3,
    }
}

impl <'a>BPlusTree<'a> {
    fn binary_search(dict_pair: Vec<Dictionary>, n_pairs: u64, t: u64) -> Option<usize> {
        let key_1: u64 = dict_pair[0].key;
        let key_2: u64 = dict_pair[1].key;

        let mut comparison: i32;

        if key_1 < key_2 {
            comparison = 1;
        } else if key_1 == key_2 {
            comparison = 0
        } else {
            comparison = -1
        }

        dict_pair.iter().position(|x| x.key == t)
    }

    fn insert(&mut self, x: u64) {
        self.root = match self.root {
            None => Some(Box::new(Node {
                parent: None,
                degree: 0,
                max_degree: 2,
                min_degree: 3,
                keys: vec![1, 2],
                children: None,
            })),

            _ => {
                println!("insert - TODO");
                Some(Box::new(Node {
                    parent: None,
                    degree: 0,
                    max_degree: 2,
                    min_degree: 3,
                    keys: vec![1, 2],
                    children: None,
                }))
            }
        }
    }

}