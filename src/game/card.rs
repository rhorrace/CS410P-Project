// Copyright © 2018 CS410P Robert Horrace
/*
  This Rust file contains the Card
  and its functions.
*/

/* Card struct */
#[derive(Clone,Copy)]
pub struct Card {
  name: i64,
  suit: i64,
}

/* Impl for card struct */
impl PartialEq for Card {
  /* == , value only, not suit*/
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
 
  /* != , value only, not suit*/
  fn ne(&self, other: &Self) -> bool {
    self.name != other.name
  }
}

/* Impl for card struct */
impl Card {
  /* Initialize the card */
  pub fn new() -> Card {
    Card { name: 0,
           suit: 0, }
  }
 
  /* Check if card Is the card */
  pub fn is(self, other: &Self) -> bool {
    self.name == other.name && self.suit == other.suit
  }

  /* Return the value of the card */
  pub fn value(self) -> i64 {
    self.name
  }

  /* Return value of suit as number */
  pub fn suit(self) -> i64 {
    self.suit
  }
 
  /* Change the card name and suit */
  pub fn change(&mut self,nm: i64,st: i64) {
    self.name = nm;
    self.suit = st;
  }

  /* Display the card */
  pub fn display(self) {
    if self.name == 0 {
      print!("Joker");
      return;
    }
    else if self.name < 11 {
      match self.name {
       1 => print!("A  ; "),
      10 => print!("{} ; ",self.name),
       _ => print!("{}  ; ",self.name),
      }
    }
    else {
      match self.name {
      11 => print!("J  ; "),
      12 => print!("Q  ; "),
      13 => print!("K  ; "),
      14 => print!("A  ; "),
       _ => print!("Joker"),
      }
    }
    match self.suit {
    1 => print!("Hearts  "),
    2 => print!("Spades  "),
    3 => print!("Diamonds"),
    4 => print!("Clubs   "),
    _ => print!(""),
    }
  }
  
  /* Display card on one line */
  pub fn display_ln(self) {
    if self.name == 0 {
      println!("Joker");
      return;
    }
    else if self.name < 11 {
      match self.name {
       1 => print!("A  ; "),
      10 => print!("{} ; ",self.name),
       _ => print!("{}  ; ",self.name),
      }
    }
    else {
      match self.name {
        11 => print!("J  ; "),
        12 => print!("Q  ; "),
        13 => print!("K  ; "),
        14 => print!("A  ; "),
        _  => print!("Joker"),
      }
    }
    match self.suit {
      1 => println!("Hearts"),
      2 => println!("Spades"),
      3 => println!("Diamonds"),
      4 => println!("Clubs"),
      _ => println!(),
    }
  }
}
