use std::fs::{File};
use std::collections::HashMap;
use std::ffi::CString;
use std::io::Read;
use std::ops::Deref;

pub struct HuffmanNode {
    value: u8,
    code: String,
    freq: i32,
    check: i32,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
    parent: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new(value:u8,freq:i32) -> HuffmanNode {
        HuffmanNode {
            value,
            freq,
            left: None,
            right: None,
            parent: None,
            check: 1,
            code: 0,
        }
    }
    fn join(mut self, mut to_be_joined: HuffmanNode) -> Result<HuffmanNode, &'static str> {
        let mut parent_node = HuffmanNode::new(
            to_be_joined.value.clone() + self.value.clone(),
            to_be_joined.freq.clone() + self.freq.clone());
        parent_node.left = Some(Box::new(self));
        parent_node.right = Some(Box::new(to_be_joined));
        Ok(parent_node)
    }
}

pub (crate) fn create_huffman_list(freq_vec: Vec<(u8, i32)>) -> Vec<HuffmanNode>{
    let mut huffman_list: Vec<HuffmanNode> = vec![];
    for (value,freq) in freq_vec {
        let huffman_node = HuffmanNode::new(value,freq);
        huffman_list.push(huffman_node);
    }
    return huffman_list;
}
pub (crate) fn create_huffman_root(mut huffman_list: Vec<HuffmanNode>) -> HuffmanNode{
    while huffman_list.len() > 1 {
        let node1 = huffman_list.pop().unwrap();
        let node2 = huffman_list.pop().unwrap();
        match node1.join(node2) {
            Ok(parent) => {
                // Handle the newly created parent node
                huffman_list.insert(0,parent);
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
    return huffman_list.pop().unwrap();
}
pub(crate) fn create_huffman_code(root: Option<Box<HuffmanNode>>){
    let mut stack = Vec::new();
    let mut current = root;
    let mut last_visited: Option<&HuffmanNode> = None;

    while !stack.is_empty() || current.is_some() {
        if let Some(mut node) = current {
            stack.push(node.as_mut()); // Push a mutable reference to the node.
            current = node.left.take(); // Move to the left child.
        } else if let Some(node) = stack.last() {
            let node = *node; // Get an immutable reference to the last node on the stack.
            if node.right.is_none() || last_visited == node.right.as_ref() {
                // If the right child is None or has been visited, process the node.
                println!("{}", node.value); // Replace this with your desired processing logic.
                last_visited = Some(node);
                stack.pop();
            } else {
                current = node.right.take(); // Move to the right child.
            }
        }
    }
}
pub(crate) fn create_huffman_tree(file_path: &str) -> HuffmanNode {
    let freq_vec = create_freq_for_ascii(file_path);
    let mut huffman_list = create_huffman_list(freq_vec);
    let root = create_huffman_root(huffman_list);
    return root;
}
pub(crate) fn create_freq_for_ascii(file_path: &str) -> Vec<(u8,i32)>{
    let mut contents = Vec::new();
    let mut file = File::open(file_path.clone()).expect("Unable to open file");
    file.read_to_end(&mut contents).expect("Unable to read file");
    let mut content_freq_vec: Vec<(u8,i32)> = Vec::new();
    let mut content_freq:HashMap<u8,i32> = HashMap::new();
    for content in contents {
        if content_freq.get(&content.clone()).is_some(){
            let mut val = content_freq.get(&content.clone()).unwrap().clone();
            val+=1;
            content_freq.insert(content.clone(), val);
        }else {
            content_freq.insert(content,1);
        }
    }
    for (key,value) in content_freq {
        let mut pair = (key.clone(),value.clone());
        content_freq_vec.push(pair);
    }
    content_freq_vec.sort_by(|(x,y), (z,s)| s.partial_cmp(y).unwrap());
    return content_freq_vec;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_huffman_tree() {
        // create_huffman_list()
    }
    #[test]
    fn create_huffman_list_test(){
        let mut freq_list :Vec<(u8,i32)> = Vec::from([(0x1,1),(0x2,2),(0x3,3),(0x4,4)]);
        let mut huffman_node_list = create_huffman_list(freq_list.clone());
        while(!huffman_node_list.is_empty()){
            let node = huffman_node_list.pop().unwrap();
            let (value,freq) = freq_list.pop().unwrap();
            println!("{}",freq);
            assert_eq!(node.freq,freq);
            assert_eq!(node.value,value);
        }
    }
    #[test]
    fn create_huffman_root_test(){
        let mut freq_list :Vec<(u8,i32)> = Vec::from([(0x4,4),(0x3,3),(0x2,2),(0x1,1)]);
        freq_list.sort_by(|(x,y), (z,s)| s.partial_cmp(y).unwrap());
        let mut huffman_node_list = create_huffman_list(freq_list.clone());
        let root = create_huffman_root(huffman_node_list);
        assert_eq!(root.value,10);
        assert_eq!(root.left.unwrap().value,3);
        assert_eq!(root.right.unwrap().value,7);
    }

}