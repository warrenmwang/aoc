use std::fs;

struct Reindeer {
    // name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
    is_flying: bool,
    counter: u32,
    distance: u32,
    points: u32,
}

impl Reindeer {
    fn add_point(&mut self) {
        self.points += 1;
    }

    fn iterate(&mut self) {
        if self.is_flying {
            self.distance += self.speed;

            self.counter += 1;
            if self.counter == self.fly_time {
                self.is_flying = false;
                self.counter = 0;
            }
        } else {
            self.counter += 1;
            if self.counter == self.rest_time {
                self.is_flying = true;
                self.counter = 0;
            }
        }
    }
}

fn main() {
    // parse the reindeer info
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = input.trim().split("\r\n").collect();

    let mut reindeer: Vec<Reindeer> = Vec::new();
    for line in lines {
        let parts: Vec<_> = line.split(" ").collect();
        let n = parts.len();

        reindeer.push(Reindeer {
            // name: String::from(parts[0]),
            speed: parts[3].parse::<u32>().unwrap(),
            fly_time: parts[6].parse::<u32>().unwrap(),
            rest_time: parts[n - 2].parse::<u32>().unwrap(),
            is_flying: true,
            counter: 0,
            distance: 0,
            points: 0,
        });
    }

    // run sim
    for _ in 0..2503 {
        for r in reindeer.iter_mut() {
            r.iterate();
        }
        // part 2 - point for being in the lead
        let curr_max_distance = reindeer
            .iter_mut()
            .max_by_key(|r| r.distance)
            .unwrap()
            .distance;

        reindeer
            .iter_mut()
            .filter(|r| r.distance == curr_max_distance)
            .for_each(|r| r.add_point());
    }

    let mut max_distance = 0;
    let mut max_points = 0;
    reindeer.iter().for_each(|r| {
        if r.distance > max_distance {
            max_distance = r.distance;
        }
        if r.points > max_points {
            max_points = r.points;
        }
    });

    println!("{}", max_distance);
    // 59100 is too high
    // 2695 correct
    println!("{}", max_points);
    // 1084 correct
}
