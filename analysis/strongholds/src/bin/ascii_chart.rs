use strongholds::sample_reader::read_sample;

use std::{fs::File, io::BufReader};

fn main() {
    let sample_list = File::open("analysis/strongholds/sample_list.txt").unwrap();
    let mut reader = BufReader::new(sample_list);

    let mut freq_map = Box::new([[0u64; 200*2]; 200*2]);

    for _ in 0..5000000 {
        let raw_positions = read_sample(&mut reader).unwrap();
        let mut positions = vec![];
        for rp in raw_positions {
            positions.push(ChunkPos::from(NormalPos { x: rp[0] as f64, z: rp[1] as f64 }))
        }
        for pos in positions {
            freq_map[(200 - pos.z) as usize][(200 + pos.x) as usize] += 1;
        }
    }

    for row in freq_map.iter() {
        for num in row {
            if num > &0 {
                print!("██");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

#[derive(Clone, Copy, Debug)]
struct NormalPos {
    x: f64,
    z: f64
}

#[derive(Clone, Copy, Debug)]
struct ChunkPos {
    x: i64,
    z: i64
}

impl From<NormalPos> for ChunkPos {
    fn from(value: NormalPos) -> Self {
        Self {
            x: (value.x/16.).floor() as i64,
            z: (value.z/16.).floor() as i64
        }
    }
}
