use std::io;

struct Cat {
    health: u8,
    hunger: u8,
    energy: u8,
    silliness: i8,
}

impl Cat {
    fn status(&self) {
        println!("Rusty the Virtual Cat!\n");

        println!(r"   |\      _,,,---,,_");
        println!(r"   /,`.-'`'    -.  ;-;;,_");
        println!(r"  |,4-  ) )-,_..;\ (  `'-'");
        println!(r" '---''(_/--'  `-'\_)");
        println!();
        println!("Status:");
        println!("Health: {}%", self.health);
        println!("Hunger: {}%", self.hunger);
        println!("Energy: {}%", self.energy);
        println!("Silliness: {}%", self.silliness);
    }

    fn feed(&self) {
        println!("Feed the cat!\n"); // makes the cat tired

        println!(r"   __________________");
        println!(r"  / .,.,. ,.,., .,., \");
        println!(r" /\__________________/\");
        println!(r"/       rusty <3       \");
        println!(r"\______________________/");
        println!();

        println!("Rusty thought the food was delicious!");
        println!("Rusty got a bit more tired.");
    }

    fn bathe(&self) {
        println!("Bathe the cat!\n"); // makes the cat hungry

        println!(r"        /\_/\    0");
        println!(r"   0   ( o.o )  0");
        println!(r"/\_.0o__> ^ <___.o_/\");
        println!(r"\      malloc()     /");
        println!(r" \_________________/");
        println!(r" o|o             o|o");
        println!();

        println!("Rusty is feeling a lot cleaner!");
        println!("Rusty got a bit more hungry.");
    }

    fn nap(&self) {
        println!("Let the cat sleep!\n"); // makes the cat bored

        println!(r"             z");
        println!(r"               z");
        println!(r"       /\_/\  z");
        println!(r" ().  ( u.u )  .()");
        println!(r"(   \~~> ^ <~~/   )");
        println!(r" (_______________)");
        println!();

        println!("Rusty became well rested!");
        println!("Rusty got a bit more bored.");
    }

    fn play(&self) {
        println!("Play with the cat!\n"); // makes the cat dirty

        println!(r"                       \`*-.                    ");
        println!(r"                       )  _`-.                 ");
        println!(r"                       .  : `. .                ");
        println!(r"                      : _   '  \               ");
        println!(r"                      ; *` _.   `*-._          ");
        println!(r"                       `-.-'          `-.       ");
        println!(r"                         ;       `       `.     ");
        println!(r"                         :.       .        \    ");
        println!(r"                         . \  .   :   .-'   .   ");
        println!(r"   .~*^*~.              '  `+.;  ;  '      :   ");
        println!(r"  . ###$$$ .             :  '  |    ;       ;-. ");
        println!(r" : &&###$$$;             ; '   : :`-:     _.`* ;");
        println!(r"  . ##$$$ .\   /\     .*' /  .*' ; .*`- +'  `*' ");
        println!(r"   '~._.~'  \_-_/     `*-*   `*-*  `*-*'");
        println!();

        println!("Rusty had a lot of fun!");
        println!("Rusty got a bit more dirty.");
    }
}

fn main() {
    let cat1 = Cat {
        health: 100,
        hunger: 100,
        energy: 100,
        silliness: 100,
    };

    loop {
        println!("What would you like to do?\n");
        println!("1. Feed the cat.");
        println!("2. Bathe the cat.");
        println!("3. Let the cat nap.");
        println!("4. Play with the cat.\n");
        println!("5. Check the cat's status.");
        println!("6. Quit.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => cat1.feed(),
            2 => cat1.bathe(),
            3 => cat1.nap(),
            4 => cat1.play(),
            5 => cat1.status(),
            6 => break,
            _ => continue,
        };
    }
}
