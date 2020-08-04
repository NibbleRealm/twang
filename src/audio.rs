use crate::chan::{Ch8, Ch16, Ch32, Ch64, Channel};
use crate::private::Sealed;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::slice::from_raw_parts_mut;
use std::any::Any;

// Channel Identification
// 0. Front Left (Mono)
// 1. Front Right
// 2. Center
// 3. Rear Left
// 4. Rear Right
// 5. LFE
// 6. Side Left
// 7. Side Right

/// Blending operation for mixing
pub trait Blend: Any + Copy + Clone {
    /// Synthesize to destination by blending destination and source.
    fn synthesize<C: Channel>(dst: &mut C, src: &C);
}

/// Source only (ignore destination)
#[derive(Clone, Copy)]
pub struct Src;
/// Destination only (ignore source)
#[derive(Clone, Copy)]
pub struct Dest;
/// Source or destination with no overlap
#[derive(Clone, Copy)]
pub struct Xor;
/// Clear (set to default)
#[derive(Clone, Copy)]
pub struct Clear;
/// VCA (Voltage Controlled Amplitude) mixing.  Multiplication of signals.
#[derive(Clone, Copy)]
pub struct Gain;
/// Standard audio mixing.  Addition of signals
#[derive(Clone, Copy)]
pub struct Add;
/// Squared compression audio mixing.  Addition of signals squared.
#[derive(Clone, Copy)]
pub struct AddSquared;
/// Minimum of destination and source
#[derive(Clone, Copy)]
pub struct Min;
/// Maximum of destination and source
#[derive(Clone, Copy)]
pub struct Max;

impl Blend for Src {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = *src;
    }
}

impl Blend for Dest {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        // leave _dst as is
    }
}

impl Blend for Xor {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = if *dst == C::MID {
            *src
        } else if *src == C::MID {
            *dst
        } else {
            C::MID
        };
    }
}

impl Blend for Clear {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = C::default();
    }
}

impl Blend for Gain {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = *src * *dst;
    }
}

impl Blend for Add {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = *src + *dst;
    }
}

impl Blend for AddSquared {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        Add::synthesize(dst, src);
        *dst = *dst * *dst;
    }
}

impl Blend for Min {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = (*dst).min(*src);
    }
}

impl Blend for Max {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = (*dst).max(*src);
    }
}

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
pub struct Hz(pub f64);

pub trait Sample: Clone + Copy + Debug + Default + PartialEq + Sealed {
    /// Channel type
    type Chan: Channel;
    /// Sample rate (in Hertz)
    const RATE: Hz;
    /// Number of channels
    const LEN: usize;

    /// Get the channels.
    fn channels(&self) -> &[Self::Chan];

    /// Get the channels mutably.
    fn channels_mut(&mut self) -> &mut [Self::Chan];

    /// Make a pixel from a slice of channels.
    fn from_channels(ch: &[Self::Chan]) -> Self;

    /// Synthesis of a sample with a slice of samples.
    fn blend_sample<O>(dst: &mut [Self], sample: &Self, op: O)
    where
        O: Blend,
    {
        for d in dst.iter_mut() {
            d.blend_channels(sample, op);
        }
    }

    /// Synthesis of two slices of samples.
    fn blend_slice<O>(dst: &mut [Self], src: &[Self], op: O)
    where
        O: Blend,
    {
        for (d, s) in dst.iter_mut().zip(src) {
            d.blend_channels(s, op);
        }
    }

    /// Synthesize two sample slices together.
    fn blend_channels<O>(&mut self, src: &Self, op: O)
        where O: Blend
    {
        for (d, s) in self.channels_mut().iter_mut().zip(src.channels().iter()) {
            O::synthesize(d, s)
        }
    }
    
