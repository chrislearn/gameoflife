use ::*;
use relm::{Relm, RemoteRelm, Widget};
use gtk::prelude::*;
use gtk::{Window, WindowType, DrawingArea, Button, ScrolledWindow};
use tokio_core::reactor::Interval;
use std::time::Duration;

use map::*;

#[derive(Clone)]
pub struct Model {}

#[derive(Clone)]
pub struct CairoSurface(cairo::Surface);
unsafe impl Send for CairoSurface {}

#[derive(SimpleMsg)]
pub enum Msg {
    Tick,
    Quit,
}

#[derive(Clone)]
pub struct Win {
    hbox: gtk::Box,
    button_box: gtk::ButtonBox,
    pause_button: Button,
    next_button: Button,
    clear_button: Button,
    zoom_in_button: Button,
    zoom_out_button: Button,
    area: DrawingArea,
    scroller: ScrolledWindow,
    window: Window,
}

impl Widget for Win {
    type Root = Window;
    type Model = Model;
    type Msg = Msg;

    fn root(&self) -> &Self::Root {
        &self.window
    }

    fn model() -> Self::Model {
        Model {}
    }

    fn subscriptions(relm: &Relm<Msg>) {
        let stream = Interval::new(Duration::from_millis(50), relm.handle()).unwrap();
        relm.connect_exec_ignore_err(stream, Msg::Tick);
    }

    fn update(&mut self, event: Msg, _model: &mut Self::Model) {
        match event {
            Msg::Tick => {
                use gdk::prelude::ContextExt;
                let cr = cairo::Context::create_from_window(&self.area.get_window().unwrap());
                cr.set_source_rgb(0., 0., 0.);
                cr.paint();
            },
            Msg::Quit => gtk::main_quit(),
            _ => {},
        }
    }

    fn view(relm: RemoteRelm<Msg>, _model: &Self::Model) -> Self {
        let window = Window::new(WindowType::Toplevel);
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let button_box = gtk::ButtonBox::new(gtk::Orientation::Vertical);
        let pause_button = Button::new_with_label("Start");
        let random_button = Button::new_with_label("Randomize");
        let next_button = Button::new_with_label("Next");
        let clear_button = Button::new_with_label("Clear");
        let zoom_in_button = Button::new_with_label("Zoom in");
        let zoom_out_button = Button::new_with_label("Zoom out");
        let area = DrawingArea::new();
        let scroller = ScrolledWindow::new(None, None);
        scroller.set_size_request(600, 600);
        // disable auto-hide scrollbar
        scroller.set_overlay_scrolling(false);
        button_box.set_layout(gtk::ButtonBoxStyle::Start);

        button_box.pack_start(&pause_button, false, false, 0);
        button_box.pack_start(&next_button, false, false, 0);
        button_box.pack_start(&random_button, false, false, 0);
        button_box.pack_start(&clear_button, false, false, 0);
        button_box.pack_start(&zoom_in_button, false, false, 0);
        button_box.pack_start(&zoom_out_button, false, false, 0);
        scroller.add(&area);
        hbox.pack_start(&scroller, false, false, 0);
        hbox.pack_start(&button_box, false, false, 0);
        window.add(&hbox);
        window.set_title("Game of Life");
        window.show_all();

        connect!(relm, window, connect_delete_event(_, _) (Some(Msg::Quit), Inhibit(false)));

        //connect!(relm, area, connect_draw(_, cr) (Some(Msg::Draw(CairoContext(cr.clone()))), Inhibit(true)));
        /*
        {
            let stream = relm.stream().clone();
            area.connect_draw(move |this, cr| {
                let (msg, return_value) = {
                    let cr = CairoContext(cr.clone());
                    (Some(Msg::Draw(cr)), Inhibit(true))
                };
                let msg: Option<_> = msg.into();
                if let Some(msg) = msg {
                    println!("Before Emit");
                    stream.emit(msg);
                    println!("After Emit");
                }
                return_value
            });
        }
        */
        /*
        area.connect_draw(move |_, cr| {
            cr.set_source_surface();
            cr.set_source_rgb(0.5, 0.5, 0.5);
            cr.paint();
            Inhibit(true)
        });
        */
        Win {
            hbox: hbox,
            button_box: button_box,
            pause_button: pause_button,
            next_button: next_button,
            clear_button: clear_button,
            zoom_in_button: zoom_in_button,
            zoom_out_button: zoom_out_button,
            area: area,
            scroller: scroller,
            window: window,
        }
    }
}
