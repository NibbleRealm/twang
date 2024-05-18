use crate::tree::{Chunk, Wave};

/// Bezier wave
///
/// Takes phase (-1 to 1) and curve (-1 to 1) as input
#[derive(Debug)]
pub struct Bezier<I, J>(pub I, pub J);

impl<I, J> Wave for Bezier<I, J>
where
    I: Wave,
    J: Wave,
{
    fn synthesize(&self, elapsed: u64, interval: u64, vars: &[f32]) -> Chunk {
        let chunk = self.0.synthesize(elapsed, interval, vars);
        let curve = self.1.synthesize(elapsed, interval, vars);
        let old = chunk.neg_abs();

        old.offset(1.0)
            .amplify(curve)
            .amplify(old)
            .mix(old)
            .copysign(chunk)
    }
}
