use eazing::bezier::Bezier;
use eazing::Easing;

fn main() {
  let bezier = Easing::CubicBezier(Bezier::Curve(0.17, 0.67, 0.83, 0.67));

  println!("\ncubic-bezier:end\n");
  println!("progress = {}", bezier.y(0.01));
  println!("progress = {}", bezier.y(0.234));
  println!("progress = {}", bezier.y(0.446));
  println!("progress = {}", bezier.y(0.608));
  println!("progress = {}", bezier.y(0.706));
  println!("progress = {}", bezier.y(0.827));
  println!("progress = {}", bezier.y(1.0));
  println!("\ncubic-bezier:end\n");
}
