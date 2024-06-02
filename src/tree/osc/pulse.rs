use crate::tree::{Chunk, Data, Wave};

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
    const STATE_LEN: usize = I::STATE_LEN + J::STATE_LEN + K::STATE_LEN;

    fn synthesize(&self, data: &mut Data<'_>) -> Chunk {
        let chunk = self.0.synthesize(data);
        let cycle = self.1.synthesize(data);
        let alias = self.2.synthesize(data);
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
        let scale = offset.neg_abs().offset(1.0).recip();

        offset.mix(pulse).amplify(scale)
    }
}
