use std::fs::File;
use std::io::{BufRead, BufReader};
#[allow(warnings)]
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
            output = check_color_number(color.to_string());
            if !output {return false}
        }

    }
    
    output
}

fn part1_game_file_reader() -> Result<(), std::io::Error> {
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

fn the_power(input: String, red: &mut i32, blue: &mut i32, green: &mut i32, )  { // red = 1, blue = 2, green = 3 : power = 1*2*3=6
    //10 red, 5 blue, 1 green
  /*   let mut red = 0;
    let mut blue = 0;
    let mut green = 0; */
    let games = input.split(",");    
    for game in games {

        let game = clean_start_space(game.to_string()); // "15 red"
        //println!("{}", game);
        
        let choices = game.split(" ").collect::<Vec<_>>(); //  ["15", "red"]  
        let n: i32 = choices[0].parse::<i32>().unwrap(); // 15   
        let color: &str = choices[1]; // "red"
                
        match color {
            "red" => {
                if n > *red {*red = n}
            }
            "blue" => {
                if n > *blue {*blue = n}
            }
            "green" => {
                if n > *green {*green = n}
            }
            _ => panic!()
        }
    }
/*     let output = red * blue * green;
    println!("{} * {} * {} = {} ", red, green, blue, output);
    output */

    //let choices = input.split(" ").collect::<Vec<_>>(); // 
    //let n = choices[0].parse::<i32>().unwrap(); // get number    

}

fn part2_game_file_reader() -> Result<(), std::io::Error> {
    let file = File::open("advent2.txt")?;

    let mut power = 0;
    let reader = BufReader::new(file);
    for line in reader.lines(){
        // get next line if exists
        let next_line = line?;
        // reset for every line
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        //split either side of :
        let line_split = next_line.split(':').collect::<Vec<_>>();
        let game_number = line_split[0];
        let game_result = line_split[1];
        let games = game_result.split(";"); //"3 red, 5 green, 1 blue"
        for game in games {
            the_power(game.to_string(), &mut red, &mut blue, &mut green);
        }
        let delta = red*blue*green;
        power += delta;
        print!("{}: red {}, green {}, blue {}", game_number, red, green,blue);
        println!("-> delta = {}, power(total) = {}", delta, power);
    }
    println!("total power is {}", power);
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
    
    //part1_game_file_reader();
    //let game_2_test = "10 red, 5 blue, 1 green".to_string();
    //the_power(game_2_test); 

    part2_game_file_reader();

    
}
