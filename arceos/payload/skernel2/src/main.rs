#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    // core::arch::asm!(
    //     "li a0, 1",
    //     "addi a1, {a_1}",
    //     "li a2, 8",
    //     "call write",
    // );
    core::arch::asm!(
        "csrr a1, mhartid",
        "ld a0, 64(zero)",
        "li a7, 8",
        "ecall",
        options(noreturn)
    )
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
