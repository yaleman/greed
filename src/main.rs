#[derive(Debug, Eq, PartialEq)]
enum DiceGroup {
    /// four D's
    DQuad,
    /// three G's
    GTrio,
    /// Three R's
    RTrio,
    /// Three Silvers
    STrio,
    /// Three Green E's
    EGTrio,
    EBTrio,
    Greed,
    SixOfAKind,
    // Because you can hold a single dice
    IndividualDice {
        value: DiceValues,
    },
}

fn find_groups(value: Vec<DiceValues>) -> (Option<DiceGroup>, Vec<DiceValues>) {
    // this allows us to go "we have these dice, is there a scoring group". here be dragons and headaches, least of which with my failure to spell.
    // we can find either greed or a six of a kind
    if value.len() == 6 {
        // look for six of a kind
        // look for greed
        if value
            .clone()
            .into_iter()
            .any(|v| &v == &DiceValues::Gold)
            && value
                .clone()
                .into_iter()
                .any(|v| &v == &DiceValues::Ruby)
            && value
                .clone()
                .into_iter()
                .any(|v| &v == &DiceValues::Emerald)
            && value
                .clone()
                .into_iter()
                .any(|v| &v == &DiceValues::Ebony)
            && value
                .clone()
                .into_iter()
                .any(|v| &v == &DiceValues::Diamond)
            && value
                .clone()
                .into_iter()
                .any(|v| v == DiceValues::Silver)
        {
            // we found a greed!
            return (Some(DiceGroup::Greed), vec![]);
        }
    }

    if value.len() >= 4 {
        // we can probably find ... things.
    }

    if value.len() >= 3 {
        // three of a kinds!

        // silvers

        // reds

        // d's

        // eblack

        // egreen

        // golds
    }

    // we're looking for single G/D's at this point
    // blah

    // we didn't find anything by the end
    (None, value)
}

impl From<DiceGroup> for u32 {
    /// Returns the score value of a group of dice
    fn from(value: DiceGroup) -> Self {
        match value {
            DiceGroup::DQuad => 1000,
            DiceGroup::GTrio => 500,
            DiceGroup::RTrio => 400,
            DiceGroup::STrio => 600,
            DiceGroup::EGTrio => 300,
            DiceGroup::EBTrio => 300,
            DiceGroup::Greed => 1000,
            DiceGroup::SixOfAKind => 5000,
            DiceGroup::IndividualDice { value } => value.into(),
        }
    }
}

/// this sets up the compiler to allow comparing different dice values
#[derive(Clone, Debug, Eq, PartialEq)]
enum DiceValues {
    Gold,
    Ruby,
    Emerald,
    Ebony,
    Diamond,
    Silver,
}

impl From<DiceValues> for u32 {
    /// returns the value of the individual dice in points, only applies for dice we can hold a single one of
    fn from(value: DiceValues) -> Self {
        match value {
            DiceValues::Gold => 50,
            DiceValues::Diamond => 100,
            _ => panic!("Uh.... no?"),
        }
    }
}

impl DiceValues {
    fn random() -> Self {
        DiceValues::Ebony
    }
}

struct Dice {
    value: Option<DiceValues>,
}

impl Dice {
    // returns a random dice value
    fn roll() -> Option<DiceValues> {
        // TODO: fix this so it's random :)
        Some(DiceValues::random())
    }
}

struct Player {
    pub name: String,
    pub score: u32,
    /// dice we're using for this turn
    pub dice: Vec<Dice>,
    /// dice we're currently holding back... this isn't right at the moment
    pub held_dice: Vec<DiceGroup>,
}

impl Player {
    /// sets up a player's state for a new turn
    fn new_turn(mut self: Player) {
        self.dice = vec![];
    }

    /// create a new player, for the start of the game
    fn new_with_name(name: String) -> Self {
        Player {
            name,
            dice: vec![],
            held_dice: vec![],
            score: 0,
        }
    }
}

fn main() {
    // set up the list of players
    let players = vec![
        Player::new_with_name("Alice".to_string()),
        Player::new_with_name("Bob".to_string()),
    ];

    loop {
        // iterate through the players, letting them have a turn
        // players.iter().map(|player| {
        //     player.new_turn();
        //     // ??
        // });

        if players.iter().any(|p| p.score > 5000) {
            println!("it was the last round!");
            break;
        }
    }
}

mod tests {
    

    #[test]
    pub fn test_for_greed_when_we_dont_have_greed() {
        let dicewehave = vec![
            DiceValues::Gold,
            DiceValues::Gold,
            DiceValues::Ebony,
            DiceValues::Emerald,
            DiceValues::Diamond,
            DiceValues::Silver,
        ];

        assert_eq!(find_groups(dicewehave.clone()), (None, dicewehave));
    }

    #[test]
    pub fn test_for_greed_when_we_have_greed() {
        let dicewehave = vec![
            DiceValues::Gold,
            DiceValues::Ruby,
            DiceValues::Ebony,
            DiceValues::Emerald,
            DiceValues::Diamond,
            DiceValues::Silver,
        ];

        let (res, leftovers) = find_groups(dicewehave);
        if !leftovers.is_empty() {
            panic!("Should get empty leftovers");
        }

        match res {
            Some(value) => match value {
                DiceGroup::Greed => println!("Success!"),
                _ => panic!("Should have gotten a greed..."),
            },
            None => panic!("Got a None result when we should have gotten greed."),
        }
    }
}
