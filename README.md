CS 410P Project - Texas Hold'em
=======

## Copyright © 2018 Robert Horrace

Author: Robert Horrace [rhorrace@pdx.edu]  

This project is a Texas Hold'em  
project written in Rust. So far,  
this is a prototype and only mostly,  
if not completely done. Currently, it compiles
with no warnings and so far it does what it  
needs to do. The program currently has a main,
and a library called files, which contains the Rust  
files used in the program. Feel free to view these  
files and give comments on how they can be improved.

Some rules for Texas Hold'em: 

  1. Highest hand wins (best 5 out of 7 cards)
  2. Ties are broken when someone:
    1. has a higher card values in hand  
    2. Has higher kickers if matching set values
    3. Otherwise, a tie
  3. There are 5 stages in this game after the welcome:
    1. Receive starting two cards to you and computer
    2. Flop stage
      * Three cards are put on the table
      * hand values are updated
    3. Turn stage
      * One card is put on the table
      * hand values are updated
    4. River stage
      * One card is put on the table
      * hand values are updated
    5. Computer shows hand, winner is declared
      * Will display you win, you lose, or tie
  4. normally there is betting, but there is no betting
     in this game at the moment.

Values of hands 0 being lowest hand, 10 being highest:  
  1. High card
  2. One pair
    * a pair of equal value cards
  3. Two pair
    * two one pairs
  4. Three of a kind
    * Three cards of same value
  5. Straight
    * 5 incremented cards (ex. 5,6,7,8,9)
  6. Flush
    * 5 cards of same suit
  7. Full house
    * A three of a kind and a pair/three of a kind
  8. Four of a kind
    * Four cards of same value
  9. Straight flush
    * 5 incremented cards of same suit
  10. Royal flush
    * highest straight flush possible, 10, J, Q, K ,A
