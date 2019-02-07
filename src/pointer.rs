use wlroots::{wlroots_dehandle,
              compositor,
              input::pointer,
              cursor};

use CompositorState;

pub struct CursorHandler;

impl cursor::Handler for CursorHandler {}

pub struct PointerHandler;

impl pointer::Handler for PointerHandler {
    #[wlroots_dehandle]
    fn on_motion_absolute(&mut self,
                          compositor_handle: compositor::Handle,
                          _pointer_handle: pointer::Handle,
                          absolute_motion_event: &pointer::event::AbsoluteMotion) {
        #[dehandle] let compositor = compositor_handle;
        let &mut CompositorState { ref cursor_handle,
                                   ref mut dirty,
                                   drawing, .. } = compositor.downcast();
        #[dehandle] let cursor = cursor_handle;
        let (x, y) = absolute_motion_event.pos();
        let (old_x, old_y) = cursor.coords();
        cursor.warp_absolute(absolute_motion_event.device(), x,  y);
        let (x, y) = cursor.coords();
        if !drawing {
            return
        }
        let delta_x = old_x as isize - x as isize;
        let delta_y = old_y as isize - y as isize;
        draw(dirty, (x as _, y as _), (old_x as _, old_y as _), (delta_x, delta_y));
    }

    #[wlroots_dehandle]
    fn on_button(&mut self,
                 compositor_handle: compositor::Handle,
                 _pointer_handle: pointer::Handle,
                 event: &pointer::event::Button) {
        #[dehandle] let compositor = compositor_handle;
        let state: &mut CompositorState = compositor.downcast();
        #[dehandle] let cursor = state.cursor_handle.clone();
        let (x, y) = cursor.coords();
        state.dirty.push((x as _, y as _));
        state.drawing = event.state() == wlroots::WLR_BUTTON_PRESSED
    }

    #[wlroots_dehandle]
    fn on_motion(&mut self,
                 compositor_handle: compositor::Handle,
                 _pointer_handle: pointer::Handle,
                 motion_event: &pointer::event::Motion) {
        #[dehandle] let compositor = compositor_handle;
        let &mut CompositorState { ref cursor_handle, ref mut dirty, drawing, .. } = compositor.downcast();
        #[dehandle] let cursor = cursor_handle;
        let (delta_x, delta_y) = motion_event.delta();
        let (old_x, old_y) = cursor.coords();
        let (old_x, old_y) = (old_x as isize, old_y as isize);
        cursor.move_to(None, delta_x, delta_y);
        if !drawing {
            return;
        }
        let (x, y) = cursor.coords();
        let (delta_x, delta_y) = (delta_x.round() as isize, delta_y.round() as isize);
        let (x, y) = (x as isize, y as isize);
        draw(dirty, (old_x, old_y), (x, y), (delta_x, delta_y));
    }
}

fn draw(dirty: &mut Vec<(usize, usize)>,
        (mut old_x, mut old_y): (isize, isize),
        (x, y): (isize, isize),
        (delta_x, delta_y): (isize, isize)) {
    while old_x != x || old_y != y {
        dirty.push(((old_x) as usize, (old_y) as usize));
        if old_x != x {
            old_x += if delta_x > 0 {1} else {-1};
        }
        if old_y != y {
            old_y += if delta_y > 0 {1} else {-1};
        }
    }
}

#[wlroots_dehandle]
pub fn pointer_added(compositor_handle: compositor::Handle,
                     pointer_handle: pointer::Handle)
                     -> Option<Box<pointer::Handler>> {
    #[dehandle] let compositor = compositor_handle;
    #[dehandle] let pointer = pointer_handle;
    let CompositorState { ref cursor_handle, ref mut xcursor_manager,
                          .. } = compositor.downcast();
    #[dehandle] let cursor = cursor_handle;
    xcursor_manager.set_cursor_image("left_ptr".to_string(), cursor);
    cursor.attach_input_device(pointer.input_device());
    Some(Box::new(PointerHandler) as Box<pointer::Handler>)
}
