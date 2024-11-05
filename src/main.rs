use rand::thread_rng;
use rand::Rng;

#[derive(Clone)]
struct AutoMaton<'a> {
    cur:&'a str
}

impl AutoMaton<'static> {
    fn new() -> AutoMaton<'static> {
        AutoMaton {
            cur: "し"
        }
    }
}

impl Iterator for AutoMaton<'static> {
    type Item = AutoMaton<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = thread_rng();
        match self.cur {
            "し" => {
                if rng.gen_bool(1.0/2.0) {
                    self.cur = "か";
                } else {
                    self.cur = "た";
                }
            },
            "か" => {
                self.cur = "の";
            },
            "の" => {
                self.cur = "こ";
            },
            "こ" => {
                if rng.gen_bool(1.0/2.0) {
                    self.cur = "の";
                } else if thread_rng().gen_bool(1.0/2.0) {
                    self.cur = "し"
                }
            },
            "た" => {
                self.cur = "ん";
            },
            "ん" => {
                if rng.gen_bool(1.0/2.0) {
                    self.cur = "　"
                } else {
                    self.cur = "た"
                }
            },
            "　" => {
                if rng.gen_bool(1.0/2.0) {
                    self.cur = "し"
                }
            },
            _ => {
                unreachable!();
            }
        }
        return Some(self.clone());
    }
}

fn main() {
    let mut buf = String::new();
    let auto = AutoMaton::new();
    const TARGET_STR:&str = "しかのこのこのここしたんたん　";
    print!("{}", auto.cur);

    /* markov chain!!! */
    for a in auto {
        if buf.len() >= TARGET_STR.len() {
            if buf == TARGET_STR {
                break;
            }
            buf.remove(0);
        }

        print!("{}", a.cur);
        buf.push_str(a.cur);
    }
    println!("");
}
