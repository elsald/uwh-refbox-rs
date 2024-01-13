use super::{style::Element, *};
use iced::{
    widget::{column, row, vertical_space},
    Length,
};

use uwh_common::game_snapshot::Color as GameColor;

pub(super) fn make_foul_add_page<'a>(
    color: Option<GameColor>,
    foul: FoulKind,
    expanded: bool,
) -> Element<'a, Message> {
    let (black_style, white_style) = match color {
        Some(GameColor::Black) => (ButtonStyle::BlackSelected, ButtonStyle::White),
        Some(GameColor::White) => (ButtonStyle::Black, ButtonStyle::WhiteSelected),
        None => (ButtonStyle::Black, ButtonStyle::White),
    };

    let mut exit_row = row![make_button("CANCEL")
        .style(ButtonStyle::Red)
        .width(Length::Fill)
        .on_press(Message::AddScoreComplete { canceled: true }),]
    .spacing(SPACING);

    exit_row = exit_row.push(
        make_button("DONE")
            .style(ButtonStyle::Green)
            .width(Length::Fill)
            .on_press(Message::NoAction),
    );
    column![
        row![
            make_button("BLACK")
                .style(black_style)
                .on_press(Message::ChangeColor(GameColor::Black)),
            button(centered_text("=").size(LARGE_TEXT))
                .padding(PADDING)
                .height(Length::Fixed(MIN_BUTTON_SIZE))
                .width(Length::Fill)
                .style(ButtonStyle::Blue)
                .on_press(Message::NoAction),
            make_button("WHITE")
                .style(white_style)
                .on_press(Message::ChangeColor(GameColor::White)),
        ]
        .spacing(SPACING),
        vertical_space(Length::Fixed(PADDING)),
        row![make_penalty_dropdown(foul, expanded)].spacing(SPACING),
        vertical_space(Length::Fill),
        exit_row,
    ]
    .into()
}
