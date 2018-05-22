// Copyright © 2018 CS410P Robert Horrace
/*
  This Rust file contains the Card
  and its functions.
*/


#[derive(Clone,Copy)]
pub struct Card {
  name: i64,
  suit: i64,
}

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
/*
impl Ord for Card {
  /* Value < Value, not suit */
  fn lt(&self, other: &Self) -> bool {
    self.name < other.name;
  }

  /* Value <= Value, not suit */
  fn le(&self, other: &Self) -> bool {
    self.name <= other.name;
  }

  /* Value > Value, not suit */
  fn gt(&self, other: &Self) -> bool {
    self.name > other.name;
  }

  /* Value >= Value, not suit */
  fn ge(&self, other: &Self) -> bool {
    self.name >= other.name;
  }
}
*/

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
      print!("{} of ",self.name);
    }
    else {
      match self.name {
        11 => print!("Jack of "),
        12 => print!("Queen of "),
        13 => print!("King of "),
        14 => print!("Ace of "),
        _  => print!("Joker"),
      }
    }
    match self.suit {
      1 => print!("Hearts"),
      2 => print!("Spades"),
      3 => print!("Diamonds"),
      4 => print!("Clubs"),
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
      print!("{} of ",self.name);
    }
    else {
      match self.name {
        11 => print!("Jack of "),
        12 => print!("Queen of "),
        13 => print!("King of "),
        14 => print!("Ace of "),
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

  /* Return the name in &str form, not the suit */
  pub fn _name(self) -> &'static str {
    match self.name {
       2 => "2",
       3 => "3",
       4 => "4",
       5 => "5",
       6 => "6",
       7 => "7",
       8 => "8",
       9 => "9",
      10 => "10",
      11 => "Jack",
      12 => "Queen",
      13 => "King",
      14 => "Ace",
      _  => "Joker",
    }
  }
}
