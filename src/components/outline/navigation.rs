use yew::prelude::*;
use crate::components::base::*;
use crate::enums::Weight;

#[function_component]
pub fn Navigation() -> Html {

    html! {
        <div>
            <Typography size={1} weight={Weight::W400} ellipsis={true}>{"Dashboard"}</Typography>
            <Typography size={1} weight={Weight::W400} ellipsis={true}>{"Todos"}</Typography>
            <Typography size={1} weight={Weight::W400} ellipsis={true}>{"Habits"}</Typography>
            <Typography size={1} weight={Weight::W400} ellipsis={true}>{"Notes"}</Typography>
            <Typography size={1} weight={Weight::W400} ellipsis={true}>{"Utils"}</Typography>
        </div>
    }
}
