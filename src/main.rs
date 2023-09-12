struct Room {
    name: String, 
    text: String, 
    selections: Vec<Choice>
}
struct Choice {
    target: RoomID, 
    triggers:Vec<String>, 
    message: Option<String> 
}

struct Bag {
    items: Vec<String>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct RoomID(usize);

fn main() {
    use std::io;
    use std::io::Write;

    // Rooms or scenes
    let rooms = [
    Room{
        name: "Kitchen Scene".into(),
        text: "You step into the pantry, the aroma of various ingredients surrounds you. Your choice of base flour will set the foundation for your cake. Type 1 for wheat, and 2 for almond.".into(),
        selections: vec![
            // rename to Choice
            Choice{target:RoomID(1), triggers:vec!["1".into()], message:Some("Reach for the all-purpose flour. A classic choice for a fluffy and light texture.".into())},
            Choice{target:RoomID(2), triggers:vec!["2".into()], message:Some("Select the almond flour. A gluten-free option with a rich and nutty flavor.".into())}
        ]
    },
    
    Room{
        name: "Kitchen_scene2".into(),
        text: "You chose flour, great choice!".into(),
        selections: vec![]
    },

    Room{
        name: "Kitchen_scene2".into(),
        text: "You chose almond flour, great choice! Type '1' to continue.".into(),
        selections: vec![
            Choice{target:RoomID(3), triggers:vec!["1".into()], message:None}
        ]
    },

    Room{
        name: "⊹˚. ♡.𖥔 ݁ ˖ Make your cake flavorful! ⊹˚. ♡.𖥔 ݁ ˖".into(),
        text: "The secret to a memorable cake often lies in its flavors. Do you want a hint of vanilla or the richness of cocoa? Or perhaps, something else entirely? 
        Type '1': Vanilla extract. 
        Type '2': Cocoa powder.
        Type '3': Grate some lemon zest.".into(),
        selections: vec![
            Choice{target:RoomID(4), triggers:vec!["1".into()], message:Some("You chose: Vanilla extract. Sweet and aromatic!".into())},
            Choice{target:RoomID(4), triggers:vec!["2".into()], message:Some("You chose: Cocoa powder. Deep and chocolaty!".into())},
            Choice{target:RoomID(4), triggers:vec!["3".into()], message:Some("You chose: Grate some lemon zest. Fresh and zingy!".into())}
        ]
    },

    Room{
        name: "⊹˚. ♡.𖥔 ݁ ˖ Mixing the Batter Interlude ⊹˚. ♡.𖥔 ݁ ˖".into(),
        text: "Cue elevator music! Mix mix mix up the batter! 
        Type 'mix' to continue".into(),
        selections: vec![
            Choice{target:RoomID(5), triggers:vec!["mix".into()], message:Some("Throw in some chocolate chips.".into())},
        ]
    },

    Room{
        name: "⊹˚. ♡.𖥔 ݁ ˖ Add Flavoring ⊹˚. ♡.𖥔 ݁ ˖ ".into(),
        text: "Some say the best part of a cake is the surprise inside. What would you like to fold into your batter?
        Type '1': Chocolate Chips. 
        Type '2': Fresh Blueberries. 
        Type '3': Chopped Walnuts".into(),
        selections: vec![
            Choice{target:RoomID(6), triggers:vec!["1".into()], message:Some("You chose: chocolate chips.".into())},
            Choice{target:RoomID(6), triggers:vec!["2".into()], message:Some("You chose: fresh blueberries.".into())},
            Choice{target:RoomID(6), triggers:vec!["3".into()], message:Some("You chose: chopped walnuts.".into())}
        ]
    },

    Room{
        name: "⊹˚. ♡.𖥔 ݁ ˖ Oven ⊹˚. ♡.𖥔 ݁ ˖ ".into(),
        text: "You slide cake into the oven to begin baking. The oven hums softly, awaiting your creation.
        Type 'bake' to continue".into(),
        selections: vec![
            Choice{target:RoomID(7), triggers:vec!["bake".into()], message:Some("Your cake is now fully baked and smells amazing!".into())}
        ]
    },

    Room{
        name: "⊹˚. ♡.𖥔 ݁ ˖ Toppings ⊹˚. ♡.𖥔 ݁ ˖".into(),
        text: "Your cake stands before you, as a blank canvas. It's time to add the finishing touches!
        Type '1': Add some strawberrries.
        Type '2': Pipe some frosted flowers.
        Type '3': Cherry on top".into(),
        selections: vec![
            Choice{target:RoomID(8), triggers:vec!["1".into()], message:Some("You chose: Add some strawberrries.".into())},
            Choice{target:RoomID(9), triggers:vec!["2".into()], message:Some("You chose: Pipe some frosted flowers.".into())},
            Choice{target:RoomID(10), triggers:vec!["3".into()], message:Some("You chose: Cherry on top.".into())},
        ]

    },

    Room{
        name: "⊹˚. ♡.𖥔 ݁ ˖  Yummy Strawberry Cake  ⊹˚. ♡.𖥔 ݁ ˖ ".into(),
        text: "You've crafted a quaint cake! She’s got lot of personality!".into(),
        selections: vec![]
    },

    Room{
        name: "⊹˚. ♡.𖥔 ݁ ˖ Pretty Flower Cake ⊹˚. ♡.𖥔 ݁ ˖ ".into(),
        text: "You've crafted a lovely cake! Bakers everywhere want to know of your coveted recipe.Great work!".into(),
        selections: vec![]
    },

    Room{
        name: "⊹˚. ♡.𖥔 ݁ ˖ Dainty Cherry Cake ⊹˚. ♡.𖥔 ݁ ˖ ".into(),
        text: "You've crafted a lovely cake! The cherry on top really just does it!".into(),
        selections: vec![]
    }
    ];

    let end_rooms = [RoomID(8),RoomID(9), RoomID(10)];
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
                if let Some(msg) = &choice.message {
                    println!("{}", msg);
                }
                at = choice.target;
                break;
            } else {
                println!("You can't do that!");
            }

            // Want to implement 1. 2. 3. option scenario
        }
    }
}