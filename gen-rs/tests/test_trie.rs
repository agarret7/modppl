use gen_rs::Trie;


// inserting a trie into a root and then removing it should yield the previous tries
#[test]
pub fn test_add_remove_inverse() {
    let mut root = Trie::<i32>::new();

    let mut subin = Trie::<i32>::new();
    subin.witness("mother", 1, 2.);
    subin.witness("world", 2, 1.14);
    
    let root_before = root.clone();

    root.insert("hello", subin.clone());
    let subout = root.remove("hello");
    assert_eq!(subin, subout.unwrap());

    assert_eq!(root_before, root);
}

// searching the address of an witnessed value should yield the leaf trie
#[test]
pub fn test_search_witnessd_value() {
    let mut root = Trie::<i32>::new();

    root.witness("test", 2, -3.4);
    let leaf = Trie::leaf(2, -3.4);
    let found = root.search("test").expect("witnessd value not found.");
    assert_eq!(*found, leaf);

    root.witness("test/deep/nested", 5, -1.2);
    let found = root.search("test/deep/nested").expect("witnessd value not found.");
    let leaf = Trie::leaf(5, -1.2);
    assert_eq!(*found, leaf);
}

// searching the address of an inserted subtrie should yield the subtrie
#[test]
pub fn test_search_inserted_subtrie() {
    let mut root = Trie::<i32>::new();
    let mut subin = Trie::<i32>::new();

    subin.witness("a", 3, -3.1);
    subin.witness("b", 1, -0.1);

    root.insert("child", subin.clone());
    let found = root.search("child").expect("inserted subtrie not found.");
    assert_eq!(*found, subin);

    root.insert("great/grand/child", subin.clone());
    let found = root.search("great/grand/child").expect("inserted subtrie not found.");
    assert_eq!(*found, subin);
}

// taking the difference of measured weights before and after
// observing a value should yield the weight of the observation
#[test]
pub fn test_weighted_observation() {
    let mut root = Trie::<i32>::new();
    root.witness("test", 0, -1.3);
    let w_before= root.weight();
    let w_sub = -5.3;
    root.witness("test/deep/nested", 3, w_sub);
    let w_after = root.weight();
    assert_eq!(w_after - w_before, w_sub)
}

// taking the difference of measured weights before and after
// inserting a subtrie should yield the weight of the subtrie
#[test]
pub fn test_weighted_subtrie() {
    let mut root = Trie::<i32>::new();
    let mut sub = Trie::leaf(6, -0.4);
    sub.witness("deep/nested", -4, 0.4);
    let w_sub = sub.weight();
    let w_before = root.weight();
    root.insert("test", sub);
    let w_after = root.weight();
    assert_eq!(w_after - w_before, w_sub);
}

// observing an occupied address should panic
#[test]
#[should_panic]
pub fn test_insert_into_occupied_panic() {
    let mut root = Trie::<(i32,u8)>::new();
    root.witness("some/address", (-10431451, 200), -0.5);
    root.witness("some/address", (-1,0), 0.);
}

// unwrapping inner value from an empty trie should panic
#[test]
#[should_panic]
pub fn test_unwrap_inner_unchecked_panic() {
    let root = Trie::<u8>::new();
    root.unwrap_inner_unchecked();
}

// taking inner value from an empty trie should panic
#[test]
#[should_panic]
pub fn test_take_inner_unchecked_panic() {
    let mut root = Trie::<u8>::new();
    root.take_inner_unchecked();
}

// an assortment of different tests of Trie
#[test]
pub fn test_ptrie_extended_example() {
    let mut trie = Trie::new();
    trie.witness("hello / world", 1.2, 1.5);
    trie.witness("hello / mom", 1.0, 1.5);
    trie.witness("hello / world / player", 1.0, 1.5);
    let t = trie.search("hello / world");
    assert_eq!(t.unwrap().weight(), 3.0);
    assert_eq!(trie.weight(), 4.5);

    let mut sub = Trie::new();
    sub.witness("test", 1.0, 1.5);
    sub.witness("test / leaf", 1.0, 2.0);
    trie.insert("other", sub);

    assert_eq!(trie.weight(), 8.0);

    let helloworld = trie.remove("hello / world").unwrap();

    assert_eq!(helloworld.weight(), 3.0);
    assert_eq!(trie.weight(), 8.0 - 3.0);

    let mut hw_dup = Trie::leaf(1.1, 1.5);
    hw_dup.witness("player", 1.0, 1.5);
    assert_ne!(helloworld, hw_dup);
    let v = hw_dup.take_inner_unchecked();
    hw_dup.replace_inner(v+0.1_f32);
    assert_eq!(helloworld, hw_dup);

    let l = helloworld.search("player").unwrap();
    assert!(l.is_leaf());
    let v = l.clone().take_inner_unchecked();
    assert_eq!(v, 1.0);
}
