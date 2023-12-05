use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_color_number(mut input: String) -> bool {  // " 15 red"
    
    let red = 12;
    let blue = 14;
    let green = 13;

    let mut output = true;
    let input = clean_start_space(input); // "15 red"
    let choices = input.split(" ").collect::<Vec<_>>(); // 
    
    let n = choices[0].parse::<i32>().unwrap(); // get number    
    let color = choices[1]; // get color
    
    match color {
        "blue" => {
            if blue < n {output = false};
        }
        "red" => {
            if red < n {output = false};
        }
        "green" => {
            if green < n {output = false}
        }
        _ => {panic!()}
    }
    output    
}

fn validate_game(input: String) -> bool {
    let mut output = true;
    let mut games = input.split(";"); //"3 red, 5 green, 1 blue"
    for game in games {
        println!("{}", game);
        let colors = game.split(","); // "5 green"
        for color in colors {
            //println!("{}", color);
            output = check_color_number(color.to_string());
            if !output {return false}
        }

    }
    
    output
}

fn game_file_reader() -> Result<(), std::io::Error> {
    let file = File::open("advent2.txt")?;
    let mut valid_gamecount = 0;
    let reader = BufReader::new(file);
    for line in reader.lines(){
        // get next line if exists
        let next_line = line?;

        //split either side of :
        let mut line_split = next_line.split(':').collect::<Vec<_>>();
        let game_number = line_split[0];
        let game_result = line_split[1];
        println!("{}", game_number);
        if validate_game(game_result.to_string()) {
            let gn_split = game_number.split(' ').collect::<Vec<_>>();
            let g_number = gn_split[1].parse::<i32>().unwrap();
            valid_gamecount += g_number;
            println!("{} is valid, total so far - {}", game_result, valid_gamecount);
        }
        else {
            println!("{} is INVALID", game_result);
            
        }
    }
    println!("total number of valid games is {}", valid_gamecount);
    Ok(())
}


// clean up results like " 15 red"
fn clean_start_space(mut input: String) -> String {
    let mut charlie = input.chars();
    if charlie.nth(0).unwrap() == ' ' {
        input = input[1..].to_string();
    }
    input

}

fn main() {
    
    game_file_reader();

    
}
