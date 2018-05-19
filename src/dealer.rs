// Copyright © 2018 CS410P Robert Horrace
/*
  This Rust file contains the dealer
  and its functions. The dealer will
  have a deck that it can shuffle,
  draw, and check the amount of cards
  in the deck.
*/

use card::*;
use deck::*;

/* Dealer handles deck of cards and distributing cards */
#[derive(Clone,Copy)]
pub struct Dealer {
  deck: Deck,
}

impl Dealer {
  /* Initialize dealer with a full deck of cards */
  pub fn new() -> Dealer {
    let mut new_deck = Deck::new();
    new_deck.build();
    Dealer{deck: new_deck}
  }

  /* Reset deck */
  pub fn reset(&mut self) {
    self.deck.reset();
  }

  /* Dealer shuffles deck */
  pub fn shuffle(& mut self) {
    self.deck.shuffle();
  }

  /* Check if size of deck is enough for a play */
  pub fn check_deck(self) -> bool {
    self.deck.size() >= 7
  }

  /* "Remove" two cards from top of deck and give to player */
  pub fn deal(&mut self) -> [Card; 2] {
    let mut hand = [Card::new(); 2];
    hand[0] = self.deck.rem();
    hand[1] = self.deck.rem();
    hand
  }

  /* Return cards for the flop */
  pub fn flop(&mut self) -> [Card; 3] {
    let mut flp = [Card::new(); 3];
    for i in 0..3 {
      flp[i] = self.deck.rem();
    }
    flp
  }

  /* Return the turn */
  pub fn turn(&mut self) -> Card {
    self.deck.rem()
  }

  /* Return the river (same as turn, 
     but easier to differentiate by name */
  pub fn river(&mut self) -> Card {
    self.deck.rem()
  }
}

