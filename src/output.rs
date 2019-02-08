use wlroots::{area::{Area, Origin, Size},
              render::{GenericRenderer, matrix},
              wlroots_dehandle, compositor, output};
use rusttype::{Font, Scale};

use crate::CompositorState;

struct OutputHandler {
    font: Font<'static>
}

impl OutputHandler {
    fn new() -> Self {
        let font_data = include_bytes!("../Roboto-Regular.ttf");
        let font = Font::from_bytes(font_data as &[u8])
            .expect("Error constructing the font");
        OutputHandler { font }
    }
}

impl output::Handler for OutputHandler {
    #[wlroots_dehandle]
    fn on_frame(&mut self,
                compositor_handle: compositor::Handle,
                output_handle: output::Handle) {
        #[dehandle] let compositor = compositor_handle;
        #[dehandle] let output = output_handle;
        let state: &mut CompositorState = compositor.data.downcast_mut().unwrap();
        let renderer = compositor.renderer.as_mut().expect("No renderer");
        self.render_drawing(state, output, renderer);
        self.render_color_change(state, output, renderer);
    }
}

impl OutputHandler {
    fn render_drawing(&mut self,
                      state: &mut CompositorState,
                      output: &mut output::Output,
                      renderer: &mut GenericRenderer) {
        if !state.drawing {
            state.dirty.drain(..);
            return
        }
        let transform_matrix = output.transform_matrix();
        let mut renderer = renderer.render(output, None);
        for (x, y) in &state.dirty {
            let area = Area::new(Origin::new(*x as _, *y as _), Size::new(1, 1));
            renderer.render_colored_rect(area, [1.0, 1.0, 1.0, 1.0], transform_matrix);
        }
    }

    fn render_color_change(&mut self,
                           state: &mut CompositorState,
                           output: &mut output::Output,
                           renderer: &mut GenericRenderer) {
        let transform_matrix = output.transform_matrix();
        let transform_inverted = output.get_transform().invert();
        let (width, height) = output.effective_resolution();
        let mut renderer = renderer.render(output, None);
        let color_state = &mut state.color_state;
        let color = match color_state.editing_color.as_mut() {
            None => return,
            Some(color) => color
        };
        let scale = Scale::uniform(32.0);
        let mut area = Area::new(Origin::new(width / 2, height / 2),
                             Size::new(100, 100)); // TODO this number is made up
        let v_metrics = self.font.v_metrics(scale);
        // layout the glyphs in a line with 5 pixel padding
        let glyphs: Vec<_> = self.font
            .layout(color.as_str(), scale, rusttype::point(5.0, 5.0 + v_metrics.ascent))
            .collect();
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x) as u32
        };
        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            // Draw the glyph into the image per-pixel by using the draw closure
            let mut bytes = vec![0u8; (glyphs_width * 4 * glyphs_height) as usize];
            glyph.draw(|x, y, v| {
                let index = ((x * 4) + (y * glyphs_width * 4) ) as usize;
                bytes[index + 0] = (v * 255.0) as u8;
                bytes[index + 1] = (v * 255.0) as u8;
                bytes[index+ 2] = (v * 255.0) as u8;
            });
            let texture = renderer.create_texture_from_pixels(wlroots::wl_shm_format::WL_SHM_FORMAT_ARGB8888,
                                                              glyphs_width * 4,
                                                              glyphs_width,
                                                              glyphs_height,
                                                              &bytes)
                .expect("Could not construct texture");
            let matrix = matrix::project_box(area,
                                             transform_inverted,
                                             0.0,
                                             transform_matrix);
            area.origin.x += area.size.width / 2 ;
            renderer.render_texture_with_matrix(&texture, matrix);
        }
    }
}

pub struct LayoutHandler;

impl output::layout::Handler for LayoutHandler {}

#[wlroots_dehandle]
pub fn output_added<'output>(compositor: compositor::Handle,
                             builder: output::Builder<'output>)
                             -> Option<output::BuilderResult<'output>> {
    let result = builder.build_best_mode(OutputHandler::new());
    #[dehandle] let compositor = compositor;
    let CompositorState { ref output_layout_handle, .. } = compositor.downcast();
    #[dehandle] let output = result.output.clone();
    #[dehandle] let output_layout = output_layout_handle;
    output_layout.add_auto(output);
    Some(result)
}
