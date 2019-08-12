extern crate flat_collections;

use flat_collections::FlatList;

#[test]
fn create() {
    let _x = FlatList::<usize>::new();
}

#[test]
fn append() {
    let mut x = FlatList::<usize>::new();
    x.append(55);
    x.append(66);
    x.append(77);
    println!("{:?}", x);
}