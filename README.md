CS 410P Project - Texas Hold'em
=======

## Copyright © 2018 Robert Horrace

This project is a Texas Hold'em  
project written in Rust. So far,  
this is a prototype and only is half,  
if not mostly done. Current;y, it compiles
with no warning s and so far does what it  
needs to do.  

There is a card that holds two integers from  
2 to 14 and the suits from 0 to 4.  
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

The game holds the table, the dealer, the player, and calc, and runs the game.

Main runs the game.

Hopefully all of this will work in the end, so far it does.
