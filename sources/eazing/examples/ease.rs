use eazing::{ease, Easing};

fn main() {
  let mut time = 0.0; // current time or progress.
  let duration = 4.0; // animation time.
  let mut p = 0.0;

  while time <= duration {
    // inside this loop until the time expires.
    p = ease(Box::new(Easing::InOutBack), time / duration, 0.0, 1.0); // interpolates "p" value from 0 to 1.

    println!("progress = {p}");

    time += 1.0; // adds one millisecond to the elapsed time..
  }

  println!("progress = {p}");
  println!("\nease:end\n");
}
