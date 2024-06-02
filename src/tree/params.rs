use crate::tree::{consts, Chunk};

pub(crate) trait Parameters {
    fn chunk(&self, index: usize) -> Chunk;
    fn mark_old(&mut self);
}

/// Audio parameters
#[derive(Debug)]
pub(crate) struct Params<const N: usize> {
    /// The previous value
    old: [f32; N],
    /// The current value
    new: [f32; N],
}

impl<const N: usize> Params<N> {
    pub(crate) fn new(initial_values: [f32; N]) -> Self {
        Self {
            old: initial_values,
            new: initial_values,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get(&mut self) -> &[f32; N] {
        &self.new
    }

    pub(crate) fn get_mut(&mut self) -> &mut [f32; N] {
        &mut self.new
    }
}

impl<const N: usize> Parameters for Params<N> {
    fn chunk(&self, index: usize) -> Chunk {
        let mut buffer = [0.0; 32];
        let old = self.old[index];
        let new = self.new[index];

        buffer
            .iter_mut()
            .zip(
                consts::FRAC_32
                    .iter()
                    .cloned()
                    .map(|x| x * old)
                    .zip(consts::FRAC_32_REV.iter().cloned().map(|x| x * new)),
            )
            .for_each(|(buf, (old, new))| *buf = old + new);

        Chunk(buffer)
    }

    fn mark_old(&mut self) {
        self.old = self.new;
    }
}
