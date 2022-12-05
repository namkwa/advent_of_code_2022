use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() -> std::io::Result<()> {
    let file = File::open("day_2/src/input.txt")?;
    let mut buffer = BufReader::new(file);
    let mut content = String::new();
    let _ = buffer.read_to_string(&mut content)?;

    let strategy: Vec<&str> = content.split("\r\n").collect();

    let mut total: i32 = 0;
    for line in strategy {
        match line {
            "A X" => total += 3,
            "A Y" => total += 4,
            "A Z" => total += 8,
            "B X" => total += 1,
            "B Y" => total += 5,
            "B Z" => total += 9,
            "C X" => total += 2,
            "C Y" => total += 6,
            "C Z" => total += 7,
            _ => total += 0,
        }
    }
    print!("{}\n", total);
    Ok(())
}
