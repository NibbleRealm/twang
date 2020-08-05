use cala::*;
use speaker::Player;
use twang::White;

exec!(async_main);
async fn async_main() {
    let mut speaker = Player::new().unwrap();
    let mut whts = White::new(None);

    loop {
        let _sample_rate = speaker.fut().await;
        let n_frames = player.play_last(shared.buffer.as_slice());

        whts.next().unwrap().into()
    }
}
