use crate::{ops::Blend, chan::{Ch16, Ch8}, gen::Generator, sample::Sample, config::Config};
use core::{fmt::Debug, slice::from_raw_parts_mut, time::Duration};
use std::convert::TryInto;

// Channel Identification
// 0. Front Left (Mono)
// 1. Front Right
// 2. Center
// 3. Rear Left
// 4. Rear Right
// 5. LFE
// 6. Side Left
// 7. Side Right

/// Newtype for hertz.
#[derive(Copy, Clone, Debug)]
pub struct Hz(pub f64);

/// An audio buffer (array of audio Samples at a specific sample rate in hertz).
#[derive(Debug)]
pub struct Audio<S: Sample> {
    s_rate: usize,
    samples: Box<[S]>,
}

impl<S: Sample> Audio<S> {
    /// Get view of samples as a `u8` slice.
    #[allow(unsafe_code)]
    pub fn as_u8_slice(&self) -> &[u8] {
        unsafe {
            let (prefix, v, suffix) = self.samples.align_to::<u8>();
            debug_assert!(prefix.is_empty());
            debug_assert!(suffix.is_empty());
            v
        }
    }

    /// Get view of samples as a mutable `u8` slice.
    #[allow(unsafe_code)]
    pub fn as_u8_slice_mut(&mut self) -> &mut [u8] {
        unsafe {
            let (prefix, v, suffix) = self.samples.align_to_mut::<u8>();
            debug_assert!(prefix.is_empty());
            debug_assert!(suffix.is_empty());
            v
        }
    }

    /// Construct an `Audio` buffer with all samples set to one value.
    pub fn with_sample(s_rate: usize, len: usize, sample: S) -> Self {
        let samples = vec![sample; len].into_boxed_slice();
        Audio { s_rate, samples }
    }

    /// Construct an `Audio` buffer with all all samples set to the default
    /// value.
    pub fn with_silence(s_rate: usize, len: usize) -> Self {
        Self::with_sample(s_rate, len, S::default())
    }

    /// Construct an `Audio` buffer with another `Audio` buffer.
    ///
    /// The audio format can be converted with this function.
    pub fn with_audio<SrcS: Sample>(s_rate: usize, src: &Audio<SrcS>) -> Self
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
    pub fn with_samples<B: Into<Box<[S]>>>(s_rate: usize, samples: B) -> Self {
        let samples = samples.into();
        Audio { s_rate, samples }
    }

    /// Construct an `Audio` buffer from a `u8` buffer.
    #[allow(unsafe_code)]
    pub fn with_u8_buffer<B>(s_rate: usize, buffer: B) -> Self
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
    #[allow(unsafe_code)]
    pub fn with_u16_buffer<B>(s_rate: usize, buffer: B) -> Self
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

    /// Check if `Audio` buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the sample rate of the `Audio` buffer.
    pub fn sample_rate(&self) -> usize {
        self.s_rate
    }

    /// Generate audio in buffer using a generator.    
    pub fn generate<G: Generator>(&mut self, generator: &mut G) {
        let time_step = Duration::new(1, 0) / self.s_rate.try_into().unwrap();
        for sample in self.samples.iter_mut() {
            let channel = generator.sample(time_step).into();
            *sample = S::from_channels(&[channel; 8][..S::Conf::CHANNEL_COUNT]);
        }
    }
    
    /// Blend `Audio` buffer with a single sample.
    pub fn blend_sample<O: Blend>(&mut self, sample: S, op: O) {
        S::blend_sample(&mut self.samples, &sample, op)
    }
    
    /// Blend `Audio` buffer with another `Audio` buffer.
    pub fn blend_audio<O: Blend>(&mut self, other: &Self, op: O) {
        S::blend_slice(&mut self.samples, &other.samples, op)
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
    #[allow(unsafe_code)]
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
    #[allow(unsafe_code)]
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
