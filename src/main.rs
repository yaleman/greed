use std::collections::HashMap;
use std::fmt::Display;

use rand::distr::StandardUniform;
use rand::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
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

#[test]
fn test_count_die() {
    use crate::*;
    let dice_holding = vec![DiceValues::Ruby, DiceValues::Ruby, DiceValues::Ruby];
    println!("holding={:?}", count_die(&dice_holding));
    assert_eq!(dice_holding.len(), 3);
}

fn count_die(dice: &[DiceValues]) -> HashMap<DiceValues, u32> {
    let res = dice.iter().fold(
        HashMap::default(),
        |mut acc: HashMap<DiceValues, u32>, die| {
            // let diename = format!("{die:?}");

            acc.entry(*die).and_modify(|e| *e += 1).or_insert(1);

            // x.update_state_how_you_want;
            acc
        },
    );
    println!("### {res:?}");
    for key in res.keys() {
        if res[key] == 6 {
            println!("  Found 6 {key}s");
        }
        if res[key] == 3 {
            println!("  Found 3 {key}s");
        }
    }
    res
}

fn find_groups(value: Vec<DiceValues>) -> (Option<DiceGroup>, Vec<DiceValues>) {
    // this allows us to go "we have these dice, is there a scoring group". here be dragons and headaches, least of which with my failure to spell.
    // we can find either greed or a six of a kind
    if value.len() == 6 {
        // look for six of a kind
        // look for greed
        if value.clone().into_iter().any(|v| v == DiceValues::Gold)
            && value.clone().into_iter().any(|v| v == DiceValues::Ruby)
            && value.clone().into_iter().any(|v| v == DiceValues::Emerald)
            && value.clone().into_iter().any(|v| v == DiceValues::Ebony)
            && value.clone().into_iter().any(|v| v == DiceValues::Diamond)
            && value.clone().into_iter().any(|v| v == DiceValues::Silver)
        {
            // we found a greed!
            return (Some(DiceGroup::Greed), vec![]);
        }
    }

    if value.len() >= 4 {
        // we can probably find ... things.
        count_die(&value);
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[allow(dead_code)] // because they're randomly generated
enum DiceValues {
    Gold,
    Ruby,
    Emerald,
    Ebony,
    Diamond,
    Silver,
}

impl Display for DiceValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl AsRef<str> for DiceValues {
    fn as_ref(&self) -> &str {
        match self {
            DiceValues::Gold => "Gold",
            DiceValues::Ruby => "Ruby",
            DiceValues::Emerald => "Emerald",
            DiceValues::Ebony => "Ebony",
            DiceValues::Diamond => "Diamond",
            DiceValues::Silver => "Silver",
        }
    }
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

impl DiceValues {}

impl Distribution<DiceValues> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DiceValues {
        match rng.random_range(0..=5) {
            0 => DiceValues::Diamond,
            1 => DiceValues::Ebony,
            2 => DiceValues::Emerald,
            3 => DiceValues::Gold,
            4 => DiceValues::Ruby,
            5 => DiceValues::Silver,
            _ => panic!("impossible value!"),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Dice {
    value: Option<DiceValues>,
}

impl Dice {
    // returns a random dice value
    fn roll() -> Dice {
        Dice {
            value: Some(rand::random()),
        }
    }
}

#[derive(Debug, Clone)]
struct Player {
    pub name: &'static str,
    pub score: u32,
    // /// dice we're using for this turn
    // pub dice: Vec<Dice>,
    // /// dice we're currently holding back... this isn't right at the moment
    // pub held_dice: Vec<DiceGroup>,
}

/// get six fresh dice
fn new_dice() -> Vec<Dice> {
    let mut dice: Vec<Dice> = vec![];
    for _ in 0..=5 {
        dice.push(Dice::roll());
    }
    dice
}

impl Player {
    /// sets up a player's state for a new turn
    fn do_turn(self: &Player) {
        let dice = new_dice();

        let _held_dice: Vec<DiceGroup> = vec![];

        // first roll
        println!("first roll: {dice:?}");

        // loop through looking for results

        let dicevalues = dice
            .into_iter()
            .filter_map(|d| match d.value.is_some() {
                true => d.value,
                false => None,
            })
            .collect();
        let (group, leftovers) = find_groups(dicevalues);
        if group.is_some() {
            panic!("Found a group! {group:?} {leftovers:?}");
        }
        // println!("{:?}", res);
    }

    /// create a new player, for the start of the game
    fn new_with_name(name: &'static str) -> Self {
        Player { name, score: 0 }
    }
}

fn main() {
    // set up the list of players
    let mut players = vec![Player::new_with_name("Alice"), Player::new_with_name("Bob")];

    let mut final_round = false;
    let mut final_round_triggered_by: Option<usize> = None;
    let mut keep_playing = true;

    while keep_playing {
        let player_list = players.to_vec();

        // iterate through the players, letting them have a turn
        let player_iter = player_list.iter().enumerate();
        for (player_index, player) in player_iter {
            let mut rng = rand::rng();

            println!("It's {}'s turn", player.name);

            if final_round && final_round_triggered_by == Some(player_index) {
                println!(
                    "{} triggered the final round, so the game is over!",
                    player.name
                );
                keep_playing = false;
                break;
            }

            players[player_index].do_turn();

            let new_score: u32 = rng.random_range(0..3000);

            println!("Adding {} to {}", new_score, players[player_index].score);
            players[player_index].score += new_score;

            if player.score >= 5000 && final_round_triggered_by.is_none() {
                final_round = true;
                final_round_triggered_by = Some(player_index);
                println!("Final round triggered by {}", player.name);
            }
        }
        // | player| {
        //     player.new_turn();
        //     // ??
        // });

        // if players.iter().any(|p| p.score > 5000) {
        //     println!("it is was the last round!");
        //     break;
        // }

        // players have their turns
    }
    show_scoreboard(players);
}

fn show_scoreboard(players: Vec<Player>) {
    let mut temp_players = players;
    temp_players.sort_by_key(|k| k.score);
    // println!("{:?}", temp_players.reverse());
    temp_players.reverse();
    temp_players.clone().iter().for_each(|p| {
        println!("{}\t{}", p.clone().score, p.name);
    });
}

mod tests {

    #[test]
    pub fn test_for_greed_when_we_dont_have_greed() {
        use crate::{find_groups, DiceValues};
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
        use crate::{find_groups, DiceGroup, DiceValues};
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
