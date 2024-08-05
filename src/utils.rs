pub mod color {
    const RESET: &str = "\x1b[0m";
    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const YELLOW: &str = "\x1b[33m";
    const BLUE: &str = "\x1b[34m";
    const MAGENTA: &str = "\x1b[35m";
    const CYAN: &str = "\x1b[36m";
    const WHITE: &str = "\x1b[37m";
    const BOLD: &str = "\x1b[1m";
    const UNDERLINE: &str = "\x1b[4m";
    const REVERSED: &str = "\x1b[7m";

    macro_rules! color {
        ($name:ident, $color:expr) => {
            pub fn $name(s: &str) -> String {
                format!("{}{}{}", $color, s, RESET)
            }
        };
    }

    color!(red, RED);
    color!(green, GREEN);
    color!(yellow, YELLOW);
    color!(blue, BLUE);
    color!(magenta, MAGENTA);
    color!(cyan, CYAN);
    color!(white, WHITE);
    color!(bold, BOLD);
    color!(underline, UNDERLINE);
    color!(reversed, REVERSED);
}
