use std::ops::{Index, IndexMut};

pub type OptIx = Option<usize>;

#[derive(Debug, Default)]
pub struct Node<T> {
    t: Option<T>,
    next: OptIx
}

#[derive(Default, Debug, Copy, Clone)]
struct List {
    head: OptIx,
    length: usize
}

#[derive(Debug)]
pub struct FlatList<T> {
    storage: Vec<Node<T>>,
    head: List,
    free: List
}

impl<T> FlatList<T> {
    pub fn new() -> FlatList<T> {
        FlatList { storage: Vec::new(), head: List::default(), free: List::default() }
    }

    pub fn append(&mut self, t: T) {
        // get an unlinked, uninitialized slot
        let slot = self.claim();
        // initialize the slot
        self[slot] = Node { t: Some(t), next: None };
        // link the slot
        self.head = self.push(self.head, slot);
    }

    // this comes up with an unlinked, unoccupied Node pointer, and sets its data
    fn claim(&mut self) -> OptIx {
        if self.free.length == 0 {
            self.storage.push(Node { t: None, next: None });
            self.free = self.push(self.free, Some(self.storage.len()-1));
        }
        let (head, tail) = self.pop(self.free);
        self.free = tail;
        head
    }

    // this pushes a pointer onto the head of the list given.  the pointer is expected to already
    // be pointing to a Node, whose next ought to currently be None
    fn push(&mut self, list: List, ix: OptIx) -> List {
        let old_head = list.head;
        let old_len = list.length;
        self[ix].next = old_head;
        List { head: ix, length: old_len + 1 }
    }

    // this pops the head of the list given
    fn pop(&mut self, list: List) -> (OptIx, List) {
        let old_head = list.head;
        // if this list is empty, we will panic dereffing the option
        (old_head, List { head: self[old_head].next, length: list.length - 1 })
    }
}

impl<T> Index<OptIx> for FlatList<T> {
    type Output = Node<T>;

    // if index doesn't point to a valid node, we panic
    fn index(&self, index: OptIx) -> &Node<T> {
        &self.storage[index.unwrap()]
    }
}

impl<T> IndexMut<OptIx> for FlatList<T> {
    // if index doesn't point to a valid node, we panic
    fn index_mut(&mut self, index: OptIx) -> &mut Node<T> {
        &mut self.storage[index.unwrap()]
    }
}

