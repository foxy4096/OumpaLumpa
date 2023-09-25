#[allow(dead_code)]
fn main() {
    enum State {
        Jharkhand,
        Bihar,
        Chennai,
        // --Snip--
    }
    enum Food {
        Idli,
        Dosa,
        PaneerBtMassala(Option<State>),
    }
    fn get_price_of_food(food: Food) -> u32 {
        match food {
            Food::Idli => 15,
            Food::Dosa => 30,
            Food::PaneerBtMassala(state) => match state {
                None => 25,
                Some(state) => match state {
                    State::Bihar => 20,
                    State::Jharkhand => 25,
                    State::Chennai => 27,
                },
            },
        }
    }
    let dosa = Food::Dosa;
    let paneer_from_bihar = Food::PaneerBtMassala(Some(State::Bihar));
    println!(
        "The cost to dosa everywhere is Rs. {}",
        get_price_of_food(dosa)
    );
    println!(
        "The cost to Paneer Bt. Massala from Bihar is Rs. {}",
        get_price_of_food(paneer_from_bihar)
    );
    let paneer_bt_massala_from_anywhere = Food::PaneerBtMassala;
    println!(
        "The cost to Paneer Bt. Massala from anywhere is Rs. {}",
        get_price_of_food(paneer_bt_massala_from_anywhere(None))
    );
}
