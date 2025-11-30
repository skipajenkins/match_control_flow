#[derive(Debug)]       

enum UsState{
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin){
    match coin{
        Coin::Penny => 1,
        // Coin:: Penny => {
        //     println!("Lucky Penny");
        //     1
        // }//Throws an error. Expecting a () but found an integer.
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25,
        // Coin::Quarter(state) => {
        //     println!("State quarter from {state:?}!");
        // 25
        // }//Same issue throws an error ...was expecting() but found an integer
    };
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll(){}

fn main() {

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => () // Any other number either reroll or return an empty tuple () 
        //other => move_player(other),
        //Other means any other number aside from 3 and 7 if rolled
        //move the dice by the number that was played that isn't 3 and 7
    }
}
