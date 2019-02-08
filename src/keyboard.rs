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
                }
                k => {
                    let state: &mut CompositorState = compositor.data.downcast_mut().unwrap();
                    state.color_state.update_key(k)
                }
            }
        }
    }
}
