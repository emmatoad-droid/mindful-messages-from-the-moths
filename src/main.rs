use std::time::{SystemTime, UNIX_EPOCH};

struct Moth {
    name: &'static str,
    message: &'static str,
    art: &'static str,
}

const MOTHS: [Moth; 8] = [
    Moth {
        name: "Luna Moth",
        message: "Begin softly. You don't need to glow loudly to be seen.",
        art: r#"
  .-.   .-.
 / o \ / o \
 \___/V\___/
    / | \
   /  |  \
  (   |   )
   ~  |  ~
      ~"#,
    },
    Moth {
        name: "Atlas Moth",
        message: "You are bigger than the day you're having. Take up your space.",
        art: r#"
 ,-~~-.  .-~~-,
/ (  ) \/ (  ) \
\  `--'||`--'  /
 `-.__ || __.-'
   /  \\||/  \\
   `-.' || '.-'
       ||"#,
    },
    Moth {
        name: "Peppered Moth",
        message: "Changing isn't losing yourself. Sometimes it's how you survive.",
        art: r#"
  \\     //
 .:::\_^_/:::.
 :.:.:\|/:.:.:
 ':.:.-|-.:.:''
   ':.:|:.:''
       |"#,
    },
    Moth {
        name: "Death's-head Hawkmoth",
        message: "Nothing lasts forever, so notice this moment. It's precious.",
        art: r#"
 ,-==-.  .-==-,
/=====\\//=====\\
\=====(x x)====/
 `-==-/||\\-==-'
      ||||
      ||||
       \/"#,
    },
    Moth {
        name: "Garden Tiger Moth",
        message: "Your boundaries are allowed to be loud.",
        art: r#"
 ,#@@#.  .#@@#,
 #@  @#\/#@  @#
 '#@@#' || '#@@#'
   '--./||\.--'
       ||
       ||"#,
    },
    Moth {
        name: "Emperor Moth",
        message: "Pause. Breathe in the weather you're in, even the wild bits.",
        art: r#"
 .-'''-.  .-'''-.
/ (@)   \\/  (@) \
\   .-'  ||  '-.  /
 '-.     ||     .-'
    '--._||_.--'"#,
    },
    Moth {
        name: "White Ermine",
        message: "Rest is allowed. Breathe out and soften.",
        art: r#"
  .--.  .--.
 /    \/    \
(  .   ||  . )
 \    /||\    /
  '--' || '--'
       ||"#,
    },
    Moth {
        name: "Lantern Moth",
        message: "Keep turning towards the light, even when you're tired.",
        art: r#"
      _
     (_)
    .-"-.
   |  *  |  \ /
   | \|/ |-(^)<
   |_____|  / \
    '-.-'"#,
    },
];

fn random_index(len: usize) -> usize {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock went backwards");
    // Scramble the nanoseconds a bit so the selection doesn't feel patterned.
    let mut x = now.as_nanos() as u64 ^ 0x9E37_79B9_7F4A_7C15;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    (x % len as u64) as usize
}

fn main() {
    let moth = &MOTHS[random_index(MOTHS.len())];

    println!("\n  * . MINDFUL MESSAGES FROM THE MOTHS . *\n");
    println!("{}", moth.art.trim_start_matches('\n'));
    println!("\n  ~ {} ~\n", moth.name);
    println!("  \"{}\"\n", moth.message);
}
