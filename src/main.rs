use fltk::{
    prelude::*,
    app,
    enums,
    image::PngImage,
    input::Input,
    frame::Frame,
    window::Window,
};

mod ui;


fn main() {
    static mut PIC_WINDOWS: Vec<ui::PicWindow> = Vec::new();
    let app = app::App::default();
    let mut ui = ui::UserInterface::make_window();
    let input = ui.input01.clone();
    let mut frame = ui.outputbox.clone();
    ui.btn01.set_callback(move |_| unsafe {
        frame.set_label(&format!("hello {}", input.value()));
        if let Ok(img) = PngImage::load("test.png") {
            let (w, h) = (img.w(), img.h());
            let pic_win = ui::PicWindow::make_window();
            let mut win = pic_win.win.clone();
            let mut pic = pic_win.pic.clone();
            println!("{:?}", (img.w(), img.h()));
            let mouse_pos = app::get_mouse();
            win.set_pos(mouse_pos.0, mouse_pos.1);
            win.set_size(w, h);
            pic.set_size(w, h);
            pic.set_image(Some(img));
            // drag & drop handler:
            let mut x = 0;
            let mut y = 0;
            win.handle(move |w, ev| unsafe {
                match ev {
                    enums::Event::Push => {
                        let coords = app::event_coords();
                        x = coords.0;
                        y = coords.1;
                        true
                    }
                    enums::Event::Drag => {
                        // println!("push:{:?}", (x, y));
                        w.set_pos(app::event_x_root() - x, app::event_y_root() - y);
                        true
                    }
                    enums::Event::KeyDown => {
                        match app::event_key() {
                            enums::Key::Escape => {
                                w.hide();
                                if let Some(index) = PIC_WINDOWS.iter().position(|x|{x.win == *w}) {
                                    PIC_WINDOWS.remove(index);
                                }
                            }
                            _ => {}
                        }
                        true
                    }
                    _ => true
                }
            });
            PIC_WINDOWS.push(pic_win);
        }
    });

    if let Ok(mut img) = PngImage::load("img01.png") {
        // img.scale(400, 200, true, true);
        ui.pic01.set_image(Some(img));
    }
    // close window callback
    ui.main_window.set_callback(move |_| unsafe {
        for pic_win in &mut PIC_WINDOWS {
            pic_win.win.hide();
        }
        app.quit();
    });
    ui.main_window.handle(move |w, ev| unsafe {
        match ev {
            enums::Event::Close => {
                for pic_win in &mut PIC_WINDOWS {
                    pic_win.win.hide();
                }
                app.quit();
                true
            }
            enums::Event::Push => {
                match app::event_mouse_button() {
                    app::MouseButton::Left => {
                        println!("app::MouseButton::Left");
                    }
                    app::MouseButton::Right => {
                        println!("app::MouseButton::Right");
                    }
                    app::MouseButton::Middle => {
                        println!("app::MouseButton::Middle");
                    }
                    _ => {}
                }
                true
            }
            _ => {
                // println!("{}", ev.bits());
                true
            }
        }
    });


    app.run().unwrap();
}
