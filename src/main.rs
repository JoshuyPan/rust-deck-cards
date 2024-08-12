use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck{
    cards: Vec<String>,
}

impl Deck{
    fn new() -> Self {
         // List of 'suits' - 'hearts', 'spades' and so on...
        let suits = ["Hearts", "Spades", "Diamonds"];
        // List of 'values' - 'ace', 'two', 'three' and bla bla
        let values = ["Ace", "Two", "Three"];

        //making an empty Vector to store the cards
        let mut cards = vec![];

        // Duoble Nested For Loop
        for suit in suits{
            for value in values{
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }} 
        Deck {cards}
    }

    fn shuffle(&mut self){
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(
            self.cards.len() - num_cards
        )
    }
}

fn main() {
   let mut deck = Deck::new();

   println!("Heres your Deck: {:#?}", deck);
   
   deck.shuffle();
   // Needs error handling!
   let cards = deck.deal(3);

   println!("Heres your Hand: {:#?}", cards);
}
