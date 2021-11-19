extern crate xcb;

fn main() {
  let rectangles: &[xcb::Rectangle] = &[
    xcb::Rectangle::new(10, 50, 40, 20),
    xcb::Rectangle::new(80, 50, 10, 40)
  ];
  let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
  let setup = conn.get_setup();
  let screen = setup.roots().nth(screen_num as usize).unwrap();
  let foreground = conn.generate_id();
  xcb::create_gc(&conn, foreground, screen.root(), &[
    (xcb::GC_FOREGROUND, screen.black_pixel()),
    (xcb::GC_GRAPHICS_EXPOSURES, 0),
  ]);
  let win = conn.generate_id();
  xcb::create_window(&conn, xcb::COPY_FROM_PARENT as u8, win, screen.root(), 0, 0, 720, 480, 10, xcb::WINDOW_CLASS_INPUT_OUTPUT as u16, screen.root_visual(), &[
    (xcb::CW_BACK_PIXEL, screen.white_pixel()),
    (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
  ]);
  xcb::map_window(&conn, win);
  conn.flush();
  loop {
    let event = conn.wait_for_event();
    match event {
      None => { break; }
      Some(event) => {
        let r = event.response_type() & !0x80;
        match r {
          xcb::EXPOSE => {
            xcb::poly_rectangle(&conn, win, foreground, &rectangles);
            conn.flush();
          },
          xcb::KEY_PRESS => {
            let key_press: &xcb::KeyPressEvent = unsafe {
              xcb::cast_event(&event)
            };
            println!("Key '{}' pressed", key_press.detail());
            break;
          },
          _ => {}
        }
      }
    }
  }
}
