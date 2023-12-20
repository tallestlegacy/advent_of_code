const INPUT_FILE_NAME: &str = "input.txt";

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn main() {
    let contents =
        std::fs::read_to_string(INPUT_FILE_NAME).expect("Something went wrong reading the file");

    let lines = contents.lines().into_iter().collect::<Vec<&str>>();

    let mut sum = 0;

    for line in lines {
        let game = get_game_from_line(line);
        let p = power_of_minimum_set(&game);
        println!("Game {}: \t{}", game.id, p);
        sum += p;
    }

    println!("Ans: {}", sum);
}

fn power_of_minimum_set(game: &Game) -> u32 {
    let mut minimum_set = Set {
        red: 0,
        green: 0,
        blue: 0,
    };

    for set in &game.sets {
        minimum_set.red = std::cmp::max(minimum_set.red, set.red);
        minimum_set.green = std::cmp::max(minimum_set.green, set.green);
        minimum_set.blue = std::cmp::max(minimum_set.blue, set.blue);
    }

    minimum_set.red * minimum_set.green * minimum_set.blue
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
