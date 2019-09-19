mod lib;
use lib::G;

fn main() {
    let mut g = G::new("0, 1, [, ]", "0", {
        let mut rules = Vec::new();
        rules.push("1 -> 11");
        rules.push("0 -> 1[0]0");

        rules
    });

    println!("\n0th generation:");
    println!("{:#?}", g.current_generation());

    g.next_generation();
    println!("\n1st generation:");
    println!("{:#?}", g.current_generation());

    g.next_generation();
    println!("\n2nd generation:");
    println!("{:#?}", g.current_generation());

    g.next_generation();
    println!("\n3rd generation:");
    println!("{:#?}", g.current_generation());

    g.advance_by(3);   

    // width, height, filename
    g.draw(1000, 1000, "output.png");
}
