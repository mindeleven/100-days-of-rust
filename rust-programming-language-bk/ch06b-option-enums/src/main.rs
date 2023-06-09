/// Option<T> enum definition in the standard library
/// enum Option<T>
/// { 
///     None,
///     Some(T)
/// }
/// <T> is the syntax for a generic type
/// it means the Some variant can hold any piece of data

fn main() {
    // Some and None can be used directly without the Option:: prefix
    let _some_number = Some(5);
    let _some_char = Some('e');
    // for None the overall Option<T> type needs to be annotated
    let _absent_number: Option<i32> = None;

    println!("Hello Option<T> enums!");

    // catch all patterns
    let dice_roll = 9;

    // act according to dice roll in a game
    match dice_roll {
        // actions according to dice roll
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // last arm covers all other possible values
        _ => reroll(),
    }

    // match with _ placeholder (catch all)
    match dice_roll {
        // actions according to dice roll
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // last arm covers all other possible values
        // _ matches any value but does not bind to any value
        other => move_player(other),
    }

    // match with _ placeholder and empty tuple (unity value)
    match dice_roll {
        // actions according to dice roll
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // last arm covers all other possible values
        // _ matches any value but does not bind to any value
        // empty tuple indicates that nothing should happen
        _ => (),
    }

    // concise control flow with if let
    // handles values that match one pattern while ignoring the rest
    let config_max = Some(3u8);
    // first with match
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // we only want to bind the value in some so we could use 'if let'
    // 'if let' takes a pattern and an expression separated by an equal sign
    // an else block can be included with an 'if let'
    if let Some(max) = config_max {
        println!("The maximum is configured to be {} (from if let block)", max);
    }

}

// dummy function to add a hat in a game
fn add_fancy_hat() {}

// dummy function to remove a hat in a game
fn remove_fancy_hat() {}

// dummy function to remove a player in a game
fn move_player(_spaces: u8) {}

// dummy function to reroll the dice in a game
fn reroll() {}