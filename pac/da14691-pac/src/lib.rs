/*
DISCLAIMER
This software is supplied by Renesas Electronics Corporation and is only intended for use with Renesas products.
No other uses are authorized. This software is owned by Renesas Electronics Corporation and is protected under all
applicable laws, including copyright laws.
THIS SOFTWARE IS PROVIDED "AS IS" AND RENESAS MAKES NO WARRANTIES REGARDING THIS SOFTWARE, WHETHER EXPRESS, IMPLIED
OR STATUTORY, INCLUDING BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NON-INFRINGEMENT.  ALL SUCH WARRANTIES ARE EXPRESSLY DISCLAIMED.TO THE MAXIMUM EXTENT PERMITTED NOT PROHIBITED BY
LAW, NEITHER RENESAS ELECTRONICS CORPORATION NOR ANY OF ITS AFFILIATED COMPANIES SHALL BE LIABLE FOR ANY DIRECT,
INDIRECT, SPECIAL, INCIDENTAL OR CONSEQUENTIAL DAMAGES FOR ANY REASON RELATED TO THIS SOFTWARE, EVEN IF RENESAS OR
ITS AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.
Renesas reserves the right, without notice, to make changes to this software and to discontinue the availability
of this software. By using this software, you agree to the additional terms and conditions found by accessing the
following link:
http://www.renesas.com/disclaimer

*/
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:24 +0000
#![cfg_attr(not(feature = "tracing"), no_std)]
#![allow(non_camel_case_types)]
#![doc = "690"]
pub mod common;
pub use common::*;

#[cfg(feature = "tracing")]
pub mod reg_name;
#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "aes_hash")]
pub mod aes_hash;
#[cfg(feature = "anamisc")]
pub mod anamisc;
#[cfg(feature = "apu")]
pub mod apu;
#[cfg(feature = "cache")]
pub mod cache;
#[cfg(feature = "charger")]
pub mod charger;
#[cfg(feature = "chip_version")]
pub mod chip_version;
#[cfg(feature = "cmac")]
pub mod cmac;
#[cfg(feature = "cmac_timer_slp")]
pub mod cmac_timer_slp;
#[cfg(feature = "crg_com")]
pub mod crg_com;
#[cfg(feature = "crg_per")]
pub mod crg_per;
#[cfg(feature = "crg_sys")]
pub mod crg_sys;
#[cfg(feature = "crg_top")]
pub mod crg_top;
#[cfg(feature = "crg_xtal")]
pub mod crg_xtal;
#[cfg(feature = "dcdc")]
pub mod dcdc;
#[cfg(feature = "dma")]
pub mod dma;
#[cfg(feature = "dw")]
pub mod dw;
#[cfg(feature = "gpadc")]
pub mod gpadc;
#[cfg(feature = "gpio")]
pub mod gpio;
#[cfg(feature = "gpreg")]
pub mod gpreg;
#[cfg(feature = "i2c")]
pub mod i2c;
#[cfg(feature = "i2c2")]
pub mod i2c2;
#[cfg(feature = "lcdc")]
pub mod lcdc;
#[cfg(feature = "lra")]
pub mod lra;
#[cfg(feature = "memctrl")]
pub mod memctrl;
#[cfg(feature = "otpc")]
pub mod otpc;
#[cfg(feature = "pdc")]
pub mod pdc;
#[cfg(feature = "pwmled")]
pub mod pwmled;
#[cfg(feature = "qspic")]
pub mod qspic;
#[cfg(feature = "qspic2")]
pub mod qspic2;
#[cfg(feature = "rfmon")]
pub mod rfmon;
#[cfg(feature = "rtc")]
pub mod rtc;
#[cfg(feature = "sau")]
pub mod sau;
#[cfg(feature = "sdadc")]
pub mod sdadc;
#[cfg(feature = "smotor")]
pub mod smotor;
#[cfg(feature = "snc")]
pub mod snc;
#[cfg(feature = "spi")]
pub mod spi;
#[cfg(feature = "spi2")]
pub mod spi2;
#[cfg(feature = "sys_wdog")]
pub mod sys_wdog;
#[cfg(feature = "timer")]
pub mod timer;
#[cfg(feature = "timer2")]
pub mod timer2;
#[cfg(feature = "timer3")]
pub mod timer3;
#[cfg(feature = "timer4")]
pub mod timer4;
#[cfg(feature = "uart")]
pub mod uart;
#[cfg(feature = "uart2")]
pub mod uart2;
#[cfg(feature = "uart3")]
pub mod uart3;
#[cfg(feature = "usb")]
pub mod usb;
#[cfg(feature = "wakeup")]
pub mod wakeup;

