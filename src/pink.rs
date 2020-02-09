// Algorithm inspired by https://github.com/Stenzel/newshadeofpink

use crate::quiet::Quiet;
use crate::Sample;

// Constant Look-up tables
const PFIRAM: f64 = 2048.0 * 1.190566;
const PFIRAM2: f64 = 2048.0 * 0.162580;
const PFIRAM3: f64 = 2048.0 * 0.002208;
const PFIRAM4: f64 = 2048.0 * 0.025475;
const PFIRAM5: f64 = 2048.0 * -0.001522;
const PFIRAM6: f64 = 2048.0 * 0.007322;

const PFIRBM: f64 = 2048.0 * 0.001774;
const PFIRBM2: f64 = 2048.0 * 0.004529;
const PFIRBM3: f64 = 2048.0 * -0.001561;
const PFIRBM4: f64 = 2048.0 * 0.000776;
const PFIRBM5: f64 = 2048.0 * -0.000486;
const PFIRBM6: f64 = 2048.0 * 0.002017;

const PNMASK: [u8; 256] = [
    0, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    8, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    4, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    8, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    2, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    8, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    4, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    8, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    1, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    8, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    4, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    8, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    2, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    8, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    4, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80,
    8, 0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80, 0x10,
        0x80, 0x40, 0x80, 0x20, 0x80, 0x40, 0x80
];

