use dip;

fn main() {
    let bundestag = dip::new();

    for i in bundestag.aktivitaeten() {
        println!("{}", i.id);
    };
}
