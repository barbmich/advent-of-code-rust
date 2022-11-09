use std::fs::read_to_string;

#[derive(Debug)]
struct Prism {
    dim_a: u64,
    dim_b: u64,
    dim_c: u64,
}

#[derive(Debug)]
struct Dimensions {}

impl Prism {
    fn new(dim_1: u64, dim_2: u64, dim_3: u64) -> Self {
        Self {
            dim_a: dim_1,
            dim_b: dim_2,
            dim_c: dim_3,
        }
    }

    fn side_surface(&self, val1: u64, val2: u64) -> u64 {
        val1 * val2
    }

    fn surface(&self) -> u64 {
        self.side_surface(self.dim_a, self.dim_b) * 2
            + self.side_surface(self.dim_b, self.dim_c) * 2
            + self.side_surface(self.dim_c, self.dim_a) * 2
    }

    fn volume(&self) -> u64 {
        self.dim_a * self.dim_b * self.dim_c
    }

    fn get_smallest_side_size(&self) -> u64 {
        *[
            self.side_surface(self.dim_a, self.dim_b),
            self.side_surface(self.dim_b, self.dim_c),
            self.side_surface(self.dim_c, self.dim_a),
        ]
        .iter()
        .min()
        .unwrap()
    }

    fn get_smallest_perimeter_size(&self) -> u64 {
        let arr_of_perimeters = [
            get_perimeter(self.dim_a, self.dim_b),
            get_perimeter(self.dim_b, self.dim_c),
            get_perimeter(self.dim_c, self.dim_a),
        ];
        *arr_of_perimeters.iter().min().unwrap()
    }
}

fn get_perimeter(dim_1: u64, dim_2: u64) -> u64 {
    dim_1 * 2 + dim_2 * 2
}

fn main() {
    let input = read_to_string("input/day-2.input").unwrap();
    let mut arr_of_prisms: Vec<Prism> = vec![];

    for line in input.lines() {
        let arr_of_prism: Vec<u64> = line.split('x').map(|x| x.parse::<u64>().unwrap()).collect();
        let prism = Prism::new(arr_of_prism[0], arr_of_prism[1], arr_of_prism[2]);

        arr_of_prisms.push(prism);
    }

    let mut total_surface: u64 = 0;

    arr_of_prisms
        .iter()
        .for_each(|prism| total_surface += prism.surface() + prism.get_smallest_side_size());

    println!("total surface needed: {}", total_surface);

    let mut total_ribbon_length: u64 = 0;

    arr_of_prisms.iter().for_each(|prism| {
        total_ribbon_length += prism.volume() + prism.get_smallest_perimeter_size()
    });

    println!("total ribbon length: {}", total_ribbon_length);
}
