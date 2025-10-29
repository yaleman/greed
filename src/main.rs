use greed::*;

fn main() {
    // set up the list of players
    let mut players = vec![Player::new("Alice"), Player::new("Bob")];

    let mut final_round = false;
    let final_round_triggered_by: Option<usize> = None;
    let mut keep_playing = true;

    let mut round = 0;

    while keep_playing {
        round += 1;
        let player_list = players.to_vec();

        // iterate through the players, letting them have a turn
        let player_iter = player_list.iter().enumerate();
        for (player_index, player) in player_iter {
            println!("It's {}'s turn", player.name);

            if final_round && final_round_triggered_by == Some(player_index) {
                println!(
                    "{} triggered the final round, so the game is over!",
                    player.name
                );
                keep_playing = false;
                break;
            }

            if players[player_index].do_turn() {
                // scoring logic would go here
                println!(
                    "{} scored this turn! stopping at round {round}",
                    player.name
                );
                final_round = true;
            };
        }
        if final_round {
            keep_playing = false;
        }
    }
    show_scoreboard(players);
}

fn show_scoreboard(players: Vec<Player>) {
    let mut temp_players = players;
    println!("\nScoreboard:");
    temp_players.sort_by_key(|k| k.score);
    // println!("{:?}", temp_players.reverse());
    temp_players.reverse();
    temp_players.clone().iter().for_each(|p| {
        println!("{}\t{}", p.clone().score, p.name);
    });
}