const PFIRA: [i32; 64] = [
    (PFIRAM * (2i32 * (0i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (0i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (0i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (0i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (0i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (0i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (0i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (0i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (0i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (0i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (0i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (0i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (0i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (0i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (0i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (0i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (0i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (0i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (0i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (0i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (0i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (0i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (0i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (0i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (0i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (0i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (0i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (0i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (0i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (0i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (0i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (0i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (0i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (0i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (0i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (0i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (0i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (0i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (0i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (0i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (0i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (0i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (0i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (0i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (0i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (0i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (0i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (0i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (8i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (8i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (8i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (8i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (8i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (8i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (8i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (8i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (8i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (8i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (8i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (8i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (8i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (8i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (8i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (8i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (8i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (8i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (8i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (8i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (8i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (8i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (8i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (8i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (8i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (8i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (8i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (8i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (8i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (8i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (8i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (8i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (8i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (8i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (8i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (8i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (8i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (8i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (8i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (8i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (8i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (8i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (8i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (8i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (8i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (8i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (8i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (8i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (16i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (16i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (16i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (16i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (16i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (16i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (16i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (16i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (16i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (16i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (16i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (16i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (16i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (16i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (16i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (16i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (16i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (16i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (16i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (16i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (16i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (16i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (16i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (16i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (16i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (16i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (16i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (16i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (16i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (16i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (16i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (16i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (16i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (16i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (16i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (16i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (16i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (16i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (16i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (16i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (16i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (16i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (16i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (16i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (16i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (16i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (16i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (16i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (24i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (24i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (24i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (24i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (24i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (24i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (24i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (24i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (24i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (24i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (24i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (24i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (24i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (24i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (24i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (24i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (24i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (24i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (24i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (24i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (24i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (24i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (24i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (24i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (24i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (24i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (24i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (24i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (24i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (24i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (24i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (24i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (24i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (24i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (24i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (24i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (24i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (24i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (24i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (24i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (24i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (24i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (24i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (24i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (24i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (24i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (24i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (24i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (32i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (32i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (32i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (32i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (32i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (32i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (32i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (32i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (32i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (32i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (32i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (32i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (32i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (32i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (32i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (32i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (32i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (32i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (32i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (32i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (32i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (32i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (32i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (32i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (32i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (32i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (32i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (32i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (32i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (32i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (32i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (32i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (32i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (32i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (32i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (32i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (32i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (32i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (32i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (32i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (32i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (32i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (32i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (32i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (32i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (32i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (32i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (32i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (40i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (40i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (40i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (40i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (40i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (40i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (40i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (40i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (40i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (40i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (40i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (40i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (40i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (40i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (40i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (40i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (40i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (40i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (40i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (40i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (40i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (40i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (40i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (40i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (40i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (40i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (40i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (40i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (40i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (40i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (40i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (40i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (40i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (40i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (40i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (40i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (40i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (40i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (40i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (40i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (40i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (40i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (40i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (40i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (40i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (40i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (40i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (40i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (48i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (48i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (48i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (48i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (48i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (48i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (48i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (48i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (48i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (48i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (48i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (48i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (48i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (48i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (48i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (48i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (48i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (48i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (48i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (48i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (48i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (48i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (48i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (48i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (48i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (48i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (48i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (48i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (48i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (48i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (48i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (48i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (48i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (48i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (48i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (48i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (48i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (48i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (48i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (48i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (48i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (48i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (48i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (48i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (48i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (48i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (48i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (48i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (56i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (56i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (56i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (56i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (56i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (56i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (56i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (56i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (56i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (56i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (56i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (56i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (56i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (56i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (56i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (56i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (56i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (56i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (56i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (56i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (56i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (56i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (56i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (56i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (56i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (56i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (56i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (56i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (56i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (56i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (56i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (56i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (56i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (56i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (56i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (56i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (56i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (56i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (56i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (56i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (56i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (56i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRAM * (2i32 * (56i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRAM2 * (2i32 * (56i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRAM3 * (2i32 * (56i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRAM4 * (2i32 * (56i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRAM5 * (2i32 * (56i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRAM6 * (2i32 * (56i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32
];

const PFIRB: [i32; 64] = [
    (PFIRBM * (2i32 * (0i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (0i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (0i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (0i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (0i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (0i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (0i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (0i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (0i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (0i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (0i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (0i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (0i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (0i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (0i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (0i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (0i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (0i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (0i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (0i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (0i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (0i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (0i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (0i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (0i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (0i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (0i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (0i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (0i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (0i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (0i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (0i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (0i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (0i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (0i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (0i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (0i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (0i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (0i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (0i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (0i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (0i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (0i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (0i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (0i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (0i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (0i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (0i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (8i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (8i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (8i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (8i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (8i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (8i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (8i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (8i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (8i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (8i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (8i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (8i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (8i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (8i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (8i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (8i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (8i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (8i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (8i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (8i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (8i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (8i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (8i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (8i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (8i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (8i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (8i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (8i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (8i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (8i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (8i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (8i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (8i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (8i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (8i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (8i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (8i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (8i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (8i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (8i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (8i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (8i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (8i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (8i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (8i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (8i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (8i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (8i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (16i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (16i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (16i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (16i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (16i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (16i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (16i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (16i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (16i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (16i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (16i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (16i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (16i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (16i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (16i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (16i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (16i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (16i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (16i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (16i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (16i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (16i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (16i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (16i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (16i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (16i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (16i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (16i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (16i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (16i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (16i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (16i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (16i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (16i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (16i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (16i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (16i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (16i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (16i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (16i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (16i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (16i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (16i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (16i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (16i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (16i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (16i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (16i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (24i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (24i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (24i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (24i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (24i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (24i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (24i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (24i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (24i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (24i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (24i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (24i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (24i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (24i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (24i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (24i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (24i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (24i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (24i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (24i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (24i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (24i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (24i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (24i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (24i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (24i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (24i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (24i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (24i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (24i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (24i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (24i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (24i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (24i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (24i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (24i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (24i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (24i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (24i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (24i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (24i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (24i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (24i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (24i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (24i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (24i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (24i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (24i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (32i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (32i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (32i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (32i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (32i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (32i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (32i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (32i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (32i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (32i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (32i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (32i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (32i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (32i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (32i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (32i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (32i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (32i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (32i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (32i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (32i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (32i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (32i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (32i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (32i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (32i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (32i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (32i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (32i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (32i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (32i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (32i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (32i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (32i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (32i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (32i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (32i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (32i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (32i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (32i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (32i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (32i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (32i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (32i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (32i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (32i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (32i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (32i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (40i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (40i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (40i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (40i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (40i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (40i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (40i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (40i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (40i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (40i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (40i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (40i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (40i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (40i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (40i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (40i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (40i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (40i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (40i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (40i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (40i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (40i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (40i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (40i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (40i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (40i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (40i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (40i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (40i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (40i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (40i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (40i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (40i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (40i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (40i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (40i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (40i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (40i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (40i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (40i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (40i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (40i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (40i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (40i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (40i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (40i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (40i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (40i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (48i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (48i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (48i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (48i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (48i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (48i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (48i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (48i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (48i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (48i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (48i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (48i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (48i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (48i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (48i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (48i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (48i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (48i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (48i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (48i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (48i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (48i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (48i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (48i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (48i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (48i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (48i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (48i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (48i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (48i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (48i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (48i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (48i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (48i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (48i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (48i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (48i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (48i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (48i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (48i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (48i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (48i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (48i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (48i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (48i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (48i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (48i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (48i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (56i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (56i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (56i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (56i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (56i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (56i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (56i32 + 1i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (56i32 + 1i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (56i32 + 1i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (56i32 + 1i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (56i32 + 1i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (56i32 + 1i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (56i32 + 2i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (56i32 + 2i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (56i32 + 2i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (56i32 + 2i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (56i32 + 2i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (56i32 + 2i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (56i32 + 3i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (56i32 + 3i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (56i32 + 3i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (56i32 + 3i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (56i32 + 3i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (56i32 + 3i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (56i32 + 4i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (56i32 + 4i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (56i32 + 4i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (56i32 + 4i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (56i32 + 4i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (56i32 + 4i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (56i32 + 5i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (56i32 + 5i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (56i32 + 5i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (56i32 + 5i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (56i32 + 5i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (56i32 + 5i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (56i32 + 6i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (56i32 + 6i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (56i32 + 6i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (56i32 + 6i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (56i32 + 6i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (56i32 + 6i32 >> 5i32 & 1i32) - 1i32) as f64) as i32,
    (PFIRBM * (2i32 * (56i32 + 7i32 >> 0i32 & 1i32) - 1i32) as f64 + PFIRBM2 * (2i32 * (56i32 + 7i32 >> 1i32 & 1i32) - 1i32) as f64 + PFIRBM3 * (2i32 * (56i32 + 7i32 >> 2i32 & 1i32) - 1i32) as f64 + PFIRBM4 * (2i32 * (56i32 + 7i32 >> 3i32 & 1i32) - 1i32) as f64 + PFIRBM5 * (2i32 * (56i32 + 7i32 >> 4i32 & 1i32) - 1i32) as f64 + PFIRBM6 * (2i32 * (56i32 + 7i32 >> 5i32 & 1i32) - 1i32) as f64) as i32
];

/// Pink Noise Sampler.
pub struct Pink {
    lfsr: i32,
    inc: i32,
    dec: i32,
    accu: i32,
    pncnt: u8,
    which: u8,
    bit: i32,
    sampler: Quiet,
}

impl Pink {
    /// Create a new Pink Noise Sampler.
    #[inline(always)]
    pub fn new(hz: Option<f64>) -> Self {
        Self {
            lfsr: 0x5eed41f5i32,
            inc: 0xccc,
            dec: 0xccc,
            accu: 0,
            pncnt: 0,
            which: 0,
            bit: 0,
            sampler: Quiet::new(hz),
        }
    }

    fn a(&mut self) -> i16 {
        self.bit = self.lfsr >> 31i32;
        self.dec &= !0x800i32;
        self.lfsr <<= 1i32;
        self.dec |= self.inc & 0x800i32;
        self.inc ^= self.bit & 0x800i32;
        self.b()
    }

    fn b(&mut self) -> i16 {
        self.accu += self.inc - self.dec;
        self.lfsr ^= self.bit & 0x46000001i32;
        (self.accu + PFIRA[(self.lfsr & 0x3fi32) as usize]
            + PFIRB[(self.lfsr >> 6i32 & 0x3fi32) as usize]) as i16
    }

    fn c(&mut self) -> i16 {
        self.bit = self.lfsr >> 31i32;
        self.dec &= !0x400i32;
        self.lfsr <<= 1i32;
        self.dec |= self.inc & 0x400i32;
        self.inc ^= self.bit & 0x400i32;
        self.b()
    }

    fn d(&mut self) -> i16 {
        self.bit = self.lfsr >> 31i32;
        self.dec &= !0x200i32;
        self.lfsr <<= 1i32;
        self.dec |= self.inc & 0x200i32;
        self.inc ^= self.bit & 0x200i32;
        self.b()
    }

    fn e(&mut self) -> i16 {
        self.bit = self.lfsr >> 31i32;
        self.dec &= !0x100i32;
        self.lfsr <<= 1i32;
        self.dec |= self.inc & 0x100i32;
        self.inc ^= self.bit & 0x100i32;
        self.b()
    }

    fn f(&mut self, mask: i32) -> i16 {
        self.bit = self.lfsr >> 31i32;
        self.dec &= !mask;
        self.lfsr <<= 1i32;
        self.dec |= self.inc & mask;
        self.inc ^= self.bit & mask;
        self.b()
    }

    fn sample(&mut self) -> f64 {
        // Different functions for each sample.
        let r = match self.which {
            0 => {
                let i = self.pncnt;
                self.pncnt = self.pncnt.wrapping_add(1);
                self.f(PNMASK[i as usize] as i32)
            },
            1 => self.a(),
            2 => self.c(),
            3 => self.a(),
            4 => self.d(),
            5 => self.a(),
            6 => self.c(),
            7 => self.a(),
            8 => self.e(),
            9 => self.a(),
            10 => self.c(),
            11 => self.a(),
            12 => self.d(),
            13 => self.a(),
            14 => self.c(),
            15 => self.a(),
            _ => unreachable!()
        } as f64 / (std::i16::MAX as f64);
        self.which = (self.which + 1) % 16;
        r
    }
}

impl Iterator for Pink {
    type Item = Sample;

    fn next(&mut self) -> Option<Sample> {
        let mut sample = self.sampler.next().unwrap();
        sample.v = self.sample();
        Some(sample)
    }
}