    #[inline(always)]
    fn convert<D>(self) -> D
    where
        D: Sample,
        D::Chan: From<Self::Chan> + From<f64>,
    {
        // Convert channels
        match (Self::LEN, D::LEN) {
            // 1:1 sampling (no resample)
            (a, b) if a == b => {
                let mut chans = [D::Chan::MID; 8];
                for (d, s) in chans.iter_mut().zip(self.channels().iter()) {
                    *d = (*s).into();
                }
                D::from_channels(&chans[..self.channels().len()])
            }
            // Downsampling
            (2, 1) => {
                let mut sum = 0.0;
                for chan in self.channels() {
                    sum += chan.to_f64() * 0.5;
                }
                D::from_channels(&[D::Chan::from(sum)])
            }
            (6, 1) => {
                let mut sum = 0.0;
                for chan in self.channels()[0..5].iter() {
                    sum += chan.to_f64() * 0.2;
                }
                sum += self.channels()[5].to_f64(); // LFE
                D::from_channels(&[D::Chan::from(sum)])
            }
            (8, 1) => {
                let mut sum = 0.0;
                for chan in self.channels()[0..7].iter() {
                    sum += chan.to_f64() * (1.0 / 7.0);
                }
                sum += self.channels()[7].to_f64(); // LFE
                D::from_channels(&[D::Chan::from(sum)])
            }
            (6, 2) => {
                let mut left = self.channels()[0].to_f64() * (1.0 / 3.0);
                let mut right = self.channels()[1].to_f64() * (1.0 / 3.0);
                left += self.channels()[2].to_f64() * (1.0 / 3.0);
                right += self.channels()[2].to_f64() * (1.0 / 3.0);
                left += self.channels()[3].to_f64() * (1.0 / 3.0);
                right += self.channels()[4].to_f64() * (1.0 / 3.0);
                left += self.channels()[5].to_f64(); // left LFE
                right += self.channels()[5].to_f64(); // right LFE
                D::from_channels(&[D::Chan::from(left), D::Chan::from(right)])
            }
            (8, 2) => {
                let mut left = self.channels()[0].to_f64() * 0.25;
                let mut right = self.channels()[1].to_f64() * 0.25;
                left += self.channels()[2].to_f64() * 0.25;
                right += self.channels()[2].to_f64() * 0.25;
                left += self.channels()[3].to_f64() * 0.25;
                right += self.channels()[4].to_f64() * 0.25;
                left += self.channels()[5].to_f64(); // left LFE
                right += self.channels()[5].to_f64(); // right LFE
                left += self.channels()[6].to_f64() * 0.25;
                right += self.channels()[7].to_f64() * 0.25;
                D::from_channels(&[D::Chan::from(left), D::Chan::from(right)])
            }
            (8, 5) => {
                let mut left = self.channels()[0].to_f64() * (2.0 / 3.0);
                let mut right = self.channels()[1].to_f64() * (2.0 / 3.0);
                let mut center = self.channels()[2].to_f64();
                let mut back_left = self.channels()[3].to_f64() * (2.0 / 3.0);
                let mut back_right = self.channels()[4].to_f64() * (2.0 / 3.0);
                let mut lfe = self.channels()[5].to_f64();
                left += self.channels()[6].to_f64() * (1.0 / 3.0);
                right += self.channels()[7].to_f64() * (1.0 / 3.0);
                back_left += self.channels()[6].to_f64() * (1.0 / 3.0);
                back_right += self.channels()[7].to_f64() * (1.0 / 3.0);
                D::from_channels(&[D::Chan::from(left), D::Chan::from(right)])
            }
            // Upsampling
            (1, 2) => {
                let mono = self.channels()[0];
                D::from_channels(&[D::Chan::from(mono), D::Chan::from(mono)])
            }
            (1, 6) => {
                let mono = self.channels()[0];
                D::from_channels(&[D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono)])
            }
            (1, 8) => {
                let mono = self.channels()[0];
                D::from_channels(&[D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono)])
            }
            (2, 6) => {
                let left = self.channels()[0].to_f64();
                let right = self.channels()[1].to_f64();
                let center = left * 0.5 + right * 0.5;
                let lfe = D::Chan::MID;
                D::from_channels(&[D::Chan::from(left), D::Chan::from(right), D::Chan::from(center), D::Chan::from(left), D::Chan::from(right), D::Chan::from(lfe)])
            }
            (2, 8) => {
                let left = self.channels()[0].to_f64();
                let right = self.channels()[1].to_f64();
                let center = left * 0.5 + right * 0.5;
                let lfe = D::Chan::MID;
                D::from_channels(&[D::Chan::from(left), D::Chan::from(right), D::Chan::from(center), D::Chan::from(left), D::Chan::from(right), D::Chan::from(lfe), D::Chan::from(left), D::Chan::from(right)])
            },
            (5, 8) => {
                let left = self.channels()[0].to_f64();
                let right = self.channels()[1].to_f64();
                let center = self.channels()[2].to_f64();
                let back_left = self.channels()[3].to_f64();
                let back_right = self.channels()[4].to_f64();
                let lfe = self.channels()[5].to_f64();
                let side_left = (left + back_left) * 0.5;
                let side_right = (right + back_right) * 0.5;
                D::from_channels(&[D::Chan::from(left), D::Chan::from(right), D::Chan::from(center), D::Chan::from(left), D::Chan::from(right), D::Chan::from(lfe), D::Chan::from(side_left), D::Chan::from(side_right)])
            },
            // Unreachable because of sealed traits
            (_, _) => unreachable!(),
        }
    }
}

/// An audio buffer (array of audio Samples).
pub struct Audio<S: Sample> {
    samples: Box<[S]>,
}

impl<S: Sample> Audio<S> {
    /// Construct an `Audio` buffer with all samples set to one value.
    pub fn with_sample(len: usize, sample: S) -> Self {
        let samples = vec![sample; len].into_boxed_slice();
        Audio { samples }
    }

    /// Construct an `Audio` buffer with all all samples set to the default
    /// value.
    pub fn with_silence(len: usize) -> Self {
        Self::with_sample(len, S::default())
    }

    /// Construt an `Audio` buffer with another `Audio` buffer.
    ///
    /// The audio format can be converted with this function.
    pub fn with_audio<SrcS: Sample>(src: &Audio<SrcS>) -> Self
        where S::Chan: From<SrcS::Chan>
    {
        let mut dst = Audio::with_silence(src.len());
        for (dst, src) in dst.samples.iter_mut().zip(src.samples.iter()) {
            *dst = src.convert();
        }
        dst
    }

    /// Construct an `Audio` buffer with owned sample data.   You can get
    /// ownership of the pixel data back from the `Audio` buffer as either a
    /// `Vec<S>` or a `Box<[S]>` by calling into().
    pub fn with_samples<B: Into<Box<[S]>>>(samples: B) -> Self {
        let samples = samples.into();
        Audio {
            samples
        }
    }

    /// Construct an `Audio` buffer from a `u8` buffer.    
    pub fn with_u8_buffer<B>(buffer: B) -> Self
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
        Audio {
            samples,
        }
    }

    /// Construct an `Audio` buffer from a `u16` buffer.
    pub fn with_u16_buffer<B>(buffer: B) -> Self
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
        Audio {
            samples,
        }
    }

    /// Get the length of the `Audio` buffer.
    pub fn len(&self) -> usize {
        self.samples.len()
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
