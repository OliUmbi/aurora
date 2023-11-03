use yew::prelude::*;
use crate::components::layout::{Direction, Horizontal, Vertical, Alignment, Wrap};

use crate::components::tile::Tile;
use crate::components::layout::sequence::Sequence;
use crate::components::layout::location::Location;
use crate::components::layout::element::Element;

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <Sequence direction={Direction::COLUMN} wrap={Wrap::NORMAL} gap={2}>
                <Location horizontal={Horizontal::CENTER} vertical={Vertical::CENTER}>
                    <Element direction={Direction::ROW} alignment={Alignment::CENTER} gap={1}>
                        <h1>{"Hello World"}</h1>
                        <p>{"Some text"}</p>
                    </Element>
                </Location>
                <Location horizontal={Horizontal::LEFT} vertical={Vertical::BOTTOM}>
                    <Element direction={Direction::COLUMN} alignment={Alignment::START} gap={1}>
                        <h1>{"Test"}</h1>
                    </Element>
                </Location>
            </Sequence>
        </>
    }
}
