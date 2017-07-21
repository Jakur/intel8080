mod cpu_8080;
use cpu_8080::*;

fn main() {
    run();
}

fn run() {
    let mem_vec = vec![0,0,0,0,0,0,0,0,];
    let mut state = State8080::new(mem_vec);
    emulate(&mut state);
    println!("Hello world");

}

