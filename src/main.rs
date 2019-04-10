use std::fs::File;
use std::io::prelude::*;

const SAMPLE_RATE: u32 = 44100;

fn make_sin(nsecs: u64, freq: f64) -> Vec<u8> {
    let nsamples = nsecs as usize * SAMPLE_RATE as usize;
    let mut buf = Vec::with_capacity(nsamples as usize);
    for t in 0..nsamples {
        let w = 2.0 * std::f64::consts::PI * freq * t as f64;
        let s = f64::sin(w / (SAMPLE_RATE as f64));
        let s = f64::floor(255.0 * (0.5 * s + 0.5)) as u8;
        buf.push(s);
    }
    buf
}

fn make_bytes<T>(v: T) -> Vec<u8>
    where T: Into<u64>
{
    let v: u64 = v.into();
    let mut b: Vec<u8> = Vec::new();
    for i in 0..std::mem::size_of::<T>() {
        b.push(((v >> (8 * i)) & 0xff) as u8);
    }
    b
}

enum Field {
    Desc(&'static [u8]),
    U16(u16),
    U32(u32),
}

impl Field {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Field::Desc(d) => d.to_vec(),
            Field::U16(u) => make_bytes(*u),
            Field::U32(u) => make_bytes(*u),
        }
    }
}

fn make_wav(nsamples: usize) -> Vec<u8> {
    let fields = vec![
        // RIFF chunk
        Field::Desc(b"RIFF"),
        Field::U32(20 + nsamples as u32),
        // WAVE chunk
        Field::Desc(b"WAVE"),
        // fmt chunk
        Field::Desc(b"fmt "),
        Field::U32(16), // fmt chunk size
        Field::U16(1),  // format code (PCM)
        Field::U16(1),  // number of channels
        Field::U32(SAMPLE_RATE), // sample rate
        Field::U32(SAMPLE_RATE), // data rate
        Field::U16(1),  // block size
        Field::U16(8),  // bits per sample
        // data chunk
        Field::Desc(b"data"),
        Field::U32(nsamples as u32),
    ];
    fields.iter().flat_map(|f| f.to_bytes()).collect::<Vec<u8>>()
}

fn main() {
    let mut file = File::create("sine.wav").unwrap();
    let sin_buf = make_sin(3, 1000.0);
    let wav_buf = make_wav(sin_buf.len());
    file.write_all(&wav_buf).unwrap();
    file.write_all(&sin_buf).unwrap();
}
