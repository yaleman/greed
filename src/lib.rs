#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::sync::LazyLock;

use rand::distr::StandardUniform;
use rand::prelude::*;

static GREED: LazyLock<HashSet<DiceValue>> = LazyLock::new(|| {
    HashSet::from_iter([
        DiceValue::Gold,
        DiceValue::Ruby,
        DiceValue::Emerald,
        DiceValue::Ebony,
        DiceValue::Diamond,
        DiceValue::Silver,
    ])
});

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DiceGroup {
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
        value: DiceValue,
    },
}

fn count_die(dice: &[DiceValue]) -> HashMap<DiceValue, u32> {
    let res = dice.iter().fold(
        HashMap::default(),
        |mut acc: HashMap<DiceValue, u32>, die| {
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

fn find_groups(value: Vec<DiceValue>) -> (Vec<DiceGroup>, Vec<DiceValue>) {
    // this allows us to go "we have these dice, is there a scoring group". here be dragons and headaches, least of which with my failure to spell.

    // we can find either greed or a six of a kind
    if value.len() == 6 {
        // look for six of a kind
        // look for greed
        let value_set: HashSet<DiceValue> = HashSet::from_iter(value.iter().cloned());
        if value_set == *GREED {
            // we found a greed!
            return (vec![DiceGroup::Greed], vec![]);
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
    (Vec::new(), value)
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]

pub enum DiceValue {
    Gold,
    Ruby,
    Ebony,
    Emerald,
    Diamond,
    Silver,
}

impl Display for DiceValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl AsRef<str> for DiceValue {
    fn as_ref(&self) -> &str {
        match self {
            DiceValue::Gold => "Gold",
            DiceValue::Ruby => "Ruby",
            DiceValue::Emerald => "Emerald",
            DiceValue::Ebony => "Ebony",
            DiceValue::Diamond => "Diamond",
            DiceValue::Silver => "Silver",
        }
    }
}

impl From<DiceValue> for u32 {
    /// returns the value of the individual dice in points, only applies for dice we can hold a single one of
    fn from(value: DiceValue) -> Self {
        match value {
            DiceValue::Gold => 50,
            DiceValue::Diamond => 100,
            _ => panic!("Uh.... no?"),
        }
    }
}

impl DiceValue {}

impl Distribution<DiceValue> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DiceValue {
        match rng.random_range(0..=5) {
            0 => DiceValue::Diamond,
            1 => DiceValue::Ebony,
            2 => DiceValue::Emerald,
            3 => DiceValue::Gold,
            4 => DiceValue::Ruby,
            5 => DiceValue::Silver,
            _ => panic!("impossible value!"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dice {
    value: Option<DiceValue>,
}

impl Dice {
    // returns a random dice value
    pub fn roll() -> Dice {
        Dice {
            value: Some(rand::random()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player {
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
    pub fn do_turn(self: &Player) {
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
        if !group.is_empty() {
            panic!("Found a group! {group:?} {leftovers:?}");
        }
        // println!("{:?}", res);
    }

    /// create a new player, for the start of the game
    pub fn new_with_name(name: &'static str) -> Self {
        Player { name, score: 0 }
    }
}
