#![no_std] // 標準ライブラリを使用しない
#![no_main] // 通常のエントリポイントを使用しない

use core::panic::PanicInfo;

mod vga_buffer;

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // この関数の名前修飾をしない
pub extern "C" fn _start() -> ! {
    // リンカはデフォルトで `_start` という名前の関数を探すので、
    // この関数がエントリポイントとなる
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop{}
}
