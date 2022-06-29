#[macro_use]
extern crate lazy_static;

use rand::distributions::Uniform;

lazy_static! {
    static ref UNI:Uniform<i32> = Uniform::from(1..5001);
}

pub mod roman_num {

    use chrono::{Local, DateTime};
    use super::UNI;
    use numerals::roman::Roman;
    use serde_derive::*;
    use std::fs::File;
    use std::io::{stdin, Write, Read};
    use regex::Regex;

    use rand::distributions::Distribution;

    pub fn single_flow_with_input() -> Result<(bool, String), anyhow::Error> {

        let mut rng = rand::thread_rng();
        let n = UNI.sample(&mut rng).to_string();
        print!("Input Roman Number for {} :", &n);
        std::io::stdout().flush()?;
        let translated = translate(n.as_str())?;
        let input_roman = take_romannum_input()?;
        let rez = input_roman == translated;
        Ok((rez, translated))
    }

    pub fn single_flow_auto() -> Result<(), anyhow::Error> {

        let mut rng = rand::thread_rng();
        let n = UNI.sample(&mut rng).to_string();
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
        Ok(buf.trim().to_string())
    }

    pub fn validate(s:&str) -> bool {

        let re = Regex::new(r"^[IVXLCDM]*$").unwrap();
        re.is_match(s) && s.len() > 0
    }

    pub fn load_history(fpath:&str) -> Result<ResultHistory, anyhow::Error> {
        // println!("@load_history - {}", fpath);
        let mut fp = match File::open(fpath) {
            Ok(fp) => fp,
            Err(_) => {
                match File::create(fpath) {
                    Ok(fp) => fp,
                    Err(e) => panic!("{:?}", e),
                }
            }
        };
        let mut buf:String = String::new();
        fp.read_to_string(&mut buf)?;
        let result_history:ResultHistory = serde_json::from_str(buf.as_str())?;
        Ok(result_history)
    }

    pub fn persist_history(fpath:&str, result:ResultHistory) -> Result<(), anyhow::Error> {
        // println!("@persist_history - {}", fpath);
        let mut fp = match File::options().read(true).write(true).open(fpath) {
            Ok(fp) => fp,
            Err(_) => {
                match File::create(fpath) {
                    Ok(fp) => fp,
                    Err(e) => panic!("{:?}", e),
                }
            }
        };
        let buf:String = serde_json::to_string(&result)?;
        write!(fp, "{}\n", buf)?;
        Ok(())
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResultHistory {
        pub total_correct:u32,
        pub total_incorrect:u32,
        pub total_rounds:u32,
        pub last_update:DateTime<Local>,
    }

    impl ResultHistory {
        pub fn init() -> ResultHistory {
            ResultHistory { 
                total_correct: 0,
                total_incorrect: 0,
                total_rounds: 0,
                last_update: Local::now(),
            }
        }
    }

    impl ToString for ResultHistory {
        fn to_string(&self) -> String {
            format!("total # of correct answer:{}\ntotal # of incorrect answer:{}\ntotal # of rounds you played:{}\nlast time you played:{}",
            self.total_correct, self.total_incorrect, self.total_rounds,
            self.last_update.format("%Y-%m-%d %H:%M:%S").to_string()
        )    
        }
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

    #[test]
    fn test_load_store_history() {
        use chrono::prelude::*;
        let fpath = "/tmp/test_romanconv_hist";
        let testdate = NaiveDateTime::parse_from_str("2022-03-01T10:00:20", "%Y-%m-%dT%H:%M:%S").unwrap();
        let testlocal_date = Local.from_local_datetime(&testdate).unwrap();
        let test_history = ResultHistory {
            total_correct:25,
            total_incorrect:18,
            total_rounds:43,
            last_update:testlocal_date,
        };
        persist_history(fpath, test_history).unwrap();
        let rez = load_history(fpath).unwrap();
        assert_eq!(25, rez.total_correct);
        assert_eq!(18, rez.total_incorrect);
        assert_eq!(43, rez.total_rounds);
        assert_eq!("total # of correct answer:25\ntotal # of incorrect answer:18\ntotal # of rounds you played:43\nlast time you played:2022-03-01 10:00:20", rez.to_string());
        assert_eq!("2022-03-01T10:00:20", rez.last_update.format("%Y-%m-%dT%H:%M:%S").to_string());
        std::fs::remove_file(fpath).unwrap();
    }

}