use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct MyComponent {
    // ComponentLink is like a reference to a component
    // It can be used to send messages to the component
    value: i64,
}

impl Component for MyComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {

            <main>
                <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
                <h1>{ "Hello World!" }</h1>
                <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
                        <button onclick={link.callback(|_| Msg::AddOne)}>{"+1"}</button>
                        <p>{self.value}</p>
            </main>
        }
    }
}

struct Post {}

struct List {}
impl Component for List {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }
}
