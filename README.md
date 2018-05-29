CS 410P Project - Texas Hold'em
=======

## Copyright © 2018 Robert Horrace

This project is a Texas Hold'em  
project written in Rust. So far,  
this is a prototype and only is half,  
if not mostly done. Current;y, it compiles
with no warning s and so far does what it  
needs to do. The program currently has a card,
a deck, a dealer, a player, a computer, a calc,  
a game, and a main. 

There is a card that holds two integers from  
2 to 14 and the suits from 0 to 3.  
The 0 value is a joker for the card, and nothing  
for the suit. 11-14 on the values is Jack to Ace  
and the suits values represent Hearts, Spades,  
Clubs, and Diamonds (1,2,3,4).

The deck hold 52 cards and contains a head index  
and a tail index, The deck will only shuffle when  
the deck is full, meaning the total amount of cards
is 52. A card is not necessarily removed from the deck,  
meaning head is incremented, and head - 1 is given. So,
it is a getter rather than a true remove function. It is  
easier to do it this way I think.

The dealer deals the deck. It shuffles the deck, checks the
size of the deck, resets the deck if needed, gives cards to 
the player, and gives cards to the table (flop, turn, river). 

The table only holds flop, turn, and river, nothing else.  
It will receive cards, display the table, and clear itself  
when the cards are done being used.

The player holds two cards (the hand) and a value for the  
combo. The hand value (combo) will be updated before the flop,  
after it, after the turn, and after the river.  

The computer has its own hand and calc for determine its  
hand's value. The functions are similar to player and will  
all be used.

The calc holds copies of the cards that have been played, which  
the cards are the hand, the flop, turn, and river. The cards are  
sorted by value for easier use. For calculating the hand, the  
flush flag is checked, if this passes, then the straight is checked.
If this flag passes, then it is check if it is a royal flush (10,J,Q,K,A).
If this fails, then straight flush is given. If the straight fails, then   
four of a kind is checked. If this fails, flush is given. If the flush fails,  
then straight is checked. If this fails, then four of a kind is checked.  
If this fails, then threes and pairs are checked. If there are at least one  
three of a kinds and/or at least one pair, it is a full house. If there is  
one three of a kind, then three of kind. If there are two or more pairs,  
two pairs, if one,one pair, and if all checks fail, high card.

The game holds the table, the dealer, the computer, the player,  
and calc, and runs the phases of the game. 

The tie module holds the tie breaking functions, meaning if both  
the player and the computer have both the same hand value, (ex. High card v High card)  
For all hands, except for the royal flush are compared by collecting the  
respected values of the player's and computer's handand comparing  
the cards from highest value to lowest value. If the values are the same,  
it is a tie. The following cards are used for the specific hands:
  * High card: Highest five cards, high to low
  * One pair: pair, 3 highest cards (kickers)
  * Two pair: Highest 2 pairs, Highest kicker (one card)
  * Three of a kind: the three of a kind, 2 kickers
  * Straight: Highest 5 card increment (ex. 1,2,3,4,5) High to low
  * Flush: Highest five cards of the same suit
  * Full House: High three of kind, then highest pair/three of kind
  * Four of a kind: Four of a kind, and highest kicker
  * Straight Flush: Highest 5 incremented card values of same suit, high to low
  * Royal flush: Automatic tie, even though it is statistically impossible

Main runs the game. The game has multiple stages. There is welcome  
stage where the program welcomes the player. There is a deal stage  
where the player and the computer gets their hands from the dealer.
There is a stage where the flop is played and the hand values are  
updated to correspond with the flop. The turn is then dealt and the  
hands are update accordingly. The river is then dealt and the hands  
are updated again afterwards. After the river, the computer's hand  
is displayed.

Hopefully all of this will work in the end, so far it does.
