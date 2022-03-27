use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::yewdux::YewduxStore;

pub struct Display;

impl Component for Display {
    type Message = ();

    type Properties = DispatchProps<BasicStore<YewduxStore>>;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <div>
            <h1>{"Count Display"}</h1>
            <p>{format!("The button was pressed {} times", ctx.props().state().count)}</p>
          </div>
        }
    }
}
