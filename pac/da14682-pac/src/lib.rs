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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:10 +0000
#![cfg_attr(not(feature = "tracing"), no_std)]
#![allow(non_camel_case_types)]
#![doc = "DA1468x"]
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
#[cfg(feature = "ble")]
pub mod ble;
#[cfg(feature = "cache")]
pub mod cache;
#[cfg(feature = "chip_version")]
pub mod chip_version;
#[cfg(feature = "coex")]
pub mod coex;
#[cfg(feature = "crg_per")]
pub mod crg_per;
#[cfg(feature = "crg_top")]
pub mod crg_top;
#[cfg(feature = "dcdc")]
pub mod dcdc;
#[cfg(feature = "dem")]
pub mod dem;
#[cfg(feature = "dma")]
pub mod dma;
#[cfg(feature = "ecc")]
pub mod ecc;
#[cfg(feature = "ftdf")]
pub mod ftdf;
#[cfg(feature = "gp_timers")]
pub mod gp_timers;
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
#[cfg(feature = "ir")]
pub mod ir;
#[cfg(feature = "kbscan")]
pub mod kbscan;
#[cfg(feature = "nvic")]
pub mod nvic;
#[cfg(feature = "otpc")]
pub mod otpc;
#[cfg(feature = "qspic")]
pub mod qspic;
#[cfg(feature = "quad")]
pub mod quad;
#[cfg(feature = "scb")]
pub mod scb;
#[cfg(feature = "spi")]
pub mod spi;
#[cfg(feature = "spi2")]
pub mod spi2;
#[cfg(feature = "systick")]
pub mod systick;
#[cfg(feature = "timer1")]
pub mod timer1;
#[cfg(feature = "trng")]
pub mod trng;
#[cfg(feature = "uart")]
pub mod uart;
#[cfg(feature = "uart2")]
pub mod uart2;
#[cfg(feature = "usb")]
pub mod usb;
#[cfg(feature = "wakeup")]
pub mod wakeup;
#[cfg(feature = "wdog")]
pub mod wdog;

