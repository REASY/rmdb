use super::*;
use utils::*;
use std::thread;
use std::sync::Arc;
use std::rc::Rc;
use std::fmt::Debug	;

pub fn split_helper<K: Ord + Debug + Clone, V: Clone + Debug>(keys: &mut Vec<K>, values: &mut Vec<V>, k: &K) -> (Vec<K>, Vec<V>, K){
	let mut fake_index = 0;
	let mut is_found = false;
	let mid = ((keys.len()) as f32 / 2 as f32).ceil() as usize;
	let keys_len = keys.len().clone();
	let mut mid_element = None;
	let mut right_keys = Vec::<K>::with_capacity(keys.len());
	let mut right_values = Vec::<V>::with_capacity(values.len());
	{
		let keys_ref: &[K] = keys.as_ref();
		for j in 0..keys_len{
			let ref i = keys_ref[j];		
			if k < i && !is_found{				
				if mid == fake_index{
					mid_element = Some((*k).clone());
				}
				fake_index += 1;
				is_found = true;
			}
			if fake_index >= mid{
				right_keys.push((*i).clone());
				right_values.push((values[j].clone()));
			}
			if mid == fake_index{
				mid_element = Some((*i).clone());
			}
			fake_index += 1;
		}
		if !is_found{
			if fake_index >= mid{
				right_keys.push((*k).clone());
			}
		}
	}
	
	let mut fake_index = 0;
	let mut index = 0;
	let mut deleted = 0;
	// println!("Mid: {:?}", mid);
	for index in 0..keys_len{
		// println!("{:?}", keys);
		if deleted == 0 && *k < keys[index]{
			if fake_index < mid{
				keys.remove(keys_len - 1);
				values.remove(keys_len - 1);
				deleted += 1;
				// keys.insert(index, (*k).clone());
			}			
			fake_index += 1;
		}
		if fake_index > mid{
			if deleted <= mid {
				let c = keys.len().clone() - 1;
				keys.remove(c);
				values.remove(c);
				deleted += 1;
			}
		}
		fake_index += 1;
	}
	if (deleted == 0){
		let len  = keys.len().clone() - 1;
		keys.remove(len);
		values.remove(len);
	}
	println!("K: {:?}", k);
	println!("Left keys: {:?}", keys);
	println!("Left values: {:?}", values);
	println!("Right keys: {:?}", right_keys);
	println!("Right values: {:?}", right_values);
	return (right_keys, right_values, mid_element.unwrap().clone());
}