#[cfg(feature = "sau")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sau {
    ptr: *mut u8,
}
#[cfg(feature = "sau")]
pub const SAU: self::Sau = self::Sau {
    ptr: 0xe000edd0u32 as _,
};
#[cfg(feature = "aes_hash")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AesHash {
    ptr: *mut u8,
}
#[cfg(feature = "aes_hash")]
pub const AES_HASH: self::AesHash = self::AesHash {
    ptr: 0x30040000u32 as _,
};
#[cfg(feature = "anamisc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anamisc {
    ptr: *mut u8,
}
#[cfg(feature = "anamisc")]
pub const ANAMISC: self::Anamisc = self::Anamisc {
    ptr: 0x50030b00u32 as _,
};
#[cfg(feature = "apu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apu {
    ptr: *mut u8,
}
#[cfg(feature = "apu")]
pub const APU: self::Apu = self::Apu {
    ptr: 0x50030600u32 as _,
};
#[cfg(feature = "cache")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cache {
    ptr: *mut u8,
}
#[cfg(feature = "cache")]
pub const CACHE: self::Cache = self::Cache {
    ptr: 0x100c0000u32 as _,
};
#[cfg(feature = "charger")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Charger {
    ptr: *mut u8,
}
#[cfg(feature = "charger")]
pub const CHARGER: self::Charger = self::Charger {
    ptr: 0x50040400u32 as _,
};
#[cfg(feature = "chip_version")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChipVersion {
    ptr: *mut u8,
}
#[cfg(feature = "chip_version")]
pub const CHIP_VERSION: self::ChipVersion = self::ChipVersion {
    ptr: 0x50040200u32 as _,
};
#[cfg(feature = "cmac")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmac {
    ptr: *mut u8,
}
#[cfg(feature = "cmac")]
pub const CMAC: self::Cmac = self::Cmac {
    ptr: 0x40000000u32 as _,
};
#[cfg(feature = "cmac_timer_slp")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmacTimerSlp {
    ptr: *mut u8,
}
#[cfg(feature = "cmac_timer_slp")]
pub const CMAC_TIMER_SLP: self::CmacTimerSlp = self::CmacTimerSlp {
    ptr: 0x50010400u32 as _,
};
#[cfg(feature = "crg_com")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgCom {
    ptr: *mut u8,
}
#[cfg(feature = "crg_com")]
pub const CRG_COM: self::CrgCom = self::CrgCom {
    ptr: 0x50020900u32 as _,
};
#[cfg(feature = "crg_per")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgPer {
    ptr: *mut u8,
}
#[cfg(feature = "crg_per")]
pub const CRG_PER: self::CrgPer = self::CrgPer {
    ptr: 0x50030c00u32 as _,
};
#[cfg(feature = "crg_sys")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgSys {
    ptr: *mut u8,
}
#[cfg(feature = "crg_sys")]
pub const CRG_SYS: self::CrgSys = self::CrgSys {
    ptr: 0x50040500u32 as _,
};
#[cfg(feature = "crg_top")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgTop {
    ptr: *mut u8,
}
#[cfg(feature = "crg_top")]
pub const CRG_TOP: self::CrgTop = self::CrgTop {
    ptr: 0x50000000u32 as _,
};
#[cfg(feature = "crg_xtal")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgXtal {
    ptr: *mut u8,
}
#[cfg(feature = "crg_xtal")]
pub const CRG_XTAL: self::CrgXtal = self::CrgXtal {
    ptr: 0x50010000u32 as _,
};
#[cfg(feature = "dcdc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdc {
    ptr: *mut u8,
}
#[cfg(feature = "dcdc")]
pub const DCDC: self::Dcdc = self::Dcdc {
    ptr: 0x50000300u32 as _,
};
#[cfg(feature = "dma")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
#[cfg(feature = "dma")]
pub const DMA: self::Dma = self::Dma {
    ptr: 0x50040800u32 as _,
};
#[cfg(feature = "dw")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dw {
    ptr: *mut u8,
}
#[cfg(feature = "dw")]
pub const DW: self::Dw = self::Dw {
    ptr: 0x30020000u32 as _,
};
#[cfg(feature = "gpadc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpadc {
    ptr: *mut u8,
}
#[cfg(feature = "gpadc")]
pub const GPADC: self::Gpadc = self::Gpadc {
    ptr: 0x50030900u32 as _,
};
#[cfg(feature = "gpio")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
#[cfg(feature = "gpio")]
pub const GPIO: self::Gpio = self::Gpio {
    ptr: 0x50020a00u32 as _,
};
#[cfg(feature = "gpreg")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpreg {
    ptr: *mut u8,
}
#[cfg(feature = "gpreg")]
pub const GPREG: self::Gpreg = self::Gpreg {
    ptr: 0x50040300u32 as _,
};
#[cfg(feature = "i2c")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C {
    ptr: *mut u8,
}
#[cfg(feature = "i2c")]
pub const I2C: self::I2C = self::I2C {
    ptr: 0x50020600u32 as _,
};
#[cfg(feature = "i2c2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2 {
    ptr: *mut u8,
}
#[cfg(feature = "i2c2")]
pub const I2C2: self::I2C2 = self::I2C2 {
    ptr: 0x50020700u32 as _,
};
#[cfg(feature = "lcdc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdc {
    ptr: *mut u8,
}
#[cfg(feature = "lcdc")]
pub const LCDC: self::Lcdc = self::Lcdc {
    ptr: 0x30030000u32 as _,
};
#[cfg(feature = "lra")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lra {
    ptr: *mut u8,
}
#[cfg(feature = "lra")]
pub const LRA: self::Lra = self::Lra {
    ptr: 0x50030a00u32 as _,
};
#[cfg(feature = "memctrl")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memctrl {
    ptr: *mut u8,
}
#[cfg(feature = "memctrl")]
pub const MEMCTRL: self::Memctrl = self::Memctrl {
    ptr: 0x50050000u32 as _,
};
#[cfg(feature = "otpc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otpc {
    ptr: *mut u8,
}
#[cfg(feature = "otpc")]
pub const OTPC: self::Otpc = self::Otpc {
    ptr: 0x30070000u32 as _,
};
#[cfg(feature = "pdc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdc {
    ptr: *mut u8,
}
#[cfg(feature = "pdc")]
pub const PDC: self::Pdc = self::Pdc {
    ptr: 0x50000200u32 as _,
};
#[cfg(feature = "pwmled")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwmled {
    ptr: *mut u8,
}
#[cfg(feature = "pwmled")]
pub const PWMLED: self::Pwmled = self::Pwmled {
    ptr: 0x50030500u32 as _,
};
#[cfg(feature = "qspic")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic {
    ptr: *mut u8,
}
#[cfg(feature = "qspic")]
pub const QSPIC: self::Qspic = self::Qspic {
    ptr: 0x38000000u32 as _,
};
#[cfg(feature = "qspic2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2 {
    ptr: *mut u8,
}
#[cfg(feature = "qspic2")]
pub const QSPIC2: self::Qspic2 = self::Qspic2 {
    ptr: 0x34000000u32 as _,
};
#[cfg(feature = "rfmon")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfmon {
    ptr: *mut u8,
}
#[cfg(feature = "rfmon")]
pub const RFMON: self::Rfmon = self::Rfmon {
    ptr: 0x50040600u32 as _,
};
#[cfg(feature = "rtc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
#[cfg(feature = "rtc")]
pub const RTC: self::Rtc = self::Rtc {
    ptr: 0x50000400u32 as _,
};
#[cfg(feature = "sdadc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadc {
    ptr: *mut u8,
}
#[cfg(feature = "sdadc")]
pub const SDADC: self::Sdadc = self::Sdadc {
    ptr: 0x50020800u32 as _,
};
#[cfg(feature = "smotor")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smotor {
    ptr: *mut u8,
}
#[cfg(feature = "smotor")]
pub const SMOTOR: self::Smotor = self::Smotor {
    ptr: 0x50030e00u32 as _,
};
#[cfg(feature = "snc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snc {
    ptr: *mut u8,
}
#[cfg(feature = "snc")]
pub const SNC: self::Snc = self::Snc {
    ptr: 0x50020c00u32 as _,
};
#[cfg(feature = "spi")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi {
    ptr: *mut u8,
}
#[cfg(feature = "spi")]
pub const SPI: self::Spi = self::Spi {
    ptr: 0x50020300u32 as _,
};
#[cfg(feature = "spi2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi2 {
    ptr: *mut u8,
}
#[cfg(feature = "spi2")]
pub const SPI2: self::Spi2 = self::Spi2 {
    ptr: 0x50020400u32 as _,
};
#[cfg(feature = "sys_wdog")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysWdog {
    ptr: *mut u8,
}
#[cfg(feature = "sys_wdog")]
pub const SYS_WDOG: self::SysWdog = self::SysWdog {
    ptr: 0x50000700u32 as _,
};
#[cfg(feature = "timer")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer {
    ptr: *mut u8,
}
#[cfg(feature = "timer")]
pub const TIMER: self::Timer = self::Timer {
    ptr: 0x50010200u32 as _,
};
#[cfg(feature = "timer2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2 {
    ptr: *mut u8,
}
#[cfg(feature = "timer2")]
pub const TIMER2: self::Timer2 = self::Timer2 {
    ptr: 0x50010300u32 as _,
};
#[cfg(feature = "timer3")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3 {
    ptr: *mut u8,
}
#[cfg(feature = "timer3")]
pub const TIMER3: self::Timer3 = self::Timer3 {
    ptr: 0x50040a00u32 as _,
};
#[cfg(feature = "timer4")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4 {
    ptr: *mut u8,
}
#[cfg(feature = "timer4")]
pub const TIMER4: self::Timer4 = self::Timer4 {
    ptr: 0x50040b00u32 as _,
};
#[cfg(feature = "uart")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart {
    ptr: *mut u8,
}
#[cfg(feature = "uart")]
pub const UART: self::Uart = self::Uart {
    ptr: 0x50020000u32 as _,
};
#[cfg(feature = "uart2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2 {
    ptr: *mut u8,
}
#[cfg(feature = "uart2")]
pub const UART2: self::Uart2 = self::Uart2 {
    ptr: 0x50020100u32 as _,
};
#[cfg(feature = "uart3")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart3 {
    ptr: *mut u8,
}
#[cfg(feature = "uart3")]
pub const UART3: self::Uart3 = self::Uart3 {
    ptr: 0x50020200u32 as _,
};
#[cfg(feature = "usb")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb {
    ptr: *mut u8,
}
#[cfg(feature = "usb")]
pub const USB: self::Usb = self::Usb {
    ptr: 0x50040000u32 as _,
};
#[cfg(feature = "wakeup")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeup {
    ptr: *mut u8,
}
#[cfg(feature = "wakeup")]
pub const WAKEUP: self::Wakeup = self::Wakeup {
    ptr: 0x50000100u32 as _,
};

pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, NVIC, SCB, SYST, TPIU};
#[doc = "Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub mod interrupt_handlers {
    unsafe extern "C" {
        pub fn SNC();
        pub fn DMA();
        pub fn CHARGER_STATE();
        pub fn CHARGER_ERROR();
        pub fn CMAC2SYS();
        pub fn UART();
        pub fn UART2();
        pub fn UART3();
        pub fn I2C();
        pub fn I2C2();
        pub fn SPI();
        pub fn SPI2();
        pub fn PCM();
        pub fn SRC_IN();
        pub fn SRC_OUT();
        pub fn USB();
        pub fn TIMER();
        pub fn TIMER2();
        pub fn RTC();
        pub fn KEY_WKUP_GPIO();
        pub fn PDC();
        pub fn VBUS();
        pub fn MRM();
        pub fn MOTOR_CONTROLLER();
        pub fn TRNG();
        pub fn DCDC();
        pub fn XTAL32M_RDY();
        pub fn GPADC();
        pub fn SDADC();
        pub fn CRYPTO();
        pub fn CAPTIMER();
        pub fn RFDIAG();
        pub fn LCD_CONTROLLER();
        pub fn PLL_LOCK();
        pub fn TIMER3();
        pub fn TIMER4();
        pub fn LRA();
        pub fn RTC_EVENT();
        pub fn GPIO_P0();
        pub fn GPIO_P1();
    }
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 40] = [
    Vector {
        _handler: interrupt_handlers::SNC,
    },
    Vector {
        _handler: interrupt_handlers::DMA,
    },
    Vector {
        _handler: interrupt_handlers::CHARGER_STATE,
    },
    Vector {
        _handler: interrupt_handlers::CHARGER_ERROR,
    },
    Vector {
        _handler: interrupt_handlers::CMAC2SYS,
    },
    Vector {
        _handler: interrupt_handlers::UART,
    },
    Vector {
        _handler: interrupt_handlers::UART2,
    },
    Vector {
        _handler: interrupt_handlers::UART3,
    },
    Vector {
        _handler: interrupt_handlers::I2C,
    },
    Vector {
        _handler: interrupt_handlers::I2C2,
    },
    Vector {
        _handler: interrupt_handlers::SPI,
    },
    Vector {
        _handler: interrupt_handlers::SPI2,
    },
    Vector {
        _handler: interrupt_handlers::PCM,
    },
    Vector {
        _handler: interrupt_handlers::SRC_IN,
    },
    Vector {
        _handler: interrupt_handlers::SRC_OUT,
    },
    Vector {
        _handler: interrupt_handlers::USB,
    },
    Vector {
        _handler: interrupt_handlers::TIMER,
    },
    Vector {
        _handler: interrupt_handlers::TIMER2,
    },
    Vector {
        _handler: interrupt_handlers::RTC,
    },
    Vector {
        _handler: interrupt_handlers::KEY_WKUP_GPIO,
    },
    Vector {
        _handler: interrupt_handlers::PDC,
    },
    Vector {
        _handler: interrupt_handlers::VBUS,
    },
    Vector {
        _handler: interrupt_handlers::MRM,
    },
    Vector {
        _handler: interrupt_handlers::MOTOR_CONTROLLER,
    },
    Vector {
        _handler: interrupt_handlers::TRNG,
    },
    Vector {
        _handler: interrupt_handlers::DCDC,
    },
    Vector {
        _handler: interrupt_handlers::XTAL32M_RDY,
    },
    Vector {
        _handler: interrupt_handlers::GPADC,
    },
    Vector {
        _handler: interrupt_handlers::SDADC,
    },
    Vector {
        _handler: interrupt_handlers::CRYPTO,
    },
    Vector {
        _handler: interrupt_handlers::CAPTIMER,
    },
    Vector {
        _handler: interrupt_handlers::RFDIAG,
    },
    Vector {
        _handler: interrupt_handlers::LCD_CONTROLLER,
    },
    Vector {
        _handler: interrupt_handlers::PLL_LOCK,
    },
    Vector {
        _handler: interrupt_handlers::TIMER3,
    },
    Vector {
        _handler: interrupt_handlers::TIMER4,
    },
    Vector {
        _handler: interrupt_handlers::LRA,
    },
    Vector {
        _handler: interrupt_handlers::RTC_EVENT,
    },
    Vector {
        _handler: interrupt_handlers::GPIO_P0,
    },
    Vector {
        _handler: interrupt_handlers::GPIO_P1,
    },
];
#[doc = "Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "Sensor Node Controller interrupt request."]
    SNC = 0,

    #[doc = "General Purpose DMA interrupt request."]
    DMA = 1,

    #[doc = "Charger State interrupt request."]
    CHARGER_STATE = 2,

    #[doc = "Charger Error interrupt request."]
    CHARGER_ERROR = 3,

    #[doc = "CMAC and mailbox interrupt request."]
    CMAC2SYS = 4,

    #[doc = "UART interrupt request."]
    UART = 5,

    #[doc = "UART2 interrupt request."]
    UART2 = 6,

    #[doc = "UART3 interrupt request."]
    UART3 = 7,

    #[doc = "I2C interrupt request."]
    I2C = 8,

    #[doc = "I2C2 interrupt request."]
    I2C2 = 9,

    #[doc = "SPI interrupt request."]
    SPI = 10,

    #[doc = "SPI2 interrupt request."]
    SPI2 = 11,

    #[doc = "PCM interrupt request."]
    PCM = 12,

    #[doc = "SRC input interrupt request."]
    SRC_IN = 13,

    #[doc = "SRC output interrupt request."]
    SRC_OUT = 14,

    #[doc = "USB interrupt request."]
    USB = 15,

    #[doc = "TIMER interrupt request."]
    TIMER = 16,

    #[doc = "TIMER2 interrupt request."]
    TIMER2 = 17,

    #[doc = "RTC interrupt request."]
    RTC = 18,

    #[doc = "Debounced button press interrupt request."]
    KEY_WKUP_GPIO = 19,

    #[doc = "Wakeup IRQ from PDC to CM33"]
    PDC = 20,

    #[doc = "VBUS presence interrupt request."]
    VBUS = 21,

    #[doc = "Cache Miss Rate Monitor interrupt request."]
    MRM = 22,

    #[doc = "MOTOR and mailbox interrupt request."]
    MOTOR_CONTROLLER = 23,

    #[doc = "True Random Number Generation interrupt request."]
    TRNG = 24,

    #[doc = "DCDC interrupt request."]
    DCDC = 25,

    #[doc = "XTAL32M trimmed and ready interrupt request."]
    XTAL32M_RDY = 26,

    #[doc = "General Purpose Analog-Digital Converter interrupt request."]
    GPADC = 27,

    #[doc = "Sigma Delta Analog-Digital Converter interrupt request."]
    SDADC = 28,

    #[doc = "Crypto interrupt request."]
    CRYPTO = 29,

    #[doc = "GPIO triggered Timer Capture interrupt request."]
    CAPTIMER = 30,

    #[doc = "Baseband or Radio Diagnostics interrupt request."]
    RFDIAG = 31,

    #[doc = "Parallel LCD Controller interrupt request."]
    LCD_CONTROLLER = 32,

    #[doc = "Pll lock interrupt request."]
    PLL_LOCK = 33,

    #[doc = "TIMER3 interrupt request."]
    TIMER3 = 34,

    #[doc = "TIMER4 interrupt request."]
    TIMER4 = 35,

    #[doc = "LRA/ERM interrupt request."]
    LRA = 36,

    #[doc = "RTC event interrupt request."]
    RTC_EVENT = 37,

    #[doc = "GPIO port 0 toggle interrupt request."]
    GPIO_P0 = 38,

    #[doc = "GPIO port 1 toggle interrupt request."]
    GPIO_P1 = 39,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[allow(non_snake_case)]
