use crate::tree::{Chunk, Wave};

/// Pulse wave
///
/// Takes phase (-1 to 1), duty (-1 to 1) and alias (0 to 1) as input
#[derive(Debug)]
pub struct Pulse<I, J, K>(pub I, pub J, pub K);

impl<I, J, K> Wave for Pulse<I, J, K>
where
    I: Wave,
    J: Wave,
    K: Wave,
{
    fn synthesize(&self, elapsed: u64, interval: u64, vars: &[f32]) -> Chunk {
        let chunk = self.0.synthesize(elapsed, interval, vars);
        let cycle = self.1.synthesize(elapsed, interval, vars);
        let alias = self.2.synthesize(elapsed, interval, vars);
        let clip = alias.recip();
        let pulse = chunk
            .abs()
            .gain(2.0)
            .offset(-1.0)
            .mix(cycle)
            .clip()
            .amplify(clip)
            .clip();
        let offset = cycle.gain(-0.5);
        let scale = offset.abs().offset(1.0).recip();

        offset.mix(pulse).amplify(scale)
    }
}
