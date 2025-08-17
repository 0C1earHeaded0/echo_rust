const HELP_STR: &str = "
RUST ECHO HELP
-------------------------------
#USAGE

echo_rust [-n|-e] [expr [expr...]]
-------------------------------
#KEYS

-n      Disable auto \\n
-e      Allow escape caracters
-------------------------------
";

pub fn show_help() {
    print!("{HELP_STR}");
}