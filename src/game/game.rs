use std::time::Duration;

use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{components::LocalStorage, game::{Board, BoardPos, Card, DECK_SIZE, DepotRole, POKER_HAND_SIZE, PokerHandCategory, RANKS, Score, SettingsState, Skin, Suit}};

pub const ANIMATION_DURATION: Duration = Duration::from_millis(200);
pub type AnimationKey = u16;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ActionRecord {
    Move(BoardPos, BoardPos),
    Score(PokerHandCategory, u8),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ScreenState {
    #[default] Game, 
    Settings, Help,
}

pub const SCORE_GOAL: Score = 240;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub deal: Vec<Card>,
    #[serde(skip)]
    pub animation_key: AnimationKey, // used for syncing and to provide animator components with cycling keys
    pub history: Vec<ActionRecord>,
    pub undo_stack: Vec<usize>,
    pub already_won: bool,
    pub num_wins: i32,
    pub high_score: Score,

    pub screen_state: ScreenState,

    pub allow_undo: bool,
    pub skin: Skin,
}

impl GameState {
    pub fn new_deal(rng: &mut impl Rng) -> Vec<Card> {
        let mut deck = Vec::with_capacity(DECK_SIZE);
        for rank in RANKS {
            for suit in Suit::iter() {
                deck.push(Card { rank, suit });
            }
        }

        deck.shuffle(rng);
        deck
    }

    pub fn init() -> Self {
        let mut res = Self {
            board: Board::empty(),
            deal: vec![],
            animation_key: 0,
            history: vec![],
            undo_stack: vec![],
            already_won: false,
            num_wins: 0,
            high_score: 0,
            screen_state: ScreenState::Game,
            allow_undo: true,
            skin: Skin::default(),
        };

        res.new_game();
        res
    }

    pub fn new_game(&mut self) {
        let deal = Self::new_deal(&mut rand::rng());
        self.board = Board::from_deal(&deal);
        self.deal = deal;
        self.history.clear();
        self.undo_stack.clear();
        self.already_won = false;

        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn can_stack(&self, back: Card, front: Card) -> bool {
        front.rank + 1 == back.rank
    }

    pub fn can_select(&self, pos: BoardPos) -> bool {
        let depot = pos.depot_index;
        let ord = pos.card_index;

        if ord >= self.board.depots[depot].len() {
            return false;
        }
        let slice = &self.board.depots[depot][ord..];

        let Some(role) = DepotRole::role(depot) else { return false };
        match role {
            DepotRole::Tableau => slice.windows(2).all(|w| self.can_stack(w[0], w[1])),
            _ => false,
        }
    }

    pub fn is_won(&self) -> bool {
        self.board.score_board.score() >= SCORE_GOAL
    }

    pub fn is_over(&self) -> bool {
        const USED_CARD_LIMIT: usize = DECK_SIZE / POKER_HAND_SIZE * POKER_HAND_SIZE;
        self.board.depots[DepotRole::Discard.id(0)].len() + self.board.depots[DepotRole::Hand.id(0)].len()
            >= USED_CARD_LIMIT
    }

    pub fn is_busy(&self) -> bool {
        self.is_acting()
    }

    pub fn is_acting(&self) -> bool {
        !self.board.animation_acts.is_empty()
    }

    pub fn undo_possible(&self) -> bool {
        self.allow_undo && !self.undo_stack.is_empty()
    }

    fn do_move_raw(&mut self, pos1: BoardPos, pos2: BoardPos) {
        self.board.do_move(pos1, pos2);
        self.history.push(ActionRecord::Move(pos1, pos2));
    }

    fn move_intent(&mut self, pos1: BoardPos, pos2: BoardPos) -> bool {
        if pos1.depot_index == pos2.depot_index { return false; }
        let depot1 = &self.board.depots[pos1.depot_index];
        let depot2 = &self.board.depots[pos2.depot_index];
        let num_moved = depot1.len() - pos1.card_index;
        // if pos2.card_index != depot2.len() { return false; }

        let card = depot1[pos1.card_index];
        let Some(role) = DepotRole::role(pos2.depot_index) else { return false };
        let history_len = self.history.len();

        match role {
            DepotRole::Tableau => {
                if pos2.card_index != depot2.len() { return false; }
                let ok = depot2.last().is_none_or(|&c| self.can_stack(c, card));
                if !ok { return false; }
                self.do_move_raw(pos1, pos2);
            },
            DepotRole::Hand => {
                if num_moved != 1 { return false; }
                if depot2.len() == POKER_HAND_SIZE {
                    self.do_move_raw(BoardPos::new(pos2.depot_index, 0), 
                        self.board.top_pos(DepotRole::Discard.id(0)));
                }
                self.do_move_raw(pos1, self.board.top_pos(pos2.depot_index));
            },
            DepotRole::Discard => return false,
        }

        self.undo_stack.push(history_len);
        true
    }

    pub fn onclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }
        if self.is_over() { return; }

