use yew::prelude::*;

use crate::enums::*;
use crate::components::base::*;
use crate::components::layout::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <Sequence direction={Direction::COLUMN} wrap={Wrap::NORMAL} gap={2}>
                <Location horizontal={Horizontal::LEFT} vertical={Vertical::TOP}>
                    <Element direction={Direction::ROW} alignment={Alignment::CENTER} gap={1}>
                        <Typography size={3} weight={Weight::W700} ellipsis={false}>{"Hello World"}</Typography>
                        <Typography size={1} weight={Weight::W400} ellipsis={false}>{"Some text"}</Typography>
                    </Element>
                </Location>
                <Location horizontal={Horizontal::LEFT} vertical={Vertical::BOTTOM}>
                    <Element direction={Direction::COLUMN} alignment={Alignment::START} gap={1}>
                        <Tile width={10} background={true} padding={2}>
                            <Typography size={1} weight={Weight::W400} ellipsis={false}>{"Test"}</Typography>
                        </Tile>
                    </Element>
                </Location>
            </Sequence>
        </>
    }
}
