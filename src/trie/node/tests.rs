use super::*;
use crate::hash::poseidon::Poseidon;
use zktrie::HashField;
use zktrie_rust::hash::AsHash;
use zktrie_rust::types::Hashable;

type OldNode = zktrie_rust::types::Node<AsHash<HashField>>;

#[test]
fn test_empty_node() {
    let expected = OldNode::new_empty_node().calc_node_hash().unwrap();
    let node_hash = expected.node_hash().unwrap();

    assert_eq!(
        unsafe { Node::<Poseidon>::empty().get_node_hash_unchecked() },
        node_hash.as_ref()
    );
    assert_eq!(
        Node::<Poseidon>::empty().canonical_value(false),
        expected.canonical_value()
    );
}

#[test]
fn test_leaf_node() {
    let expected =
        OldNode::new_leaf_node(AsHash::from_bytes(&[1u8; 32]).unwrap(), 0, vec![[2u8; 32]])
            .calc_node_hash()
            .unwrap();
    let node_hash = expected.node_hash().unwrap();

    let node = Node::<Poseidon>::new_leaf(
        Poseidon::new_hash_try_from_bytes(&[1u8; 32]).unwrap(),
        vec![[2u8; 32]],
        0,
        None,
    )
    .unwrap();

    assert_eq!(
        node.get_or_calculate_node_hash().unwrap(),
        node_hash.as_ref()
    );
    assert_eq!(node.canonical_value(false), expected.canonical_value());

    let expected = OldNode::new_leaf_node(
        AsHash::from_bytes(&[1u8; 32]).unwrap(),
        1,
        vec![[1u8; 32], [2u8; 32], [3u8; 32]],
    )
    .calc_node_hash()
    .unwrap();
    let node_hash = expected.node_hash().unwrap();

    let node = Node::<Poseidon>::new_leaf(
        Poseidon::new_hash_try_from_bytes(&[1u8; 32]).unwrap(),
        vec![[1u8; 32], [2u8; 32], [3u8; 32]],
        1,
        None,
    )
    .unwrap();

    assert_eq!(
        node.get_or_calculate_node_hash().unwrap(),
        node_hash.as_ref()
    );
    assert_eq!(node.canonical_value(false), expected.canonical_value());
}

#[test]
fn test_branch_node() {
    let expected = OldNode::new_parent_node(
        zktrie_rust::types::NodeType::NodeTypeBranch0,
        AsHash::from_bytes(&[1u8; 32]).unwrap(),
        AsHash::from_bytes(&[2u8; 32]).unwrap(),
    )
    .calc_node_hash()
    .unwrap();
    let node_hash = expected.node_hash().unwrap();

    let node = Node::<Poseidon>::new_branch(
        BranchLTRT,
        Poseidon::new_hash_try_from_bytes(&[1u8; 32]).unwrap(),
        Poseidon::new_hash_try_from_bytes(&[2u8; 32]).unwrap(),
    );

    assert_eq!(
        node.get_or_calculate_node_hash().unwrap(),
        node_hash.as_ref()
    );
    assert_eq!(node.canonical_value(false), expected.canonical_value());
}

#[test]
fn test_parse_node() {
    let node_hash =
        hex::decode("24bae2eed47e458247d74b2b18dea76b9870a6f97fc66ced0173003082afdd36").unwrap();
    let node_bytes = hex::decode("04247aa484d0a3c2ca12e8ecbe1c1cd6311a82dbdc62e05fbfe0ae6438e02dd9db01010000000000000000000000000000000000000000000000000000000000000005bda820944702ae7610eeed8ed8a4bd20be476f4411088f0672704dbfe6e1def9c88c29").unwrap();

    let node = Node::<Poseidon>::try_from(node_bytes.as_slice()).unwrap();
    assert_eq!(
        node.get_or_calculate_node_hash().unwrap(),
        node_hash.as_slice()
    );
    let expected = OldNode::new_node_from_bytes(&node_bytes)
        .unwrap()
        .calc_node_hash()
        .unwrap();
    assert_eq!(expected.node_hash().unwrap().as_ref(), node_hash.as_slice());
    assert_eq!(node.canonical_value(false), expected.canonical_value());
}
