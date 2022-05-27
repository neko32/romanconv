use romanconv::roman_num::take_romannum_input;

fn main() -> Result<(), anyhow::Error> {

    let input_str:String = take_romannum_input()?;
    println!("{}", &input_str);

    Ok(())
}
