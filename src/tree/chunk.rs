#[derive(Copy, Clone, Debug)]
pub struct Chunk(pub(super) [f32; 32]);

impl Chunk {
    #[inline(always)]
    pub(super) fn for_each_sample(&mut self, f: impl FnMut(&mut f32)) {
        self.0.iter_mut().for_each(f);
    }

    #[inline(always)]
    pub(super) fn offset(&mut self, amt: f32) {
        self.for_each_sample(|sample| *sample += amt);
    }

    #[inline(always)]
    pub(super) fn amplify(&mut self, amt: f32) {
        self.for_each_sample(|sample| *sample *= amt);
    }

    #[inline(always)]
    pub(super) fn cosine(&mut self) {
        self.for_each_sample(|sample| *sample = libm::cosf(*sample));
    }

    #[inline(always)]
    pub(super) fn invert(&mut self) {
        self.for_each_sample(|sample| *sample = -*sample);
    }
}
