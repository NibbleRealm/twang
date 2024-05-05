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
        let mut chunk = self.0.synthesize(elapsed, interval, vars);
        let curve = self.1.synthesize(elapsed, interval, vars);

        for (src, curve) in chunk.0.iter_mut().zip(curve.0.iter()) {
            let sign = src.signum();
            let mut output = -src.abs();

            output += 1.0;
            output = output + output * curve * (output - 1.0) - 1.0;
            *src = output.copysign(sign);
        }

        chunk
    }
}
