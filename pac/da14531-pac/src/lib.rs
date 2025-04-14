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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:19 +0000
#![cfg_attr(not(feature = "tracing"), no_std)]
#![allow(non_camel_case_types)]
#![doc = "DA14531"]
pub mod common;
pub use common::*;

#[cfg(feature = "tracing")]
pub mod reg_name;
#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "adplldig")]
pub mod adplldig;
#[cfg(feature = "anamisc")]
pub mod anamisc;
#[cfg(feature = "ble")]
pub mod ble;
#[cfg(feature = "chip_version")]
pub mod chip_version;
#[cfg(feature = "crg_aon")]
pub mod crg_aon;
#[cfg(feature = "crg_tim")]
pub mod crg_tim;
#[cfg(feature = "crg_top")]
pub mod crg_top;
#[cfg(feature = "gpadc")]
pub mod gpadc;
#[cfg(feature = "gpio")]
pub mod gpio;
#[cfg(feature = "gpreg")]
pub mod gpreg;
#[cfg(feature = "i2c")]
pub mod i2c;
#[cfg(feature = "kbrd")]
pub mod kbrd;
#[cfg(feature = "mbist_sram12")]
pub mod mbist_sram12;
#[cfg(feature = "mbist_sram3")]
pub mod mbist_sram3;
#[cfg(feature = "nvic")]
pub mod nvic;
#[cfg(feature = "otpc")]
pub mod otpc;
#[cfg(feature = "patch")]
pub mod patch;
#[cfg(feature = "quadec")]
pub mod quadec;
#[cfg(feature = "rfcu")]
pub mod rfcu;
#[cfg(feature = "rfcu_power")]
pub mod rfcu_power;
#[cfg(feature = "rfmon")]
pub mod rfmon;
#[cfg(feature = "rtc")]
pub mod rtc;
#[cfg(feature = "scb")]
pub mod scb;
#[cfg(feature = "spi")]
pub mod spi;
#[cfg(feature = "sys_wdog")]
pub mod sys_wdog;
#[cfg(feature = "systick")]
pub mod systick;
#[cfg(feature = "timer0")]
pub mod timer0;
#[cfg(feature = "timer1")]
pub mod timer1;
#[cfg(feature = "uart")]
pub mod uart;
#[cfg(feature = "uart2")]
pub mod uart2;
#[cfg(feature = "wkup")]
pub mod wkup;

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
#[cfg(feature = "adplldig")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adplldig {
    ptr: *mut u8,
}
#[cfg(feature = "adplldig")]
pub const ADPLLDIG: self::Adplldig = self::Adplldig {
    ptr: 0x40003000u32 as _,
};
#[cfg(feature = "anamisc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anamisc {
    ptr: *mut u8,
}
#[cfg(feature = "anamisc")]
pub const ANAMISC: self::Anamisc = self::Anamisc {
    ptr: 0x50001600u32 as _,
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
#[cfg(feature = "chip_version")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChipVersion {
    ptr: *mut u8,
}
#[cfg(feature = "chip_version")]
pub const CHIP_VERSION: self::ChipVersion = self::ChipVersion {
    ptr: 0x50003200u32 as _,
};
#[cfg(feature = "crg_aon")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgAon {
    ptr: *mut u8,
}
#[cfg(feature = "crg_aon")]
pub const CRG_AON: self::CrgAon = self::CrgAon {
    ptr: 0x50000300u32 as _,
};
#[cfg(feature = "crg_tim")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgTim {
    ptr: *mut u8,
}
#[cfg(feature = "crg_tim")]
pub const CRG_TIM: self::CrgTim = self::CrgTim {
    ptr: 0x50004200u32 as _,
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
#[cfg(feature = "gpadc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpadc {
    ptr: *mut u8,
}
#[cfg(feature = "gpadc")]
pub const GPADC: self::Gpadc = self::Gpadc {
    ptr: 0x50001500u32 as _,
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
    ptr: 0x50001300u32 as _,
};
#[cfg(feature = "kbrd")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kbrd {
    ptr: *mut u8,
}
#[cfg(feature = "kbrd")]
pub const KBRD: self::Kbrd = self::Kbrd {
    ptr: 0x50001400u32 as _,
};
#[cfg(feature = "mbist_sram12")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbistSram12 {
    ptr: *mut u8,
}
#[cfg(feature = "mbist_sram12")]
pub const MBIST_SRAM12: self::MbistSram12 = self::MbistSram12 {
    ptr: 0x50003700u32 as _,
};
#[cfg(feature = "mbist_sram3")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbistSram3 {
    ptr: *mut u8,
}
#[cfg(feature = "mbist_sram3")]
pub const MBIST_SRAM3: self::MbistSram3 = self::MbistSram3 {
    ptr: 0x50003800u32 as _,
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
#[cfg(feature = "patch")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Patch {
    ptr: *mut u8,
}
#[cfg(feature = "patch")]
pub const PATCH: self::Patch = self::Patch {
    ptr: 0x40080000u32 as _,
};
#[cfg(feature = "quadec")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Quadec {
    ptr: *mut u8,
}
#[cfg(feature = "quadec")]
pub const QUADEC: self::Quadec = self::Quadec {
    ptr: 0x50000200u32 as _,
};
#[cfg(feature = "rfcu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfcu {
    ptr: *mut u8,
}
#[cfg(feature = "rfcu")]
pub const RFCU: self::Rfcu = self::Rfcu {
    ptr: 0x40001000u32 as _,
};
#[cfg(feature = "rfcu_power")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfcuPower {
    ptr: *mut u8,
}
#[cfg(feature = "rfcu_power")]
pub const RFCU_POWER: self::RfcuPower = self::RfcuPower {
    ptr: 0x40001200u32 as _,
};
#[cfg(feature = "rfmon")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfmon {
    ptr: *mut u8,
}
#[cfg(feature = "rfmon")]
pub const RFMON: self::Rfmon = self::Rfmon {
    ptr: 0x50003500u32 as _,
};
#[cfg(feature = "rtc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
#[cfg(feature = "rtc")]
pub const RTC: self::Rtc = self::Rtc {
    ptr: 0x50004100u32 as _,
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
#[cfg(feature = "sys_wdog")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysWdog {
    ptr: *mut u8,
}
#[cfg(feature = "sys_wdog")]
pub const SYS_WDOG: self::SysWdog = self::SysWdog {
    ptr: 0x50003100u32 as _,
};
#[cfg(feature = "timer0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0 {
    ptr: *mut u8,
}
#[cfg(feature = "timer0")]
pub const TIMER0: self::Timer0 = self::Timer0 {
    ptr: 0x50003400u32 as _,
};
#[cfg(feature = "timer1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1 {
    ptr: *mut u8,
}
#[cfg(feature = "timer1")]
pub const TIMER1: self::Timer1 = self::Timer1 {
    ptr: 0x50004000u32 as _,
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
#[cfg(feature = "wkup")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wkup {
    ptr: *mut u8,
}
#[cfg(feature = "wkup")]
pub const WKUP: self::Wkup = self::Wkup {
    ptr: 0x50000100u32 as _,
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
    #[cfg(feature = "adplldig")]
    pub ADPLLDIG: self::Adplldig,
    #[cfg(feature = "anamisc")]
    pub ANAMISC: self::Anamisc,
    #[cfg(feature = "ble")]
    pub BLE: self::Ble,
    #[cfg(feature = "chip_version")]
    pub CHIP_VERSION: self::ChipVersion,
    #[cfg(feature = "crg_aon")]
    pub CRG_AON: self::CrgAon,
    #[cfg(feature = "crg_tim")]
    pub CRG_TIM: self::CrgTim,
    #[cfg(feature = "crg_top")]
    pub CRG_TOP: self::CrgTop,
    #[cfg(feature = "gpadc")]
    pub GPADC: self::Gpadc,
    #[cfg(feature = "gpio")]
    pub GPIO: self::Gpio,
    #[cfg(feature = "gpreg")]
    pub GPREG: self::Gpreg,
    #[cfg(feature = "i2c")]
    pub I2C: self::I2C,
    #[cfg(feature = "kbrd")]
    pub KBRD: self::Kbrd,
    #[cfg(feature = "mbist_sram12")]
    pub MBIST_SRAM12: self::MbistSram12,
    #[cfg(feature = "mbist_sram3")]
    pub MBIST_SRAM3: self::MbistSram3,
    #[cfg(feature = "otpc")]
    pub OTPC: self::Otpc,
    #[cfg(feature = "patch")]
    pub PATCH: self::Patch,
    #[cfg(feature = "quadec")]
    pub QUADEC: self::Quadec,
    #[cfg(feature = "rfcu")]
    pub RFCU: self::Rfcu,
    #[cfg(feature = "rfcu_power")]
    pub RFCU_POWER: self::RfcuPower,
    #[cfg(feature = "rfmon")]
    pub RFMON: self::Rfmon,
    #[cfg(feature = "rtc")]
    pub RTC: self::Rtc,
    #[cfg(feature = "spi")]
    pub SPI: self::Spi,
    #[cfg(feature = "sys_wdog")]
    pub SYS_WDOG: self::SysWdog,
    #[cfg(feature = "timer0")]
    pub TIMER0: self::Timer0,
    #[cfg(feature = "timer1")]
    pub TIMER1: self::Timer1,
    #[cfg(feature = "uart")]
    pub UART: self::Uart,
    #[cfg(feature = "uart2")]
    pub UART2: self::Uart2,
    #[cfg(feature = "wkup")]
    pub WKUP: self::Wkup,
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
            #[cfg(feature = "adplldig")]
            ADPLLDIG: crate::ADPLLDIG,
            #[cfg(feature = "anamisc")]
            ANAMISC: crate::ANAMISC,
            #[cfg(feature = "ble")]
            BLE: crate::BLE,
            #[cfg(feature = "chip_version")]
            CHIP_VERSION: crate::CHIP_VERSION,
            #[cfg(feature = "crg_aon")]
            CRG_AON: crate::CRG_AON,
            #[cfg(feature = "crg_tim")]
            CRG_TIM: crate::CRG_TIM,
            #[cfg(feature = "crg_top")]
            CRG_TOP: crate::CRG_TOP,
            #[cfg(feature = "gpadc")]
            GPADC: crate::GPADC,
            #[cfg(feature = "gpio")]
            GPIO: crate::GPIO,
            #[cfg(feature = "gpreg")]
            GPREG: crate::GPREG,
            #[cfg(feature = "i2c")]
            I2C: crate::I2C,
            #[cfg(feature = "kbrd")]
            KBRD: crate::KBRD,
            #[cfg(feature = "mbist_sram12")]
            MBIST_SRAM12: crate::MBIST_SRAM12,
            #[cfg(feature = "mbist_sram3")]
            MBIST_SRAM3: crate::MBIST_SRAM3,
            #[cfg(feature = "otpc")]
            OTPC: crate::OTPC,
            #[cfg(feature = "patch")]
            PATCH: crate::PATCH,
            #[cfg(feature = "quadec")]
            QUADEC: crate::QUADEC,
            #[cfg(feature = "rfcu")]
            RFCU: crate::RFCU,
            #[cfg(feature = "rfcu_power")]
            RFCU_POWER: crate::RFCU_POWER,
            #[cfg(feature = "rfmon")]
            RFMON: crate::RFMON,
            #[cfg(feature = "rtc")]
            RTC: crate::RTC,
            #[cfg(feature = "spi")]
            SPI: crate::SPI,
            #[cfg(feature = "sys_wdog")]
            SYS_WDOG: crate::SYS_WDOG,
            #[cfg(feature = "timer0")]
            TIMER0: crate::TIMER0,
            #[cfg(feature = "timer1")]
            TIMER1: crate::TIMER1,
            #[cfg(feature = "uart")]
            UART: crate::UART,
            #[cfg(feature = "uart2")]
            UART2: crate::UART2,
            #[cfg(feature = "wkup")]
            WKUP: crate::WKUP,
        }
    }
}
