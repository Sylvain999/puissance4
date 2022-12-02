mod view;
pub mod model;

use view::main_component::MainComponent;

fn main() {
    yew::Renderer::<MainComponent>::new().render();
}
