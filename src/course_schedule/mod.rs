use std::collections::{HashMap, HashSet, VecDeque};

type CourseGraph = HashMap<i32, Vec<i32>>;

fn make_course_graph(prerequisites: Vec<Vec<i32>>) -> CourseGraph {
    let mut courses = HashSet::new();
    let mut course_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for p in prerequisites {
        let (course, dependency) = (p[0], p[1]);

        courses.insert(course);
        if let Some(dependents) = course_map.get_mut(&dependency) {
            dependents.push(course);
        } else {
            course_map.insert(dependency, vec![]);
        }
    }

    course_map
}

fn unique(g: &CourseGraph) -> usize {
    g.len()
}

fn breadth_first_search(
    g: &CourseGraph,
    seen: &mut HashSet<i32>,
    indegree: &mut HashMap<i32, i32>,
) {
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(node) = queue.pop_front() {
        if !seen.contains(&node) {
            seen.insert(node);
            for next_node in g.get(&node).unwrap() {
                if let Some(slot) = indegree.get_mut(next_node) {
                    *slot += 1;
                } else {
                    indegree.insert(*next_node, 1);
                }

                queue.push_back(*next_node);
            }
        }
    }
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    // this is an adjacency list representation
    let course_graph = make_course_graph(prerequisites);

    // essentially walking the graph with a BFS will result in a complete
    // visitation of all nodes and their adjacent edges
    // if multiple nodes lead into a singular node, then the BFS would
    // 'double count' that destination node.
    let mut prereq_count = HashMap::new();
    let mut seen = HashSet::new();
    for key in course_graph.keys() {
        if !seen.contains(key) {
            breadth_first_search(&course_graph, &mut seen, &mut prereq_count);
        }
    }

    // basically sorting the graph by indegree yields 'starting points'
    // these are nodes that only have edges fanning out from them (it's a DAG)
    let mut map_values = prereq_count.clone().into_iter().collect::<Vec<_>>();
    map_values.sort_by(|(_k, v), (_k2, v2)| v.cmp(v2));

    // walking through this this map would then yield
    // a traversal that tries to satisfy dependencies from the bottom up
    // 'satisfying' a dependency means that if you removed its node from the graph
    // and then counted all of its edges as no longer present, then you could
    // say that that prerequisite was met
    // if every edge is accounted for after walking every node in prerequisite order
    // then it means that the graph fully resolves
    // which means it's a valid, topologically sortable graph.
    for (_ref_count, course_num) in map_values {
        if let Some(neighbors) = course_graph.get(&course_num) {
            for &neighbor in neighbors {
                if let Some(num_edges_in) = prereq_count.get_mut(&neighbor) {
                    if *num_edges_in == 0 {
                        continue;
                    }

                    *num_edges_in -= 1;
                }
            }
        }
    }

    prereq_count.into_values().all(|x| x == 0)
}

//  a node with 0 indegree is a source
// if you can deplete the full graph by removing all sources and the edges
// (essentially satisfying dependencies from the bottom up)
// then you can topologically sort this
// and therefore this is a valid schedule
