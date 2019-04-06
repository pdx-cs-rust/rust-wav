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

fn make_u32(v: u32) -> [u8;4] {
    let mut b = [0u8; 4];
    for i in 0..4 {
        b[i] = ((v >> (8 * i)) & 0xff) as u8;
    }
    b
}

fn make_u16(v: u16) -> [u8;2] {
    let mut b = [0u8; 2];
    for i in 0..2 {
        b[i] = ((v >> (8 * i)) & 0xff) as u8;
    }
    b
}

fn make_wav(nsamples: usize) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"RIFF");
    let rsize = make_u32(20 + nsamples as u32);
    buf.extend_from_slice(& rsize); // WAVE chunk size

    // WAVE chunk
    buf.extend_from_slice(b"WAVE");

    // fmt chunk
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(& make_u32(16)); // fmt chunk size
    buf.extend_from_slice(& make_u16(1));  // format code (PCM)
    buf.extend_from_slice(& make_u16(1));  // number of channels
    buf.extend_from_slice(& make_u32(SAMPLE_RATE));
    buf.extend_from_slice(& make_u32(SAMPLE_RATE)); // data rate
    buf.extend_from_slice(& make_u16(1));  // block size
    buf.extend_from_slice(& make_u16(8));  // bits per sample


    // data chunk
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(& make_u32(nsamples as u32)); // data chunk size

    buf
}

fn main() {
    let mut file = File::create("sine.wav").unwrap();
    let sin_buf = make_sin(3, 1000.0);
    let wav_buf = make_wav(sin_buf.len());
    file.write_all(&wav_buf).unwrap();
    file.write_all(&sin_buf).unwrap();
}
