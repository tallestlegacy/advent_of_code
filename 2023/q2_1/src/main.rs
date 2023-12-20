use std::fs;

const INPUT_FILE_NAME: &str = "input.txt";

#[derive(Debug, Clone)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}
#[derive(Debug, Clone)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

const REF_SET: Set = Set {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() {
    let contents =
        fs::read_to_string(INPUT_FILE_NAME).expect("Something went wrong reading the file");

    let lines = contents.lines().into_iter().collect::<Vec<&str>>();

    let mut sum = 0;

    for line in lines {
        let game = get_game_from_line(line);
        if is_game_valid(game.clone()) {
            println!("Game {} is valid", game.id);
            sum += game.id;
        }
    }

    println!("Ans: {}", sum);
}

fn get_game_from_line(line: &str) -> Game {
    // assuming all lines follow the pattern
    // "Game id: a color, b color; i color, j color; x color,y color"

    let mut game = Game {
        id: line.split(":").collect::<Vec<&str>>()[0]
            .split(" ")
            .collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap(),
        sets: Vec::new(),
    };

    for i in line.split(":").collect::<Vec<&str>>()[1]
        .split(";")
        .collect::<Vec<&str>>()
    {
        game.sets.push(get_set_from_round(i));
    }

    game
}

fn get_set_from_round(round: &str) -> Set {
    let mut set = Set {
        red: 0,
        green: 0,
        blue: 0,
    };

    for draw in round.split(",").collect::<Vec<&str>>() {
        let draw_split = draw
            .strip_prefix(" ")
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>();
        let value = draw_split[0].parse::<u32>();
        let color = draw_split[1];

        match color {
            "red" => set.red = value.unwrap(),
            "green" => set.green = value.unwrap(),
            "blue" => set.blue = value.unwrap(),
            _ => panic!("invalid color {color}"),
        }
    }

    set
}

fn is_game_valid(game: Game) -> bool {
    for set in game.sets.iter() {
        if set.red > REF_SET.red {
            return false;
        }
        if set.green > REF_SET.green {
            return false;
        }
        if set.blue > REF_SET.blue {
            return false;
        }
    }
    true
}
