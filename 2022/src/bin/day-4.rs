use std::fs::read_to_string;

fn parse_to_u8(text: &str) -> u8 {
    text.parse::<u8>().unwrap()
}

struct Section {
    from: u8,
    to: u8,
}

impl Section {
    fn new(text: &str) -> Self {
        let split: Vec<&str> = text.split('-').collect();
        Section {
            from: parse_to_u8(split[0]),
            to: parse_to_u8(split[1]),
        }
    }
}

struct Sections {
    team_1: Section,
    team_2: Section,
}

impl Sections {
    fn new(line: &str) -> Self {
        let split: Vec<&str> = line.split(',').collect();
        Self {
            team_1: Section::new(split[0]),
            team_2: Section::new(split[1]),
        }
    }
}

fn main() {
    let input = read_to_string("input/day-4.input").unwrap();
    let mut part_1: u16 = 0;
    for line in input.lines() {
        let sections = Sections::new(line);
        if (sections.team_1.from >= sections.team_2.from
            && sections.team_1.to <= sections.team_2.to)
            || (sections.team_1.from <= sections.team_2.from
                && sections.team_1.to >= sections.team_2.to)
        {
            part_1 += 1
        }
    }
    println!(
        "part 1 - amount of assignment pairs contained in the other: {}",
        part_1
    );

    let mut part_2: u16 = 0;
    for line in input.lines() {
        let sections = Sections::new(line);
        if (sections.team_1.from >= sections.team_2.to
            && sections.team_1.to <= sections.team_2.from)
            || (sections.team_1.from <= sections.team_2.to
                && sections.team_1.to >= sections.team_2.from)
        {
            part_2 = part_2 + 1
        }
    }
    println!(
        "part 2 - amount of assignments not overlapping each other: {}",
        part_2
    );
}
