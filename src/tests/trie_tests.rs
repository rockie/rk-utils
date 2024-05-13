use crate::str::StringUtil;
use crate::trie::Trie;

#[test]
fn test_trie_longest_match() {
    let mut trie = Trie::new();

    let route1 = "/cloud/instance/".url_to_nodes();
    let route2 = "/cloud/".url_to_nodes();
    let route3 = "/builder/instance".url_to_nodes();
    let route4 = "/builder".url_to_nodes();
    let route5 = "/".url_to_nodes();

    trie.insert(route1, 1);
    trie.insert(route2, 2);
    trie.insert(route3, 3);
    trie.insert(route4, 4);
    trie.insert(route5, 5);

    let input1 = "/cloud/instance/xxx".url_to_nodes();
    assert_eq!(trie.find_longest_match(input1), Some(&1));

    let input2 = "/cloud/xxx".url_to_nodes();
    assert_eq!(trie.find_longest_match(input2), Some(&2));

    let input3 = "/builder/instance/".url_to_nodes();
    assert_eq!(trie.find_longest_match(input3), Some(&3));

    let input4 = "/fjeao".url_to_nodes();
    assert_eq!(trie.find_longest_match(input4), Some(&5));
}
