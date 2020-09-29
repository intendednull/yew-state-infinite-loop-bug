use yew::prelude::*;
use yew_state::{component, SharedHandle, StateView};

struct Model;
impl Component for Model {
    type Properties = ();
    type Message = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let counts = std::iter::repeat(()).map(|_| view_count()).take(5);
        html! { for counts }
    }
}

fn view_count() -> Html {
    type Handle = SharedHandle<usize>;
    let view = component::view(|handle: &Handle| {
        handle.reduce(|count| *count += 1);
        html! { <p>{"Count: "}{ handle.state() }</p> }
    });
    html! {
        <StateView<Handle> view=view />
    }
}

fn main() {
    yew::start_app::<Model>();
}
