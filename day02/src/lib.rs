enum Command {
    Forward(i64),
    Down(i64),
    Up(i64)
}

fn parse(line: &str) -> Command {
    let parts = line.split(" ").collect::<Vec<&str>>();
    let arg = parts[1].parse::<i64>().unwrap();
    match parts[0] {
        "forward" => Command::Forward(arg),
        "down" => Command::Down(arg),
        "up" => Command::Up(arg),
        _ => panic!("error parsing {}", parts[0])
    }
}

struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn updated(self, c: Command) -> Position {
        match c {
            Command::Forward(dx) => Position{x: self.x + dx, ..self},
            Command::Down(dy) => Position{y: self.y + dy, ..self},
            Command::Up(dy) => Position{y: self.y - dy, ..self},
        }
    }
}

pub fn part1(input: &Vec<&str>) -> i64 {
    let end_pos = input.iter()
        .map(|line| parse(line))
        .fold(Position{x: 0, y: 0}, |p, c| p.updated(c));
    end_pos.x * end_pos.y
}

struct State {
    pos: Position,
    aim: i64,
}

impl State {
    fn updated(self, c: Command) -> State {
        match c {
            Command::Forward(d) => State{
                pos: Position {x: self.pos.x + d, y: self.pos.y + d * self.aim},
            ..self},
            Command::Down(dy) => State{aim: self.aim + dy, ..self},
            Command::Up(dy) => State{aim: self.aim - dy, ..self},
        }
    }
}

pub fn part2(input: &Vec<&str>) -> i64 {
    let end = input.iter()
        .map(|line| parse(line))
        .fold(State{pos: Position{x: 0, y: 0}, aim: 0}, |s, c| s.updated(c));
    end.pos.x * end.pos.y
}
