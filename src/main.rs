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

fn make_wav(nsamples: usize) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"RIFF");
    let rsize = make_bytes(20 + nsamples as u32);
    buf.extend_from_slice(& rsize); // WAVE chunk size

    // WAVE chunk
    buf.extend_from_slice(b"WAVE");

    // fmt chunk
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(& make_bytes(16u32)); // fmt chunk size
    buf.extend_from_slice(& make_bytes(1u16));  // format code (PCM)
    buf.extend_from_slice(& make_bytes(1u16));  // number of channels
    buf.extend_from_slice(& make_bytes(SAMPLE_RATE));
    buf.extend_from_slice(& make_bytes(SAMPLE_RATE)); // data rate
    buf.extend_from_slice(& make_bytes(1u16));  // block size
    buf.extend_from_slice(& make_bytes(8u16));  // bits per sample


    // data chunk
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(& make_bytes(nsamples as u32)); // data chunk size

    buf
}

fn main() {
    let mut file = File::create("sine.wav").unwrap();
    let sin_buf = make_sin(3, 1000.0);
    let wav_buf = make_wav(sin_buf.len());
    file.write_all(&wav_buf).unwrap();
    file.write_all(&sin_buf).unwrap();
}
