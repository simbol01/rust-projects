
    /*
        let mut square_texture: Texture = texture_creator
            .create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
            .expect("Failed to create a texture");
        // We use the canvas to draw into our square texture.
        canvas
            .with_texture_canvas(&mut square_texture, |texture| {
                // We set the draw color to green.
                texture.set_draw_color(Color::RGB(0, 255, 0));
                // We "clear" our texture so it'll be fulfilled with green.
                texture.clear();
            })
            .expect("Failed to color a texture");
        // First we get the event handler:
        let mut event_pump = sdl_context.event_pump().expect(
            "Failed
    to get SDL event pump",
        );

        */

let video_subsystem = sdl_context.video().expect(
        "Couldn't get
SDL video subsystem",
    );
    sdl2::image::init(INIT_PNG | INIT_JPG).expect("Couldn't initialize image");

    // Parameters are: title, width, height
   // To make things easier to read, we'll create a constant which will be the texture's size.
    //const TEXTURE_SIZE: u32 = 32;
    // We create a texture with a 32x32 size.
    //
    // Playing with images
    let image_texture = texture_creator
        .load_texture("assets/passport.png")
        .expect("Couldn't load image");

    let green_square = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Green,
        TEXTURE_SIZE,
    )
    .expect("Cannot create green color");
    let blue_square = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Blue,
        TEXTURE_SIZE,
    )
    .expect("Cannot create Blue color");

    let timer = SystemTime::now();

    // event handler
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get sdl event pump");
    // Then we create an infinite loop to loop over events:
    'running: loop {
     





for event in event_pump.poll_iter() {
            match event {
                // If we receive a 'quit' event or if the user press the'ESC' key, we quit.
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running; // We "break" the infinite loop.
                }
                _ => {}
            }
        }

        // Load and save my image
        /*
                // We set fulfill our window with red.
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                // We draw it.
                canvas.clear();
                canvas
                    .copy(&image_texture, None, None)
                    .expect("Couldn't render image");
                canvas.present();
                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }
        }
        */

        // color switches
        let display_green = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() % 2 == 0,
            Err(_) => true,
        };

        let square_texture = if display_green {
            &green_square
        } else {
            &blue_square
        };

        // Copy our texture into the window.
        canvas
            .copy(
                &square_texture,
                None,
                // We copy it at the top-left of the window with a 32x32 size.
                Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE),
            )
            .expect("Couldn't copy texture into window");
        // We update window's display.
        canvas.present();
        // We sleep enough to get ~60 fps. If we don't call this, the program will take
        // 100% of a CPU time.
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
