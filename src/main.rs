use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::rc::Rc;

extern crate day13;
use day13::points::Point;

fn main() {
    //let seed = 10;
    let seed = 1362;
    let start = Point::new(1, 1,0);
    //let finish = Point::new(7, 4, 0);
    let finish = Point::new(31, 39, 0);
    let mut closed = HashSet::new();
    let mut opened = BinaryHeap::new();
    opened.push(start.clone());

    while let Some(point) = opened.pop() {
        if closed.contains(&point) {
            continue;
        }
        if point.track > 50 {
           continue;
        }
        /*if point == finish {
            point.traverse_back();
            break
        }*/
        let descendants = Point::descendants(&point, finish.as_ref());
        let filtered = descendants.as_ref().into_iter()
            .filter(|p| p.check_empty(seed));

        for p in filtered {
            opened.push(p.to_owned());
        }
        closed.insert(point);
    }

    println!("Total is {}", closed.len());
    draw(32, 40, seed, &closed);
    let mut result: Vec<Rc<Point>> = closed.into_iter().collect();
    result.sort_unstable_by_key(|p| p.track);
    for p in result {
        p.print();
    }
    //println!("The distance is {}", Point::distance(&start, &finish));
}

fn draw (x_in: isize, y_in: isize, seed: isize, container: &HashSet<Rc<Point>>) {
    print!("\t");
    for r in 0..x_in {
        print!("{} ", r);
    }
    println!();
    for y in 0..y_in {
        print!("{}\t", y);
        for x in 0..x_in {
            let probe = Point::new(x, y, 0);
            if container.contains(&probe) {
                print!("O ");
            } else if probe.check_empty(seed) {
                print!(". ");
            } else {
                print!("# ");
            }
        }
        println!();
    }
}
