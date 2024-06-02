use crate::tree::{Chunk, Data, Wave};

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
    const STATE_LEN: usize = I::STATE_LEN + J::STATE_LEN;

    fn synthesize(&self, data: &mut Data<'_>) -> Chunk {
        let chunk = self.0.synthesize(data);
        let curve = self.1.synthesize(data);
        let old = chunk.neg_abs();

        old.offset(1.0)
            .amplify(curve)
            .amplify(old)
            .mix(old)
            .copysign(chunk)
    }
}
