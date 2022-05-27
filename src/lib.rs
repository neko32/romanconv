#[macro_use]
extern crate lazy_static;

use rand::distributions::Uniform;

lazy_static! {

    static ref uni:Uniform<i32> = Uniform::from(1..5001);

}

pub mod roman_num {

    use super::uni;
    use numerals::roman::Roman;
    use std::io::stdin;
    use std::io::Write;
    use regex::Regex;

    use rand::distributions::Distribution;

    pub fn single_flow_auto() -> Result<(), anyhow::Error> {

        let mut rng = rand::thread_rng();
        let n = uni.sample(&mut rng).to_string();
        println!("generated number as string is {}", &n);
        let translated = translate(n.as_str())?;
        println!("translated to {}", &translated);

        Ok(())
    }

    pub fn translate(num_str:&str) -> Result<String, anyhow::Error> {
        let num = num_str.to_string().parse::<i16>()?;
        Ok(format!("{:X}", Roman::from(num)))
    }

    pub fn take_romannum_input() -> Result<String, anyhow::Error> {
        print!("input your answer: ");
        std::io::stdout().flush()?;
        let mut buf:String = String::new();
        let std_in = stdin();
        std_in.read_line(&mut buf)?;

        Ok(buf)
    }

    pub fn validate(s:&str) -> bool {

        let re = Regex::new(r"^[IVXLCDM]*$").unwrap();
        re.is_match(s) && s.len() > 0
    }

}


#[cfg(test)]
mod tester {

    use super::roman_num::*;

    #[test]
    fn test_validate_onlyvalid_ivl() {
        let s = "LIV";
        assert!(validate(s))
    }

    #[test] 
    fn test_validate_onlyvalid_mdc() {
        let s = "MDC";
        assert!(validate(s))
    }

    #[test]
    fn test_validate_onlyvalid_allromannum() {
        let s = "MDCLXVI";
        assert!(validate(s))
    }

    #[test]
    fn test_validate_empty_fail() {
        let s = "";
        assert!(!validate(s))
    }

    #[test]
    fn test_validate_roman_and_nonroman_char() {
        let s = "IVXJL";
        assert!(!validate(s))
    }

    #[test]
    fn test_validate_only_nonroman_char() {
        let s = "JWKOP018";
        assert!(!validate(s))
    }

    #[test]
    fn test_translate_minimum() -> Result<(), String> {
        let s = "1";
        match translate(s) {
            Ok(r) => {
                assert_eq!(r, "I".to_string());
                Ok(())
            },
            Err(e) => Err(e.to_string()),
        }
    }

    #[test]
    fn test_translate_maxmum() -> Result<(), String> {
        let s = "5000";
        match translate(s) {
            Ok(r) => {
                println!("{}", &r);
                assert_eq!(r, "MMMMM".to_string());
                Ok(())
            },
            Err(e) => Err(e.to_string()),
        }
    }

    #[test]
    fn test_translate_4999() -> Result<(), String> {
        let s = "4999";
        match translate(s) {
            Ok(r) => {
                println!("{}", &r);
                assert_eq!(r, "MMMMCMXCIX".to_string());
                Ok(())
            },
            Err(e) => Err(e.to_string()),
        }
    }

    #[test]
    fn test_translate_489() -> Result<(), String> {
        let s = "489";
        match translate(s) {
            Ok(r) => {
                println!("{}", &r);
                assert_eq!(r, "CDLXXXIX".to_string());
                Ok(())
            },
            Err(e) => Err(e.to_string()),
        }
    }

    #[test]
    fn test_single_flow() {
        match single_flow_auto() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

}