use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::rem, game::ColorMode};

pub trait SkinTrait<C>: PartialEq + Clone {
    fn get_color(&self, card: &C, mode: ColorMode) -> String;
    fn render_rank(&self, card: &C) -> Element;
    fn render_suit(&self, card: &C) -> Element;
    fn render_suit_text(&self, card: &C) -> Element;
}

pub const BASE_CARD_WIDTH: f32 = 12.;
pub const BASE_CARD_HEIGHT: f32 = 13.;
pub const CARD_HEIGHT_RATIO: f32 = BASE_CARD_HEIGHT / BASE_CARD_WIDTH;
pub const CARD_BORDER_RADIUS_RATIO: f32 = 1. / BASE_CARD_WIDTH;
pub const CARD_FONT_RATIO: f32 = 5. / 12.;

#[component]
pub fn CardComponent<C: PartialEq + Clone + 'static, S: SkinTrait<C> + 'static>(
    position: Vec2,
    width: f32,
    card: C,
    skin: S,

    // number_hint: Option<usize>,
    #[props(default = ColorMode::Light)]
    color_mode: ColorMode,

    #[props(default)]
    onclick: EventHandler<MouseEvent>,
    #[props(default)]
    ondoubleclick: EventHandler<MouseEvent>,
) -> Element {
    let pt = width / BASE_CARD_WIDTH;
    let pt = |x: f32| {
        rem(x * pt)
    };

    rsx! {
        div {
            style: "place-items: center;",
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),
            background_color: color_mode.choose("#fff", "#101"),
            width: pt(BASE_CARD_WIDTH - 1.),
            height: pt(BASE_CARD_HEIGHT - 1.),
            border: "{pt(0.25)} solid",
            border_color: color_mode.choose("#000", "#868"),
            border_radius: rem(width * CARD_BORDER_RADIUS_RATIO),
            display: "grid",
            grid_template_columns: "50% 50%",
            grid_template_rows: "50% 50%",
            font_size: rem(width * CARD_FONT_RATIO),
            text_align: "center",
            padding: pt(0.25),
            color: skin.get_color(&card, !color_mode),

            onclick, ondoubleclick,

            div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_rank(&card)}},
            div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_suit(&card)}},
            div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_suit(&card)}},
            div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_rank(&card)}},
        }
    }
}

pub const CARD_FRAME_DEFAULT_COLOR: &str = "#aaa";

#[component]
pub fn CardFrame(
    position: Vec2,
    width: f32,
    hint: Option<Element>,
    #[props(default = CARD_FRAME_DEFAULT_COLOR.to_string())] 
    color: String,
    onclick: EventHandler<MouseEvent>,
    // oncontextmenu: EventHandler<MouseEvent>,

    number_hint: Option<i32>,
) -> Element {
    let color = color.as_str();
    let pt = width / BASE_CARD_WIDTH;
    let pt = |x: f32| {
        rem(x * pt)
    };
    rsx! {
        div {
            display: "flex",
            align_items: "center",
            justify_content: "center",
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),
            margin: pt(0.25), // frame must be slightly smaller than card to prevent peeking out in some platforms
            width: pt(BASE_CARD_WIDTH - 2.),
            height: pt(BASE_CARD_HEIGHT - 2.),
            border: "{pt(0.5)} solid {color}",
            text_align: "center",
            color,
            border_radius: rem(width * CARD_BORDER_RADIUS_RATIO),
            font_size: rem(width * CARD_FONT_RATIO),
            padding: pt(0.25),
            onclick, 
            //oncontextmenu,

            if let Some(hint) = hint {
                div {
                    {hint},
                }
            }
        }
    }
}

#[component]
pub fn CardText<C: PartialEq + Clone + 'static, S: SkinTrait<C> + 'static>(card: C, skin: S, color_mode: ColorMode) -> Element {
    rsx! {
        span {
            font_size: "1.2em",
            white_space: "nowrap",
            line_height: 1.2,
            color: skin.get_color(&card, color_mode),
            {skin.render_rank(&card)},
            span {display: "inline-block", min_width: "0.1em"},
            {skin.render_suit_text(&card)},
        }
    }
}