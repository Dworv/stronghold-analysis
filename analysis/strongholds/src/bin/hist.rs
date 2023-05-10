use plotters::prelude::*;
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
    
    let drawing_area = SVGBackend::new("histogram_chunks.svg", (400, 400)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart_builder = Box::new(ChartBuilder::on(&drawing_area));
    chart_builder.margin(12).set_left_and_bottom_label_area_size(20);
    let mut chart_context = Box::new(chart_builder.build_cartesian_2d((1..400).into_segmented(), 0..800).unwrap());
    chart_context.configure_mesh().draw().unwrap();
    chart_context.draw_series(Histogram::vertical(&chart_context).style(BLUE.filled()).margin(0)
        .data(freq_map.into_iter().flatten().map(|x| (x as i32, 1)))).unwrap();
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