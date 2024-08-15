pub fn cycle64() -> u64 {
    riscv::register::cycle::read64()
}

pub fn cycle() -> usize {
    riscv::register::cycle::read()
}