        if let Some(src) = self.board.selected {
            if pos == src { 
                self.board.selected = None; 
                return;
            }
            if src.depot_index == pos.depot_index && self.can_select(pos) {
                self.board.selected = Some(pos);
                return;
            }

            let dest = BoardPos { depot_index: pos.depot_index, card_index: pos.card_index.wrapping_add(1) };
            self.move_intent(src, dest);
        } else {
            if self.can_select(pos) {
                self.board.selected = Some(pos);
            } else {
                let role = DepotRole::role(pos.depot_index).unwrap();
                match role {
                    DepotRole::Tableau => {},
                    DepotRole::Hand | DepotRole::Discard => {
                        let hand = DepotRole::Hand.id(0);
                        if self.board.depots[hand].len() == POKER_HAND_SIZE {
                            let history_len = self.history.len();
                            self.do_move_raw(BoardPos::new(hand, 0), 
                                self.board.top_pos(DepotRole::Discard.id(0)));
                            self.undo_stack.push(history_len);
                        }
                    },
                }
            }
        }
    }

    pub fn ondoubleclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }
        if self.is_over() { return; }
        if !self.can_select(pos) { return; } // needed, or illegal stacks can still be moved this way!

        // double click moves cards to the hand if possible
        self.move_intent(pos, self.board.top_pos(DepotRole::Hand.id(0)));
    }

    pub fn advance_animations(&mut self, key: AnimationKey) {
        if key != self.animation_key { return; }
        self.animation_key = self.animation_key.wrapping_add(1);
        
        self.board.advance_actions();

        if let Some((category, score)) = self.board.hand_score() {
            let entry = &mut self.board.score_board.category_scores[category];
            let old_score = *entry;
            if score > old_score {
                self.history.push(ActionRecord::Score(category, old_score));
                *entry = score;
                self.high_score = self.high_score.max(self.board.score_board.score());
            }
        }

        if self.is_won() {
            if !self.already_won {
                self.num_wins += 1;
                self.already_won = true;
            }
        } else {
            // self.check_auto_moves();
        }

        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn undo(&mut self) {
        if self.is_busy() || !self.undo_possible() { return; }
        let Some(target_len) = self.undo_stack.pop() else {return};
        while self.history.len() > target_len {
            let rec = self.history.pop().unwrap();
            match rec {
                ActionRecord::Move(pos1, pos2) => {
                    self.board.do_move(pos2, pos1)
                },
                ActionRecord::Score(category, score) => {
                    self.board.score_board.category_scores[category] = score;
                },
            }
            self.board.advance_actions(); // no animation, as repeated card moves on same card causes problems
        }
        LocalStorage.save_game_state(&self);
    }

    pub fn restart(&mut self) {
        if self.history.is_empty() || !self.undo_possible() { return; }
        self.board = Board::from_deal(&self.deal);
        self.history.clear();
        self.undo_stack.clear();
        LocalStorage.save_game_state(&self);
    }

    pub fn new_settings_state(&self) -> SettingsState {
        SettingsState {
            allow_undo: self.allow_undo,
            skin: self.skin,
        }
    }

    pub fn apply_settings(&mut self, settings: &SettingsState) {
        self.allow_undo = settings.allow_undo;
        self.skin = settings.skin;
        LocalStorage.save_game_state(&self);
    }
}