use riscv::register::sstatus::{self, Sstatus, SPP};
use bit_field::BitField;


fn main() {
    app_init_context();
}

fn app_init_context() {
    let mut sstatus = sstatus::read();
    sstatus.bits = 0x1;
    //println!("{:?}", sstatus.spp());
    /*
    match sstatus.spp() {
        SPP::Supervisor => { println!("Supervisor"); },
        SPP::User => { println!("SPP::User"); },
        //_ => { println!("spp error"); },

    }
    */
    /*
    unsafe {
        sstatus::set_spp(SPP::User);
    }
    */
}
