use dip;

fn main() {
    println!("Hello, world!");

    let bundestag = dip::new();

    for i in bundestag.aktivitaeten().take(2) {
        println!("{}", i);
    };
}
