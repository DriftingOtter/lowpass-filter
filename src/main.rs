use rand;
use rand::Rng;
use plotters::prelude::*;
use plotters::coord::Shift;

const LFP_BETA: f64 = 0.3;
const OUT_FILE_NAME: &str = "/home/daksh/Documents/lowpass_filter/src/lowpass_filter.png";

fn get_sample() -> f64 {
    return rand::thread_rng().gen_range(-1.0..=1.0);
}

fn draw_chart(area: DrawingArea<BitMapBackend, Shift>, caption: &str, signal: Vec<f64>, color: &RGBColor) -> Result<(), Box<dyn std::error::Error>> {
    let mut chart = ChartBuilder::on(&area)
        .margin(5)
        .set_all_label_area_size(50)
        .caption(caption, ("sans-serif", 40))
        .build_cartesian_2d(0..signal.len(), 0.001f64..1.0f64)?;

    chart.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .y_label_formatter(&|v| format!("{:.2}", v))
        .y_desc("Value (log scale)")
        .x_desc("Sample Index")
        .draw()?;

    chart.draw_series(LineSeries::new(
            signal.iter().enumerate().map(|(x, &y)| (x, y)),
            color,
    ))?
        .label(caption)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));

    chart.configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

fn plot_signals(input_signal: Vec<f64>, output_signal: Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new(OUT_FILE_NAME, (1920, 1080)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let root_area = root_area.titled("Raw Signal Vs. Lowpass Filter", ("sans-serif", 60))?;
    let (upper, lower) = root_area.split_vertically(540);

    draw_chart(upper, "Raw Signal", input_signal.clone(), &RED)?;
    draw_chart(lower, "Lowpass Filtered Signal", output_signal.clone(), &BLUE)?;

    root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}

fn main() {
    // Initialize the vectors with 100 elements
    let mut input_signal:  Vec<f64> = vec![0.0; 100];
    let mut output_signal: Vec<f64> = vec![0.0; 100];

    // Populate the input_signal with samples
    for sample in 0..100 {
        input_signal[sample] = get_sample();
    }

    let mut output_sample: f64 = 0.0;

    // Apply the low-pass filter to generate output_signal
    for sample in 0..100 {
        let input_sample = input_signal[sample];

        output_sample = (output_sample - (LFP_BETA * (output_sample - input_sample))).clamp(-1.0, 1.0);

        output_signal[sample] = output_sample;
    }

    // Plot signals
    plot_signals(input_signal, output_signal)
        .expect("Could not plot signals.");
}

