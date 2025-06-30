struct Reindeer {
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

pub fn day_14(input: &str) {
    // parse the reindeer info
    let input = input.replace("\r", "");
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let mut reindeer: Vec<Reindeer> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let n = parts.len();

        reindeer.push(Reindeer {
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

    println!("Part 1: {}", max_distance);
    println!("Part 2: {}", max_points);
}
