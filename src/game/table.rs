// Copyright © 2018 CS410P Robert Horrace
/*
  This is the Rust file that
  contains the table and its 
  functions. It only holds the
  cards on the table (flop,turn,river)
  and will give them to calc.
*/

use game::card::*;

/* For flop,turn,river */
#[derive(Clone,Copy)]
pub struct Table {
    f_t_r: [Card; 5],
}

/* Impl for Table structure */
impl Table {
  /* New function for Table */
  pub fn new() -> Table {
    Table {f_t_r: [Card::new(); 5]}
  }  

  pub fn clear(& mut self) {
    self.f_t_r = [Card::new(); 5];
  }

  /* Receive the flop and store accordingly */
  pub fn add_flop(&mut self,flp: [Card; 3]) {
    for i in 0..3 {
      self.f_t_r[i] = flp[i];
    }
  }

  /* Receive the turn and store accordingly */
  pub fn add_turn(&mut self,trn: Card) {
    self.f_t_r[3] = trn;
  }

  /* Receive the river and store accordingly */
  pub fn add_river(&mut self,rvr: Card) {
    self.f_t_r[4] = rvr;
  }

  /* Display flop */
  fn disp_flop(self) {
    print!("\t Flop: ");
    for i in 0..3 {
      self.f_t_r[i].display();
      if i != 2 {
        print!(" | ");
      }
    }
    println!();
  }

  fn disp_card(self, i: usize) {
    if self.f_t_r[i].value() != 0 {
      match i {
        3 => print!("\t Turn: "),
        4 => print!("\tRiver: "),
        _ => print!(""),
      }
      self.f_t_r[i].display_ln();
    }
  }

  pub fn display(self) {
    println!("Table: ");
    self.disp_flop();
    for i in 3..5 {
      self.disp_card(i);
    }
  }
} 
