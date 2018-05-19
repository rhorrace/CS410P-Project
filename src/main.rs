// Copyright © 2018 CS410P Robert Horrace
/*
  This is the main Rust file
  which will manage the Texas Hold'em
  game.
*/

mod game;
mod card;
mod deck;
mod dealer;
mod player;
mod table;
mod calc;

use game::*;

fn main() {
  let mut game = Game::new();
  game.set();
  print!("{}[2J",27 as char);
  println!("Welcome to the Texas Hold'em Player.");
  println!("Warning: This is a prototope.\n");
  println!("It can only operate with one player\n");
  println!("and it is not meant for public use.");
  println!("ENTER: Continue, 1: Quit");
  if read_user() == 1 {
    print!("{}[2J",27 as char);
    println!("Goodbye.");
    return;
  }
  loop {
    print!("{}[2J",27 as char);
    game.deal();
    game.player1();
    println!("---------------------");
    println!("ENTER: Continue, 1: Quit");
    if read_user() == 1 {
      break;
    }
    print!("{}[2J",27 as char);
    game.flop();
    game.table();
    println!("---------------------");
    game.player1();
    println!("---------------------");
    println!("ENTER: Continue, 1: Quit");
    if read_user() == 1 {
      break;
    }
    print!("{}[2J",27 as char);
    game.turn();
    game.table();
    println!("---------------------");
    game.player1();
    println!("---------------------");
    println!("ENTER: Continue, 1: Quit");
    if read_user() == 1 {
      break;
    }
    print!("{}[2J",27 as char);
    game.river();
    game.table();
    println!("---------------------");
    game.player1();
    println!("---------------------");
    println!("ENTER: Continue, 1: Quit");
    if read_user() == 1 {
      break;
    }
    game.clear();
    if !game.check() {
      game.reset();
    }
  }

  print!("{}[2J",27 as char);
  println!("Goodbye.")
}

