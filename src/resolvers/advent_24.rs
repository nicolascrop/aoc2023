use colored::Color::Black;

#[derive(Debug)]
struct Hailstone {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Hailstone {
    fn get_a_b(&self) -> (f64, f64) {
        /*
         * y = ax + b
         * a = (yb - ya) / (xb - xa)
         * b = -((vy / vx) * px - py)
         */
        let a: f64 = self.y.1 as f64 / self.x.1 as f64;
        let b: f64 = -(a * self.x.0 as f64 - self.y.0 as f64);
        return (a, b);
    }

    pub fn intersect(&self, b: &Hailstone) -> (f64, f64) {
        let h1_a_b = self.get_a_b();
        let h2_a_b = b.get_a_b();
        println!("H1 y = {}x + {}", h1_a_b.0, h1_a_b.1);
        println!("H2 y = {}x + {}", h2_a_b.0, h2_a_b.1);

        // a1*x + b1 = a2*x + b2
        // a1*x - a2*x = b2 - b1
        // (a1 - a2)x = b2 - b1
        // x = (b2 - b1) / (a1 - a2)
        let x = (h2_a_b.1 - h1_a_b.1) / (h1_a_b.0 - h2_a_b.0);
        let y = h1_a_b.0 * x + h1_a_b.1;
        println!("X: {}, Y: {}", x, y);
        return (x, y);
    }
}

const TEST_AREA_MIN: f64 = 200000000000000f64;
const TEST_AREA_MAX: f64 = 400000000000000f64;


pub fn resolve(input: &String) {
    let mut hailstones: Vec<Hailstone> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" @ ").collect();
        let pos: Vec<i64> = parts[0]
            .split(", ")
            .filter(|x| x.len() > 0)
            .map(|p| -> i64 {
                p.trim().parse().unwrap()
            })
            .collect();
        let velocities: Vec<i64> = parts[1]
            .split(", ")
            .filter(|x| x.len() > 0)
            .map(|v| -> i64 {
                v.trim().parse().unwrap()
            })
            .collect();
        assert_eq!(pos.len(), velocities.len(), "Tab size does not match");
        hailstones.push(Hailstone {
            x: (pos[0], velocities[0]),
            y: (pos[1], velocities[1]),
            z: (pos[2], velocities[2]),
        });
    }

    let mut sum = 0;
    let mut sum_with_past = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            println!("\nHailstone 1 {:?}", hailstones[i]);
            println!("Hailstone 2 {:?}", hailstones[j]);
            let intersection_point = hailstones[i].intersect(&hailstones[j]);

            if intersection_point.0 >= TEST_AREA_MIN && intersection_point.0 <= TEST_AREA_MAX && intersection_point.1 >= TEST_AREA_MIN && intersection_point.1 <= TEST_AREA_MAX {
                sum_with_past += 1;
            }

            if !(((intersection_point.0 - hailstones[i].x.0 as f64 > 0f64) == (hailstones[i].x.1 > 0)) &&
                ((intersection_point.1 - hailstones[i].y.0 as f64 > 0f64) == (hailstones[i].y.1 > 0))) {
                continue;
            }
            if !(((intersection_point.0 - hailstones[j].x.0 as f64 > 0f64) == (hailstones[j].x.1 > 0)) &&
                ((intersection_point.1 - hailstones[j].y.0 as f64 > 0f64) == (hailstones[j].y.1 > 0))) {
                continue;
            }
            // Test if intersection is in past

            if intersection_point.0 >= TEST_AREA_MIN && intersection_point.0 <= TEST_AREA_MAX && intersection_point.1 >= TEST_AREA_MIN && intersection_point.1 <= TEST_AREA_MAX {
                println!("Collide at {:?}", intersection_point);
                sum += 1;
            }
        }
    }
    println!("Nb intersection in test area {} | {}", sum, sum_with_past);

}