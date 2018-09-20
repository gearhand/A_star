use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

#[derive(Clone)]
pub struct Point {
    x: isize,
    y: isize,
    pub track: usize,
    heuristic: usize,
    parent: Option<Rc<Point>>,
}

impl PartialEq for Point {
    fn eq (&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl PartialOrd for Point {
    fn partial_cmp (&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp (&self, other: &Point) -> Ordering {
        other.heuristic.cmp(&self.heuristic) // Inversion, motherfucker!
    }
}

impl Hash for Point {
    fn hash<H> (&self, state: &mut H)
    where H: Hasher
    {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Point {
    pub fn new (x: isize, y: isize, track: usize) -> Rc<Point> {
        Rc::new(Point {x, y, track, heuristic: 0, parent: None})
    }

    pub fn from_parent (x: isize, y: isize, parent: &Rc<Point>, finish: &Point) -> Rc<Point> {
        let mut temp = Box::new(Point {
            x,
            y,
            track: parent.track + 1,
            heuristic: 0,
            parent: Some(parent.clone())
        });
        //temp.heuristic(finish);
        Rc::from(temp)
    }

    pub fn distance (lhs: &Point, rhs: &Point) -> usize {
        let result = (lhs.x - rhs.x).abs() + (lhs.y - rhs.y).abs();
        result as usize
    }

    pub fn heuristic (&mut self, finish: &Point) -> usize {
        self.heuristic = self.track + Point::distance(&self, finish);
        self.heuristic
    }

    pub fn descendants (parent: &Rc<Point>, finish: &Point) -> [Rc<Point>; 4] {
        let result: [Rc<Point>; 4] = [
            Point::from_parent(parent.x, parent.y + 1, parent, finish),
            Point::from_parent(parent.x + 1, parent.y, parent, finish),
            Point::from_parent(parent.x, parent.y - 1, parent, finish),
            Point::from_parent(parent.x - 1, parent.y, parent, finish),
        ];
        result
    }

    pub fn check_empty (&self, seed: isize) -> bool {
        let x = self.x;
        let y = self.y;
        if x < 0 || y < 0 {
            return false
        }
        let mut checksum = x.pow(2) + 3*x + 2*x*y + y + y.pow(2) + seed;
        let mut bits = 0;
        while checksum > 0 {
            bits += checksum % 2;
            checksum = checksum >> 1;
        }
        bits % 2 == 0
    }

    pub fn traverse_back (&self) {
        self.print();
        if let Some(ref p) = self.parent {
            p.traverse_back();
        }
    }

    pub fn print (&self) {
        println!("Point: ({}, {}), track {}, heuristic {}.",
                 self.x,
                 self.y,
                 self.track,
                 self.heuristic
        );
    }
}

/*
pub struct Solution<'a> {
    distance: usize,
    heuristic: usize,
    track: Vec<Point>,
    finish: Point,
}

impl<'a> Solution<'a> {
    pub fn append(mut self, point: Point<'a>) {
        self.distance += 1;
        self.heuristic = self.distance + Point::distance(&point, self.finish);
        self.track.push(point);
    }
}

impl<'a> PartialEq for Solution<'a> {
    fn eq (&self, other: &Solution) -> bool {
        self.distance == other.distance
        && self.heuristic == other.heuristic
        && self.track == other.track
    }
}

impl<'a> Eq for Solution<'a> {}

impl<'a> Ord for Solution<'a> {
    fn cmp (&self, other: &Solution) -> Ordering {
        other.heuristic.cmp(&self.heuristic)
    }
}

impl<'a> PartialOrd for Solution<'a> {
    fn partial_cmp (&self, other: &Solution) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Hash for Solution<'a> {
    fn hash<H> (&self, state: &mut H)
    where H: Hasher
    {
        self.heuristic.hash(state);
    }
}
*/
