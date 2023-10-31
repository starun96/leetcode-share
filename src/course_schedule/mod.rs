use std::collections::{HashMap, HashSet, VecDeque};

struct CourseGraph {
    /// maps 'dependency' courses to the courses that follow
    course_map: HashMap<i32, Vec<i32>>,

    /// all the unique courses
    courses: HashSet<i32>,
}

impl CourseGraph {
    pub fn make_course_graph(prerequisites: Vec<Vec<i32>>) -> Self {
        let mut courses = HashSet::new();
        let mut course_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for p in prerequisites {
            let (course, dependency) = (p[0], p[1]);

            courses.insert(course);
            courses.insert(dependency);

            if let Some(dependents) = course_map.get_mut(&dependency) {
                dependents.push(course);
            } else {
                course_map.insert(dependency, vec![]);
            }
        }

        Self {
            course_map,
            courses,
        }
    }
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let course_graph = CourseGraph::make_course_graph(prerequisites);
    let queue = VecDeque::new();
    for i in bfs(course_graph) {
        queue.push_front(all_dependents);
        for dependent in all_dependents {
            prereq_count[dependent] += 1
        }
    }

    // start with the lowest prereqCount
    // i.e. a node that looks like a starting point in this graph
    // if you remove this starting node and rip out all of the edges
    // that fan out from it, then you've accounted for satisfying
    // that dependency

    // the way can track this is by decrementing the counts generated above
    // by 1 each time we remove a 'source node'

    // do this exhaustively from A source node and if this is a valid graph
    // then you will end up in a situation where you end up with 1 final node
    // that can then just be satisfied with no constraint

    // that implies that the process of removing all of the sources recursively should result in all edges in the entire graph being accounted for fully and uniquely

    for prereqCountpair in prereCounts.sort() {
        let (course_num, prereq_count) = prereq_count_pair;
    }
}

//  a node with 0 indegree is a source
// if you can deplete the full graph by removing all sources and the edges
// (essentially satisfying dependencies from the bottom up)
// then you can topologically sort this
// and therefore this is a valid schedule