pub fn get_insertion_index<K: Ord + Debug + Clone>(values: &[K], k: &K) -> usize	{
	let mut index = 0;
	for v in values{
		if *v >= *k { break;}
		index += 1;
	}
	return index;
}
pub fn get_insertion_index2<K: Ord + Debug + Clone, V: Clone + Debug>(values: &[(K, V)], k: &K) -> usize	{
	let mut index = 0;
	for v in values{
		if v.0 >= *k { break;}
		index += 1;
	}
	return index;
}
pub fn upper_bound<K: Ord + Debug + Clone>(keys: &[K], key: &K) -> usize{
	let mut index: i32 = -1;
	for i in 0..keys.len(){
		let ref k = keys[i];
		if *key < *k { index = i as i32; break;}
	}
	if index == -1{ index = keys.len() as i32; }
	return index as usize;
}
fn choose_subtree<'a, K: Ord + Debug + Clone, V:  Debug + Clone>(root: &'a mut Box<Node<K, V>>, k: &K) ->  &'a mut Box<Node<K, V>> {
	match root.node_type() {
		NodeType::Leaf => { return root;}
		NodeType::Node	=> {
			let index = upper_bound(root.keys(), &k);    			
			println!("Child index: {:?}", index);
			match root.childern{
				Some(ref mut node) => { choose_subtree(&mut node[index], k)},
				None => panic!(""),
			}
		}
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum NodeType{
	Node,
	Leaf
}
#[derive(Debug)]
struct Node<K, V> {
    node_type: NodeType,
    keys: Vec<K>,
    values: Option<Vec<V>>,
    childern: Option<Vec<Box<Node<K, V>>>>,
    next: Option<Rc<Node<K, V>>>
}
impl<K: Ord + Debug + Clone, V: Clone + Debug> Node<K, V> {
	fn min_key(&self) -> K { self.keys[0].clone()}
	fn max_key(&self) -> K { self.keys[self.keys.len() - 1].clone()}
	fn keys(&self) -> &[K]{ self.keys.as_slice()}
	fn keys_count(&self) -> usize{ self.keys.len() }
	fn node_type(&self) -> NodeType { self.node_type }
	fn add_key_value(&mut self, k: K, v: V) {
		println!("add_key_value. k: {:?}, v: {:?}", k, v);
		assert!(self.node_type() != NodeType::Node);
		let index = get_insertion_index(&self.keys, &k);
		assert!(self.keys.capacity() > index);
		self.keys.insert(index, k);
		self.values.as_mut().unwrap().insert(index, v);		
	}
	fn insert(&mut self, max_degree: usize, k: K, v: V) -> (Option<Box<Node<K,V>>>, Option<K>) {
		match self.node_type{
			NodeType::Leaf => { return self.insert_leaf(max_degree, k, v); },
			NodeType::Node => { return self.insert_node(max_degree, k, v); },
		}
	}
	fn insert_leaf(&mut self, max_degree: usize, k: K, v: V) -> (Option<Box<Node<K,V>>>, Option<K>) {
		println!("insert_leaf. k: {:?}, v: {:?}", k, v);
		if self.keys_count() >= max_degree - 1{
			let (mut new_leaf, min_key_in_new_leaf) = self.split_leaf();
			if k < min_key_in_new_leaf{
				self.add_key_value(k, v);
			}
			else {
				new_leaf.add_key_value(k,v);
			}
			return (Option::None, Option::None);
		}
		else{
			self.add_key_value(k, v);
			return (Option::None, Option::None);
		}
	}
	fn insert_node(&mut self, max_degree: usize, k: K, v: V) -> (Option<Box<Node<K,V>>>, Option<K>) {
		println!("insert_node. k: {:?}, v: {:?}", k, v);
		return (Option::None, Option::None);
	}
	fn split_leaf(&mut self) -> (Box<Node<K, V>>, K) {
		// Break down the node into two partitions.
		// The first should hold ceil of (N-1)/2 key values
		// The second partition can hold rest of the key values
		let keys_cnt = self.keys_count();
		let first_hold = ((keys_cnt - 1) as f32 / 2 as f32).ceil() as usize;
		println!("first_hold: {:?}. keys: {:?}", first_hold, self.keys);
		let mut new_leaf = Box::new(Node::<K, V>::new_leaf(self.keys.capacity()));
		let min_key_in_new_leaf = self.keys[first_hold].clone();
		// let mut original_next = None;
		// match self.next{
		// 	Some(ref n) => original_next = Some(n.clone()),
		// 	None => original_next = None
		// }
		let mut values = self.values.as_mut().unwrap();
		for i in first_hold..keys_cnt{
			let k = self.keys[i].clone();
			let v = values[i].clone();
			new_leaf.add_key_value(k, v);
		}
		for i in first_hold..keys_cnt {
			self.keys.remove(first_hold);
			values.remove(first_hold);
		}
		return (new_leaf, min_key_in_new_leaf);
	}
    pub fn new_leaf(count: usize) -> Self {
	    Node {
	    	node_type: NodeType::Leaf,
	        keys: Vec::<K>::with_capacity(count),
	        values: Some(Vec::<V>::with_capacity(count)),
	        childern: None,
	        next: None
	    }
	}
	pub fn new_node(count: usize) -> Self {
	    Node {
	    	node_type: NodeType::Node,
	        keys: Vec::<K>::with_capacity(count),
	        values: None,
	        childern: Some(Vec::<Box<Node<K, V>>>::with_capacity(count + 1)),
	        next: None
	    }
	}
}

#[derive(Debug)]
pub struct BPlusTree<K, V> {
    root: Box<Node<K, V>>,
    max_degree: u16,
}
impl<K: Ord + Debug + Clone, V:  Debug + Clone> BPlusTree<K, V> {
    pub fn new(max_degree: u16) -> Self {
        let leaf = Node::<K, V>::new_leaf(max_degree as usize);
        BPlusTree {
            root: Box::new(leaf),
            max_degree: max_degree,
        }
    }
    pub fn insert(&mut self, k: K, v: V){
    	let root_node_type = self.root.node_type();
    	let keys_count = self.root.keys_count()  as u16;
    	println!("Root type: {:?} max_degree: {}. leaf.keys_count: {}", root_node_type, self.max_degree, self.root.keys_count());
    	let node = choose_subtree(&mut self.root, &k);
    	println!("Choose Node type: {:?}", node.node_type());
    	let (a, b) = node.insert(self.max_degree as usize, k, v);
    	print_type_of(node.as_mut());
    }
}