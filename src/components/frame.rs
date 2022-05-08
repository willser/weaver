use eframe::egui::epaint::Shadow;
use eframe::egui::layers::ShapeIdx;
use eframe::egui::style::Margin;
use eframe::egui::{
    Color32, InnerResponse, Rect, Response, Rounding, Sense, Shape, Stroke, Style, Ui,
};
use eframe::epaint;

/// This frame is refactor from `eframe::egui::Frame`.Aim to support event of click and style change.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    /// Margin within the painted frame.
    pub inner_margin: Margin,
    /// Margin outside the painted frame.
    pub outer_margin: Margin,
    pub rounding: Rounding,
    pub shadow: Shadow,
    pub fill: Color32,
    pub stroke: Stroke,
    pub sense: Sense,
}

impl Default for Frame {
    fn default() -> Self {
        Frame {
            inner_margin: Default::default(),
            outer_margin: Default::default(),
            rounding: Default::default(),
            shadow: Default::default(),
            fill: Default::default(),
            stroke: Default::default(),
            sense: Sense::hover(),
        }
    }
}

impl Frame {
    #[allow(dead_code)]
    pub fn none() -> Self {
        Self::default()
    }

    /// For when you want to group a few widgets together within a frame.
    #[allow(dead_code)]
    pub fn group(style: &Style) -> Self {
        Self {
            inner_margin: Margin::same(6.0), // same and symmetric looks best in corners when nesting groups
            rounding: style.visuals.widgets.noninteractive.rounding,
            stroke: style.visuals.widgets.noninteractive.bg_stroke,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub(crate) fn side_top_panel(style: &Style) -> Self {
        Self {
            inner_margin: Margin::symmetric(8.0, 2.0),
            rounding: Rounding::none(),
            fill: style.visuals.window_fill(),
            stroke: style.visuals.window_stroke(),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub(crate) fn central_panel(style: &Style) -> Self {
        Self {
            inner_margin: Margin::same(8.0),
            rounding: Rounding::none(),
            fill: style.visuals.window_fill(),
            stroke: Default::default(),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn window(style: &Style) -> Self {
        Self {
            inner_margin: style.spacing.window_margin,
            rounding: style.visuals.window_rounding,
            shadow: style.visuals.window_shadow,
            fill: style.visuals.window_fill(),
            stroke: style.visuals.window_stroke(),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn menu(style: &Style) -> Self {
        Self {
            inner_margin: Margin::same(1.0),
            rounding: style.visuals.widgets.noninteractive.rounding,
            shadow: style.visuals.popup_shadow,
            fill: style.visuals.window_fill(),
            stroke: style.visuals.window_stroke(),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn popup(style: &Style) -> Self {
        Self {
            inner_margin: style.spacing.window_margin,
            rounding: style.visuals.widgets.noninteractive.rounding,
            shadow: style.visuals.popup_shadow,
            fill: style.visuals.window_fill(),
            stroke: style.visuals.window_stroke(),
            ..Default::default()
        }
    }

    /// A canvas to draw on.
    ///
    /// In bright mode this will be very bright,
    /// and in dark mode this will be very dark.
    #[allow(dead_code)]
    pub fn canvas(style: &Style) -> Self {
        Self {
            inner_margin: Margin::same(2.0),
            rounding: style.visuals.widgets.noninteractive.rounding,
            fill: style.visuals.extreme_bg_color,
            stroke: style.visuals.window_stroke(),
            ..Default::default()
        }
    }

    /// A dark canvas to draw on.
    #[allow(dead_code)]
    pub fn dark_canvas(style: &Style) -> Self {
        Self {
            fill: Color32::from_black_alpha(250),
            ..Self::canvas(style)
        }
    }
}

impl Frame {
    #[allow(dead_code)]
    pub fn fill(mut self, fill: Color32) -> Self {
        self.fill = fill;
        self
    }

    #[allow(dead_code)]
    pub fn stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = stroke;
        self
    }

    #[allow(dead_code)]
    pub fn rounding(mut self, rounding: impl Into<Rounding>) -> Self {
        self.rounding = rounding.into();
        self
    }

    /// Margin within the painted frame.
    #[allow(dead_code)]
    pub fn inner_margin(mut self, inner_margin: impl Into<Margin>) -> Self {
        self.inner_margin = inner_margin.into();
        self
    }

    /// Margin outside the painted frame.
    #[allow(dead_code)]
    pub fn outer_margin(mut self, outer_margin: impl Into<Margin>) -> Self {
        self.outer_margin = outer_margin.into();
        self
    }

    #[allow(dead_code)]
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = shadow;
        self
    }
    #[allow(dead_code)]
    pub fn multiply_with_opacity(mut self, opacity: f32) -> Self {
        self.fill = self.fill.linear_multiply(opacity);
        self.stroke.color = self.stroke.color.linear_multiply(opacity);
        self.shadow.color = self.shadow.color.linear_multiply(opacity);
        self
    }
}

pub struct Prepared {
    pub frame: Frame,
    where_to_put_background: ShapeIdx,
    pub content_ui: Ui,
}

impl Frame {
    pub fn begin(self, ui: &mut Ui) -> Prepared {
        let where_to_put_background = ui.painter().add(Shape::Noop);
        let outer_rect_bounds = ui.available_rect_before_wrap();

        let mut inner_rect = outer_rect_bounds;
        inner_rect.min += self.outer_margin.left_top() + self.inner_margin.left_top();
        inner_rect.max -= self.outer_margin.right_bottom() + self.inner_margin.right_bottom();

        // Make sure we don't shrink to the negative:
        inner_rect.max.x = inner_rect.max.x.max(inner_rect.min.x);
        inner_rect.max.y = inner_rect.max.y.max(inner_rect.min.y);

        let content_ui = ui.child_ui(inner_rect, *ui.layout());

        // content_ui.set_clip_rect(outer_rect_bounds.shrink(self.stroke.width * 0.5)); // Can't do this since we don't know final size yet

        Prepared {
            frame: self,
            where_to_put_background,
            content_ui,
        }
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        self.show_dyn(ui, Box::new(add_contents))
    }

    fn show_dyn<'c, R>(
        self,
        ui: &mut Ui,
        add_contents: Box<dyn FnOnce(&mut Ui) -> R + 'c>,
    ) -> InnerResponse<R> {
        let mut prepared = self.begin(ui);
        let ret = add_contents(&mut prepared.content_ui);
        let response = prepared.end(ui, self.sense);

        InnerResponse::new(ret, response)
    }

    pub fn paint(&self, outer_rect: Rect) -> Shape {
        let Self {
            inner_margin: _,
            outer_margin: _,
            rounding,
            shadow,
            fill,
            stroke,
            ..
        } = *self;

        let frame_shape = Shape::Rect(epaint::RectShape {
            rect: outer_rect,
            rounding,
            fill,
            stroke,
        });

        if shadow == Default::default() {
            frame_shape
        } else {
            let shadow = shadow.tessellate(outer_rect, rounding);
            let shadow = Shape::Mesh(shadow);
            Shape::Vec(vec![shadow, frame_shape])
        }
    }
}

impl Prepared {
    fn paint_rect(&self) -> Rect {
        let mut rect = self.content_ui.min_rect();
        rect.min -= self.frame.inner_margin.left_top();
        rect.max += self.frame.inner_margin.right_bottom();
        rect
    }

    pub fn end(self, ui: &mut Ui, sense: Sense) -> Response {
        let paint_rect = self.paint_rect();

        let Prepared {
            frame,
            where_to_put_background,
            ..
        } = self;

        if ui.is_rect_visible(paint_rect) {
            let shape = frame.paint(paint_rect);
            ui.painter().set(where_to_put_background, shape);
        }
        ui.allocate_rect(paint_rect, sense)
    }
}
