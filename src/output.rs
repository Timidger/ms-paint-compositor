use wlroots::{area::{Area, Origin, Size}, wlroots_dehandle, compositor, output};

use crate::CompositorState;

struct OutputHandler;

impl output::Handler for OutputHandler {
    #[wlroots_dehandle]
    fn on_frame(&mut self,
                compositor_handle: compositor::Handle,
                output_handle: output::Handle) {
        #[dehandle] let compositor = compositor_handle;
        #[dehandle] let output = output_handle;
        let state: &mut CompositorState = compositor.data.downcast_mut().unwrap();
        if state.dirty.is_empty() {
            return
        }
        let transform_matrix = output.transform_matrix();
        let renderer = compositor.renderer.as_mut().expect("No renderer");
        let mut renderer = renderer.render(output, None);
        for (x, y) in &state.dirty {
            let area = Area::new(Origin::new(*x as _, *y as _), Size::new(1, 1));
            renderer.render_colored_rect(area, [1.0, 1.0, 1.0, 1.0], transform_matrix);
        }
    }
}

pub struct LayoutHandler;

impl output::layout::Handler for LayoutHandler {}

#[wlroots_dehandle]
pub fn output_added<'output>(compositor: compositor::Handle,
                             builder: output::Builder<'output>)
                             -> Option<output::BuilderResult<'output>> {
    let result = builder.build_best_mode(OutputHandler);
    #[dehandle] let compositor = compositor;
    let CompositorState { ref output_layout_handle, .. } = compositor.downcast();
    #[dehandle] let output = result.output.clone();
    #[dehandle] let output_layout = output_layout_handle;
    output_layout.add_auto(output);
    Some(result)
}
