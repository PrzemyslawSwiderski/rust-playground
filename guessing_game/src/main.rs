use rand::{Rng, RngCore, thread_rng};

const DEFAULT_MIN_NUM: i16 = 1;
const DEFAULT_MAX_NUM: i16 = 7;

struct GuessGame {
    rand_gen: Box<dyn RngCore>,
    min_num: i16,
    max_num: i16,
}

impl GuessGame {
    pub fn new(rand_gen: Box<dyn RngCore>) -> Self {
        return GuessGame {
            rand_gen,
            min_num: DEFAULT_MIN_NUM,
            max_num: DEFAULT_MAX_NUM,
        };
    }

    pub fn new_by_range(rand_gen: Box<dyn RngCore>, min_num: i16, max_num: i16) -> Self {
        return GuessGame {
            rand_gen,
            min_num,
            max_num,
        };
    }

    fn play_round(self, input: &str) -> Result<i16, String> {
        let parse_res = input.trim().parse::<i16>();
        if parse_res.is_err() {
            return Err(parse_res.unwrap_err().to_string());
        }

        let guessed_num = parse_res.unwrap();
        let mut generator = self.rand_gen;

        let secret_number: i16 = generator.gen_range(self.min_num..self.max_num);

        return if guessed_num != secret_number {
            Err(format!(
                "GAME OVER, You guessed = {}, correct number was = {}",
                guessed_num, secret_number
            ))
        } else {
            Ok(secret_number)
        };
    }
}

fn main() {
    use std::io::stdin;

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    stdin().read_line(&mut guess).expect("Failed to read line");

    let guess_game = GuessGame::new(Box::new(thread_rng()));

    let result = guess_game.play_round(&guess);
    if result.is_ok() {
        println!(
            "CONGRATS, YOU WON!!! Your lucky number is: {luckyNum}",
            luckyNum = result.unwrap()
        )
    } else {
        eprintln!("{msg}", msg = result.unwrap_err())
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::mock::StepRng;

    use crate::GuessGame;

    #[test]
    fn test_guess() {
        let generator = StepRng::new(0, 0); // mock generator (`gen_range` will always return min number)

        let test_input = "1";
        let game = GuessGame::new(Box::new(generator));
        let result = game.play_round(test_input);

        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_guess_different_min_num() {
        let generator = StepRng::new(0, 0); // mock generator (`gen_range` will always return min number)

        let test_input = "5";
        let game = GuessGame::new_by_range(Box::new(generator), 5, 7);
        let result = game.play_round(test_input);

        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn test_guess_with_new_line() {
        let generator = StepRng::new(0, 0); // mock generator (`gen_range` will always return min number)

        let test_input = "1\n \n";
        let game = GuessGame::new(Box::new(generator));
        let result = game.play_round(test_input);

        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_guess_wrong_guess() {
        let generator = StepRng::new(0, 0); // mock generator (`gen_range` will always return min number)

        let test_input = "9\n \n";
        let game = GuessGame::new(Box::new(generator));
        let result = game.play_round(test_input);

        assert_eq!(
            result.unwrap_err(),
            "GAME OVER, You guessed = 9, correct number was = 1"
        );
    }
}
