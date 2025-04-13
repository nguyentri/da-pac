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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:24:51 +0000
#![cfg_attr(not(feature = "tracing"), no_std)]
#![allow(non_camel_case_types)]
#![doc = "Ultra-Low power Bleutooth 4.2 SoC with Optimized Boot Time from Dialog Semiconductor"]
pub mod common;
pub use common::*;

#[cfg(feature = "tracing")]
pub mod reg_name;
#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "adc580_bif_nl01")]
pub mod adc580_bif_nl01;
#[cfg(feature = "anamisc580_nl01")]
pub mod anamisc580_nl01;
#[cfg(feature = "ble580_gr01")]
pub mod ble580_gr01;
#[cfg(feature = "chip_version")]
pub mod chip_version;
#[cfg(feature = "crg580_dcdc_nl01")]
pub mod crg580_dcdc_nl01;
#[cfg(feature = "crg580_nl01")]
pub mod crg580_nl01;
#[cfg(feature = "gpio580_ports_nl01")]
pub mod gpio580_ports_nl01;
#[cfg(feature = "i2c580_nl00")]
pub mod i2c580_nl00;
#[cfg(feature = "kbrd580_nl01")]
pub mod kbrd580_nl01;
#[cfg(feature = "nvic")]
pub mod nvic;
#[cfg(feature = "otpc580_gr01")]
pub mod otpc580_gr01;
#[cfg(feature = "quadec580_gr01")]
pub mod quadec580_gr01;
#[cfg(feature = "riscutil580_gpreg_nl01")]
pub mod riscutil580_gpreg_nl01;
#[cfg(feature = "riscutil580_wdog_nl00")]
pub mod riscutil580_wdog_nl00;
#[cfg(feature = "scb")]
pub mod scb;
#[cfg(feature = "spi443_nl00")]
pub mod spi443_nl00;
#[cfg(feature = "systick")]
pub mod systick;
#[cfg(feature = "tmr580_nl01")]
pub mod tmr580_nl01;
#[cfg(feature = "uart1")]
pub mod uart1;
#[cfg(feature = "uart2")]
pub mod uart2;
#[cfg(feature = "wkup580_nl01")]
pub mod wkup580_nl01;

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
#[cfg(feature = "adc580_bif_nl01")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc580BifNl01 {
    ptr: *mut u8,
}
#[cfg(feature = "adc580_bif_nl01")]
pub const ADC580_BIF_NL01: self::Adc580BifNl01 = self::Adc580BifNl01 {
    ptr: 0x50001500u32 as _,
};
#[cfg(feature = "anamisc580_nl01")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anamisc580Nl01 {
    ptr: *mut u8,
}
#[cfg(feature = "anamisc580_nl01")]
pub const ANAMISC580_NL01: self::Anamisc580Nl01 = self::Anamisc580Nl01 {
    ptr: 0x50001600u32 as _,
};
#[cfg(feature = "ble580_gr01")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ble580Gr01 {
    ptr: *mut u8,
}
#[cfg(feature = "ble580_gr01")]
pub const BLE580_GR01: self::Ble580Gr01 = self::Ble580Gr01 {
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
#[cfg(feature = "crg580_dcdc_nl01")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crg580DcdcNl01 {
    ptr: *mut u8,
}
#[cfg(feature = "crg580_dcdc_nl01")]
pub const CRG580_DCDC_NL01: self::Crg580DcdcNl01 = self::Crg580DcdcNl01 {
    ptr: 0x50000080u32 as _,
};
#[cfg(feature = "crg580_nl01")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crg580Nl01 {
    ptr: *mut u8,
}
#[cfg(feature = "crg580_nl01")]
pub const CRG580_NL01: self::Crg580Nl01 = self::Crg580Nl01 {
    ptr: 0x50000000u32 as _,
};
#[cfg(feature = "gpio580_ports_nl01")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio580PortsNl01 {
    ptr: *mut u8,
}
#[cfg(feature = "gpio580_ports_nl01")]
pub const GPIO580_PORTS_NL01: self::Gpio580PortsNl01 = self::Gpio580PortsNl01 {
    ptr: 0x50003000u32 as _,
};
#[cfg(feature = "i2c580_nl00")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C580Nl00 {
    ptr: *mut u8,
}
#[cfg(feature = "i2c580_nl00")]
pub const I2C580_NL00: self::I2C580Nl00 = self::I2C580Nl00 {
    ptr: 0x50001300u32 as _,
};
#[cfg(feature = "kbrd580_nl01")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kbrd580Nl01 {
    ptr: *mut u8,
}
#[cfg(feature = "kbrd580_nl01")]
pub const KBRD580_NL01: self::Kbrd580Nl01 = self::Kbrd580Nl01 {
    ptr: 0x50001400u32 as _,
};
#[cfg(feature = "otpc580_gr01")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otpc580Gr01 {
    ptr: *mut u8,
}
#[cfg(feature = "otpc580_gr01")]
pub const OTPC580_GR01: self::Otpc580Gr01 = self::Otpc580Gr01 {
    ptr: 0x40008000u32 as _,
};
#[cfg(feature = "quadec580_gr01")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Quadec580Gr01 {
    ptr: *mut u8,
}
#[cfg(feature = "quadec580_gr01")]
pub const QUADEC580_GR01: self::Quadec580Gr01 = self::Quadec580Gr01 {
    ptr: 0x50000200u32 as _,
};
#[cfg(feature = "riscutil580_gpreg_nl01")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Riscutil580GpregNl01 {
    ptr: *mut u8,
}
#[cfg(feature = "riscutil580_gpreg_nl01")]
pub const RISCUTIL580_GPREG_NL01: self::Riscutil580GpregNl01 = self::Riscutil580GpregNl01 {
    ptr: 0x50003300u32 as _,
};
#[cfg(feature = "riscutil580_wdog_nl00")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Riscutil580WdogNl00 {
    ptr: *mut u8,
}
#[cfg(feature = "riscutil580_wdog_nl00")]
pub const RISCUTIL580_WDOG_NL00: self::Riscutil580WdogNl00 = self::Riscutil580WdogNl00 {
    ptr: 0x50003100u32 as _,
};
#[cfg(feature = "spi443_nl00")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi443Nl00 {
    ptr: *mut u8,
}
#[cfg(feature = "spi443_nl00")]
pub const SPI443_NL00: self::Spi443Nl00 = self::Spi443Nl00 {
    ptr: 0x50001200u32 as _,
};
#[cfg(feature = "tmr580_nl01")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr580Nl01 {
    ptr: *mut u8,
}
#[cfg(feature = "tmr580_nl01")]
pub const TMR580_NL01: self::Tmr580Nl01 = self::Tmr580Nl01 {
    ptr: 0x50003400u32 as _,
};
#[cfg(feature = "uart1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart1 {
    ptr: *mut u8,
}
#[cfg(feature = "uart1")]
pub const UART1: self::Uart1 = self::Uart1 {
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
#[cfg(feature = "wkup580_nl01")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wkup580Nl01 {
    ptr: *mut u8,
}
#[cfg(feature = "wkup580_nl01")]
pub const WKUP580_NL01: self::Wkup580Nl01 = self::Wkup580Nl01 {
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
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
extern "C" {
    fn BLE_WAKEUP_LP();
    fn BLE_GEN();
    fn UART();
    fn UART2();
    fn I2C();
    fn SPI();
    fn ADC();
    fn KEYBRD();
    fn BLE_RF_DIAG();
    fn RF_CAL();
    fn GPIO0();
    fn GPIO1();
    fn GPIO2();
    fn GPIO3();
    fn GPIO4();
    fn SWTIM();
    fn WKUP_QUADEC();
    fn PCM();
    fn SRC_IN();
    fn SRC_OUT();
    fn DMA();
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 21] = [
    Vector {
        _handler: BLE_WAKEUP_LP,
    },
    Vector { _handler: BLE_GEN },
    Vector { _handler: UART },
    Vector { _handler: UART2 },
    Vector { _handler: I2C },
    Vector { _handler: SPI },
    Vector { _handler: ADC },
    Vector { _handler: KEYBRD },
    Vector {
        _handler: BLE_RF_DIAG,
    },
    Vector { _handler: RF_CAL },
    Vector { _handler: GPIO0 },
    Vector { _handler: GPIO1 },
    Vector { _handler: GPIO2 },
    Vector { _handler: GPIO3 },
    Vector { _handler: GPIO4 },
    Vector { _handler: SWTIM },
    Vector {
        _handler: WKUP_QUADEC,
    },
    Vector { _handler: PCM },
    Vector { _handler: SRC_IN },
    Vector { _handler: SRC_OUT },
    Vector { _handler: DMA },
];
#[doc = "Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "Wake-up from Low Power (Extended Sleep) interrupt from BLE"]
    BLE_WAKEUP_LP = 0,
    #[doc = "BLE Interrupt from various BLE sources."]
    BLE_GEN = 1,
    #[doc = "UART interrupt"]
    UART = 2,
    #[doc = "UART2 interrupt"]
    UART2 = 3,
    #[doc = "I2C interrupt"]
    I2C = 4,
    #[doc = "SPI interrupt"]
    SPI = 5,
    #[doc = "Analog-Digital Converter interrupt."]
    ADC = 6,
    #[doc = "Keyboard interrupt."]
    KEYBRD = 7,
    #[doc = "Baseband or Radio Diagnostics Interrupt"]
    BLE_RF_DIAG = 8,
    #[doc = "RF Calibration Interrupt"]
    RF_CAL = 9,
    #[doc = "GPIO0 interrupt through debounce"]
    GPIO0 = 10,
    #[doc = "GPIO1 interrupt through debounce"]
    GPIO1 = 11,
    #[doc = "GPIO2 interrupt through debounce"]
    GPIO2 = 12,
    #[doc = "GPIO3 interrupt through debounce"]
    GPIO3 = 13,
    #[doc = "GPIO4 interrupt through debounce"]
    GPIO4 = 14,
    #[doc = "Software Timer interrupt"]
    SWTIM = 15,
    #[doc = "Combines the Wake up Capture Timer interrupt, the GPIO interrupt and the QuadDecoder interrupt"]
    WKUP_QUADEC = 16,
    #[doc = "PCM interrupt"]
    PCM = 17,
    #[doc = "Sample rate converter input interrupt"]
    SRC_IN = 18,
    #[doc = "Sample rate converter output interrupt"]
    SRC_OUT = 19,
    #[doc = "DMA interrupt"]
    DMA = 20,
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
    #[cfg(feature = "nvic")]
    pub NVIC: self::Nvic,
    #[cfg(feature = "scb")]
    pub SCB: self::Scb,
    #[cfg(feature = "systick")]
    pub SYSTICK: self::SysTick,
    #[cfg(feature = "adc580_bif_nl01")]
    pub ADC580_BIF_NL01: self::Adc580BifNl01,
    #[cfg(feature = "anamisc580_nl01")]
    pub ANAMISC580_NL01: self::Anamisc580Nl01,
    #[cfg(feature = "ble580_gr01")]
    pub BLE580_GR01: self::Ble580Gr01,
    #[cfg(feature = "chip_version")]
    pub CHIP_VERSION: self::ChipVersion,
    #[cfg(feature = "crg580_dcdc_nl01")]
    pub CRG580_DCDC_NL01: self::Crg580DcdcNl01,
    #[cfg(feature = "crg580_nl01")]
    pub CRG580_NL01: self::Crg580Nl01,
    #[cfg(feature = "gpio580_ports_nl01")]
    pub GPIO580_PORTS_NL01: self::Gpio580PortsNl01,
    #[cfg(feature = "i2c580_nl00")]
    pub I2C580_NL00: self::I2C580Nl00,
    #[cfg(feature = "kbrd580_nl01")]
    pub KBRD580_NL01: self::Kbrd580Nl01,
    #[cfg(feature = "otpc580_gr01")]
    pub OTPC580_GR01: self::Otpc580Gr01,
    #[cfg(feature = "quadec580_gr01")]
    pub QUADEC580_GR01: self::Quadec580Gr01,
    #[cfg(feature = "riscutil580_gpreg_nl01")]
    pub RISCUTIL580_GPREG_NL01: self::Riscutil580GpregNl01,
    #[cfg(feature = "riscutil580_wdog_nl00")]
    pub RISCUTIL580_WDOG_NL00: self::Riscutil580WdogNl00,
    #[cfg(feature = "spi443_nl00")]
    pub SPI443_NL00: self::Spi443Nl00,
    #[cfg(feature = "tmr580_nl01")]
    pub TMR580_NL01: self::Tmr580Nl01,
    #[cfg(feature = "uart1")]
    pub UART1: self::Uart1,
    #[cfg(feature = "uart2")]
    pub UART2: self::Uart2,
    #[cfg(feature = "wkup580_nl01")]
    pub WKUP580_NL01: self::Wkup580Nl01,
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
            #[cfg(feature = "adc580_bif_nl01")]
            ADC580_BIF_NL01: crate::ADC580_BIF_NL01,
            #[cfg(feature = "anamisc580_nl01")]
            ANAMISC580_NL01: crate::ANAMISC580_NL01,
            #[cfg(feature = "ble580_gr01")]
            BLE580_GR01: crate::BLE580_GR01,
            #[cfg(feature = "chip_version")]
            CHIP_VERSION: crate::CHIP_VERSION,
            #[cfg(feature = "crg580_dcdc_nl01")]
            CRG580_DCDC_NL01: crate::CRG580_DCDC_NL01,
            #[cfg(feature = "crg580_nl01")]
            CRG580_NL01: crate::CRG580_NL01,
            #[cfg(feature = "gpio580_ports_nl01")]
            GPIO580_PORTS_NL01: crate::GPIO580_PORTS_NL01,
            #[cfg(feature = "i2c580_nl00")]
            I2C580_NL00: crate::I2C580_NL00,
            #[cfg(feature = "kbrd580_nl01")]
            KBRD580_NL01: crate::KBRD580_NL01,
            #[cfg(feature = "otpc580_gr01")]
            OTPC580_GR01: crate::OTPC580_GR01,
            #[cfg(feature = "quadec580_gr01")]
            QUADEC580_GR01: crate::QUADEC580_GR01,
            #[cfg(feature = "riscutil580_gpreg_nl01")]
            RISCUTIL580_GPREG_NL01: crate::RISCUTIL580_GPREG_NL01,
            #[cfg(feature = "riscutil580_wdog_nl00")]
            RISCUTIL580_WDOG_NL00: crate::RISCUTIL580_WDOG_NL00,
            #[cfg(feature = "spi443_nl00")]
            SPI443_NL00: crate::SPI443_NL00,
            #[cfg(feature = "tmr580_nl01")]
            TMR580_NL01: crate::TMR580_NL01,
            #[cfg(feature = "uart1")]
            UART1: crate::UART1,
            #[cfg(feature = "uart2")]
            UART2: crate::UART2,
            #[cfg(feature = "wkup580_nl01")]
            WKUP580_NL01: crate::WKUP580_NL01,
        }
    }
}
