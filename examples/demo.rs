use std::fs;

use plotters::{
    chart::ChartBuilder,
    coord::Shift,
    prelude::{DrawingArea, IntoDrawingArea, IntoLinspace, PathElement, Rectangle},
    series::{LineSeries, SurfaceSeries},
    style::{Color, BLACK, BLUE, WHITE},
};
use skia_plotters_backend::SkiaBackend;
use skia_safe::{
    surfaces::raster_n32_premul, textlayout::FontCollection, EncodedImageFormat, FontMgr,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = (800, 600);

    // Create canvas and font collection
    let mut surface = raster_n32_premul((width, height)).expect("Failed to create the surface.");
    surface.canvas().clear(skia_safe::Color::WHITE);
    let mut font_collection = FontCollection::new();
    let font_manager = FontMgr::new();
    font_collection.set_default_font_manager(font_manager, None);

    // Create the skia backend
    let backend = SkiaBackend::new(surface.canvas(), &mut font_collection, (width, height))
        .into_drawing_area();

    // Render the plot into the backend
    render_plot(backend)?;

    // Save canvas to file
    let image = surface.image_snapshot();
    let mut context = surface.direct_context();
    let image_data = image
        .encode(context.as_mut(), EncodedImageFormat::PNG, None)
        .expect("Failed to encode the snapshot.");
    fs::write("./demo.png", image_data.as_bytes())?;

    Ok(())
}

// Code straight from https://github.com/plotters-rs/plotters/blob/master/plotters/examples/3d-plot.rs
fn render_plot(
    backend: DrawingArea<SkiaBackend<'_>, Shift>,
) -> Result<(), Box<dyn std::error::Error>> {
    backend.fill(&WHITE).unwrap();

    let x_axis = (-3.0..3.0).step(0.1);
    let z_axis = (-3.0..3.0).step(0.1);

    let mut chart = ChartBuilder::on(&backend)
        .caption("3D Plot Test", ("sans", 20))
        .build_cartesian_3d(x_axis.clone(), -3.0..3.0, z_axis.clone())?;

    chart.with_projection(|mut pb| {
        pb.yaw = 0.5;
        pb.scale = 0.9;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()?;

    chart
        .draw_series(
            SurfaceSeries::xoz(
                (-30..30).map(|f| f as f64 / 10.0),
                (-30..30).map(|f| f as f64 / 10.0),
                |x, z| (x * x + z * z).cos(),
            )
            .style(BLUE.mix(0.2).filled()),
        )?
        .label("Surface")
        .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));

    chart
        .draw_series(LineSeries::new(
            (-100..100)
                .map(|y| y as f64 / 40.0)
                .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
            &BLACK,
        ))?
        .label("Line")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));

    chart.configure_series_labels().border_style(BLACK).draw()?;

    Ok(())
}
