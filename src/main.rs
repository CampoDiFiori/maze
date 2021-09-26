use core::panic;
use std::{collections::HashMap, collections::VecDeque};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
enum FieldState {
    Unhandled,
    InPriorityQueue,
    Handled,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Deserialize, Debug)]
struct DeserializablePoint {
    x: usize,
    y: usize,
    finish: Option<bool>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Serialize)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
struct MazeField {
    turns: usize,
    direction: Direction,
    state: FieldState,
}

struct Maze {
    points: HashMap<Point, MazeField>,
    queue: VecDeque<Point>,
    endpoint: String,
    finish_point: Option<Point>,
}

impl MazeField {
    fn new(turns: usize, direction: Direction, state: FieldState) -> Self {
        Self {
            turns,
            direction,
            state,
        }
    }
}

impl Maze {
    fn new(endpoint: String) -> Self {
        Self {
            points: HashMap::new(),
            queue: VecDeque::new(),
            endpoint,
            finish_point: None,
        }
    }

    fn relax_neighbor(&mut self, curr_point: Point, curr_field: MazeField, direction: Direction) {
        let Point { x, y } = curr_point;

        let neighboring_point = match direction {
            Direction::Down => Point::new(x, y + 1),
            Direction::Up => {
                if y == 0 {
                    return;
                }
                Point::new(x, y - 1)
            }
            Direction::Left => {
                if x == 0 {
                    return;
                }
                Point::new(x - 1, y)
            }
            Direction::Right => Point::new(x + 1, y),
        };

        let turns = if curr_field.direction == direction {
            curr_field.turns
        } else {
            curr_field.turns + 1
        };

        if let Some(neighboring_field) = self.points.get_mut(&neighboring_point) {
            match neighboring_field.state {
                FieldState::Unhandled => {
                    neighboring_field.turns = turns;
                    neighboring_field.direction = direction;
                    neighboring_field.state = FieldState::InPriorityQueue;
                    self.queue.push_back(neighboring_point);
                }
                FieldState::InPriorityQueue => {
                    if turns < neighboring_field.turns {
                        neighboring_field.turns = turns;
                        neighboring_field.direction = direction;
                    }
                }
                FieldState::Handled => {}
            }
        }
    }

    fn pop_queue_front_field(&mut self) -> Option<Point> {
        let curr_point = self.queue.pop_front()?;
        let curr_field = {
            let curr_field = self.points.get_mut(&curr_point)?;
            curr_field.state = FieldState::Handled;
            *curr_field
        };

        self.cache_neighbors(curr_point);

        [
            Direction::Right,
            Direction::Left,
            Direction::Up,
            Direction::Down,
        ]
        .iter()
        .for_each(|&direction| self.relax_neighbor(curr_point, curr_field, direction));

        Some(curr_point)
    }

    fn cache_neighbors(&mut self, point: Point) {
        use websocket::{ClientBuilder, Message, OwnedMessage};
        let mut client = ClientBuilder::new(&self.endpoint)
            .unwrap()
            .connect_insecure()
            .unwrap();

        client
            .send_message(&Message::text(serde_json::to_string(&point).unwrap()))
            .unwrap();

        let response = client.recv_message().unwrap();

        if let OwnedMessage::Text(response) = response {
            let neighbors: Vec<DeserializablePoint> = serde_json::from_str(&response).unwrap();
            neighbors.iter().for_each(|neighboring_point| {
                let point = Point::new(neighboring_point.x, neighboring_point.y);
                if self.points.get(&point).is_none() {
                    self.points.insert(
                        point,
                        MazeField::new(usize::MAX, Direction::Up, FieldState::Unhandled),
                    );
                }

                if neighboring_point.finish.is_some() {
                    self.finish_point = Some(point);
                }
            })
        } else {
            panic!("Reveived a malformed response");
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let endpoint = args[1].clone();

    let mut maze = Maze::new(endpoint);

    let starting_point = Point::new(0, 1);
    let starting_field = MazeField::new(0, Direction::Right, FieldState::InPriorityQueue);

    maze.points.insert(starting_point, starting_field);
    maze.queue.push_back(starting_point);

    while let Some(point) = maze.pop_queue_front_field() {
        println!("Popped {:?} off queue", point);
        if let Some(true) = maze.finish_point.map(|finish_point| finish_point.x == point.x && finish_point.y == point.y) {
            println!(
                "Number of least turns at exit: {:?}",
                maze.points
                    .get(&point)
                    .map(|exit_field| exit_field.turns)
                    .expect("Finish field did not show up")
            );
            return;            
        }
    }

    println!("Finish point did not show up");
}
