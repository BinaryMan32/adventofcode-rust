use im::{HashMap, HashSet};
use itertools::Itertools;

struct Graph<'a> {
    neighbors: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Graph<'a> {

    fn parse(lines: &Vec<&'a str>) -> Graph<'a> {
        let mut neighbor_tuples = lines.into_iter()
            .map(|&line| Graph::parse_edge(line))
            .flat_map(|(s, e)| [(s, e), (e, s)])
            .filter(|(s, e)| s != &"end" && e != &"start")
            .collect::<Vec<_>>();
        neighbor_tuples.sort();
        let neighbors = HashMap::from_iter(
            neighbor_tuples
                .into_iter()
                .group_by(|(s, _e)| *s)
                .into_iter()
                .map(|(s, tuples)|
                    (s, tuples.into_iter().map(|(_s, e)| e).collect::<Vec<&'a str>>())
                )
        );
        Graph {neighbors}
    }

    fn parse_edge(line: &str) -> (&str, &str) {
        let parts = line.split('-').collect::<Vec<_>>();
        (parts[0], parts[1])
    }

    fn count_paths(&self) -> usize {
        self.count_paths_from("start", &HashSet::<&'a str>::new())
    }

    fn count_paths_from(&self, node: &str, explored_small: &HashSet<&'a str>) -> usize {
        if node == "end" {
            1
        } else {
            self.neighbors.get(node).unwrap().iter().map(|neighbor|
                if char::is_ascii_uppercase(&neighbor.chars().next().unwrap()) {
                    self.count_paths_from(neighbor, explored_small)
                } else if explored_small.contains(neighbor) {
                    0
                } else {
                    self.count_paths_from(neighbor, &explored_small.update(neighbor))
                }
            ).sum()
        }
    }

}

pub fn part1(input: &Vec<&str>) -> i64 {
    Graph::parse(input).count_paths() as i64
}

pub fn part2(input: &Vec<&str>) -> i64 {
    0
}
