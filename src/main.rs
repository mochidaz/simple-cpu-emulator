mod cpu;
mod mem;

use cpu::CPU;

fn main() {
    let mut cpu = CPU::new();
    cpu.run("instruction-set.txt");
}
