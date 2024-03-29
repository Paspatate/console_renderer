use std::fmt::Display;

use crate::{math, Drawable, Screen};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Line {
    pub point1: math::Vector2<i32>,
    pub point2: math::Vector2<i32>,
    pub color: char,
}

#[allow(dead_code)]
pub struct Cube {
    position: math::Vector3<i32>,
    vertices: [math::Vector3<i32>; 8],
}

impl Drawable for Line {
    fn draw(&self, destination: &Screen) {
        // Bresenham algorithm modified to work on all octant
        // original code not from me
        let (mut x, mut y, x2, y2) = (self.point1.x, self.point1.y, self.point2.x, self.point2.y);
        let w = x2 - x;
        let h = y2 - y;
        let (mut dx1, mut dy1, mut dx2, mut dy2) = (0, 0, 0, 0);

        if w < 0 {
            dx1 = -1;
        } else if w > 0 {
            dx1 = 1;
        }
        if h < 0 {
            dy1 = -1;
        } else if h > 0 {
            dy1 = 1;
        }
        if w < 0 {
            dx2 = -1;
        } else if x > 0 {
            dx2 = 1;
        }

        let mut longest = w.abs();
        let mut shortest = h.abs();

        if !(longest > shortest) {
            longest = h.abs();
            shortest = w.abs();
            if h < 0 {
                dy2 = -1;
            } else if h > 0 {
                dy2 = 1;
            }
            dx2 = 0;
        }

        let mut numerator: i32 = longest >> 1; // divide by 2;
        for _i in 0..longest {
            if x >= 0 && y >= 0 {
                destination.set_at(x as usize, y as usize, self.color);
            }
            numerator += shortest;
            if !(numerator < longest) {
                numerator -= longest;
                x += dx1;
                y += dy1;
            } else {
                x += dx2;
                y += dy2;
            }
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[({}, {}), ({}, {})]",
            self.point1.x, self.point1.y, self.point2.x, self.point2.y
        )
    }
}
impl Line {
    pub fn new(p1: math::Vector2<i32>, p2: math::Vector2<i32>, color: char) -> Self {
        Line {
            point1: p1,
            point2: p2,
            color: color,
        }
    }
}

impl Cube {
    pub fn new(edge_size: i32, position: &math::Vector3<i32>) -> Cube {
        let mut vertices = [math::Vector3::<i32>::new(position.x, position.y, position.z); 8];
        vertices[1].x += edge_size;

        vertices[2].x += edge_size;
        vertices[2].y += edge_size;

        vertices[3].y += edge_size;

        vertices[4].z += edge_size;

        vertices[5].x += edge_size;
        vertices[5].z += edge_size;

        vertices[6].x += edge_size;
        vertices[6].y += edge_size;
        vertices[6].z += edge_size;

        vertices[7].y += edge_size;
        vertices[7].z += edge_size;
        Cube {
            vertices,
            position: position.clone(),
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, destination: &Screen) {
        //calculate de projection of each vertice
        //create the line and draw the line

        let mut projected_vertice = Vec::new();
        //calculation of the projection
        for vertex in self.vertices {
            // calculated to project from the center of the screen
            let x_screen_proj: i32 =
                (destination.focal_lenght * vertex.x) / (destination.focal_lenght + vertex.z);
            let y_screen_proj: i32 =
                (destination.focal_lenght * vertex.y) / (destination.focal_lenght + vertex.z);

            // translation to get the draw coordinate
            // the destination.size is a tuple made as (num rows, num columns)
            let x_drawing_proj: i32 = (destination.size.1 as i32 / 2) + x_screen_proj;
            let y_drawing_proj: i32 = (destination.size.0 as i32 / 2) + y_screen_proj;

            projected_vertice.push(math::Vector2::new(x_drawing_proj, y_drawing_proj));
        }

        let color = '#';
        let mut lines = Vec::new();
        // front face
        lines.push(Line::new(projected_vertice[0], projected_vertice[1], color));
        lines.push(Line::new(projected_vertice[1], projected_vertice[2], color));
        lines.push(Line::new(projected_vertice[2], projected_vertice[3], color));
        lines.push(Line::new(projected_vertice[3], projected_vertice[0], color));

        // back face
        lines.push(Line::new(projected_vertice[4], projected_vertice[5], color));
        lines.push(Line::new(projected_vertice[5], projected_vertice[6], color));
        lines.push(Line::new(projected_vertice[6], projected_vertice[7], color));
        lines.push(Line::new(projected_vertice[7], projected_vertice[4], color));

        // link between the two
        lines.push(Line::new(projected_vertice[0], projected_vertice[4], color));
        lines.push(Line::new(projected_vertice[1], projected_vertice[5], color));
        lines.push(Line::new(projected_vertice[2], projected_vertice[6], color));
        lines.push(Line::new(projected_vertice[3], projected_vertice[7], color));

        lines.iter().for_each(|elem| elem.draw(destination));
    }
}
