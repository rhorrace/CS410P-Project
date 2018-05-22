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
  print!("{}[2J",27 as char);
  print!("{}[2J",27 as char);
  let mut game = Game::new();
  game.set();
  println!("Welcome to the Texas Hold'em Player.");
  println!("Warning: This is a prototope.\n");
  println!("It can only operate with one player\n");
  println!("and it is not meant for public use.");
  println!("ENTER: Continue, 1: Quit");
  
  /* Check user roption */
  if read_user() == 1 {
    print!("{}[2J",27 as char);
    println!("Goodbye.");
    return;
  }

  /* Start game */
  loop {
    print!("{}[2J",27 as char);
    game.deal();
    game.player1();
    println!("---------------------");
    println!("ENTER: Continue, 1: Quit");

    /* Check user option */
    if read_user() == 1 {
      break;
    }
    print!("{}[2J",27 as char);

    /* Flop stage */
    game.flop();
    game.table();
    println!("---------------------");
    game.player1();
    println!("---------------------");
    println!("ENTER: Continue, 1: Quit");

    /* Check user option */
    if read_user() == 1 {
      break;
    }
    print!("{}[2J",27 as char);

    /* Turn stage */
    game.turn();
    game.table();
    println!("---------------------");
    game.player1();
    println!("---------------------");
    println!("ENTER: Continue, 1: Quit");

    /* Check user option */
    if read_user() == 1 {
      break;
    }
    print!("{}[2J",27 as char);

    /* River stage */
    game.river();
    game.table();
    println!("---------------------");
    game.player1();
    println!("---------------------");
    println!("ENTER: Continue, 1: Quit");

    /* Check user option */
    if read_user() == 1 {
      break;
    }

    /* Clear stage */
    game.clear();
    if !game.check() {
      game.reset();
    }
  }

  print!("{}[2J",27 as char);
  println!("Goodbye.")
}

