#![no_std]
#![no_main]

use riscv as _;
use flash_algorithm::*;
use rtt_target::{rprintln, rtt_init_print};

struct Algorithm;

algorithm!(Algorithm, {
    device_name: "r9a02g021",
    device_type: DeviceType::Onchip,
    flash_address: 0x0,
    flash_size: 0x20000,
    page_size: 0x800,
    empty_value: 0xFF,
    program_time_out: 100,
    erase_time_out: 100,
    sectors: [{
        size: 0x800,
        address: 0x0,
    }]
});

impl FlashAlgorithm for Algorithm {
    fn new(_address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        rtt_init_print!();
        rprintln!("Init");
        // TODO: Add setup code for the flash algorithm.
        Ok(Self)
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        rprintln!("Erase All");
        // TODO: Add code here that erases the entire flash.
        Err(ErrorCode::new(0x70d0).unwrap())
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        rprintln!("Erase sector addr:{}", addr);
        // TODO: Add code here that erases a page to flash.
        Ok(())
    }

    fn program_page(&mut self, addr: u32, data: &[u8]) -> Result<(), ErrorCode> {
        rprintln!("Program Page addr:{} size:{}", addr, data.len());
        // TODO: Add code here that writes a page to flash.
        Ok(())
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {
        // TODO: Add code here to uninitialize the flash algorithm.
    }
}
