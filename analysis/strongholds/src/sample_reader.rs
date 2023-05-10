use std::{io::{BufReader, BufRead}, fs::File};

pub fn read_sample(reader: &mut BufReader<File>) -> Option<[[i64; 2]; 3]> {
    let mut buf = String::new();
    reader.read_line(&mut buf).ok()?;

    let raw_positions: Vec<&str> = buf.split(';').collect();

    let mut positions = [[0; 2]; 3];

    for (i, position) in raw_positions.into_iter().take(3).enumerate() {
        let mut coords = position.split(',').map(|x| x.trim());
        positions[i][0] = coords.next().unwrap().parse().unwrap();
        positions[i][1] = coords.next().unwrap().parse().unwrap();
    }

    Some(positions)
}