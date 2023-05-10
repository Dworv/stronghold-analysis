use plotters::prelude::*;
use strongholds::sample_reader::read_sample;

use std::{fs::File, io::BufReader};

fn main() {
    let sample_list = File::open("analysis/strongholds/sample_list.txt").unwrap();
    let mut reader = BufReader::new(sample_list);

    let mut freq_map = Box::new([[0u64; 200*2]; 200*2]);

    for _ in 0..9999998 {
        let raw_positions = read_sample(&mut reader).unwrap();
        let mut positions = vec![];
        for rp in raw_positions {
            positions.push(ChunkPos::from(NormalPos { x: rp[0] as f64, z: rp[1] as f64 }))
        }
        for pos in positions {
            freq_map[(200 - pos.z) as usize][(200 + pos.x) as usize] += 1;
        }
    }
    
    let drawing_area = SVGBackend::new("analysis/strongholds/chart_map.svg", (400, 400)).into_drawing_area();
    drawing_area.fill(&BLACK).unwrap();
    let max = freq_map.iter().flatten().max().unwrap();

    for (r, row) in freq_map.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            drawing_area.draw_pixel((r as i32, c as i32), &RGBColor (((*val as f64)/(*max as f64) * 255.) as u8, 0, 0)).unwrap();
        }
    }

    // drawing_area.draw_pixel((0, 0), &RGBColor (100, 100, 100)).unwrap();
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