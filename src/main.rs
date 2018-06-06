// Copyright © 2018 CS410P Robert Horrace
/*
  This is the main Rust file
  which will manage the Texas Hold'em
  game.
*/

extern crate library;

use library::game::*;

fn main() {
  for _ in 0..100 {
    println!();
  }
  let mut game = Game::new();
  game.set();
  println!("Welcome to the Texas Hold'em Player.");
  println!("You will be facing a computer"); 
  println!("and the game is able to determine who is a winner,");
  println!("even in the case of a tie.");
  println!("The following hands are ranked from lowest to highest (up to down):");
  println!("\tHigh card\n\tOne pair\n\tTwo pair\n\tThree of a kind");
  println!("\tStraight\n\tFlush\n\tFull house\n\tFour of a kind");
  println!("\tStraight flush\n\tRoyal flush");
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
    game.player();
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
    game.player();
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
    game.player();
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
    game.player();
    println!("---------------------");
    println!("ENTER: Continue, 1: Quit");

    /* Check user option */
    if read_user() == 1 {
      break;
    }
    print!("{}[2J",27 as char);
    
    /* Display computer's hand stage */
    game.computer();
    println!("---------------------");
    game.table();
    println!("---------------------");
    game.player();
    println!("---------------------");
    game.winner();
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

