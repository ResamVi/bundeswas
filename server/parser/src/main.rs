// use std::fs;
use nom::IResult;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::digit0;
use nom::combinator::map;
use nom::multi::{many0, many1};
use nom::sequence::{tuple, preceded};

use dip;

fn clean(text: &str) -> String {
    // Remove...
    text
        .replace("\n", " ") // ...newlines. Join words that were separated.
        .replace("  ", " ") // ..."holes". Words that are seperated by multiple spaces.
        .replace("- ", "") // ... hyphenated words. Words that were cut to fit on a line.
}

fn main() {
    let bundestag = dip::new();

    let x = bundestag.plenarprotokoll_text().skip(5).next().unwrap();
    let x = x.text.unwrap();
    let x = clean(&x);

    let (remain, _) = parse_title(&x).expect("could not parse title");
    // let (remain, _) = skip(&remain).expect("could not skip");


    let (remain, res) = parse_inhalt(&remain).expect("could not parse inhalt");

    // println!("{:?}", &remain[..500]);
    println!("{:?}", res);

    // let (remain, _) = parse_inhalt(remain).expect("could not parse inhalt");

    // println!("{:?}", &remain[..500]);

    // println!("{:?}", result);
    // fs::write("output.txt", clean(&content)).expect("could not write to file");
}

#[derive(Debug)]
#[allow(dead_code)]
struct Title {
    sitzungsnummer: i32,
    datum: String,
}

fn parse_title(s: &str) -> IResult<&str, Title> {
    map(
        tuple((
            preceded(tag("Deutscher Bundestag Stenografischer Bericht "), digit0::<&str, _>),
            preceded(tag(". Sitzung Berlin, "), take_until(" I n h a l t : ")),
            tag(" I n h a l t : ")
        )),
        |(number, date, _)|{
            Title{
                sitzungsnummer: number.parse().unwrap(), 
                datum: date.to_string()
            }
        }
    )(s)
}


fn skip(s: &str) -> IResult<&str, &str> {
    tag("Erweiterung und Abwicklung der Tagesordnung")(s)
}

fn parse_inhalt(s: &str) -> IResult<&str, (&str, Vec<&str>, &str)> {
    tuple((
        tag("Erweiterung und Abwicklung der Tagesordnung"),
        many1(tag(" .")),
        // tag(" . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . ."),
        tag(" 8357 A"),
    ))(s)
}
