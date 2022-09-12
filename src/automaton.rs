use rand::{self, Rng};
use std::collections::BTreeSet;

pub struct Automaton {
    cells: BTreeSet<(u32, u32)>,
    update_list: BTreeSet<(u32, u32)>,
    width: u32,
    height: u32,
}

impl Automaton {
    pub fn new(width: u32, height: u32) -> Self {
        Automaton {
            cells: BTreeSet::new(),
            update_list: BTreeSet::new(),
            width,
            height,
        }
    }

    pub fn set_cells(&mut self, init_array: Vec<(u32, u32)>) {
        for point in &init_array {
            self.update_list.extend(self.get_surrounding(point).iter());
            self.update_list.extend(init_array.clone());
        }
        self.cells.extend(init_array.iter());
    }

    pub fn random_init(&mut self) {
        let ratio = 0.5;
        let mut rng = rand::thread_rng();
        self.cells.clear();
        while self.cells.len() < ((self.width * self.width) as f64 * ratio) as usize {
            self.cells
                .insert((rng.gen_range(0..self.width), rng.gen_range(0..self.width)));
        }
        for point in &self.cells{
            self.update_list.insert(*point);
            self.update_list.extend(self.get_surrounding(point).iter());
        }
    }

    fn get_surrounding(&self, point: &(u32, u32)) -> [(u32, u32); 8] {
        let point = (point.0 as i32, point.1 as i32);
        let mut surroundings: [(u32, u32); 8] = [(0, 0); 8];
        let mut index = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let mut x = point.0 + i;
                x = match x {
                    x if x < 0 => x + self.width as i32,
                    x if x > self.width as i32 - 1 => x - self.width as i32,
                    _ => x,
                };
                let mut y = point.1 + j;
                y = match y {
                    y if y < 0 => y + self.height as i32,
                    y if y > self.height as i32 - 1 => y - self.height as i32,
                    _ => y,
                };
                surroundings[index] = (x as u32, y as u32);
                index += 1;
            }
        }
        surroundings
    }

    pub fn step(&mut self) {
        let mut update_list_new: BTreeSet<(u32, u32)> = BTreeSet::new();
        let mut remove_set = BTreeSet::new();
        let mut insert_set = BTreeSet::new();
        for pair in &self.update_list {
            let surroundings = self.get_surrounding(pair);
            let mut live_count = 0;
            for surrounding in &surroundings {
                if self.cells.get(surrounding).is_some() {
                    live_count += 1;
                }
            }

            if self.cells.get(&pair).is_some() {
                match live_count {
                    0..=1 | 4.. => {
                        remove_set.insert(*pair);
                        update_list_new.extend(surroundings.iter());
                        update_list_new.insert(*pair);
                    }
                    2..=3 => {}
                    _ => {}
                }
            } else {
                match live_count {
                    3 => {
                        insert_set.insert(*pair);
                        update_list_new.extend(surroundings.iter());
                        update_list_new.insert(*pair);
                    }
                    _ => {}
                }
            }
        }
        for point in remove_set {
            self.cells.remove(&point);
        }
        for point in insert_set {
            self.cells.insert(point);
        }
        self.update_list = update_list_new;
    }

    pub fn get_points(&self) -> Vec<(u32, u32)> {
        let mut points: Vec<(u32, u32)> = vec![];
        for pair in &self.cells {
            points.push((pair.0 as u32, pair.1 as u32));
        }
        points
    }
}

#[test]
fn test_surrounding() {
    let automaton = Automaton::new(7, 7);
    assert_eq!(
        automaton.get_surrounding(&(3, 3)),
        [
            (2, 2),
            (2, 3),
            (2, 4),
            (3, 2),
            (3, 4),
            (4, 2),
            (4, 3),
            (4, 4)
        ]
    );

    assert_eq!(
        automaton.get_surrounding(&(0, 0)),
        [
            (6, 6),
            (6, 0),
            (6, 1),
            (0, 6),
            (0, 1),
            (1, 6),
            (1, 0),
            (1, 1)
        ]
    );
}
