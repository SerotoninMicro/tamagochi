use std::thread;
use std::time::{Duration, Instant};
use std::io::{self, Write};

#[derive(PartialEq)] 
enum FacialExpression {
    Calm,
    Happy,
    Sad,
    Confused,
    Surprised,
    Angry,
    Sleeping,
}

struct Tamagotchi {
    face: FacialExpression,
    memory: Vec<String>,
    pleasure_scale: i32,
    hp: i32,
    sleep_timer: Instant,
    light_on: bool,
}

impl Tamagotchi {
    fn new() -> Tamagotchi {
        Tamagotchi {
            face: FacialExpression::Calm,
            memory: Vec::new(),
            pleasure_scale: 0,
            hp: 10,
            sleep_timer: Instant::now(),
            light_on: true, 
        }
    }

    fn change_face(&mut self, new_face: FacialExpression) {
        self.face = new_face;
        self.display_face();
    }

    fn display_face(&self) {
        print!("\x1B[2J\x1B[1;1H"); 
        match self.face {
            FacialExpression::Calm => println!("(─‿‿─)"),
            FacialExpression::Happy => println!("o(≧▽≦)o"),
            FacialExpression::Sad => println!("(￣ヘ￣)"),
            FacialExpression::Confused => println!("╮(￣ω￣;)╭"),
            FacialExpression::Surprised => println!("ヽ(°〇°)ﾉ"),
            FacialExpression::Angry => println!("(╯°□°)╯︵ ┻━┻"),
            FacialExpression::Sleeping => println!("(－ω－) zzZ"),
        }
    }

    fn show_action_window(&self) {
        println!("Action Window:");
        println!("Press 'E' to eat.");
        if self.light_on {
            println!("Press 'L' to turn off the light.");
        } else {
            println!("Press 'L' to turn on the light.");
        }
        println!("Press 'X' to exit.");
    }

    fn eat(&mut self) {
        println!("Tamagotchi is eating...");
        self.change_face(FacialExpression::Happy);
        thread::sleep(Duration::from_secs(3));
        self.change_face(FacialExpression::Calm);
        self.pleasure_scale = (self.pleasure_scale + 1).min(10);
    }

    fn toggle_light(&mut self) {
        self.light_on = !self.light_on; 
        if self.light_on {
            println!("The light is turned on.");
            if self.face == FacialExpression::Sleeping {
                self.change_face(FacialExpression::Calm); 
            }
        } else {
            println!("The light is turned off. Tamagotchi will sleep.");
            if self.face != FacialExpression::Sleeping {
                self.change_face(FacialExpression::Sleeping);
            }
        }
    }

    fn check_hp(&self) {
        println!("HP: {}", self.hp);
    }

    fn restore_hp(&mut self) {
        if self.sleep_timer.elapsed().as_secs() >= 20 {
            self.hp = (self.hp + 1).min(10);
            println!("Tamagotchi restored 1 HP.");
            self.sleep_timer = Instant::now();
        } else {
            println!("Tamagotchi is not sleeping or hasn't slept for 20 seconds yet.");
        }
    }

    fn display_pleasure_scale(&self) {
        println!("Pleasure Scale:");
        let filled_divisions = self.pleasure_scale;
        let empty_divisions = 10 - filled_divisions;

        print!("┌");
        for _ in 0..filled_divisions {
            print!("▓");
        }
        for _ in 0..empty_divisions {
            print!("░");
        }
        println!("┐");
    }

    fn process_action(&mut self, action: &str) {
        if self.memory.len() >= 3 && self.memory.iter().all(|a| a == action) {
            self.change_face(FacialExpression::Angry);
            self.pleasure_scale = (self.pleasure_scale - 1).max(0);
        } else {
            match action {
                "E" | "e" => {
                    self.eat();
                    self.memory.push(String::from(action));
                },
                "L" | "l" => self.toggle_light(),
                "H" | "h" => self.check_hp(),
                _ => println!("Invalid action!"),
            }
        }
    }
}

fn main() {
    let mut tamagotchi = Tamagotchi::new();

loop {
        tamagotchi.display_face();
        tamagotchi.show_action_window();
        tamagotchi.check_hp();
        tamagotchi.display_pleasure_scale(); 

        let mut input = String::new();
        print!("Enter your action: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "X" | "x" => {
                println!("Exiting Tamagotchi application.");
                break; 
            },
            action => tamagotchi.process_action(action),
        }

        if !tamagotchi.light_on && tamagotchi.face == FacialExpression::Sleeping {
            tamagotchi.restore_hp();
        }

        thread::sleep(Duration::from_secs(2));
    }
}