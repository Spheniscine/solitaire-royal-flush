use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{CARD_BORDER_RADIUS_RATIO, CARD_HEIGHT_RATIO, CardComponent, CardFrame, Movement, rem}, game::{AnimationAct, AnimationKey, Board, BoardPos, Card, DepotRole, NUM_DEPOTS, POKER_HAND_SIZE, PokerHandCategory, SCORE_GOAL, Score, ScoreBoard, Skin}};

#[component]
pub fn ScoreBoardComponent(
    position: Vec2,
    width: f32,
    hand_score: Option<(PokerHandCategory, u8)>,
    score_board: ScoreBoard,
    high_score: Score,
) -> Element {
    let hand_score_node = if let Some((category, score)) = hand_score {
        rsx! {
            td {
                "{category.display_string()}"
            }
            td {
                "{score}"
            }
        }
    } else {
        rsx! {
            td {
                " "
            }
        }
    };

    let score_rows = score_board.category_scores.iter().map(|(category, &score)| {
        rsx! {
            tr {
                class: if score == 0 {"muted"},
                td {
                    "{category.display_string()}"
                }
                td {
                    "{score}"
                }
            }
        }
    });

    rsx! {
        table {
            class: "scoreboard",
            position: "absolute",
            left: rem(position.x),
            top: rem(position.y),
            width: rem(width),

            tr {
                {hand_score_node}
            }

            tr {
                td {
                    " "
                }
            }

            {score_rows}

            tr {
                td {
                    " "
                }
            }

            tr {
                class: "highlight",
                td {
                    "Score"
                }
                td {
                    "{score_board.score()}"
                }
            }
            tr {
                class: "highlight",
                td {
                    "Goal"
                }
                td {
                    "{SCORE_GOAL}"
                }
            }
            tr {
                td {
                    "High score"
                }
                td {
                    "{high_score}"
                }
            }
        }
    }
}

#[component]
pub fn BoardComponent(
    position: Vec2,
    board: Board,
    skin: Skin,
    #[props(default)]
    onclick: EventHandler<BoardPos>,
    #[props(default)]
    ondoubleclick: EventHandler<BoardPos>,
    #[props(default)]
    animation_key: AnimationKey,
    #[props(default)]
    is_won: bool,
    #[props(default)]
    is_over: bool,

    high_score: Score,
) -> Element {
    let card_width = 11f32;
    let card_height = card_width * CARD_HEIGHT_RATIO;
    let spacer_x = 1f32;
    let spacer_y = 1f32;

    let margin_x = 2f32;
    let pos_x = |i: usize| {
        margin_x + i as f32 * (card_width + spacer_x)
    };
    let margin_y = 2f32;
    let pos_y = |i: usize| {
        margin_y + i as f32 * (card_height + spacer_y)
    };

    let x_card_offset = Vec2::new(card_width / 2., 0.);
    let y_card_offset = Vec2::new(0., card_height / 2.);

    let discard_x = 100. - margin_x - card_width;
    let hand_pos = Vec2::new(discard_x, pos_y(1)) - x_card_offset * (POKER_HAND_SIZE - 1) as f32;

    let get_pos = |depot: usize, ord: usize| {
        let (role, index) = DepotRole::role_and_subindex(depot).unwrap();
        use DepotRole::*;
        match role {
            Tableau => Vec2::new(pos_x(index), pos_y(0)) + y_card_offset * ord as f32,
            Hand => hand_pos + x_card_offset * ord as f32,
            Discard => Vec2::new(discard_x, pos_y(0)),
        }
    };

    let get_hint = |depot: usize| {
        let role = DepotRole::role(depot).unwrap();
        match role {
            DepotRole::Discard => Some(
                    rsx!{
                        span {
                            font_family: "'Noto Emoji'",
                            "♻"
                        }
                    }
                ),
            DepotRole::Tableau => Some(rsx!{}),
            _ => None,
        }
    };

    let is_face_up = |depot: usize| {
        DepotRole::role(depot).unwrap().is_face_up()
    };

    let selected_height = if let Some(BoardPos { depot_index, card_index }) = board.selected {
        let d = if DepotRole::role(depot_index).unwrap() == DepotRole::Tableau {
            board.depots[depot_index].len() - card_index - 1
        } else {
            0
        };

        card_height + y_card_offset.y * d as f32
    } else {0.};

    let moving_card = |p1: Vec2, p2: Vec2, card: Card| rsx! {
        Movement {
            src_translate_vec: p1 - p2,
            CardComponent {
                position: p2,
                width: card_width,
                card: card,
                skin,
            }
        }
    };

    let anims = board.animation_acts.iter().enumerate().map(|(i, act)| {
        match act {
            AnimationAct::Move (cards, pos1, pos2) => {
                let mut pos1 = *pos1;
                let mut pos2 = *pos2;

                let nodes = cards.iter().map(move |card| {
                    let p1 = get_pos(pos1.depot_index, pos1.card_index);
                    let p2 = get_pos(pos2.depot_index, pos2.card_index);
                    let res = moving_card(p1, p2, *card);
                    pos1.card_index += 1;
                    pos2.card_index += 1;
                    res
                });

                rsx! {
                    Fragment {
                        key: "{animation_key},{i}", // needed to force remounts, so animations don't get "stale" and refuse to replay
                        {nodes}
                    }
                }
            },
        }
    });

    let score_board_pos = Vec2::new(pos_x(5), pos_y(2));
    let score_board_width = 100. - margin_x - score_board_pos.x;
    let score_board = rsx! {
        ScoreBoardComponent { 
            position: score_board_pos,
            width: score_board_width,
            hand_score: board.hand_score(),
            score_board: board.score_board,
            high_score,
        }
    };

    let hand_frames = rsx! {
        for i in 0..POKER_HAND_SIZE {
            CardFrame { 
                position: get_pos(DepotRole::Hand.id(0), i),
                width: card_width,
                onclick: move |_| {
                    onclick.call(BoardPos::new(DepotRole::Hand.id(0), !0))
                },
            }
        }
    };

    rsx! {
        div {
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),

            {hand_frames},

            for depot in 0..NUM_DEPOTS {
                if let Some(hint) = get_hint(depot) {
                    CardFrame { 
                        position: get_pos(depot, 0),
                        width: card_width,
                        hint,
                        onclick: move |_| {
                            onclick.call(BoardPos::new(depot, !0))
                        },
                    }
                }

                for i in 0..board.depots[depot].len() {
                    if board.selected == Some(BoardPos::new(depot, i)) {
                        div {
                            position: "absolute",
                            top: rem(get_pos(depot, i).y),
                            left: rem(get_pos(depot, i).x),
                            width: rem(card_width),
                            height: rem(selected_height),
                            background_color: "#ff0",
                            border_radius: rem(card_width * CARD_BORDER_RADIUS_RATIO),
                            class: "selected-halo",
                        }
                    }

                    CardComponent { 
                        position: get_pos(depot, i),
                        width: card_width,
                        card: if is_face_up(depot) {board.depots[depot][i]},
                        // number_hint: if !is_face_up(depot) {i + 1},
                        skin,
                        onclick: move |_| {
                            onclick.call(BoardPos::new(depot, i))
                        },
                    }
                }
            }

            {score_board},
            {anims},
        }
    }
}