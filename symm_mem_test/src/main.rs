use shmem;
use std::mem;
use uname::uname;

fn main() {
    let node = uname().unwrap().nodename;

    shmem::init();

    {
        let me = shmem::my_pe();
        let n = shmem::n_pes();

        let mut dest = shmem::SymmMem::<i32>::new(1);

        *dest = 5;

        println!("{}: PE {:>6} dest = {:>6}", node, me, *dest);
    }

    shmem::finalize();
}