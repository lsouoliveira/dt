mod settings;
mod commands;
mod cli;
mod application;

use application::Application;

fn main() {
    let application = Application::init();

    application.run();
}
