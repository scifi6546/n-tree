pub fn new_tree<Data>() -> L0Node<Data> {
    L0Node { data: None }
}
pub struct L0Node<Data> {
    data: Option<(Data, usize)>,
}
impl<Data: Clone> L0Node<Data> {
    pub fn empty() -> Self {
        Self { data: None }
    }
    /// Sets self data
    pub fn set(&mut self, data: Data, index: usize) {
        self.data = Some((data, index));
    }
    /// Gets the index of the data
    pub fn index(&self) -> Option<usize> {
        if self.data.is_some() {
            Some(self.data.clone().unwrap().1)
        } else {
            None
        }
    }
    /// Gets if the node is empty, used for calcuating free space
    pub fn is_empty(&self) -> bool {
        !self.data.is_some()
    }
}
struct L1Node<Data: Clone> {
    left: Option<(Data, Index)>,
    right: Option<(Data, Index)>,
}
impl<Data: Clone> L1Node<Data> {
    fn empty() -> Self {
        Self {
            left: None,
            right: None,
        }
    }
    fn total_free(&self) -> usize {
        let f = if self.left.is_some() { 1 } else { 2 } + if self.left.is_some() { 1 } else { 2 };
        return f;
    }
    fn set_left(&mut self, data: Data, index: usize) {
        self.left = Some((data, index));
    }
    fn set_right(&mut self, data: Data, index: usize) {
        self.right = Some((data, index));
    }
    fn left_index(&self) -> Option<usize> {
        if let Some((_data, index)) = &self.left {
            Some(*index)
        } else {
            None
        }
    }
    fn right_index(&self) -> Option<usize> {
        if let Some((_data, index)) = &self.right {
            Some(*index)
        } else {
            None
        }
    }
    fn left_empty(&self) -> bool {
        !self.left.is_some()
    }
    fn right_empty(&self) -> bool {
        !self.right.is_some()
    }
}
type Index = usize;
/// Top Level Node for Datastructure.
pub struct L2Node<Data: Clone> {
    /// left node containing data
    left: L1Node<Data>,
    /// right node containing data
    right: L1Node<Data>,
    /// Number of free blocks to the left of the node. Used to calculate adjacent space between
    /// this node and sibling nodes.
    left_free: usize,
    /// Largest free contigious space. Used to find free space.
    center_free: usize,
    /// Number of free blocks at right end of block. Also used to calculate adjacent space between
    /// this node and sibling nodes.
    right_free: usize,
}
impl<Data: Clone> L2Node<Data> {
    pub fn new() -> Self {
        Self {
            left: L1Node::empty(),
            left_free: 2 * 2,
            right: L1Node::empty(),
            right_free: 2 * 2,
            center_free: 2 * 2,
        }
    }
    /// Preconditions (for now): Data is No larger then 4 and greater then equal to 1
    ///
    pub fn insert(&mut self, data: Vec<Data>) -> Index {
        //check if there is free space in index
        if self.center_free >= data.len() {
            //insert in self tree
            if data.len() >= 3 {
                todo!()
            } else if data.len() == 2 {
                if self.left.left_empty() && self.left.right_empty() {
                    //the index will be zero
                    let index = 0;
                    self.left.set_left(data[0].clone(), index);
                    self.left.set_right(data[1].clone(), index);
                    self.center_free = self.right.total_free();

                    todo!("set left free and right free, figure out index")
                } else if self.right.left_empty() && self.right.right_empty() {
                    //index is the incremented version of the previous node
                    let index = if let Some(i) = self.left.left_index() {
                        //allowing space for a middle node
                        i + 2
                    } else if let Some(i) = self.left.right_index() {
                        i + 1
                    } else {
                        panic!("invalid state: left should have at least one index")
                    };

                    self.right.set_left(data[0].clone(), index);
                    self.right.set_right(data[1].clone(), index);
                    todo!("figure out index");
                } else {
                    panic!("tree is full");
                }
            } else if data.len() == 1 {
                if self.left.left_empty() {
                    self.left.set_left(data[0].clone(), 0);
                    todo!("figure out index and free");
                } else if self.right.left_empty() {
                    //index is the incremented version of the previous node
                    let index = if let Some(i) = self.left.left_index() {
                        //allowing space for a middle node
                        i + 2
                    } else if let Some(i) = self.left.right_index() {
                        i + 1
                    } else {
                        panic!("invalid state: left should have at least one index")
                    };

                    self.right.set_left(data[0].clone(), index);
                    todo!("figure out index and free");
                } else if self.left.right_empty() {
                    let index = if let Some(index) = self.left.left_index() {
                        index + 1
                    } else {
                        panic!("left should have an index")
                    };
                    self.left.set_right(data[0].clone(), index);
                    todo!("figure out index and free");
                } else if self.right.right_empty() {
                    let index = if let Some(index) = self.right.left_index() {
                        index + 1
                    } else {
                        panic!("left should have a index")
                    };
                    self.right.set_right(data[0].clone(), index);
                    todo!("figure out index and free");
                } else {
                    panic!("invalid state");
                }
            } else {
                panic!("violated precondition: data.len()<1")
            }
        } else {
            //create higher node
            todo!("data larger then 4")
        }
    }
    pub fn get(&self, index: &Index) -> &Data {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        let tree: L2Node<u8> = L2Node::new();
    }
    #[test]
    fn basic_insert() {
        let mut tree: L2Node<u8> = L2Node::new();
        let k = tree.insert(vec![5]);
        assert_eq!(tree.get(&k), &5);
    }
    #[test]
    fn mass_insert() {
        let mut tree: L2Node<u32> = L2Node::new();
        let mut keys = vec![];
        for i in 0..1000 {
            let k = tree.insert(vec![i.clone()]);
            assert_eq!(tree.get(&k), &i);
            keys.push(k);
        }
        let mut i = 0;
        for k in keys.iter() {
            assert_eq!(tree.get(k), &i);
            i += 1;
        }
        let k = tree.insert(vec![5]);
        assert_eq!(tree.get(&k), &5);
    }
}
