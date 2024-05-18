use core::borrow::Borrow;

trait ForEachSample {
    #[must_use]
    fn for_each_sample(self, f: impl FnMut((&mut f32, f32))) -> Chunk;
}

impl ForEachSample for (Chunk, &Chunk) {
    #[inline(always)]
    fn for_each_sample(mut self, f: impl FnMut((&mut f32, f32))) -> Chunk {
        self.0
             .0
            .iter_mut()
            .zip(self.1 .0.iter().cloned())
            .for_each(f);
        self.0
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Chunk(pub(super) [f32; 32]);

impl Chunk {
    #[inline(always)]
    #[must_use]
    pub(super) fn for_each_sample(mut self, f: impl FnMut(&mut f32)) -> Self {
        self.0.iter_mut().for_each(f);
        self
    }

    #[inline(always)]
    #[must_use]
    pub(super) fn offset(self, amt: f32) -> Self {
        self.for_each_sample(|sample| *sample += amt)
    }

    #[inline(always)]
    #[must_use]
    pub(super) fn mix(self, other: impl Borrow<Self>) -> Self {
        (self, other.borrow()).for_each_sample(|(s, o)| *s += o)
    }

    #[inline(always)]
    #[must_use]
    pub(super) fn gain(self, amt: f32) -> Self {
        self.for_each_sample(|sample| *sample *= amt)
    }

    #[inline(always)]
    #[must_use]
    pub(super) fn amplify(self, other: impl Borrow<Self>) -> Self {
        (self, other.borrow()).for_each_sample(|(s, o)| *s *= o)
    }

    #[inline(always)]
    #[must_use]
    pub(super) fn cosine(self) -> Self {
        self.for_each_sample(|sample| *sample = libm::cosf(*sample))
    }

    #[inline(always)]
    #[must_use]
    pub(super) fn invert(self) -> Self {
        self.for_each_sample(|sample| *sample = -*sample)
    }

    #[inline(always)]
    #[must_use]
    pub(super) fn neg_abs(self) -> Self {
        self.for_each_sample(|sample| *sample = -sample.abs())
    }

    #[inline(always)]
    #[must_use]
    pub(super) fn copysign(self, sign: impl Borrow<Self>) -> Self {
        (self, sign.borrow()).for_each_sample(|(sample, sign)| {
            *sample = libm::copysignf(*sample, sign)
        })
    }
}
