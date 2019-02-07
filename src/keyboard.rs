use wlroots::{wlroots_dehandle, compositor,
              input::keyboard,
              xkbcommon::xkb::keysyms};

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
        for key in key_event.pressed_keys() {
            match key {
                keysyms::KEY_Escape => {
                    wlroots::compositor::terminate()
                },
                keysyms::KEY_XF86Switch_VT_1 ..= keysyms::KEY_XF86Switch_VT_12 => {
                    #[dehandle] let compositor = compositor_handle;
                    let backend = compositor.backend_mut();
                    if let Some(mut session) = backend.get_session() {
                        session.change_vt(key - keysyms::KEY_XF86Switch_VT_1 + 1);
                    }
                }
                _ => { /* Do nothing */ }
            }
        }
    }
}
