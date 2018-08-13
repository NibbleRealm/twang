extern crate twang; // for sound generation / effects
extern crate adi; // for speaker

use twang as t;

fn main() {
	let mut speaker = adi::speaker::Speaker::new(0, false).unwrap();
	let mut gen = t::Generator::new(220.0, 1.0); // A3
	let mut gen2 = t::Generator::new(220.0 * 32.0 / 27.0, 1.0); // third
	let mut gen3 = t::Generator::new(220.0 * 3.0 / 2.0, 1.0); // fifth
	let mut _pnk = t::PnkGenerator::new();
	let mut _wht = t::WhtGenerator::new();

	let piano = [
		0.700, 0.243, 0.229, 0.095, 0.139,
		0.087, 0.288, 0.199, 0.124, 0.090,
	];

	loop {
		speaker.update(&mut || {
			// Play A Minor Chord on Synthesized Piano
			let x1 = t::ovr(gen.next(), &mut t::sin, &piano);
			let x2 = t::ovr(gen2.next(), &mut t::sin, &piano);
			let x3 = t::ovr(gen3.next(), &mut t::sin, &piano);

			t::out(t::mix(&[x1, x2, x3]))

//			let x = gen.next();

			// Generate Muted trumpet/trombone
/*			t::out(t::hrd(t::sft(t::add(&[
				t::mul(&[
					t::hrd(t::sin(x), 4.0),
					t::saw(x),
				]) * 0.85,
				t::mul(&[
					t::sin(x),
					t::saw(x),
					t::pnk(&mut _pnk)
				]) * 0.15
			]), 4.0), 1.5))*/

			// Generate Unmuted trumpet/trombone
/*			t::out(t::hrd(t::mul(&[
				t::dst(t::sin(x), 255),
				t::sqr(x)
			]), 5.0))*/

			// Generate organ
/*			t::out(t::add(&[
				t::pos(t::sin(x)),
				t::neg(t::tri(x))
			]))*/

			// Generate voice
/*			t::out(t::mul(&[
				t::dst(t::sin(x), 2),
				t::dst(t::saw(x), 2)
			]))*/
		});
	}
}
