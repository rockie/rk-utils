use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// DepGraph is a dependency graph.
/// It is represented as a HashMap where the key is the node and the value is a HashSet of its dependencies.
pub type DepGraph<T> = HashMap<T, HashSet<T>>;

#[inline]
fn add_edge<T>(graph: &mut DepGraph<T>, from: T, to: T)
where
    T: Eq + Hash + Clone,
{
    graph
        .entry(from)
        .and_modify(|deps| {
            deps.insert(to.clone());
        })
        .or_insert_with(|| {
            let mut deps = HashSet::new();
            deps.insert(to);
            deps
        });
}

/// TopoSort is a topological sort implementation.
pub struct TopoSort<T> {
    depends_on: DepGraph<T>,
    dependents: DepGraph<T>,
    no_deps: Vec<T>,
}

/// topo_sort sorts the dependencies topologically.
/// It returns a Vec of the sorted nodes.
/// If a cyclic reference is detected, it returns an error.
/// 
/// # Example
/// 
/// ```rust
/// use rk_utils::topo_sort;
/// use std::collections::{ HashMap, HashSet };
/// 
/// let mut deps = HashMap::new();
/// deps.insert("b".to_string(), HashSet::from(["a".to_string()]));
/// deps.insert("c".to_string(), HashSet::from(["b".to_string()]));
/// 
/// let sorted = topo_sort(&deps).unwrap();
/// assert_eq!(sorted, ["a", "b", "c"]);
/// ```
pub fn topo_sort<T, Id>(deps: T) -> Result<Vec<Id>, Box<dyn std::error::Error>>
where
    Id: Eq + Hash + Clone + std::fmt::Debug,
    TopoSort<Id>: From<T>,
{
    let mut state = TopoSort::from(deps);
    let mut sorted = vec![];

    while let Some(node) = state.no_deps.pop() {
        sorted.push(node.clone());

        if let Some(dependents) = state.get_dependents(&node) {
            for dependent in dependents.clone() {
                state.resolve(&dependent, &node);
            }
        }
    }

    if state.is_resolved() {
        Ok(sorted)
    } else {
        Err(format!(
            "Cyclic reference detected for id: {:?}",
            state.unresolved().collect::<Vec<_>>()
        )
        .into())
    }
}

impl<T> Default for TopoSort<T> {
    fn default() -> Self {
        Self {
            depends_on: HashMap::new(),
            dependents: HashMap::new(),
            no_deps: vec![],
        }
    }
}

impl<T: Eq + Hash + Clone> TopoSort<T> {
    #[inline]
    pub fn get_dependents(&self, dependency: &T) -> Option<&HashSet<T>> {
        self.dependents.get(dependency)
    }

    #[inline]
    pub fn is_resolved(&self) -> bool {
        self.depends_on.is_empty()
    }

    #[inline]
    pub fn resolve(&mut self, dependent: &T, dependency: &T) {
        if let Some(dependencies) = self.depends_on.get_mut(dependent) {
            dependencies.remove(dependency);
            if dependencies.is_empty() {
                self.no_deps.push(dependent.clone());

                self.depends_on.remove(dependent);
            }
        }
    }

    #[inline]
    pub fn unresolved(&self) -> impl Iterator<Item = &T> {
        self.depends_on.keys()
    }
}

impl From<&DepGraph<String>> for TopoSort<String> {
    fn from(deps: &DepGraph<String>) -> TopoSort<String> {
        let mut topo = TopoSort::default();
        // only track nodes that are being depended on but not appearing in id field of the deps
        let mut nodes_being_depended_on = HashSet::new();

        for (id, dependencies) in deps.iter() {
            if dependencies.is_empty() {
                topo.no_deps.push(id.clone());
            } else {
                for dependency_id in dependencies {
                    nodes_being_depended_on.insert(dependency_id.clone());
                    add_edge(&mut topo.depends_on, id.clone(), dependency_id.clone());
                    add_edge(&mut topo.dependents, dependency_id.clone(), id.clone());
                }
            }
        }

        // remove those id that already on topo.depends_on from nodes_being_depended_on
        for id in topo.depends_on.keys() {
            nodes_being_depended_on.remove(id);
        }

        // move the rest of the nodes to topo.no_deps
        for id in nodes_being_depended_on {
            topo.no_deps.push(id);
        }

        topo
    }
}
