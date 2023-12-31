mod mmix;

fn main() {
    #[allow(unused_mut)]
    let mut my_computer = mmix::MmixMachine::new();
    println!("{:?}", my_computer.gp_regs.len());
    println!("{:?}", my_computer.sp_regs.len());
    println!("{:?}", mmix::MEMORY_SIZE);

    {
        let slice = unsafe { std::slice::from_raw_parts_mut(my_computer.memory, mmix::MEMORY_SIZE as usize)};
        slice[100] = 1;
    }

    let slice = unsafe { std::slice::from_raw_parts_mut(my_computer.memory, mmix::MEMORY_SIZE as usize)};
    println!("{}", slice[99]);
    println!("{}", slice[100]);
    println!("{}", slice[101]);
}
