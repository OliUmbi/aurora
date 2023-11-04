use yew::prelude::*;
use crate::components::layout::{Direction, Horizontal, Vertical, Alignment, Wrap};

use crate::components::base::tile::Tile;
use crate::components::base::typography::Typography;
use crate::components::base::Weight;
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
                        <Typography size={3} weight={Weight::W700} ellipsis={false}>{"Hello World"}</Typography>
                        <Typography size={1} weight={Weight::W400} ellipsis={false}>{"Some text"}</Typography>
                    </Element>
                </Location>
                <Location horizontal={Horizontal::LEFT} vertical={Vertical::BOTTOM}>
                    <Element direction={Direction::COLUMN} alignment={Alignment::START} gap={1}>
                        <Typography size={1} weight={Weight::W400} ellipsis={false}>{"Test"}</Typography>
                    </Element>
                </Location>
            </Sequence>
        </>
    }
}
