use dip;

fn main() {
    let bundestag = dip::new();

    // for i in bundestag.aktivitaeten().take(5) {
    //     println!("{}", i.id);
    // };

    for plenarprotokoll in bundestag.plenarprotokolle().skip(1).take(1) {
        println!("{}", plenarprotokoll);

        let vorgaenge = bundestag.vorgaenge(plenarprotokoll.id);
        for vorgang in vorgaenge.take(1) {
            println!("{}", vorgang);
        }
    };
}
