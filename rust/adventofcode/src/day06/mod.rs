/* This is my first time using Rc to this degree.
 * It feels like something is not quite right.
 *
 */
use crate::solution::Solution;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Day06 {}

pub fn create_solution() -> Day06 {
    Day06 {}
}

const COM: &str = "COM";

#[derive(Debug, Clone)]
enum Object {
    CenterOfMass,
    Satellite { name: String, ancestor: Rc<Object> },
}

impl Object {
    fn name(&self) -> &str {
        match *self {
            Object::CenterOfMass => COM,
            Object::Satellite { ref name, .. } => &name,
        }
    }

    fn ancestor(&self) -> Option<Rc<Object>> {
        match *self {
            Object::CenterOfMass => None,
            Object::Satellite { ref ancestor, .. } => Some(Rc::clone(ancestor)),
        }
    }
}

fn ancestors(object: Rc<Object>) -> AncestorIterator {
    AncestorIterator {
        current_object: Rc::clone(&object),
    }
}

#[derive(Debug)]
struct AncestorIterator {
    current_object: Rc<Object>,
}

impl Iterator for AncestorIterator {
    type Item = Rc<Object>;

    fn next(&mut self) -> Option<Rc<Object>> {
        if let Some(ref ancestor) = self.current_object.ancestor() {
            self.current_object = Rc::clone(ancestor);
            Some(Rc::clone(&self.current_object))
        } else {
            None
        }
    }
}

type OrbitGraph = HashMap<String, Rc<Object>>;
type EntryMap = HashMap<String, Vec<String>>;

fn add_to_graph(graph: &mut OrbitGraph, entries: &EntryMap, ancestor: Rc<Object>, name: &str) {
    let child = Rc::new(Object::Satellite {
        name: name.to_string(),
        ancestor,
    });
    graph.insert(name.to_string(), Rc::clone(&child));

    if let Some(grandchildren) = entries.get(name) {
        grandchildren
            .iter()
            .for_each(|gc| add_to_graph(graph, entries, Rc::clone(&child), gc));
    }
}

fn build_orbit_graph(entries: &EntryMap) -> OrbitGraph {
    let mut graph = OrbitGraph::new();
    let com = match entries.get("COM") {
        None => panic!("COM missing"),
        Some(e) => e,
    };

    let com_object = /*CoCreateInstance*/ Rc::new(Object::CenterOfMass);

    graph.insert("COM".to_string(), Rc::clone(&com_object));

    com.iter()
        .for_each(|c| add_to_graph(&mut graph, &entries, Rc::clone(&com_object), c));

    graph
}

fn build_entry_map(input: &str) -> EntryMap {
    let mut entries = EntryMap::new();

    for line in input.lines() {
        let mut split = line.split(')');

        entries
            .entry(split.next().unwrap().to_uppercase())
            .or_insert_with(Vec::new)
            .push(split.next().unwrap().to_uppercase());
    }

    entries
}

fn count_all_orbits(graph: &OrbitGraph) -> usize {
    graph
        .values()
        .map(|o| ancestors(Rc::clone(o)).count())
        .sum()
}

fn find_common_ancestor(obj1: Rc<Object>, obj2: Rc<Object>) -> Rc<Object> {
    let obj1_ancestor_names: HashSet<_> = ancestors(Rc::clone(&obj1))
        .map(|o| o.name().to_string())
        .collect();

    for obj2_ancestor in ancestors(Rc::clone(&obj2)) {
        if obj1_ancestor_names.contains(obj2_ancestor.name()) {
            return obj2_ancestor;
        }
    }

    unreachable!();
}

fn build_graph(input: &str) -> OrbitGraph {
    build_orbit_graph(&build_entry_map(input))
}

fn find_distance_between(obj1: Rc<Object>, obj2: Rc<Object>, common_ancestor: Rc<Object>) -> usize {
    ancestors(Rc::clone(&obj1))
        .take_while(|o| o.name() != common_ancestor.name())
        .count()
        + ancestors(Rc::clone(&obj2))
            .take_while(|o| o.name() != common_ancestor.name())
            .count()
}

impl Solution for Day06 {
    fn problem1(&self, input: &str) -> String {
        let graph = build_graph(input);

        count_all_orbits(&graph).to_string()
    }

    fn problem2(&self, input: &str) -> String {
        let graph = build_graph(input);
        let you = Rc::clone(&graph.get("YOU").expect("Couldnt find YOU"));
        let santa = Rc::clone(&graph.get("SAN").expect("Couldnt find SAN"));

        let common_ancestor = find_common_ancestor(Rc::clone(&you), Rc::clone(&santa));
        let count = find_distance_between(
            Rc::clone(&you),
            Rc::clone(&santa),
            Rc::clone(&common_ancestor),
        );

        count.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_common_ancestry() {
        let input = "COM)B\nB)C\nC)D\nD)E\n E)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";

        let graph = build_graph(input);

        let you = Rc::clone(&graph.get("YOU").expect("Couldnt find YOU"));
        let santa = Rc::clone(&graph.get("SAN").expect("Couldnt find SAN"));

        let common_ancestor = find_common_ancestor(Rc::clone(&you), Rc::clone(&santa));

        assert_eq!(common_ancestor.name(), "D");
    }

    #[test]
    fn test_ancestry_distance() {
        let input = "COM)B\nB)C\nC)D\nD)E\n E)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";

        let graph = build_graph(input);

        let you = Rc::clone(&graph.get("YOU").expect("Couldnt find YOU"));
        let santa = Rc::clone(&graph.get("SAN").expect("Couldnt find SAN"));

        let common_ancestor = find_common_ancestor(Rc::clone(&you), Rc::clone(&santa));
        let count = find_distance_between(
            Rc::clone(&you),
            Rc::clone(&santa),
            Rc::clone(&common_ancestor),
        );

        assert_eq!(count, 4);
    }
}
