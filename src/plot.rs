// plot.rs
use plotters::prelude::*;

pub fn plot_results(results: Vec<(f32, f32)>) -> Result<(), Box<dyn std::error::Error>> {
    print!("PLOT");
    if results.is_empty() {
        print!("no data");
        return Ok(());
    }

    // Define output file
    let output_path = "survivors_vs_density.png";
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Determine min and max values for scaling
    let min_density = results
        .iter()
        .map(|(d, _)| *d)
        .fold(f32::INFINITY, f32::min);
    let max_density = results
        .iter()
        .map(|(d, _)| *d)
        .fold(f32::NEG_INFINITY, f32::max);
    let min_survivors = results
        .iter()
        .map(|(_, s)| *s)
        .fold(f32::INFINITY, f32::min);
    let max_survivors = results
        .iter()
        .map(|(_, s)| *s)
        .fold(f32::NEG_INFINITY, f32::max);

    // Add some padding to the ranges for better visualization
    let density_range = max_density - min_density;
    let survivors_range = max_survivors - min_survivors;
    let x_range = (
        min_density - 0.05 * density_range,
        max_density + 0.05 * density_range,
    );
    let y_range = (
        min_survivors - 0.05 * survivors_range,
        max_survivors + 0.05 * survivors_range,
    );

    // Create the chart
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Surviving Trees vs. Density",
            ("sans-serif", 30).into_font(),
        )
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(x_range.0..x_range.1, y_range.0..y_range.1)?;

    // Configure the chart
    chart
        .configure_mesh()
        .x_desc("Tree Density")
        .y_desc("Average Surviving Trees")
        .axis_desc_style(("sans-serif", 15).into_font())
        .x_label_formatter(&|x| format!("{:.1}", x))
        .y_label_formatter(&|y| format!("{:.0}", y))
        .draw()?;

    // Draw the line series
    chart
        .draw_series(LineSeries::new(results.iter().map(|&(d, s)| (d, s)), &BLUE))?
        .label("Survivors")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Configure the legend
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    // Finalize the plot
    root.present()?;
    println!("Plot saved as {}", output_path);

    Ok(())
}
