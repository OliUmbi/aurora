use yew::prelude::*;

use crate::components::tile::Tile;

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <h1>{"Home"}</h1>
            <Tile>
                <p>{"löjdasf"}</p>
            </Tile>
        </>
    }
}
