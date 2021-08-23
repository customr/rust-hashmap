use crate::hash::*;

#[test]
fn hash_string(){
    let x: String = String::from("test1test");
    assert_eq!(x.hash(), 249907798550280470);
}

#[test]
fn hash_integers(){
    let x: usize = 5;
    assert_eq!(x.hash(), 5);
}