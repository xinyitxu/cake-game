struct Room {
    name: String,
    text: String,
    selections: Vec<Choices>
}

struct Choices {
    target: Vec<RoomID>, // More about this in a minute
    triggers: Vec<String>,
    message: Option<String> // What message, if any, to print when the doorway is traversed
    // Any other info about the door would go here
}

struct Bag {
    items: Vec<String>, // a vec holding every item collected by the player
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct RoomID(usize); // points to the index of a room from our list of rooms



// try 1 -> return
// then try using crossterm
// instead of standrd io read - use crossterm event read, read waits until a 
// user action is availabel pop from queue and give to program
fn main() {
    use std::io;
    use std::io::Write;

    // Rooms or scenes
    let rooms = [
    Room{
        name: "Kitchen_scene1".into(),
        text: "You step into the pantry, the aroma of various ingredients surrounds you. Your choice of base flour will set the foundation for your cake. Press 1 for wheat, and 2 for almond.".into(),
        selections: vec![
            // rename to Choice
            Choice{target:RoomID(1), triggers:vec!["1".into()], message:vec!["Reach for the all-purpose flour. A classic choice for a fluffy and light texture."]},
            Choice{target:RoomID(2), triggers:vec!["2".into()], message:vec!["Select the almond flour. A gluten-free option with a rich and nutty flavor."]}
        ]
    },
    
    Room{
        name: "Kitchen_scene2".into(),
        text: "You chose flour, great choice!".into(),
        selections: vec![]
    },

    Room{
        name: "Kitchen_scene2".into(),
        text: "You chose almond flour, great choice!".into(),
        selections: vec![]
    }

];

    let end_room = [RoomID(1), RoomID(2)];
    let mut input = String::new();

    // What does this line accomplish?
    let mut at = RoomID(0);
    println!("Hello, cooking world!");

    loop {
        let here = &rooms[at.0];
        println!("{}\n{}", here.name, here.text);
        // What does .contains&at do?
        if end_rooms.contains(&at){
            break;
        }
        loop {
            print!("What will you choose?\n>");

            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            // choice is a value, its type --- result of fine is an option &choice
            if let Some(choice) = here.selections.iter().find(|d| d.triggers.iter().any(|t| *t == input)) {
                if let Some(msg) = &selections.message {
                    println!("{}", msg);
                }
                at = door.target;
                break;
            } else {
                println!("You can't do that!");
            }

            // Want to implement 1. 2. 3. option scenario
        }
    }
}
