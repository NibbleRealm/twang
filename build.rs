use std::io::Write;

fn pfir(pfirm: [f64; 6]) -> [i32; 64] {
    let mut pfir = [0; 64];
    for (i, v) in pfir.iter_mut().enumerate() {
        let i = i as i32;
        let a = i / 8;
        let a = a * 8;
        let b = match i % 8 {
            0 => [0, 1, 2, 3, 4, 5],
            b => [b; 6],
        };
        *v = (pfirm[0] * (2 * (a >> b[0] & 1) - 1) as f64
            + pfirm[1] * (2 * (a >> b[1] & 1) - 1) as f64
            + pfirm[2] * (2 * (a >> b[2] & 1) - 1) as f64
            + pfirm[3] * (2 * (a >> b[3] & 1) - 1) as f64
            + pfirm[4] * (2 * (a >> b[4] & 1) - 1) as f64
            + pfirm[5] * (2 * (a >> b[5] & 1) - 1) as f64) as i32
    }
    pfir
}

fn main() {
    let pfira = pfir([
        2048.0 * 1.190566,
        2048.0 * 0.162580,
        2048.0 * 0.002208,
        2048.0 * 0.025475,
        2048.0 * -0.001522,
        2048.0 * 0.007322,
    ]);
    let pfirb = pfir([
        2048.0 * 0.001774,
        2048.0 * 0.004529,
        2048.0 * -0.001561,
        2048.0 * 0.000776,
        2048.0 * -0.000486,
        2048.0 * 0.002017,
    ]);

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("pink.rs");
    let mut dest_file = std::fs::File::create(dest_path).unwrap();
    write!(
        dest_file,
        "const PFIRA: [i32; 64] = {:?};\
         const PFIRB: [i32; 64] = {:?};",
        pfira, pfirb,
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
