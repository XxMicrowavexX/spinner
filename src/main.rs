use rand::Rng;

#[derive(Debug)]
enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

struct Spinner {
    pointing: Directions
}

impl Spinner {
    fn spin(&mut self) {
        let mut rng = rand::thread_rng();
        let number: i32 = rng.gen_range(1..5);

        self.pointing = match number {
            1 => Directions::UP,
            2 => Directions::DOWN,
            3 => Directions::LEFT,
            4 | _ => Directions::RIGHT,
        };

    }
}

fn main() {
    let mut spinner = Spinner{pointing: Directions::UP};
    spinner.spin();

    println!("The Spinner Spun On: {:?}", spinner.pointing);

}
