//! Minimal WAV file writer.
//!
//! http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html

use fon::{chan::Ch16, stereo::Stereo16, Audio, Sample};
use std::convert::TryInto;
use std::{fs, io, mem::size_of};

/// Write a 16-bit PCM WAV file
pub(super) fn write<S: Sample>(
    audio: Audio<S>,
    filename: &str,
) -> io::Result<()>
where
    Ch16: From<S::Chan>,
{
    let audio = Audio::<Stereo16>::with_audio(audio.sample_rate(), &audio);
    let mut buf = vec![];
    write_header(&mut buf, &audio);
    write_fmt_header(&mut buf, &audio);
    write_audio_data(&mut buf, &audio);
    fs::write(filename, buf)
}

fn write_header(buf: &mut Vec<u8>, audio: &Audio<Stereo16>) {
    // Predict size of WAV subchunks.
    let n: u32 = audio.len().try_into().unwrap();
    // RIFF Chunk: ckID
    buf.extend(b"RIFF");
    // RIFF Chunk: cksize
    buf.extend(&(36u32 + n).to_le_bytes());
    // RIFF Chunk: WAVEID
    buf.extend(b"WAVE");
}

fn write_fmt_header(buf: &mut Vec<u8>, audio: &Audio<Stereo16>) {
    // RIFF Subchunk: "fmt "
    buf.extend(b"fmt ");
    // Chunk size: 16, 18 or 40
    buf.extend(&(16u32).to_le_bytes());
    // 0: WAVE_FORMAT_PCM
    buf.extend(&(0x0001u16).to_le_bytes());
    // 2: Stereo
    buf.extend(&(2u16).to_le_bytes());
    // 4: Sampling Rate
    buf.extend(&(audio.sample_rate() as u32).to_le_bytes());
    // 8: Bytes per second (i16 * 2 * sample rate)
    buf.extend(&((4 * audio.sample_rate()) as u32).to_le_bytes());
    // 12. Data block size (bytes: i16 * 2)
    buf.extend(&(size_of::<u16>() as u16 * 2u16).to_le_bytes());
    // 14. Bits per sample
    buf.extend(&(16u16).to_le_bytes());
}

fn write_audio_data(buf: &mut Vec<u8>, audio: &Audio<Stereo16>) {
    // RIFF Subchunk: "data"
    buf.extend(b"data");
    // cksize (Bytes): Stereo (2) * i16 (2) * Frame Length
    buf.extend(&(4 * audio.len() as u32).to_le_bytes());
    // Sampled data
    for sample in audio.samples() {
        for channel in sample.channels().iter().cloned() {
            let channel: i16 = channel.into();
            buf.extend(&channel.to_le_bytes());
        }
    }
}
