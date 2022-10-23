use splotch::{Chart, Domain, plot};
use fon::{Audio, chan::{Ch16, Channel}, pos::Left};

// Plots 2200 samples of audio (440Hz wave 5 cycles)
pub(super) fn write(audio: &Audio<Ch16, 2>) {
/*    let mut data = Vec::new();
    for (i, frame) in audio.iter().enumerate() {
        let sample = frame[Left];
        let point = (i as i16, i16::from(sample));
        data.push(point);
    }
    let point = [(2200.0f32, 1.0f32)];
    let domain = Domain::from_data(&data).with_x(&point).with_y(&point);
    let plot = plot::Line::new(&domain, &data);
    let chart = Chart::builder()
        .with_plot(&plot)
        .build();*/

    let data = vec![(13, 74), (111, -37), (125, 52), (190, 66)];
    let domain = Domain::from_data(&data).with_x(&[0.0, 200.0]);
    let plot = plot::Line::new(&domain, &data);
    let chart = Chart::builder()
        .with_title("Line Plot")
        .with_axis(domain.x_axis().with_name("X Axis Name"))
        .with_axis(domain.y_axis().with_name("Y Axis Name").on_right())
        .with_plot(&plot)
        .build();

    std::fs::write("chart.svg", chart.to_string());
}
