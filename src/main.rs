use core::panic;
use std::{collections::HashMap, collections::VecDeque};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
enum FieldState {
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

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Serialize, Deserialize)]
struct Point {
    x: usize,
    y: usize,
    finish: Option<bool>,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y, finish: None }
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
    fn new() -> Self {
        Self {
            points: HashMap::new(),
            queue: VecDeque::new(),
        }
    }

    fn relax_neighbor(&mut self, curr_point: Point, curr_field: MazeField, direction: Direction) {
        let Point { x, y, finish } = curr_point;

        let neighboring_point = match direction {
            Direction::Down => {
                if y == 7 {
                    return;
                }
                Point::new(x, y + 1)
            }
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
            Direction::Right => {
                if x == 8 {
                    return;
                }
                Point::new(x + 1, y)
            }
        };

        let neighboring_field = self.points.get_mut(&neighboring_point);

        let turns = if curr_field.direction == direction {
            curr_field.turns
        } else {
            curr_field.turns + 1
        };

        if let Some(neighboring_field) = neighboring_field {
            match neighboring_field.state {
                FieldState::Handled => {}
                FieldState::InPriorityQueue => {
                    if turns < neighboring_field.turns {
                        neighboring_field.turns = turns;
                        neighboring_field.direction = direction;
                    }
                }
            }
        } else if MAZE[neighboring_point.x][neighboring_point.y] == 1 {
            let unhandled_neighbor = MazeField::new(turns, direction, FieldState::InPriorityQueue);
            self.queue.push_back(neighboring_point);
            self.points.insert(neighboring_point, unhandled_neighbor);
        }
    }

    fn relax_field(&mut self) -> Option<Point> {
        let curr_point = self.queue.pop_front()?;
        let curr_field = {
            let curr_field = self.points.get_mut(&curr_point)?;
            curr_field.state = FieldState::Handled;
            *curr_field
        };

        [
            Direction::Right,
            Direction::Left,
            Direction::Up,
            Direction::Down,
        ]
        .iter()
        // .filter(|&&direction| direction != curr_point.get().direction.opposite())
        .for_each(|&direction| self.relax_neighbor(curr_point, curr_field, direction));

        Some(curr_point)
    }
}

// const MAZE: [[u8; 9]; 8] = [
//     [0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [1, 1, 1, 1, 0, 1, 1, 1, 0],
//     [0, 1, 0, 1, 0, 1, 0, 1, 0],
//     [0, 1, 0, 1, 1, 1, 0, 1, 0],
//     [0, 1, 0, 0, 0, 0, 0, 1, 0],
//     [0, 1, 1, 1, 1, 0, 0, 1, 0],
//     [0, 1, 0, 0, 1, 1, 1, 1, 1],
//     [0, 0, 0, 0, 0, 0, 0, 0, 0],
// ];

const MAZE: [[u8; 8]; 9] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 0, 0, 0, 1, 0, 1],
    [0, 1, 1, 1, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 1, 1, 0],
    [0, 1, 1, 1, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [0, 0, 0, 0, 0, 0, 1, 0],
];

fn main() {
    let mut maze = Maze::new();

    let starting_point = Point::new(0, 1);
    let starting_field = MazeField::new(0, Direction::Right, FieldState::InPriorityQueue);

    maze.points.insert(starting_point, starting_field);

    //   ws://rekrutacja.westeurope.cloudapp.azure.com/maze1
    //   ws://rekrutacja.westeurope.cloudapp.azure.com/maze2
    //   ws://rekrutacja.westeurope.cloudapp.azure.com/maze3
    //   ws://rekrutacja.westeurope.cloudapp.azure.com/maze4

    maze.queue.push_back(starting_point);

    while let Some(point) = maze.relax_field() {
        println!("{:?}", point);
    }

    // println!("{:?}", maze.points.get(&Point::new(6, 8)));

    for (i, row) in MAZE.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if let Some(field) = maze.points.get(&Point::new(i, j)) {
                print!("{} ", field.turns);
            } else {
                print!("- ");
            }
        }
        println!();
    }

    check_neighbors(Point::new(0, 1));
}

fn check_neighbors(point: Point) -> Vec<Point> {
    use websocket::{ClientBuilder, Message, OwnedMessage};
    let mut client = ClientBuilder::new("ws://rekrutacja.westeurope.cloudapp.azure.com/maze2")
        .unwrap()
        .connect_insecure()
        .unwrap();

    client
        .send_message(&Message::text(serde_json::to_string(&point).unwrap()))
        .unwrap();

    let response = client.recv_message().unwrap();

    if let OwnedMessage::Text(response) = response {
        println!("{}", response);
        let neighbors: Vec<Point> = serde_json::from_str(&response).unwrap();
        println!("{:?}", neighbors);
        return neighbors;
    }

    panic!("Couldnt send a ws message");
}
