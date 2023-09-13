use::core::fmt;

struct Room {
    name: String,
    text: String,
    selections: Vec<Choice>
}

struct Choice {
    target: RoomID,
    triggers: Vec<String>,
    message: Option<String>, // What message, if any, to print when the doorway is traversed
    item: Option<String>
}

struct Bag {
    items: Vec<String>, // a vec holding every item collected by the player
}

impl fmt::Display for Bag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Here, you can format the `items` vector as a string
        // For example, you can join the items with commas and print them.
        let item_list: String = self.items.join(", ");
        write!(f, "Bag items: [{}]", item_list)
    }
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
        name: "starting_Kitchen_scene".into(),
        text: "You step into the pantry, the aroma of various ingredients surrounds you. Your choice of base flour will set the foundation for your cake. Press 1 for wheat, and 2 for almond.".into(),
        selections: vec![
                                                                        // message read after user picks the option
            Choice{target:RoomID(1), triggers:vec!["1".into()], message:Some("You reach for the all-purpose flour, a classic choice for a fluffy and light texture.".into()), item:Some("All-purpose Flour".into())},
            Choice{target:RoomID(2), triggers:vec!["2".into()], message:Some("You reach for the almond flour, a gluten-free option with a rich and nutty flavor.".into()), item:Some("Almond Flour".into())}
        ]
    },
    
    Room{
        name: "Kitchen_scene_A1".into(),
        // text read as soon as player makes choice
        text: "You got: all-purpose flour!".into(),
        selections: vec![]
    },

    Room{
        name: "Kitchen_scene2_B1".into(),
        text: "You got: almond flour!".into(),
        selections: vec![]
    }

];


    // VVV GAME LOOP STARTS BELOW VVV

    // Win condition == reaching end rooms
    let end_rooms = [RoomID(1), RoomID(2)];
    let mut input = String::new();
    let mut at = RoomID(0);
    // inventory == empty bag object
    let mut inventory = Bag{items: vec![]};
    println!("Hello, cooking world!");

    loop {
        let here = &rooms[at.0];
        println!("{}\n{}", here.name, here.text);
        // if our current room, at, equals an end room, terminate game loop
        if end_rooms.contains(&at){
            break;
        }
        loop {
            print!("What will you do?\n>");

            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            // choice is a value, its type --- result of find is an option &choice
            if let Some(choice) = here.selections.iter().find(|d| d.triggers.iter().any(|t| *t == input)) {
                if let Some(msg) = &choice.message {
                    println!("{}", msg);
                }
                // update current location, at, to choice's room id (target)
                at = choice.target;

                if let Some(item) = &choice.item {
                    inventory.items.push(item.clone()); // Clone the String and push it
                }

                println!("{}", inventory);
                break;
            } else {
                println!("You can't do that!");
            }

            // Want to implement 1. 2. 3. option scenario
        }
    }
}
