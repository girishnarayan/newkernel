use anyhow::Result;
use std::process::Command;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

use core::panic::PanicInfo;
use core::arch::asm;

const GPIO_LED: u8= 23;

fn main() -> Result<()> {

       let mut cmd = Command::new("aktualizr");
    cmd.arg("update"); // Specify the aktualizr subcommand and arguments
    cmd.output();
    
    println!("Blinking an led on a {}.",DeviceInfo::new()?.model());

    let mut pin  = Gpio::new()?
    .get(GPIO_LED)?
    .into_output();


    loop{
        pin.toggle();
        thread::sleep(Duration::from_millis(500));
    }

    fn foo (i:i32) -> i32  {
    return i+1 ;
}

}

