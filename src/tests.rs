#[test]
pub fn check_for_greed_when_we_dont_have_greed() {
    use crate::{find_groups, DiceValue};
    let dicewehave = vec![
        DiceValue::Gold,
        DiceValue::Gold,
        DiceValue::Ebony,
        DiceValue::Emerald,
        DiceValue::Diamond,
        DiceValue::Silver,
    ];

    assert_eq!(find_groups(dicewehave.clone()), (Vec::new(), dicewehave));
}

#[test]
pub fn check_for_greed_when_we_have_greed() {
    use crate::{find_groups, DiceGroup, DiceValue};
    let dicewehave = vec![
        DiceValue::Gold,
        DiceValue::Ruby,
        DiceValue::Ebony,
        DiceValue::Emerald,
        DiceValue::Diamond,
        DiceValue::Silver,
    ];

    let (res, leftovers) = find_groups(dicewehave);
    if !leftovers.is_empty() {
        panic!("Should get empty leftovers");
    }

    assert_eq!(res, vec![DiceGroup::Greed]);
}

#[test]
fn count_die() {
    use crate::*;
    let dice_holding = vec![DiceValue::Ruby, DiceValue::Ruby, DiceValue::Ruby];
    println!("holding={:?}", count_die(&dice_holding));
    assert_eq!(dice_holding.len(), 3);
}
