use crate::{Drawable, math, Screen};

#[allow(dead_code)]
pub struct Line{
    point1: math::Vector2<i32>,
    point2: math::Vector2<i32>,
    color: char,
}

impl Drawable for Line {
    fn draw(&self, destination:&Screen) {
        /*
        public void line(int x,int y,int x2, int y2, int color) {
    int w = x2 - x ;
    int h = y2 - y ;
    int dx1 = 0, dy1 = 0, dx2 = 0, dy2 = 0 ;
    if (w<0) dx1 = -1 ; else if (w>0) dx1 = 1 ;
    if (h<0) dy1 = -1 ; else if (h>0) dy1 = 1 ;
    if (w<0) dx2 = -1 ; else if (w>0) dx2 = 1 ;
    int longest = Math.abs(w) ;
    int shortest = Math.abs(h) ;
    if (!(longest>shortest)) {
        longest = Math.abs(h) ;
        shortest = Math.abs(w) ;
        if (h<0) dy2 = -1 ; else if (h>0) dy2 = 1 ;
        dx2 = 0 ;            
    }
    int numerator = longest >> 1 ;
    for (int i=0;i<=longest;i++) {
        putpixel(x,y,color) ;
        numerator += shortest ;
        if (!(numerator<longest)) {
            numerator -= longest ;
            x += dx1 ;
            y += dy1 ;
        } else {
            x += dx2 ;
            y += dy2 ;
        }
    }
}
         */
        let (mut x, mut y, mut x2, mut y2) = (self.point1.x, self.point1.y, self.point2.x, self.point2.y);
        let w = x2 - x;
        let h = y2 - y;
        let (mut dx1, mut dy1, mut dx2, mut dy2) = (0,0,0,0);
        
        if w < 0 {dx1 = -1;} else if w > 0 {dx1 = 1;}
        if h < 0 {dy1 = -1;} else if h > 0 {dy1 = 1;}
        if w < 0 {dx2 = -1;} else if x > 0 {dx2 = 1;}

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
            destination.set_at(x as usize, y as usize, self.color);
            numerator += shortest;
            if !(numerator < longest){
                numerator -= longest;
                x += dx1;
                y += dy1;
            }else {
                x += dx2;
                y += dy2;
            }
        }
    }
}

impl Line {
    pub fn new(p1: math::Vector2<i32>, p2: math::Vector2<i32>, color: char) -> Self{
        Line {
                point1: p1,
                point2: p2,
                color: color,
            }
    }
}
