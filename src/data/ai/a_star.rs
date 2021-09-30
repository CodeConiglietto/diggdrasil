use std::{cmp::Ordering, collections::BinaryHeap, convert::TryFrom};

use ndarray::{Array2, ArrayView2};

#[derive(Debug, Clone)]
pub enum VisitedNode {
    Unvisited,
    Visited {
        distance: u32,
    },
    VisitedFrom {
        distance: u32,
        previous: (usize, usize),
    },
}

impl VisitedNode {
    pub fn is_visited(&self) -> bool {
        match self {
            Self::Unvisited => false,
            Self::Visited { .. } | Self::VisitedFrom { .. } => true,
        }
    }

    pub fn distance(&self) -> Option<u32> {
        match self {
            Self::Unvisited => None,
            Self::Visited { distance } | Self::VisitedFrom { distance, .. } => Some(*distance),
        }
    }

    pub fn previous(&self) -> Option<(usize, usize)> {
        match self {
            Self::Unvisited | Self::Visited { .. } => None,
            Self::VisitedFrom { previous, .. } => Some(*previous),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FrontierNode {
    pub pos: (usize, usize),
    pub distance: u32,
    pub heuristic: u32,
}

impl FrontierNode {
    pub fn estimate(&self) -> u32 {
        self.distance + self.heuristic
    }
}

impl PartialEq for FrontierNode {
    fn eq(&self, rhs: &Self) -> bool {
        self.estimate().eq(&rhs.estimate())
    }
}

impl Eq for FrontierNode {}

impl PartialOrd for FrontierNode {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(&rhs))
    }
}

impl Ord for FrontierNode {
    fn cmp(&self, rhs: &Self) -> Ordering {
        // BinaryHeap is a max-heap, so we invert the Ord results to make it a min-heap
        self.estimate().cmp(&rhs.estimate()).reverse()
    }
}

pub struct AStarPath<'a> {
    a_star: &'a AStar,
    current: Option<(usize, usize)>,
}

impl<'a> Iterator for AStarPath<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|current| {
            self.current = self.a_star.visited[current].previous();
            current
        })
    }
}

#[derive(Debug, Clone)]
pub struct AStar {
    visited: Array2<VisitedNode>,
    frontier: BinaryHeap<FrontierNode>,
}

impl AStar {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            visited: Array2::from_elem((width, height), VisitedNode::Unvisited),
            frontier: BinaryHeap::with_capacity(width * height),
        }
    }

    pub fn clear(&mut self) {
        self.visited.fill(VisitedNode::Unvisited);
        self.frontier.clear();
    }

    pub fn add_to_frontier(&mut self, node: FrontierNode, previous: Option<(usize, usize)>) {
        self.visited[node.pos] = if let Some(previous) = previous {
            VisitedNode::VisitedFrom {
                distance: node.distance,
                previous,
            }
        } else {
            VisitedNode::Visited {
                distance: node.distance,
            }
        };

        self.frontier.push(node);
    }

    pub fn visited(&self) -> ArrayView2<VisitedNode> {
        self.visited.view()
    }

    pub fn a_star<S, I, G>(&mut self, mut successors: S, mut goal: G) -> Option<FrontierNode>
    where
        S: FnMut(usize, usize) -> I,
        I: IntoIterator<Item = FrontierNode>,
        G: FnMut(usize, usize) -> bool,
    {
        while let Some(current) = self.frontier.pop() {
            if goal(current.pos.0, current.pos.1) {
                return Some(current);
            }

            for successor in successors(current.pos.0, current.pos.1) {
                let distance = current.distance + successor.distance;

                let better_path =
                    if let Some(visited_distance) = &self.visited[successor.pos].distance() {
                        distance < *visited_distance
                    } else {
                        true
                    };

                if better_path {
                    self.add_to_frontier(
                        FrontierNode {
                            pos: successor.pos,
                            distance,
                            heuristic: successor.heuristic,
                        },
                        Some(current.pos),
                    );
                }
            }
        }

        None
    }

    pub fn extract_path(&self, end: (usize, usize)) -> AStarPath {
        AStarPath {
            a_star: self,
            current: Some(end),
        }
    }

    pub fn a_star_simple<F>(
        &mut self,
        start: (usize, usize),
        goal: (usize, usize),
        travel_fn: F,
    ) -> Option<AStarPath>
    where
        F: Fn(usize, usize) -> Option<u32>,
    {
        let (width, height) = self.visited.dim();

        self.clear();

        self.add_to_frontier(
            FrontierNode {
                pos: start,
                distance: 0,
                heuristic: chebyshev(start, goal),
            },
            None,
        );

        let successors = |x, y| {
            (-1..=1)
                .flat_map(|dy| (-1..=1).map(move |dx| (dx, dy)))
                .filter_map(move |(dy, dx)| {
                    let xx = usize::try_from(x as i64 + dx).ok();
                    let yy = usize::try_from(y as i64 + dy).ok();

                    xx.and_then(|xx| yy.map(|yy| (xx, yy)))
                })
                .filter(|(xx, yy)| *xx < width && *yy < height)
                .filter_map(|(xx, yy)| {
                    travel_fn(xx, yy).map(|distance| FrontierNode {
                        pos: (xx, yy),
                        distance,
                        heuristic: chebyshev((xx, yy), goal),
                    })
                })
        };

        self.a_star(successors, |x, y| (x, y) == goal)
            .map(move |result| self.extract_path(result.pos))
    }
}

