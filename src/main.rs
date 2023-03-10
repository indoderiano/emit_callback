mod app;
mod child;
mod other;

fn main() {
    yew::start_app::<app::App>();
    // yew::start_app::<other::ParentComponent>();
}