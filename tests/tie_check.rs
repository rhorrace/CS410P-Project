extern crate library;

use library::files::tie::*;
use library::files::card::*;

#[test]
fn high_card_chck() {
  /* For checking tie */
  let c1 = [0,1,1,0,1,0,1,0,1,0,1,0,1,0,1];
  let p1 = [0,1,1,0,1,0,1,0,1,0,1,0,1,0,1];
  assert_eq!(high_card(c1,p1),0);

  /* Check computer wins */
  let c2 = [0,1,0,0,1,0,1,0,1,0,1,0,1,1,1];
  let p2 = [0,1,1,0,1,0,1,0,1,0,1,0,1,0,1];
  assert_eq!(high_card(c2,p2),-1);

  /* Check player wins */
  let c3 = [0,1,1,0,1,0,1,0,1,0,1,0,1,0,1];
  let p3 = [0,1,0,0,1,0,1,0,1,0,1,0,1,1,1];
  assert_eq!(high_card(c3,p3),1);
}

#[test]
fn one_pair_chck() {
  /* For checking tie */
  let c1   = [0,2,0,0,1,0,1,0,1,0,1,0,1,0,2];
  let p1   = [0,2,0,0,1,0,1,0,1,0,1,0,1,0,2];
  assert_eq!(one_pair(c1,p1),0);

  /* Check computer wins */
  let c2   = [0,0,0,0,1,0,1,0,1,0,1,0,1,2,0];
  let p2   = [0,0,0,0,1,0,1,0,1,0,1,0,2,1,0];
  let c2_k = [0,0,0,0,0,0,1,0,1,0,1,1,1,2,0];
  let p2_k = [0,0,0,0,0,0,1,0,1,1,1,1,0,2,0];
  assert_eq!(one_pair(c2,p2),-1);
  assert_eq!(one_pair(c2_k,p2_k),-1);

  /* Check player wins */
  let c3   = [0,1,1,0,1,0,1,0,1,0,1,0,2,0,1];
  let p3   = [0,1,0,0,1,0,1,0,1,0,1,0,1,2,1];
  let c3_k = [0,0,0,0,0,0,1,0,1,1,1,1,0,2,0];
  let p3_k = [0,0,0,0,0,0,1,0,1,0,1,1,1,2,0];
  assert_eq!(one_pair(c3,p3),1);
  assert_eq!(one_pair(c3_k,p3_k),1);
}

#[test]
fn two_pair_chck() {
  /* For checking tie */
  let c1   = [0,0,0,0,0,0,1,0,1,0,1,0,2,2,0];
  let p1   = [0,0,0,0,0,0,1,0,1,0,1,0,2,2,0];
  assert_eq!(two_pair(c1,p1),0);

  /* Check computer wins */
  let c2   = [0,0,0,0,0,0,1,0,1,0,1,0,2,2,0];
  let p2   = [0,0,0,0,0,0,1,0,1,0,2,0,2,1,0];
  let c2_k = [0,0,0,0,0,0,0,0,0,1,1,1,2,2,0];
  let p2_k = [0,0,0,0,0,0,0,0,1,1,1,0,2,2,0];
  assert_eq!(two_pair(c2,p2),-1);
  assert_eq!(two_pair(c2_k,p2_k),-1);

  /* Check player wins */
  let c3   = [0,0,0,0,0,0,1,0,1,0,2,0,2,1,0];
  let p3   = [0,0,0,0,0,0,1,0,1,0,1,0,2,2,0];
  let c3_k = [0,0,0,0,0,0,0,0,1,1,1,0,2,2,0];
  let p3_k = [0,0,0,0,0,0,0,0,0,1,1,1,2,2,0];
  assert_eq!(two_pair(c3,p3),1);
  assert_eq!(two_pair(c3_k,p3_k),1);
}

#[test]
fn three_of_kind_chck() {
  /* For checking tie */
  let c1   = [0,0,0,0,0,0,1,0,1,0,1,0,1,3,0];
  let p1   = [0,0,0,0,0,0,1,0,1,0,1,0,1,3,0];
  assert_eq!(three_of_kind(c1,p1),0);

  /* Check computer wins */
  let c2   = [0,0,0,0,0,0,1,0,1,0,1,0,1,3,0];
  let p2   = [0,0,0,0,0,0,1,0,1,0,1,0,3,1,0];
  let c2_k = [0,0,0,0,0,0,0,0,0,1,1,1,1,3,0];
  let p2_k = [0,0,0,0,0,0,0,0,1,1,1,0,1,3,0];
  assert_eq!(three_of_kind(c2,p2),-1);
  assert_eq!(three_of_kind(c2_k,p2_k),-1);

  /* Check player wins */
  let c3   = [0,0,0,0,0,0,1,0,1,0,1,0,3,1,0];
  let p3   = [0,0,0,0,0,0,1,0,1,0,1,0,1,3,0];
  let c3_k = [0,0,0,0,0,0,0,0,1,1,1,0,1,3,0];
  let p3_k = [0,0,0,0,0,0,0,0,0,1,1,1,1,3,0];
  assert_eq!(three_of_kind(c3,p3),1);
  assert_eq!(three_of_kind(c3_k,p3_k),1);
}

