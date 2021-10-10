use std::{cmp::Ordering, collections::BinaryHeap, convert::TryFrom};

use ndarray::{Array2, ArrayView2};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum VisitedNode {
    Unvisited,
    Visited { distance: u32 },
    VisitedFrom { distance: u32, previous: UPosition },
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

    pub fn previous(&self) -> Option<UPosition> {
        match self {
            Self::Unvisited | Self::Visited { .. } => None,
            Self::VisitedFrom { previous, .. } => Some(*previous),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FrontierNode {
    pub pos: UPosition,
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
    current: Option<UPosition>,
}

impl<'a> Iterator for AStarPath<'a> {
    type Item = UPosition;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|current| {
            self.current = self.a_star.visited[current.to_idx().unwrap()].previous();
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

    pub fn add_to_frontier(&mut self, node: FrontierNode, previous: Option<UPosition>) {
        self.visited[node.pos.to_idx().unwrap()] = if let Some(previous) = previous {
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
        S: FnMut(UPosition) -> I,
        I: IntoIterator<Item = FrontierNode>,
        G: FnMut(UPosition) -> bool,
    {
        while let Some(current) = self.frontier.pop() {
            if goal(current.pos) {
                return Some(current);
            }

            for successor in successors(current.pos) {
                let distance = current.distance + successor.distance;

                let better_path = if let Some(visited_distance) =
                    &self.visited[successor.pos.to_idx().unwrap()].distance()
                {
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

    pub fn extract_path(&self, end: UPosition) -> AStarPath {
        AStarPath {
            a_star: self,
            current: Some(end),
        }
    }

    pub fn a_star_simple<F>(
        &mut self,
        start: UPosition,
        goal: UPosition,
        travel_fn: F,
    ) -> Option<AStarPath>
    where
        F: Fn(UPosition, UPosition) -> Option<u32>,
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

        let successors = |pos| {
            let travel_fn = &travel_fn;

            (-1..=1)
                .flat_map(|dy| (-1..=1).map(move |dx| IPosition::new(dx, dy)))
                .filter_map(move |diff| {
                    UPosition::try_from(IPosition::try_from(pos).unwrap() + diff).ok()
                })
                .filter(|dpos| {
                    usize::try_from(dpos.x).unwrap() < width
                        && usize::try_from(dpos.y).unwrap() < height
                })
                .filter_map(move |dpos| {
                    travel_fn(pos, dpos).map(|distance| FrontierNode {
                        pos: dpos,
                        distance,
                        heuristic: chebyshev(dpos, goal),
                    })
                })
        };

        self.a_star(successors, |pos| pos == goal)
            .map(move |result| self.extract_path(result.pos))
    }
}

fn chebyshev(a: UPosition, b: UPosition) -> u32 {
    ((b.x as isize - a.x as isize).abs() as u32).max((b.y as isize - a.y as isize).abs() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    use ndarray::array;

    #[test]
    fn test_2d_path_1() {
        let map = test_map();
        check_path(
            map.view(),
            &a_star_2d(map.view(), UPosition::new(0, 0), UPosition::new(19, 9)).unwrap(),
        );
    }

    #[test]
    fn test_2d_path_1_rev() {
        let map = test_map();
        let path = a_star_2d(map.view(), UPosition::new(0, 0), UPosition::new(19, 9)).unwrap();
        let rev_path = a_star_2d(map.view(), UPosition::new(19, 9), UPosition::new(0, 0)).unwrap();

        check_path(map.view(), &path);
        check_path(map.view(), &rev_path);

        assert_eq!(path.len(), rev_path.len());
    }

    #[test]
    fn test_2d_path_2() {
        let map = test_map();
        check_path(
            map.view(),
            &a_star_2d(map.view(), UPosition::new(0, 2), UPosition::new(19, 9)).unwrap(),
        );
    }

    #[test]
    fn test_2d_path_2_rev() {
        let map = test_map();
        let path = a_star_2d(map.view(), UPosition::new(0, 2), UPosition::new(19, 9)).unwrap();
        let rev_path = a_star_2d(map.view(), UPosition::new(19, 9), UPosition::new(0, 2)).unwrap();

        check_path(map.view(), &path);
        check_path(map.view(), &rev_path);

        assert_eq!(path.len(), rev_path.len());
    }

    fn a_star_2d(
        map: ArrayView2<bool>,
        start: UPosition,
        goal: UPosition,
    ) -> Option<Vec<UPosition>> {
        let (height, width) = map.dim();

        let mut a_star = AStar::new(height, width);

        a_star
            .a_star_simple(start, goal, |_, pos| {
                map.get(pos.to_idx().unwrap())
                    .and_then(|passable| passable.then(|| 1))
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

    fn check_path(map: ArrayView2<bool>, path: &[UPosition]) {
        let mut display = map.map(|v| if *v { ' ' } else { '█' });

        for pos in path.iter() {
            let idx = pos.to_idx().unwrap();
            assert!(map[idx]);
            display[idx] = '.';
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
