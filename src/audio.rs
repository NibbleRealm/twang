use crate::chan::{Ch16, Ch8};
use crate::gen::Generator;
use crate::private::Sealed;
use crate::sample::Sample;
use std::fmt::Debug;
use std::slice::from_raw_parts_mut;
use std::time::Duration;

// Channel Identification
// 0. Front Left (Mono)
// 1. Front Right
// 2. Center
// 3. Rear Left
// 4. Rear Right
// 5. LFE
// 6. Side Left
// 7. Side Right

/// 1 Channel (front center)
#[derive(Debug, Copy, Clone)]
pub struct Mono;
/// 2 Channels (front left, front right)
#[derive(Debug, Copy, Clone)]
pub struct Stereo;
/// 6 Channels ITU 5.1 Surround Sound Standard (most common surround sound
/// configuration).
#[derive(Debug, Copy, Clone)]
pub struct Surround;
/// 8 Channels Blu-ray 7.1 Surround Sound.
#[derive(Debug, Copy, Clone)]
pub struct SurroundTheater;

pub trait Sources: Copy + Clone + Debug + Sealed {
    /// Number of channels for this configuration
    const CHANNEL_COUNT: usize;
}

impl Sources for Mono {
    const CHANNEL_COUNT: usize = 1;
}

impl Sources for Stereo {
    const CHANNEL_COUNT: usize = 2;
}

impl Sources for Surround {
    const CHANNEL_COUNT: usize = 6;
}

impl Sources for SurroundTheater {
    const CHANNEL_COUNT: usize = 8;
}

/// Newtype for hertz.
#[derive(Copy, Clone)]
pub struct Hz(pub f64);

/// An audio buffer (array of audio Samples at a specific sample rate in hertz).
pub struct Audio<S: Sample> {
    s_rate: Hz,
    samples: Box<[S]>,
}

impl<S: Sample> Audio<S> {
    /// Construct an `Audio` buffer with all samples set to one value.
    pub fn with_sample(s_rate: Hz, len: usize, sample: S) -> Self {
        let samples = vec![sample; len].into_boxed_slice();
        Audio { s_rate, samples }
    }

    /// Construct an `Audio` buffer with all all samples set to the default
    /// value.
    pub fn with_silence(s_rate: Hz, len: usize) -> Self {
        Self::with_sample(s_rate, len, S::default())
    }

    /// Construct an `Audio` buffer with another `Audio` buffer.
    ///
    /// The audio format can be converted with this function.
    pub fn with_audio<SrcS: Sample>(s_rate: Hz, src: &Audio<SrcS>) -> Self
    where
        S::Chan: From<SrcS::Chan>,
    {
        let mut dst = Audio::with_silence(s_rate, src.len());
        for (dst, src) in dst.samples.iter_mut().zip(src.samples.iter()) {
            *dst = src.convert();
        }
        dst
    }

    /// Construct an `Audio` buffer with owned sample data.   You can get
    /// ownership of the pixel data back from the `Audio` buffer as either a
    /// `Vec<S>` or a `Box<[S]>` by calling into().
    pub fn with_samples<B: Into<Box<[S]>>>(s_rate: Hz, samples: B) -> Self {
        let samples = samples.into();
        Audio { s_rate, samples }
    }

    /// Construct an `Audio` buffer from a `u8` buffer.    
    pub fn with_u8_buffer<B>(s_rate: Hz, buffer: B) -> Self
    where
        B: Into<Box<[u8]>>,
        S: Sample<Chan = Ch8>,
    {
        let buffer: Box<[u8]> = buffer.into();
        let len = buffer.len() / std::mem::size_of::<S>();
        assert_eq!(0, buffer.len() % std::mem::size_of::<S>());
        let slice = Box::<[u8]>::into_raw(buffer);
        let samples: Box<[S]> = unsafe {
            let ptr = (*slice).as_mut_ptr() as *mut S;
            Box::from_raw(from_raw_parts_mut(ptr, len))
        };
        Audio { s_rate, samples }
    }

    /// Construct an `Audio` buffer from a `u16` buffer.
    pub fn with_u16_buffer<B>(s_rate: Hz, buffer: B) -> Self
    where
        B: Into<Box<[u16]>>,
        S: Sample<Chan = Ch16>,
    {
        let buffer: Box<[u16]> = buffer.into();
        let bytes = buffer.len() * std::mem::size_of::<u16>();
        let len = bytes / std::mem::size_of::<S>();
        assert_eq!(0, bytes % std::mem::size_of::<S>());
        let slice = Box::<[u16]>::into_raw(buffer);
        let samples: Box<[S]> = unsafe {
            let ptr = (*slice).as_mut_ptr() as *mut S;
            Box::from_raw(from_raw_parts_mut(ptr, len))
        };
        Audio { s_rate, samples }
    }

    /// Get the length of the `Audio` buffer.
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Get the sample rate of the `Audio` buffer.
    pub fn sample_rate(&self) -> Hz {
        self.s_rate
    }

    /// Generate audio in buffer using a generator.    
    pub fn generate<G: Generator>(&mut self, generator: &mut G) {
        let time_step = Duration::new(1, 0).div_f64(self.s_rate.0);
        for sample in self.samples.iter_mut() {
            let channel = generator.sample(time_step).into();
            *sample = S::from_channels(&[channel; 8][..S::LEN]);
        }
    }
}

impl<S: Sample> From<Audio<S>> for Box<[S]> {
    /// Get internal pixel data as boxed slice.
    fn from(audio: Audio<S>) -> Self {
        audio.samples
    }
}

impl<S: Sample> From<Audio<S>> for Vec<S> {
    /// Get internal pixel data as `Vec` of samples.
    fn from(audio: Audio<S>) -> Self {
        audio.samples.into()
    }
}

impl<S> From<Audio<S>> for Box<[u8]>
where
    S: Sample<Chan = Ch8>,
{
    /// Get internal pixel data as boxed slice of *u8*.
    fn from(audio: Audio<S>) -> Self {
        let samples = audio.samples;
        let capacity = samples.len() * std::mem::size_of::<S>();
        let slice = Box::<[S]>::into_raw(samples);
        let buffer: Box<[u8]> = unsafe {
            let ptr = (*slice).as_mut_ptr() as *mut u8;
            Box::from_raw(from_raw_parts_mut(ptr, capacity))
        };
        buffer
    }
}

impl<S> From<Audio<S>> for Box<[u16]>
where
    S: Sample<Chan = Ch16>,
{
    /// Get internal pixel data as boxed slice of *u16*.
    fn from(audio: Audio<S>) -> Self {
        let samples = audio.samples;
        let capacity = samples.len() * std::mem::size_of::<S>() / 2;
        let slice = Box::<[S]>::into_raw(samples);
        let buffer: Box<[u16]> = unsafe {
            let ptr = (*slice).as_mut_ptr() as *mut u16;
            Box::from_raw(from_raw_parts_mut(ptr, capacity))
        };
        buffer
    }
}
