use dioxus::prelude::*;

use crate::{components::{SkinTrait, rem}, game::{Card, GameState, RANK_ACE, RANK_JACK, RANK_KING, RANK_QUEEN, ScreenState, Skin, Suit}};

#[component]
fn Emph(children: Element) -> Element {
    rsx! {
        strong {
            color: "#ff0",
            {children}
        }
    }
}

fn rank_text(skin: Skin, rank: u8) -> Element {
    rsx! {
        span {
            font_size: "1.1em",
            {skin.render_rank(&Card { rank, suit: Suit::Spades })}
        }
    }
}

// fn join_ranks(skin: Skin, ranks: impl IntoIterator<Item = u8>, separator: Element) -> Element {
//     let mut ite = ranks.into_iter().map(|rank| {
//         rank_text(skin, rank)
//     });

//     let Some(first) = ite.next() else {return rsx!{}};

//     rsx! {
//         {first},
//         for remaining in ite {
//             {separator.clone()},
//             {remaining}
//         }
//     }
// }

#[component]
pub fn Help(mut game_state: Signal<GameState>) -> Element {
    let st = game_state.read();
    let skin = st.skin;

    let rank_text = |rank: u8| rank_text(skin, rank);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; font-size: 4rem; color: #fff; padding: 4rem;",
            class: "help",

            div {
                text_align: "left",

                p {
                    margin_top: "0",
                    "The deck is a standard 52-card deck. There are 4 suits, each with ranks: ",{rank_text(2)},
                    "~",{rank_text(10)},", Jack, Queen, King; and also an Ace, which may be either high or low depending on context."
                }

                p {
                    "Cards stack by " Emph {"decrementing ranks"} " in the " Emph {"tableau"} ". (Aces are always low in the tableau.) 
                    Such stacks of any size can be moved as a unit. "
                }

                p {
                    "Single cards may be moved to the " Emph {"hand"} ". (Cards can’t be removed from the hand.)
                    Once all 5 slots of the hand are filled, the cards are evaluated as a " Emph {"poker hand"} ". 
                    It is assigned a poker hand " Emph {"category"} ", and points are awarded by the sum of the values of the cards 
                    that contributed to that category ("
                    {rank_text(RANK_JACK)} "=11, "
                    {rank_text(RANK_QUEEN)} "=12, "
                    {rank_text(RANK_KING)} "=13, "
                    {rank_text(RANK_ACE)} "=14 if high and 1 if low"
                    "). Only one hand in each category will contribute to the final score; if you 
                    score several hands of the same category, only the highest-scoring hand will contribute."
                }

                p {
                    "To " Emph{"win the game"} ", score " Emph{"240 points"} " or more before the end of the game. 
                    The game ends after 10 hands have been evaluated (as the 2 remaining cards can’t be used)."
                }

                p {
                    Emph{"Shortcut note:"}," Double-clicking on a card will move it to the hand."
                }
            }

            div {
                position: "absolute",
                bottom: rem(2.),
                width: "92rem",
                display: "flex",
                justify_content: "center",

                a {
                    // href: VIDEO_GAMEPLAY,
                    // target: "_blank",
                    text_decoration: "none",
                    margin_right: rem(4.),
                    div {
                        width: rem(30.),
                        position: "relative",
                        // class: "game-button",
                        class: "game-button-disabled",
                        "Example video"
                    }
                }

                div {
                    width: rem(30.),
                    position: "relative",
                    class: "game-button",
                    onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                    "Back to game"
                }
            }
        }
    }
}