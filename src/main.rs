use romanconv::roman_num::single_flow_with_input;

fn main() -> Result<(), anyhow::Error> {

    let mut counter = 1;
    let mut correct = 0;
    let mut incorrect = 0;

    println!("(=^_^=) ROMAN NUMBER TRANSLATION GAME!! (=^_^=) by Tanuki & Neko");
    println!("To quit the game, enter ctrl + c.");

    loop {
        print!("[{}]", counter);
        let (rez, ans) = single_flow_with_input()?;
        if rez {
            correct += 1;
            println!("CORRECT!");
        } else {
            incorrect += 1;
            println!("Incorrect .. correct answer is {}", &ans);
        }
        println!("Results so far - total:{}, correct:{}, incorrect:{}", counter, correct, incorrect);
        counter += 1;
    }

    Ok(())
}
