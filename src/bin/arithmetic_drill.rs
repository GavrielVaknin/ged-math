use std::io;

fn main() {
    let rounds = 10;
    let mut score = 0;

    for round in 1..=rounds {
        let (question, correct) = make_problem();
        println!("Problem {round}: What is {question}?");

        let answer = read_number();

        if answer == correct {
            println!("Correct!");
            score += 1;
        } else {
            println!("Wrong. The answer is {correct}.");
        }
    }
    
    println!("You got {score} out of {rounds}.");
}

fn make_problem() -> (String, i32) {
    let a: i32 = rand::random_range(1..=10);
    let b: i32 = rand::random_range(1..=10);

    match rand::random_range(0..4) {
        0 => (format!("{a} + {b}"), a + b),
        1 => {
            let big = a.max(b);
            let small = a.min(b);
            (format!("{big} - {small}"), big - small)
        }
        2 => (format!("{a} * {b}"), a * b),
        _ => (format!("{} / {b}", a * b), a),
    }
}

fn read_number() -> i32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Fail to read line");

        match input.trim().parse() {
            Ok(number) => return number,
            Err(_) => println!("That's not a whole number. Try again:"),
        }
    }
}

