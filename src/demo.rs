extern crate whetstone;

use egui::{vec2, Color32, Pos2, Rangef, Rect, Response, Stroke, Style, Ui, Vec2, Visuals};
use whetstone::{syntax::Syntax, Equation, Parser};

static X_MIN: f32 = -10.0;
static X_MAX: f32 = 10.0;
static RANGE: f32 = X_MAX - X_MIN;
static INTERVALS: usize = 500;
static STEP: f32 = RANGE / (INTERVALS as f32);

fn main() -> eframe::Result {
    env_logger::init();

    const WINDOW_SIZE: Vec2 = vec2(1200.0, 800.0);
    const MIN_WINDOW_SIZE: Vec2 = vec2(100.0, 100.0);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(WINDOW_SIZE)
            .with_min_inner_size(MIN_WINDOW_SIZE),

        ..Default::default()
    };

    eframe::run_native(
        "WhetstoneDemo",
        native_options,
        Box::new(|cc| {
            let style = Style {
                visuals: Visuals::dark(),
                ..Style::default()
            };
            cc.egui_ctx.set_style(style);
            Ok(Box::new(DemoApp::new(cc)))
        }),
    )
}

pub struct DemoApp {
    equation_str: String,
    equation: Equation<f32>,
    syntax: Syntax,
    error: Option<String>,
    x_coords: Vec<f32>,
    y_coords: Vec<f32>,
    equation_factory: Parser<f32>,
}

impl DemoApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut x_coords = Vec::new();
        for i in 0..=INTERVALS {
            x_coords.push(X_MIN + (i as f32) * STEP);
        }
        let y_coords = x_coords.clone();
        let equation_factory = Parser::<f32>::new(Syntax::Standard).unwrap();
        let equation_str = "x";
        Self {
            equation_str: equation_str.to_string(),
            equation: equation_factory.parse(equation_str).unwrap(),
            syntax: Syntax::Standard,
            x_coords,
            y_coords,
            equation_factory: Parser::<f32>::new(Syntax::Standard).unwrap(),
            error: None,
        }
    }
}

impl eframe::App for DemoApp {
    /// Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let last_syntax = self.syntax.clone();

        let settings = egui::TopBottomPanel::top("settings");
        let settings_drawn: Response = settings
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Equation");

                    let color = if self.error.is_some() {
                        Color32::from_rgb(100, 0, 0)
                    } else {
                        Color32::default()
                    };
                    if ui
                        .add(
                            egui::TextEdit::singleline(&mut self.equation_str)
                                .background_color(color),
                        )
                        .changed()
                    {
                        let result = self.equation_factory.parse(&self.equation_str);
                        if result.is_ok() {
                            self.equation = result.unwrap();
                            self.error = None;
                        } else {
                            self.error = Some(result.err().unwrap().message);
                        }
                    }

                    ui.radio_value(&mut self.syntax, Syntax::Standard, "Standard");
                    ui.radio_value(&mut self.syntax, Syntax::LaTeX, "LaTeX");

                    if self.error.is_some() {
                        ui.label(
                            egui::RichText::new(self.error.as_ref().unwrap()).color(Color32::RED),
                        );
                    }
                });
            })
            .response;

        if self.syntax != last_syntax {
            self.equation_factory = Parser::<f32>::new(self.syntax.clone()).unwrap();
        }

        // draws the simulation in the main panel of the window
        let style = Style::default();
        let _ = egui::CentralPanel::default()
            .frame(egui::Frame::canvas(&style))
            .show(ctx, |ui| {
                let canvas_extent = Rect::from_two_pos(
                    Pos2::new(ctx.screen_rect().left(), settings_drawn.rect.bottom()),
                    Pos2::new(ctx.screen_rect().right(), ctx.screen_rect().bottom()),
                );

                let canvas = Canvas::new(
                    ui,
                    canvas_extent,
                    Rangef {
                        min: X_MIN,
                        max: X_MAX,
                    },
                );

                canvas.draw_grid_lines();
                canvas.draw_axes();

                for (i, point) in self.y_coords.iter_mut().enumerate() {
                    *point = self
                        .equation
                        .evaluate(&[("x", self.x_coords[i])])
                        .unwrap_or_else(|err| -> f32 {
                            self.error = Some(err.message);
                            return 0.0;
                        });
                }

                canvas.draw_points(
                    &self.x_coords,
                    &self.y_coords,
                    &Color32::from_rgb(255, 50, 50),
                );
            })
            .response;
    }
}

// Helper struct for drawing objects in world space onto the screen.
// This maintains aspect ratio, so `visible_world` is clipped to fill the screen area.
pub struct Canvas<'a> {
    ui: &'a Ui,
    screen_extent: Rect,
    range: Rect,
    scale: f32,
}

impl<'a> Canvas<'a> {
    pub fn new(ui: &'a Ui, screen_extent: Rect, visible_x_axis: Rangef) -> Self {
        let y_span = visible_x_axis.span() / screen_extent.aspect_ratio();

        let range = Rect::from_x_y_ranges(visible_x_axis, Rangef::new(-y_span / 2.0, y_span / 2.0));

        let scale = screen_extent.width() / range.width();

        Canvas {
            ui,
            screen_extent,
            range,
            scale,
        }
    }

    fn world_to_screen_x(&self, x: f32) -> f32 {
        self.screen_extent.min.x + self.scale * (x - self.range.min.x)
    }

    fn world_to_screen_y(&self, y: f32) -> f32 {
        self.screen_extent.min.y + self.scale * (-y - self.range.min.y)
    }

    pub fn draw_grid_lines(&self) {
        const MAX_GRIDLINES: f32 = 20.0;
        let step = (self.range.x_range().span() / MAX_GRIDLINES).round();

        let mut y = step * (self.range.min.y / step).round();
        while y < self.range.max.y {
            self.ui.painter().hline(
                self.screen_extent.x_range(),
                self.world_to_screen_y(y),
                Stroke::new(1.0, Color32::from_rgb(15, 15, 15)),
            );
            y += step;
        }
        let mut x = step * (self.range.min.x / step).round();
        while x < self.range.max.x {
            self.ui.painter().vline(
                self.world_to_screen_x(x),
                self.screen_extent.y_range(),
                Stroke::new(1.0, Color32::from_rgb(15, 15, 15)),
            );
            x += step;
        }
    }

    pub fn draw_axes(&self) {
        self.ui.painter().vline(
            self.world_to_screen_x(0.0),
            self.screen_extent.y_range(),
            Stroke::new(2.0, Color32::from_rgb(20, 20, 20)),
        );
        self.ui.painter().hline(
            self.screen_extent.x_range(),
            self.world_to_screen_y(0.0),
            Stroke::new(2.0, Color32::from_rgb(20, 20, 20)),
        );
    }

    pub fn draw_points(&self, x_points: &[f32], y_points: &[f32], colour: &Color32) {
        if (x_points.len() < 2) || (x_points.len() != y_points.len()) {
            log::error!(
                "Slices passed to draw_points have invalid sizes ({}x{})",
                x_points.len(),
                y_points.len()
            );
            return;
        }
        let mut screen_points = Vec::with_capacity(x_points.len());
        for (x, y) in x_points.iter().zip(y_points) {
            screen_points.push(Pos2::new(
                self.world_to_screen_x(*x),
                self.world_to_screen_y(*y),
            ));
        }
        self.ui
            .painter()
            .line(screen_points, Stroke::new(2.0, *colour));
    }
}
