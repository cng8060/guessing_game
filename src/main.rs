use std::io;
use std::io::Write;

fn play_game() {
    use text_io::read;
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let ai_num: i32 = rng.gen_range(0..101);
    let mut usr_num: i32 = -1;
    let mut usr_play = 'y';

    println!("Bet ya can't guess the number I'm thinking of! It's between 0 and 100!");

    while usr_play != 'n' {
        if usr_play != 'y' {
            println!("I'm sorry, I don't understand {}", usr_play);
            println!("Please enter again! (Y/N): ");
            usr_play = read!();
            usr_play = usr_play.to_ascii_lowercase();
            continue;
        } else {
            while ai_num != usr_num {
                print!("Your Guess: ");
                io::stdout().flush().unwrap();
                usr_num = read!();

                if usr_num == ai_num {
                    println!("Good job, it was {}!", ai_num);
                } else {
                    if usr_num > ai_num {
                        println!("Nope, it's lower than {}!", usr_num);
                    } else {
                        println!("Nope, it's higher than {}!", usr_num);
                    }
                }
            }
        }

        usr_num = -1;
        println!("Goob job, do you wanna play again? (Y/N): ");
        usr_play = read!();
        usr_play = usr_play.to_ascii_lowercase();
    }
}

fn main() {
    play_game();

    println!("Great playing with you, bye!");
}
