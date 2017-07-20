use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;

pub fn handle_event(event: &Event) -> bool {
    let mut should_quit: bool = false;

    match event {
          &Event::Quit { .. }
        | &Event::KeyDown { keycode: Some(Keycode::Escape), .. }
        => {
            should_quit = true;
        },
        &Event::KeyDown { keycode: Some(key), .. } => {
            match key {
                Keycode::A => (),
                Keycode::S => (),
                Keycode::D => (),
                Keycode::F => (),
                Keycode::Q => (),
                Keycode::E => (),
                Keycode::Up => (),
                Keycode::Down => (),
                Keycode::Right => (),
                Keycode::Left => (),
                Keycode::Space => (),
                _ => (),
            }
        },
        &Event::Window { win_event: w_event, ..} => {
            match w_event {
                WindowEvent::Enter => (),
                WindowEvent::Leave => (),
                WindowEvent::SizeChanged(new_x, new_y) => {
                    println!("Window size changed to: ({},{})", new_x, new_y);
                },
                _ => (),
            }
        },
        _ => {
            println!("recieved: {:?}", event);
        }
    }

    should_quit
}
