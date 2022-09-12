use crate::automaton::Automaton;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

pub enum Init {
    Random,
    Cells(Vec<(u32, u32)>),
}

pub fn points_to_rects(
    points: &Vec<(u32, u32)>,
    width: u32,
    height: u32,
    window_width: u32,
    window_height: u32,
) -> Vec<Rect> {
    const PAD_RATIO: f64 = 0.2;
    let len_h = window_width as f64 / (width as f64 + width as f64 * PAD_RATIO + PAD_RATIO);
    let pad_h = len_h * PAD_RATIO;
    let len_v = window_height as f64 / (height as f64 + height as f64 * PAD_RATIO + PAD_RATIO);
    let pad_v = len_v * PAD_RATIO;

    let mut rects: Vec<Rect> = vec![];
    for point in points {
        assert!(point.0 < width);
        assert!(point.1 < height);
        let rect = Rect::new(
            ((len_h + pad_h) * point.0 as f64 + pad_h) as i32,
            ((len_v + pad_v) * point.1 as f64+ pad_v) as i32,
            len_h as u32,
            len_v as u32,
        );
        rects.push(rect);
    }
    rects
}

pub fn render_loop(init: Init) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width = 800;
    let window_height = 800;
    let window = video_subsystem
        .window("Cellular Automaton", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let map_size = 200;
    let mut automaton = Automaton::new(map_size, map_size);
    match init {
        Init::Random => automaton.random_init(),
        Init::Cells(cells) => automaton.set_cells(cells),
    };
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.set_draw_color(Color::BLACK);
        let rects = points_to_rects(
            &automaton.get_points(),
            map_size,
            map_size,
            window_width,
            window_height,
        );
        canvas.fill_rects(&rects).expect("Draw rectangles failed");
        automaton.step();
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(100));
    }
}
