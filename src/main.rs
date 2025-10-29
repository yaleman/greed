use greed::*;
use rand::Rng;

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
