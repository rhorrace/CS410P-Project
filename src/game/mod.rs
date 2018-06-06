// Copyright © 2018 CS410P Robert Horrace
/*
  This is the game manager
  that deals with the player(s),
  the dealer, the table, and the 
  card calc. It also holds the read
  from user function, reading 
*/

pub mod card;
pub mod deck;
pub mod dealer;
pub mod table;
pub mod player;
pub mod calc;
pub mod com;

use std::io::*;
use self::{card::*,dealer::*,player::*,table::*,calc::*,com::*};

/* Read input from the user */
pub fn read_user() -> i64 {
  let mut buffer = String::new();
  let x: i64;
  loop {
    buffer.clear();
    let _ = stdout().flush();
    stdin().read_line(&mut buffer)
           .expect("failed");
    if buffer.trim()
             .is_empty() {
      x = 0;
      break;
    }
    else if buffer.trim()
                  .parse::<i64>()
                  .is_ok() {
      x = buffer.trim()
                .parse::<i64>()
                .unwrap();
      if x != 1 {
        println!("Error: Not a valid number");
        println!("ENTER: Continue, 1: Quit");
      }
      break;
    }
    else {
      println!("Error: Not a valid number");
      println!("ENTER: Continue, 1: Quit");
    }
  }
  x
}

/* Game that holds the player(s), dealer, table, and calc*/
#[derive(Clone,Copy)]
pub struct Game {
  player: Player,
  computer: Com,
  dealer: Dealer,
  table: Table,
  calc: Calc,
}

impl Game {
  pub fn new() -> Game {
    Game { player: Player::new(),
           computer: Com::new(),
           dealer: Dealer::new(),
           table: Table::new(),
           calc: Calc::new(), }
  }
 
  /* Check if dealer has enough cards */
  pub fn check(self) -> bool {
    self.dealer.check_deck()
  }
 
  /* Shuffle dealer's deck */
  pub fn set(&mut self) {
    self.dealer.shuffle();
  }

  /* Reset dealer's with a full deck */
  pub fn reset(&mut self) {
    self.dealer.reset();
    self.dealer.shuffle();
  }
  
  /*Clear table, calc, computer and player*/
  pub fn clear(&mut self) {
    self.player.clear();
    self.computer.clear();
    self.calc.clear();
    self.table.clear();
  }

  pub fn deal(&mut self) {
    let hands = self.dealer.deal();
    self.player.rcv_hand(hands[0]);
    self.calc.add_hand(hands[0]);
    self.computer.rcv_hand(hands[1]);
  }

  /* Flop stage, put flop on table, update player's hand value */
  pub fn flop(&mut self) { 
    let flop = self.dealer.flop();
    self.table.add_flop(flop);
    self.calc.add_cards(flop);
    self.player.update(self.calc.calc_hand());
    self.computer.add_cards(flop);
  }

  /* Turn stage, put turn on table, update player's hand value */
  pub fn turn(&mut self) { 
    let turn = self.dealer.turn();
    self.table.add_turn(turn);
    self.calc.add(turn);
    self.player.update(self.calc.calc_hand());
    self.computer.add(turn);
  }

  /* River stage, put river on table, update player's hand value */
  pub fn river(&mut self) {
    let river = self.dealer.river();
    self.table.add_river(river);
    self.calc.add(river);
    self.player.update(self.calc.calc_hand());
    self.computer.add(river);
  }

  /* Display table */
  pub fn table(self) {
    self.table.display();
  }
  
  pub fn player(self) {
    print!("Your ");
    self.player.display();
    println!("Hand value: {}", self.player.get_combo());
  }

  pub fn computer(self) {
    self.computer.display();
  }

  pub fn winner(self) {
    let c = self.computer.get_val();
    let p = self.player.get_val();
    let com_hand = self.computer.best_hand();
    let plyr_hand = self.calc.best_hand(p);
    let mut count = 0;
    print!("Computer's hand: ");
    for card in com_hand.iter() {
      card.display();
      count += 1;
      if count < 5 {
        print!(" | ");
      } 
      else {
        println!();
      }
    }
    count = 0;
    print!("Your hand: \t ");
    for card in plyr_hand.iter() {
      card.display();
      count += 1;
      if count < 5 {
        print!(" | ");
      } 
      else {
        println!();
      }
    }
    if c > p {                          // If com has higher value
      println!("Sorry, you lose");
    }
    else if p > c {                     // If player has higher value
      println!("Yay, you win");
    }
    else {                              // If tie
      let win = self.tie(com_hand,plyr_hand);
      match win {
        -1 => println!("Sorry, you lose"),
         1 => println!("Yay, you win"),
         _ => println!("It's a tie"),
      }
    }
  }

  /* Tie breaker function */
  fn tie(self,com: [Card; 5],plyr: [Card; 5]) -> i64 {
    for (i,j) in com.iter()
                    .zip(plyr.iter()) {
      if i.value() > j.value() {        // Computer wins
        return -1;
      }
      else if i.value() < j.value() {   // Computer wins
        return 1;
      }
    }
    0                                   // Tie
  }
}
