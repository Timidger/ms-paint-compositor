use wlroots::{wlroots_dehandle, compositor,
              input::keyboard,
              xkbcommon::xkb::keysyms};

use crate::CompositorState;

pub fn keyboard_added(_compositor_handle: compositor::Handle,
                  _keyboard_handle: keyboard::Handle)
                  -> Option<Box<keyboard::Handler>> {
    Some(Box::new(KeyboardHandler::default()))
}

#[derive(Default)]
struct KeyboardHandler;

impl keyboard::Handler for KeyboardHandler {
    #[wlroots_dehandle]
    fn on_key(&mut self,
              compositor_handle: compositor::Handle,
              _keyboard_handle: keyboard::Handle,
              key_event: &keyboard::event::Key) {
        #[dehandle] let compositor = compositor_handle;
        if key_event.key_state() == wlroots::WLR_KEY_RELEASED {
            return
        }
        for key in key_event.pressed_keys() {
            match key {
                keysyms::KEY_Escape => {
                    wlroots::compositor::terminate()
                },
                keysyms::KEY_XF86Switch_VT_1 ..= keysyms::KEY_XF86Switch_VT_12 => {
                    let backend = compositor.backend_mut();
                    if let Some(mut session) = backend.get_session() {
                        session.change_vt(key - keysyms::KEY_XF86Switch_VT_1 + 1);
                    }
                },
                keysyms::KEY_numbersign => {
                    let state: &mut CompositorState = compositor.data.downcast_mut().unwrap();
                    if state.color_state.editing_color.is_some() {
                        continue
                    }
                    state.color_state.editing_color = Some("#".into());
                    state.color_state.index = Some(0);
                }
                k => {
                    let state: &mut CompositorState = compositor.data.downcast_mut().unwrap();
                    state.color_state.update_key(k)
                }
            }
        }
        let state: &mut CompositorState = compositor.data.downcast_mut().unwrap();
        state.drawing = true;
        #[dehandle] let output_layout = state.output_layout_handle;
        for (output_handle, _) in output_layout.outputs() {
            #[dehandle] let output = output_handle;
            output.schedule_frame();
        }
    }
}
