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

fn join_ranks(skin: Skin, ranks: impl IntoIterator<Item = u8>, separator: Element) -> Element {
    let mut ite = ranks.into_iter().map(|rank| {
        rank_text(skin, rank)
    });

    let Some(first) = ite.next() else {return rsx!{}};

    rsx! {
        {first},
        for remaining in ite {
            {separator.clone()},
            {remaining}
        }
    }
}

const NUM_PAGES: usize = 2;

#[component]
fn Paginator(mut page_num: Signal<usize>, num_pages: usize) -> Element {
    let curr_page_num = *page_num.read();
    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            font_size: "4rem",
            margin_bottom: "1rem",

            span {
                font_family: "Noto Sans Symbols 2",
                font_size: "4.5rem",
                visibility: if curr_page_num == 0 {"hidden"},
                onclick: move |_| { *page_num.write() -= 1; },
                "◀"
            }

            span {
                width: "30rem",
                text_align: "center",
                "Page {curr_page_num + 1} of {num_pages}"
            }
            

            span {
                font_family: "Noto Sans Symbols 2",
                font_size: "4.5rem",
                visibility: if curr_page_num + 1 == num_pages {"hidden"},
                onclick: move |_| { *page_num.write() += 1; },
                "▶"
            }
        }
    }
}

#[component]
pub fn Help(mut game_state: Signal<GameState>) -> Element {
    let st = game_state.read();
    let skin = st.skin;

    let rank_text = |rank: u8| rank_text(skin, rank);

    let page_num = use_signal(|| 0);

    let page = match *page_num.read() {
        0 => rsx! {
            div {
                font_size: "3.75rem",
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
                    "). Only the highest-scoring hand in each category will contribute to the total score."
                }

                p {
                    "After a hand is filled, click the hand or the " Emph {"discard"} " pile to discard it. You may also move another
                    card to the hand; the old hand will be discarded automatically."
                }

                p {
                    "To " Emph{"win the game"} ", score " Emph{"240 points"} " or more before the end of the game. 
                    The game ends after 10 hands are filled (as the 2 remaining cards can’t be used)."
                }

                p {
                    Emph{"Shortcut note:"}," Double-clicking on a card will move it to the hand."
                }
            }
        },
        _ => {
            let broadway = [RANK_ACE, RANK_KING, RANK_QUEEN, RANK_JACK, 10];
            rsx! {
                div {
                    font_size: "3.75rem",
                    text_align: "left",

                    p {
                        margin_top: "0",
                        
                        ul {
                            li {
                                "Order of cards within a hand do not matter."
                            }
                            li {
                                "Aces are usually high in poker hands, but may be either low or high for a Straight or Straight Flush."
                            }
                            li {
                                "A hand is always assigned to the highest-ranking category that it qualifies for."
                            }
                        }
                    }

                    p {
                        "The hand categories, from highest-ranked to low, are:"
                        ul {
                            li {
                                Emph {"Royal flush"} ": " {join_ranks(skin, broadway, rsx!{"–"})}
                                " of one suit."
                            }

                            li {
                                Emph {"Straight flush"} ": Five sequential ranks of one suit."
                            }

                            li {
                                Emph {"Four of a kind"} ": Four cards of the same rank."
                            }

                            li {
                                Emph {"Full house"} ": Three cards of a rank, and two of another rank."
                            }

                            li {
                                Emph {"Flush"} ": Five cards of the same suit."
                            }

                            li {
                                Emph {"Straight"} ": Five cards of sequential ranks."
                            }

                            li {
                                Emph {"Three of a kind"} ": Three cards of the same rank."
                            }

                            li {
                                Emph {"Two pairs"} ": Two cards of a rank, and two of another rank."
                            }

                            li {
                                Emph {"Pair"} ": Two cards of the same rank."
                            }

                            li {
                                Emph {"High card"} ": Hands that don’t qualify for any other category. 
                                Scores by the single highest card."
                            }
                        }
                    }
                }
            }
        },
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; color: #fff; padding: 4rem;",
            class: "help",

            {page},

            div {
                position: "absolute",
                bottom: rem(2.),
                width: "92rem",
                display: "flex",
                flex_direction: "column",
                justify_content: "center",

                Paginator {
                    page_num,
                    num_pages: NUM_PAGES,
                }

                div {
                    display: "flex",
                    flex_direction: "row",
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
}