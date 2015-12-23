use phoronix::article::Article;
use phoronix::homepage;
use gtk;
use gtk::traits::*;
use gdk::ffi::GdkRGBA;
use pango;

macro_rules! color {
    (white) => (GdkRGBA{red: 1f64, green: 1f64, blue: 1f64, alpha: 1f64});
    (black) => (GdkRGBA{red: 0f64, green: 0f64, blue: 0f64, alpha: 1f64});
    (green) => (GdkRGBA{red: 0.2f64, green: 0.2f64, blue: 0.5f64, alpha: 1f64});
}

pub fn launch() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
    let scrolled_window = gtk::ScrolledWindow::new(None, None).unwrap();
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    let articles = Article::get_articles(&homepage::offline());

    generate_article_widgets(&container, &articles);
    configure_window(&window);

    container.override_background_color(gtk::StateFlags::empty(), &color!(green));

    scrolled_window.set_min_content_width(800);
    scrolled_window.add(&container);

    window.add(&scrolled_window);
    window.show_all();

    gtk::main();
}

fn configure_window(window: &gtk::Window) {
    let (width, height) = (800, 600);

    window.set_title("Phoronix Reader");
    window.set_default_size(width, height);

    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        gtk::signal::Inhibit(true)
    });

    window.connect_key_press_event(move |_, key| {
        match key.keyval as i32 {
            65307 => gtk::main_quit(),
            _ => ()
        }
        gtk::signal::Inhibit(false)
    });
}

fn generate_article_widgets(container: &gtk::Box, articles: &Vec<Article>) {
    for article in articles {
        let url = format!("https://phoronix.com/{}", article.link);
        let title_and_url = gtk::LinkButton::new_with_label(&url, &article.title).unwrap();
        let details = gtk::TextView::new().unwrap();
        let summary = gtk::TextView::new().unwrap();
        let mut bold = pango::FontDescription::new();

        bold.set_weight(pango::Weight::Heavy);

        title_and_url.set_halign(gtk::Align::Start);
        title_and_url.set_margin_start(0);
        title_and_url.override_background_color(gtk::StateFlags::empty(), &color!(green));
        title_and_url.override_color(gtk::StateFlags::empty(), &color!(white));
        title_and_url.override_font(&bold);

        details.set_halign(gtk::Align::Start);
        details.set_left_margin(10);
        details.set_right_margin(10);
        details.set_editable(false);
        details.get_buffer().unwrap().set_text(&article.details);
        details.override_background_color(gtk::StateFlags::empty(), &color!(green));
        details.override_color(gtk::StateFlags::empty(), &color!(white));
        details.override_font(&bold);

        summary.set_wrap_mode(gtk::WrapMode::Word);
        summary.set_left_margin(10);
        summary.set_right_margin(10);
        summary.set_pixels_above_lines(10);
        summary.set_pixels_below_lines(10);
        summary.set_editable(false);
        summary.get_buffer().unwrap().set_text(&article.summary);
        summary.override_background_color(gtk::StateFlags::empty(), &color!(white));
        summary.override_color(gtk::StateFlags::empty(), &color!(black));

        container.add(&title_and_url);
        container.add(&details);
        container.add(&summary);
        container.add(&gtk::Separator::new(gtk::Orientation::Horizontal).unwrap());
    }
}