/// Required for compatibility with RTIC and other frameworks
pub struct Peripherals {
    #[cfg(feature = "sau")]
    pub SAU: self::Sau,
    #[cfg(feature = "aes_hash")]
    pub AES_HASH: self::AesHash,
    #[cfg(feature = "anamisc")]
    pub ANAMISC: self::Anamisc,
    #[cfg(feature = "apu")]
    pub APU: self::Apu,
    #[cfg(feature = "cache")]
    pub CACHE: self::Cache,
    #[cfg(feature = "charger")]
    pub CHARGER: self::Charger,
    #[cfg(feature = "chip_version")]
    pub CHIP_VERSION: self::ChipVersion,
    #[cfg(feature = "cmac")]
    pub CMAC: self::Cmac,
    #[cfg(feature = "cmac_timer_slp")]
    pub CMAC_TIMER_SLP: self::CmacTimerSlp,
    #[cfg(feature = "crg_com")]
    pub CRG_COM: self::CrgCom,
    #[cfg(feature = "crg_per")]
    pub CRG_PER: self::CrgPer,
    #[cfg(feature = "crg_sys")]
    pub CRG_SYS: self::CrgSys,
    #[cfg(feature = "crg_top")]
    pub CRG_TOP: self::CrgTop,
    #[cfg(feature = "crg_xtal")]
    pub CRG_XTAL: self::CrgXtal,
    #[cfg(feature = "dcdc")]
    pub DCDC: self::Dcdc,
    #[cfg(feature = "dma")]
    pub DMA: self::Dma,
    #[cfg(feature = "dw")]
    pub DW: self::Dw,
    #[cfg(feature = "gpadc")]
    pub GPADC: self::Gpadc,
    #[cfg(feature = "gpio")]
    pub GPIO: self::Gpio,
    #[cfg(feature = "gpreg")]
    pub GPREG: self::Gpreg,
    #[cfg(feature = "i2c")]
    pub I2C: self::I2C,
    #[cfg(feature = "i2c2")]
    pub I2C2: self::I2C2,
    #[cfg(feature = "lcdc")]
    pub LCDC: self::Lcdc,
    #[cfg(feature = "lra")]
    pub LRA: self::Lra,
    #[cfg(feature = "memctrl")]
    pub MEMCTRL: self::Memctrl,
    #[cfg(feature = "otpc")]
    pub OTPC: self::Otpc,
    #[cfg(feature = "pdc")]
    pub PDC: self::Pdc,
    #[cfg(feature = "pwmled")]
    pub PWMLED: self::Pwmled,
    #[cfg(feature = "qspic")]
    pub QSPIC: self::Qspic,
    #[cfg(feature = "qspic2")]
    pub QSPIC2: self::Qspic2,
    #[cfg(feature = "rfmon")]
    pub RFMON: self::Rfmon,
    #[cfg(feature = "rtc")]
    pub RTC: self::Rtc,
    #[cfg(feature = "sdadc")]
    pub SDADC: self::Sdadc,
    #[cfg(feature = "smotor")]
    pub SMOTOR: self::Smotor,
    #[cfg(feature = "snc")]
    pub SNC: self::Snc,
    #[cfg(feature = "spi")]
    pub SPI: self::Spi,
    #[cfg(feature = "spi2")]
    pub SPI2: self::Spi2,
    #[cfg(feature = "sys_wdog")]
    pub SYS_WDOG: self::SysWdog,
    #[cfg(feature = "timer")]
    pub TIMER: self::Timer,
    #[cfg(feature = "timer2")]
    pub TIMER2: self::Timer2,
    #[cfg(feature = "timer3")]
    pub TIMER3: self::Timer3,
    #[cfg(feature = "timer4")]
    pub TIMER4: self::Timer4,
    #[cfg(feature = "uart")]
    pub UART: self::Uart,
    #[cfg(feature = "uart2")]
    pub UART2: self::Uart2,
    #[cfg(feature = "uart3")]
    pub UART3: self::Uart3,
    #[cfg(feature = "usb")]
    pub USB: self::Usb,
    #[cfg(feature = "wakeup")]
    pub WAKEUP: self::Wakeup,
}