fn chebyshev((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> u32 {
    ((x2 as isize - x1 as isize).abs() as u32).max((y2 as isize - y1 as isize).abs() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    use ndarray::array;

    #[test]
    fn test_2d_path_1() {
        let map = test_map();
        check_path(map.view(), &a_star_2d(map.view(), (0, 0), (19, 9)).unwrap());
    }

    #[test]
    fn test_2d_path_1_rev() {
        let map = test_map();
        let path = a_star_2d(map.view(), (0, 0), (19, 9)).unwrap();
        let rev_path = a_star_2d(map.view(), (19, 9), (0, 0)).unwrap();

        check_path(map.view(), &path);
        check_path(map.view(), &rev_path);

        assert_eq!(path.len(), rev_path.len());
    }

    #[test]
    fn test_2d_path_2() {
        let map = test_map();
        check_path(map.view(), &a_star_2d(map.view(), (0, 2), (19, 9)).unwrap());
    }

    #[test]
    fn test_2d_path_2_rev() {
        let map = test_map();
        let path = a_star_2d(map.view(), (0, 2), (19, 9)).unwrap();
        let rev_path = a_star_2d(map.view(), (19, 9), (0, 2)).unwrap();

        check_path(map.view(), &path);
        check_path(map.view(), &rev_path);

        assert_eq!(path.len(), rev_path.len());
    }

    fn a_star_2d(
        map: ArrayView2<bool>,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        let (height, width) = map.dim();

        let mut a_star = AStar::new(height, width);

        a_star
            .a_star_simple(start, goal, |x, y| {
                map.get((x, y)).and_then(|passable| passable.then(|| 1))
            })
            .map(|path| {
                let mut path: Vec<_> = path.collect();
                path.reverse();
                path
            })
    }

    fn test_map() -> Array2<bool> {
        let map = array![
            [1, 0, 1, 1, 1, 1, 1, 1, 0, 1],
            [1, 0, 1, 1, 1, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 1, 1, 0, 1],
            [1, 1, 1, 1, 1, 0, 1, 1, 0, 1],
            [0, 0, 1, 0, 1, 0, 1, 1, 0, 1],
            [0, 0, 1, 1, 1, 0, 1, 1, 0, 1],
            [1, 1, 1, 1, 1, 0, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 1, 1, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
            [1, 0, 1, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 0, 1, 0, 1, 1, 1],
            [1, 0, 1, 0, 1, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 1, 0, 1, 1, 1, 0, 1],
            [1, 0, 1, 1, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 0, 1]
        ];

        map.map(|v| *v > 0)
    }

    fn check_path(map: ArrayView2<bool>, path: &[(usize, usize)]) {
        let mut display = map.map(|v| if *v { ' ' } else { '█' });

        for (x, y) in path.iter() {
            assert!(map[[*x, *y]]);
            display[[*x, *y]] = '.';
        }

        println!("{:?}", path);

        for row in display.rows() {
            for tile in row.iter() {
                print!("{}", tile);
            }
            println!();
        }
    }
}
