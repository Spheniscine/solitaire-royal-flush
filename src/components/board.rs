use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::rem, game::{PokerHandCategory, SCORE_GOAL, Score, ScoreBoard}};

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