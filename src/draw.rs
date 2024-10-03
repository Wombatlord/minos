use svg::{
    node::element::{path::Data, Path},
    Document,
};

use crate::{
    graph::{Cardinal, Node},
    Maze,
};

const CELL_SIDE: usize = 10;
const STROKE_WIDTH: usize = 2;

fn make_line(from: (usize, usize), relative_to: (usize, usize)) -> Path {
    let data = Data::new().move_to(from).line_by(relative_to);

    Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", STROKE_WIDTH)
        .set("stroke-linejoin", "square")
        .set("stroke-linecap", "square")
        .set("d", data)
}

fn add_node_paths(paths: &mut Vec<Path>, maze: &Maze, node: Node, walls: Vec<Cardinal>) {
    let left_corner = (node.x * CELL_SIDE, node.y * CELL_SIDE);
    let (left_corner_x, left_corner_y) = left_corner;

    for wall in walls {
        match wall {
            Cardinal::North => {
                let path = make_line(left_corner, (CELL_SIDE, 0));
                paths.push(path)
            }
            Cardinal::East => {
                if node.x == maze.width - 1 {
                    let path =
                        make_line((left_corner_x + CELL_SIDE, left_corner_y), (0, CELL_SIDE));
                    paths.push(path)
                }
            }
            Cardinal::South => {
                if node.y == maze.height - 1 {
                    let path =
                        make_line((left_corner_x, left_corner_y + CELL_SIDE), (CELL_SIDE, 0));
                    paths.push(path)
                }
            }
            Cardinal::West => {
                let path = make_line(left_corner, (0, CELL_SIDE));
                paths.push(path)
            }
        }
    }
}

pub fn draw(maze: &Maze) -> Document {
    let mut paths = vec![];
    for row in 0..maze.height {
        for col in 0..maze.width {
            let n = Node { x: row, y: col };
            add_node_paths(&mut paths, maze, n, maze.walls.get(&n).unwrap().to_vec());
        }
    }

    let (width, height) = (maze.width * CELL_SIDE, maze.height * CELL_SIDE);

    let document = Document::new()
        .set("viewBox", (0, 0, width, height))
        .set("style", "background-color: white;");
    let d = paths.into_iter().fold(document, |doc, path| doc.add(path));

    return d;
}
