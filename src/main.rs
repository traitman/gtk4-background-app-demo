use gtk::{glib, prelude::*};
use glib::{clone, Propagation};

use env_logger::{Builder as LogBuilder, Target as LogTarget};
use log::LevelFilter;

fn main() -> glib::ExitCode {

    LogBuilder::new()
        .filter_level(LevelFilter::Debug) // set default level
        .parse_default_env() // then, if exists, respect the env config
        .target(LogTarget::Stdout)
        .init();

    let application = gtk::Application::builder()
        .application_id("com.github.gtkrs.BackgroundDemo")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    log::info!("present window");

    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("Background GTK Program demo"));
    window.set_default_size(350, 70);

    let button = gtk::Button::with_label("Click me to hide the window in the background!");

    button.connect_clicked(clone!(@weak window => move |_| {
        log::info!("button clicked");
        window.set_visible(false);
    }));

    window.set_child(Some(&button));


    window.connect_close_request(|win| {
        // win.minimize();
        win.set_visible(false);
        // Propagation::Proceed default handlers (widget will destroy),
        // set to true here means prevent default
        Propagation::Stop
        // gtk::main_quit();
        // Propagation::Proceed
    });

    let ctx = glib::MainContext::default();
    ctx.spawn_local(clone!(@weak application as application => async move {
          portal_request_background(&application).await
    }));

    window.present();
}

use ashpd::{desktop::background::Background, WindowIdentifier};

async fn portal_request_background(app: &gtk::Application) {
    if let Some(window) = app.active_window() {
        log::info!("portal_request_background begin");
        let root = window.native().unwrap();
        let identifier = WindowIdentifier::from_native(&root).await;
        let request = Background::request().identifier(identifier).reason("gtk4-demo needs to run in the background");

        match request.send().await.and_then(|r| r.response()) {
            Ok(response) => {
                log::info!("Background request successful: {:?}", response);
            }
            Err(err) => {
                log::warn!("Background request denied: {}", err);
            }
        }
    } else {
        log::warn!("portal_request_background failed: no active window");
    }
}