impl Peripherals {
    /// Returns Peripheral struct multiple times
    /// Required for compatibility with RTIC and other frameworks
    #[inline]
    pub fn take() -> Option<Self> {
        Some(Self::steal())
    }

    /// Returns Peripheral struct multiple times
    /// Required for compatibility with RTIC and other frameworks
    #[inline]
    pub fn steal() -> Self {
        Peripherals {
            #[cfg(feature = "sau")]
            SAU: crate::SAU,
            #[cfg(feature = "aes_hash")]
            AES_HASH: crate::AES_HASH,
            #[cfg(feature = "anamisc")]
            ANAMISC: crate::ANAMISC,
            #[cfg(feature = "apu")]
            APU: crate::APU,
            #[cfg(feature = "cache")]
            CACHE: crate::CACHE,
            #[cfg(feature = "charger")]
            CHARGER: crate::CHARGER,
            #[cfg(feature = "chip_version")]
            CHIP_VERSION: crate::CHIP_VERSION,
            #[cfg(feature = "cmac")]
            CMAC: crate::CMAC,
            #[cfg(feature = "cmac_timer_slp")]
            CMAC_TIMER_SLP: crate::CMAC_TIMER_SLP,
            #[cfg(feature = "crg_com")]
            CRG_COM: crate::CRG_COM,
            #[cfg(feature = "crg_per")]
            CRG_PER: crate::CRG_PER,
            #[cfg(feature = "crg_sys")]
            CRG_SYS: crate::CRG_SYS,
            #[cfg(feature = "crg_top")]
            CRG_TOP: crate::CRG_TOP,
            #[cfg(feature = "crg_xtal")]
            CRG_XTAL: crate::CRG_XTAL,
            #[cfg(feature = "dcdc")]
            DCDC: crate::DCDC,
            #[cfg(feature = "dma")]
            DMA: crate::DMA,
            #[cfg(feature = "dw")]
            DW: crate::DW,
            #[cfg(feature = "gpadc")]
            GPADC: crate::GPADC,
            #[cfg(feature = "gpio")]
            GPIO: crate::GPIO,
            #[cfg(feature = "gpreg")]
            GPREG: crate::GPREG,
            #[cfg(feature = "i2c")]
            I2C: crate::I2C,
            #[cfg(feature = "i2c2")]
            I2C2: crate::I2C2,
            #[cfg(feature = "lcdc")]
            LCDC: crate::LCDC,
            #[cfg(feature = "lra")]
            LRA: crate::LRA,
            #[cfg(feature = "memctrl")]
            MEMCTRL: crate::MEMCTRL,
            #[cfg(feature = "otpc")]
            OTPC: crate::OTPC,
            #[cfg(feature = "pdc")]
            PDC: crate::PDC,
            #[cfg(feature = "pwmled")]
            PWMLED: crate::PWMLED,
            #[cfg(feature = "qspic")]
            QSPIC: crate::QSPIC,
            #[cfg(feature = "qspic2")]
            QSPIC2: crate::QSPIC2,
            #[cfg(feature = "rfmon")]
            RFMON: crate::RFMON,
            #[cfg(feature = "rtc")]
            RTC: crate::RTC,
            #[cfg(feature = "sdadc")]
            SDADC: crate::SDADC,
            #[cfg(feature = "smotor")]
            SMOTOR: crate::SMOTOR,
            #[cfg(feature = "snc")]
            SNC: crate::SNC,
            #[cfg(feature = "spi")]
            SPI: crate::SPI,
            #[cfg(feature = "spi2")]
            SPI2: crate::SPI2,
            #[cfg(feature = "sys_wdog")]
            SYS_WDOG: crate::SYS_WDOG,
            #[cfg(feature = "timer")]
            TIMER: crate::TIMER,
            #[cfg(feature = "timer2")]
            TIMER2: crate::TIMER2,
            #[cfg(feature = "timer3")]
            TIMER3: crate::TIMER3,
            #[cfg(feature = "timer4")]
            TIMER4: crate::TIMER4,
            #[cfg(feature = "uart")]
            UART: crate::UART,
            #[cfg(feature = "uart2")]
            UART2: crate::UART2,
            #[cfg(feature = "uart3")]
            UART3: crate::UART3,
            #[cfg(feature = "usb")]
            USB: crate::USB,
            #[cfg(feature = "wakeup")]
            WAKEUP: crate::WAKEUP,
        }
    }
}
