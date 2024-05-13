use crate::topo_sort::{topo_sort, DepGraph};
use std::collections::HashSet;

#[test]
fn topolotical_sort_test() {
    let mut graph = DepGraph::new();

    graph.insert(
        "A".to_string(),
        HashSet::from(["B".to_string(), "C".to_string(), "D".to_string()]),
    );
    graph.insert(
        "B".to_string(),
        HashSet::from(["C".to_string(), "E".to_string()]),
    );
    graph.insert("C".to_string(), HashSet::from(["D".to_string()]));
    graph.insert("E".to_string(), HashSet::from(["D".to_string()]));

    let ordering = topo_sort(&graph).unwrap();
    assert_eq!(ordering.len(), 5);
    assert_eq!(
        *ordering.first().unwrap(),
        "D".to_string()
    );
    assert_eq!(
        *ordering.last().unwrap(),
        "A".to_string()
    );
}
