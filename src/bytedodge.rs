use std::collections::{BinaryHeap, HashMap};

use crate::grid;

#[derive(Debug)]
pub struct ByteDodge {
    grid: Vec<Vec<char>>,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct WeightedLocation {
    weight: u64,
    location: grid::Location,
}

impl From<Vec<Vec<char>>> for ByteDodge {
    fn from(value: Vec<Vec<char>>) -> Self {
        Self { grid: value }
    }
}

impl Ord for WeightedLocation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for WeightedLocation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ByteDodge {
    pub fn min_steps(&self) -> u64 {
        let (start, end) = self.get_start_end();
        let mut locations: HashMap<grid::Location, u64> =
            HashMap::from([(start.location, start.weight)]);
        let mut unvisited: BinaryHeap<WeightedLocation> = BinaryHeap::from([start]);

        loop {
            let wloc = self.next_wlocation(&locations, &mut unvisited);
            if wloc.location == end {
                return wloc.weight;
            }

            //
        }
    }

    fn next_wlocation(
        &self,
        locations: &HashMap<grid::Location, u64>,
        unvisited: &mut BinaryHeap<WeightedLocation>,
    ) -> WeightedLocation {
        let mut wloc = unvisited.pop().unwrap();
        while locations.get(&wloc.location).unwrap() < &wloc.weight {
            wloc = unvisited.pop().unwrap();
        }
        wloc
    }

    fn get_start_end(&self) -> (WeightedLocation, grid::Location) {
        let start = WeightedLocation {
            weight: 0,
            location: grid::Location { row: 0, col: 0 },
        };
        let end = grid::Location {
            row: self.grid.len() - 1,
            col: self.grid[0].len() - 1,
        };
        (start, end)
    }
}