#[test]
fn straight_chck() {
  /* For checking tie */
  let c1   = [0,0,0,0,0,0,1,0,1,1,1,1,1,1,0];
  let p1   = [0,0,0,0,0,0,1,0,1,1,1,1,1,1,0];
  assert_eq!(straight(c1,p1),0);

  /* Check computer wins */
  let c2   = [0,0,0,0,0,0,1,0,1,1,1,1,1,1,0];
  let p2   = [0,0,0,0,0,1,0,1,1,1,1,1,1,0,0];
  assert_eq!(straight(c2,p2),-1);

  /* Check player wins */
  let c3   = [0,0,0,0,0,0,1,1,1,1,1,1,1,0,0];
  let p3   = [0,0,0,0,0,0,1,0,1,1,1,1,1,1,0];
  assert_eq!(straight(c3,p3),1);
}

#[test]
fn flush_chck() {
  let c_suit = 1;
  let p_suit = 2;
  let mut c = [Card::new(); 7];
  let mut p = [Card::new(); 7];
  let mut val = 2;
  for i in 0..7 {
    c[i].change(val,c_suit);
    p[i].change(val,p_suit);
    val += 2;
  }
  /* For checking tie */
  assert_eq!(flush(c,c_suit,p,p_suit),0);

  /* Check computer wins */
  p[6].change(14,3);
  assert_eq!(flush(c,c_suit,p,p_suit),-1);
  p[6].change(14,p_suit);

  /* check player wins*/
  c[6].change(14,3);
  assert_eq!(flush(c,c_suit,p,p_suit),1);
}

#[test]
fn full_house_chck() {
  /* For checking tie */
  let c1   = [0,0,0,0,0,0,0,0,1,0,1,0,2,3,0];
  let p1   = [0,0,0,0,0,0,0,0,1,0,1,0,2,3,0];
  assert_eq!(full_house(c1,p1),0);

  /* Check computer wins */
  let c2   = [0,0,0,0,0,0,0,0,0,0,1,1,2,3,0];
  let p2   = [0,0,0,0,0,0,0,0,0,0,1,1,3,2,0];
  let c2_2 = [0,0,0,0,0,0,0,0,0,0,1,1,2,3,0];
  let p2_2 = [0,0,0,0,0,0,0,0,0,1,1,2,0,3,0];
  assert_eq!(full_house(c2,p2),-1);
  assert_eq!(full_house(c2_2,p2_2),-1);

  /* Check player wins */
  let c3   = [0,0,0,0,0,0,0,0,0,0,1,1,3,2,0];
  let p3   = [0,0,0,0,0,0,0,0,0,0,1,1,2,3,0];
  let c3_2 = [0,0,0,0,0,0,0,0,0,1,1,2,0,3,0];
  let p3_2 = [0,0,0,0,0,0,0,0,0,0,1,1,2,3,0];
  assert_eq!(full_house(c3,p3),1);
  assert_eq!(full_house(c3_2,p3_2),1);
}

#[test]
fn four_of_kind_chck() {
  /* For checking tie */
  let c1   = [0,0,0,0,0,0,0,0,1,0,1,0,1,4,0];
  let p1   = [0,0,0,0,0,0,0,0,1,0,1,0,1,4,0];
  assert_eq!(four_of_kind(c1,p1),0);

  /* Check computer wins */
  let c2   = [0,0,0,0,0,0,0,0,1,0,1,0,1,4,0];
  let p2   = [0,0,0,0,0,0,0,0,1,0,1,0,4,1,0];
  let c2_k = [0,0,0,0,0,0,0,0,0,0,1,1,1,4,0];
  let p2_k = [0,0,0,0,0,0,0,0,0,1,1,1,0,4,0];
  assert_eq!(four_of_kind(c2,p2),-1);
  assert_eq!(four_of_kind(c2_k,p2_k),-1);

  /* Check player wins */
  let c3   = [0,0,0,0,0,0,0,0,1,0,1,0,4,1,0];
  let p3   = [0,0,0,0,0,0,0,0,1,0,1,0,1,4,0];
  let c3_k = [0,0,0,0,0,0,0,0,0,1,1,1,0,4,0];
  let p3_k = [0,0,0,0,0,0,0,0,0,0,1,1,1,4,0];
  assert_eq!(four_of_kind(c3,p3),1);
  assert_eq!(four_of_kind(c3_k,p3_k),1);
}

#[test]
fn straight_flush_chck() {
  let c_suit = 1;
  let p_suit = 2;
  let mut c = [Card::new(); 7];
  let mut p = [Card::new(); 7];
  let mut val = 2;
  for i in 0..7 {
    c[i].change(val,c_suit);
    p[i].change(val,p_suit);
    val += 1;
  }
  /* For checking tie */
  assert_eq!(flush(c,c_suit,p,p_suit),0);

  /* Check computer wins */
  p[6].change(8,3);
  assert_eq!(flush(c,c_suit,p,p_suit),-1);
  p[6].change(8,p_suit);

  /* check player wins*/
  c[6].change(8,3);
  assert_eq!(flush(c,c_suit,p,p_suit),1);
}