#[cfg(feature = "nvic")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvic {
    ptr: *mut u8,
}
#[cfg(feature = "nvic")]
pub const NVIC: self::Nvic = self::Nvic {
    ptr: 0xe000e100u32 as _,
};
#[cfg(feature = "scb")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scb {
    ptr: *mut u8,
}
#[cfg(feature = "scb")]
pub const SCB: self::Scb = self::Scb {
    ptr: 0xe000ed00u32 as _,
};
#[cfg(feature = "systick")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysTick {
    ptr: *mut u8,
}
#[cfg(feature = "systick")]
pub const SYSTICK: self::SysTick = self::SysTick {
    ptr: 0xe000e010u32 as _,
};
#[cfg(feature = "aes_hash")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AesHash {
    ptr: *mut u8,
}
#[cfg(feature = "aes_hash")]
pub const AES_HASH: self::AesHash = self::AesHash {
    ptr: 0x40020000u32 as _,
};
#[cfg(feature = "anamisc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anamisc {
    ptr: *mut u8,
}
#[cfg(feature = "anamisc")]
pub const ANAMISC: self::Anamisc = self::Anamisc {
    ptr: 0x50001b00u32 as _,
};
#[cfg(feature = "apu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apu {
    ptr: *mut u8,
}
#[cfg(feature = "apu")]
pub const APU: self::Apu = self::Apu {
    ptr: 0x50004000u32 as _,
};
#[cfg(feature = "ble")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ble {
    ptr: *mut u8,
}
#[cfg(feature = "ble")]
pub const BLE: self::Ble = self::Ble {
    ptr: 0x40000000u32 as _,
};
#[cfg(feature = "cache")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cache {
    ptr: *mut u8,
}
#[cfg(feature = "cache")]
pub const CACHE: self::Cache = self::Cache {
    ptr: 0x400c3000u32 as _,
};
#[cfg(feature = "chip_version")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChipVersion {
    ptr: *mut u8,
}
#[cfg(feature = "chip_version")]
pub const CHIP_VERSION: self::ChipVersion = self::ChipVersion {
    ptr: 0x50003200u32 as _,
};
#[cfg(feature = "coex")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coex {
    ptr: *mut u8,
}
#[cfg(feature = "coex")]
pub const COEX: self::Coex = self::Coex {
    ptr: 0x50002f00u32 as _,
};
#[cfg(feature = "crg_per")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgPer {
    ptr: *mut u8,
}
#[cfg(feature = "crg_per")]
pub const CRG_PER: self::CrgPer = self::CrgPer {
    ptr: 0x50001c00u32 as _,
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
#[cfg(feature = "dcdc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdc {
    ptr: *mut u8,
}
#[cfg(feature = "dcdc")]
pub const DCDC: self::Dcdc = self::Dcdc {
    ptr: 0x50000080u32 as _,
};
#[cfg(feature = "dem")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dem {
    ptr: *mut u8,
}
#[cfg(feature = "dem")]
pub const DEM: self::Dem = self::Dem {
    ptr: 0x50002e00u32 as _,
};
#[cfg(feature = "dma")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
#[cfg(feature = "dma")]
pub const DMA: self::Dma = self::Dma {
    ptr: 0x50003500u32 as _,
};
#[cfg(feature = "ecc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecc {
    ptr: *mut u8,
}
#[cfg(feature = "ecc")]
pub const ECC: self::Ecc = self::Ecc {
    ptr: 0x50006000u32 as _,
};
#[cfg(feature = "ftdf")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftdf {
    ptr: *mut u8,
}
#[cfg(feature = "ftdf")]
pub const FTDF: self::Ftdf = self::Ftdf {
    ptr: 0x40080000u32 as _,
};
#[cfg(feature = "gp_timers")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpTimers {
    ptr: *mut u8,
}
#[cfg(feature = "gp_timers")]
pub const GP_TIMERS: self::GpTimers = self::GpTimers {
    ptr: 0x50003400u32 as _,
};
#[cfg(feature = "gpadc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpadc {
    ptr: *mut u8,
}
#[cfg(feature = "gpadc")]
pub const GPADC: self::Gpadc = self::Gpadc {
    ptr: 0x50001900u32 as _,
};
#[cfg(feature = "gpio")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
#[cfg(feature = "gpio")]
pub const GPIO: self::Gpio = self::Gpio {
    ptr: 0x50003000u32 as _,
};
#[cfg(feature = "gpreg")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpreg {
    ptr: *mut u8,
}
#[cfg(feature = "gpreg")]
pub const GPREG: self::Gpreg = self::Gpreg {
    ptr: 0x50003300u32 as _,
};
#[cfg(feature = "i2c")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C {
    ptr: *mut u8,
}
#[cfg(feature = "i2c")]
pub const I2C: self::I2C = self::I2C {
    ptr: 0x50001400u32 as _,
};
#[cfg(feature = "i2c2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2 {
    ptr: *mut u8,
}
#[cfg(feature = "i2c2")]
pub const I2C2: self::I2C2 = self::I2C2 {
    ptr: 0x50001500u32 as _,
};
#[cfg(feature = "ir")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ir {
    ptr: *mut u8,
}
#[cfg(feature = "ir")]
pub const IR: self::Ir = self::Ir {
    ptr: 0x50001700u32 as _,
};
#[cfg(feature = "kbscan")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kbscan {
    ptr: *mut u8,
}
#[cfg(feature = "kbscan")]
pub const KBSCAN: self::Kbscan = self::Kbscan {
    ptr: 0x50001600u32 as _,
};
#[cfg(feature = "otpc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otpc {
    ptr: *mut u8,
}
#[cfg(feature = "otpc")]
pub const OTPC: self::Otpc = self::Otpc {
    ptr: 0x7f40000u32 as _,
};
#[cfg(feature = "qspic")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic {
    ptr: *mut u8,
}
#[cfg(feature = "qspic")]
pub const QSPIC: self::Qspic = self::Qspic {
    ptr: 0xc000000u32 as _,
};
#[cfg(feature = "quad")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Quad {
    ptr: *mut u8,
}
#[cfg(feature = "quad")]
pub const QUAD: self::Quad = self::Quad {
    ptr: 0x50001a00u32 as _,
};
#[cfg(feature = "spi")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi {
    ptr: *mut u8,
}
#[cfg(feature = "spi")]
pub const SPI: self::Spi = self::Spi {
    ptr: 0x50001200u32 as _,
};
#[cfg(feature = "spi2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi2 {
    ptr: *mut u8,
}
#[cfg(feature = "spi2")]
pub const SPI2: self::Spi2 = self::Spi2 {
    ptr: 0x50001300u32 as _,
};
#[cfg(feature = "timer1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1 {
    ptr: *mut u8,
}
#[cfg(feature = "timer1")]
pub const TIMER1: self::Timer1 = self::Timer1 {
    ptr: 0x50000200u32 as _,
};
#[cfg(feature = "trng")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trng {
    ptr: *mut u8,
}
#[cfg(feature = "trng")]
pub const TRNG: self::Trng = self::Trng {
    ptr: 0x50005000u32 as _,
};
#[cfg(feature = "uart")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart {
    ptr: *mut u8,
}
#[cfg(feature = "uart")]
pub const UART: self::Uart = self::Uart {
    ptr: 0x50001000u32 as _,
};
#[cfg(feature = "uart2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2 {
    ptr: *mut u8,
}
#[cfg(feature = "uart2")]
pub const UART2: self::Uart2 = self::Uart2 {
    ptr: 0x50001100u32 as _,
};
#[cfg(feature = "usb")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb {
    ptr: *mut u8,
}
#[cfg(feature = "usb")]
pub const USB: self::Usb = self::Usb {
    ptr: 0x50001800u32 as _,
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
#[cfg(feature = "wdog")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdog {
    ptr: *mut u8,
}
#[cfg(feature = "wdog")]
pub const WDOG: self::Wdog = self::Wdog {
    ptr: 0x50003100u32 as _,
};

pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[allow(non_snake_case)]
/// Required for compatibility with RTIC and other frameworks
pub struct Peripherals {
    #[cfg(feature = "nvic")]
    pub NVIC: self::Nvic,
    #[cfg(feature = "scb")]
    pub SCB: self::Scb,
    #[cfg(feature = "systick")]
    pub SYSTICK: self::SysTick,
    #[cfg(feature = "aes_hash")]
    pub AES_HASH: self::AesHash,
    #[cfg(feature = "anamisc")]
    pub ANAMISC: self::Anamisc,
    #[cfg(feature = "apu")]
    pub APU: self::Apu,
    #[cfg(feature = "ble")]
    pub BLE: self::Ble,
    #[cfg(feature = "cache")]
    pub CACHE: self::Cache,
    #[cfg(feature = "chip_version")]
    pub CHIP_VERSION: self::ChipVersion,
    #[cfg(feature = "coex")]
    pub COEX: self::Coex,
    #[cfg(feature = "crg_per")]
    pub CRG_PER: self::CrgPer,
    #[cfg(feature = "crg_top")]
    pub CRG_TOP: self::CrgTop,
    #[cfg(feature = "dcdc")]
    pub DCDC: self::Dcdc,
    #[cfg(feature = "dem")]
    pub DEM: self::Dem,
    #[cfg(feature = "dma")]
    pub DMA: self::Dma,
    #[cfg(feature = "ecc")]
    pub ECC: self::Ecc,
    #[cfg(feature = "ftdf")]
    pub FTDF: self::Ftdf,
    #[cfg(feature = "gp_timers")]
    pub GP_TIMERS: self::GpTimers,
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
    #[cfg(feature = "ir")]
    pub IR: self::Ir,
    #[cfg(feature = "kbscan")]
    pub KBSCAN: self::Kbscan,
    #[cfg(feature = "otpc")]
    pub OTPC: self::Otpc,
    #[cfg(feature = "qspic")]
    pub QSPIC: self::Qspic,
    #[cfg(feature = "quad")]
    pub QUAD: self::Quad,
    #[cfg(feature = "spi")]
    pub SPI: self::Spi,
    #[cfg(feature = "spi2")]
    pub SPI2: self::Spi2,
    #[cfg(feature = "timer1")]
    pub TIMER1: self::Timer1,
    #[cfg(feature = "trng")]
    pub TRNG: self::Trng,
    #[cfg(feature = "uart")]
    pub UART: self::Uart,
    #[cfg(feature = "uart2")]
    pub UART2: self::Uart2,
    #[cfg(feature = "usb")]
    pub USB: self::Usb,
    #[cfg(feature = "wakeup")]
    pub WAKEUP: self::Wakeup,
    #[cfg(feature = "wdog")]
    pub WDOG: self::Wdog,
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
            #[cfg(feature = "nvic")]
            NVIC: crate::NVIC,
            #[cfg(feature = "scb")]
            SCB: crate::SCB,
            #[cfg(feature = "systick")]
            SYSTICK: crate::SYSTICK,
            #[cfg(feature = "aes_hash")]
            AES_HASH: crate::AES_HASH,
            #[cfg(feature = "anamisc")]
            ANAMISC: crate::ANAMISC,
            #[cfg(feature = "apu")]
            APU: crate::APU,
            #[cfg(feature = "ble")]
            BLE: crate::BLE,
            #[cfg(feature = "cache")]
            CACHE: crate::CACHE,
            #[cfg(feature = "chip_version")]
            CHIP_VERSION: crate::CHIP_VERSION,
            #[cfg(feature = "coex")]
            COEX: crate::COEX,
            #[cfg(feature = "crg_per")]
            CRG_PER: crate::CRG_PER,
            #[cfg(feature = "crg_top")]
            CRG_TOP: crate::CRG_TOP,
            #[cfg(feature = "dcdc")]
            DCDC: crate::DCDC,
            #[cfg(feature = "dem")]
            DEM: crate::DEM,
            #[cfg(feature = "dma")]
            DMA: crate::DMA,
            #[cfg(feature = "ecc")]
            ECC: crate::ECC,
            #[cfg(feature = "ftdf")]
            FTDF: crate::FTDF,
            #[cfg(feature = "gp_timers")]
            GP_TIMERS: crate::GP_TIMERS,
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
            #[cfg(feature = "ir")]
            IR: crate::IR,
            #[cfg(feature = "kbscan")]
            KBSCAN: crate::KBSCAN,
            #[cfg(feature = "otpc")]
            OTPC: crate::OTPC,
            #[cfg(feature = "qspic")]
            QSPIC: crate::QSPIC,
            #[cfg(feature = "quad")]
            QUAD: crate::QUAD,
            #[cfg(feature = "spi")]
            SPI: crate::SPI,
            #[cfg(feature = "spi2")]
            SPI2: crate::SPI2,
            #[cfg(feature = "timer1")]
            TIMER1: crate::TIMER1,
            #[cfg(feature = "trng")]
            TRNG: crate::TRNG,
            #[cfg(feature = "uart")]
            UART: crate::UART,
            #[cfg(feature = "uart2")]
            UART2: crate::UART2,
            #[cfg(feature = "usb")]
            USB: crate::USB,
            #[cfg(feature = "wakeup")]
            WAKEUP: crate::WAKEUP,
            #[cfg(feature = "wdog")]
            WDOG: crate::WDOG,
        }
    }
}
