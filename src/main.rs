use maze::{Direction, FieldState, Maze, MazeField, Point};

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
