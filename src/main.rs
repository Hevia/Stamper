use nannou::prelude::*;
use nannou::ui::prelude::*;
fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
struct Shape {
    resolution: usize,
    scale: f32,
    rotation: f32,
    color: Rgb,
    position: Point2,
}
impl Shape {
    fn new(resolution: usize, scale: f32, rotation: f32, color: Rgb, position: Point2) -> Self {
        Shape {
            resolution,
            scale,
            rotation,
            color,
            position,
        }
    }
}
struct Model {
    ui: Ui,
    ids: Ids,
    resolution: usize,
    scale: f32,
    rotation: f32,
    color: Rgb,
    position: Point2,
    shapes: Vec<Shape>,
}
struct Ids {
    resolution: widget::Id,
    scale: widget::Id,
    rotation: widget::Id,
    random_color: widget::Id,
    position: widget::Id,
    complete: widget::Id,
}
fn model(app: &App) -> Model {
    // Set the loop mode to wait for events, an energy-efficient option for pure-GUI apps.
    app.set_loop_mode(LoopMode::Wait);
    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();
    // Generate some ids for our widgets.
    let ids = Ids {
        resolution: ui.generate_widget_id(),
        scale: ui.generate_widget_id(),
        rotation: ui.generate_widget_id(),
        random_color: ui.generate_widget_id(),
        position: ui.generate_widget_id(),
        complete: ui.generate_widget_id(),
    };
    // Init our variables
    let resolution = 6;
    let scale = 200.0;
    let rotation = 0.0;
    let position = pt2(0.0, 0.0);
    let color = rgb(1.0, 0.0, 1.0);
    Model {
        ui,
        ids,
        resolution,
        scale,
        rotation,
        position,
        color,
        shapes: Vec::new(),
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();
    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }
    for value in slider(model.resolution as f32, 3.0, 15.0)
        .top_left_with_margin(20.0)
        .label("Resolution")
        .set(model.ids.resolution, ui)
    {
        model.resolution = value as usize;
    }
    for value in slider(model.scale, 10.0, 500.0)
        .down(10.0)
        .label("Scale")
        .set(model.ids.scale, ui)
    {
        model.scale = value;
    }
    for value in slider(model.rotation, -PI, PI)
        .down(10.0)
        .label("Rotation")
        .set(model.ids.rotation, ui)
    {
        model.rotation = value;
    }
    for _click in widget::Button::new()
        .down(10.0)
        .w_h(200.0, 60.0)
        .label("Random Color")
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(model.ids.random_color, ui)
    {
        model.color = rgb(random(), random(), random());
    }
    for (x, y) in widget::XYPad::new(
        model.position.x,
        -200.0,
        200.0,
        model.position.y,
        -200.0,
        200.0,
    )
    .down(10.0)
    .w_h(200.0, 200.0)
    .label("Position")
    .label_font_size(15)
    .rgb(0.3, 0.3, 0.3)
    .label_rgb(1.0, 1.0, 1.0)
    .border(0.0)
    .set(model.ids.position, ui)
    {
        model.position = Point2::new(x, y);
    }
    for _click in widget::Button::new()
        .down(10.0)
        .w_h(200.0, 60.0)
        .label("Stamp")
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(model.ids.complete, ui)
    {
        // Stamp code goes here
        model.shapes.push(Shape::new(
            model.resolution,
            model.scale,
            model.rotation,
            model.color,
            model.position,
        ));
    }
}
// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().rgb(0.02, 0.02, 0.02);
    draw.ellipse()
        .xy(model.position)
        .radius(model.scale)
        .resolution(model.resolution)
        .rotate(model.rotation)
        .color(model.color);
    for shape in model.shapes.iter() {
        draw.ellipse()
            .xy(shape.position)
            .radius(shape.scale)
            .resolution(shape.resolution)
            .rotate(shape.rotation)
            .color(shape.color);
    }
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}
