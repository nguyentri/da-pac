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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:28 +0000
#![cfg_attr(not(feature = "tracing"), no_std)]
#![allow(non_camel_case_types)]
#![doc = "D2798AA"]
pub mod common;
pub use common::*;

#[cfg(feature = "tracing")]
pub mod reg_name;
#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "aes_hash")]
pub mod aes_hash;
#[cfg(feature = "anamisc_bif")]
pub mod anamisc_bif;
#[cfg(feature = "cache")]
pub mod cache;
#[cfg(feature = "charger")]
pub mod charger;
#[cfg(feature = "chg_det")]
pub mod chg_det;
#[cfg(feature = "crg_aud")]
pub mod crg_aud;
#[cfg(feature = "crg_ctrl")]
pub mod crg_ctrl;
#[cfg(feature = "crg_gpu")]
pub mod crg_gpu;
#[cfg(feature = "crg_snc")]
pub mod crg_snc;
#[cfg(feature = "crg_sys")]
pub mod crg_sys;
#[cfg(feature = "crg_top")]
pub mod crg_top;
#[cfg(feature = "crg_vsys")]
pub mod crg_vsys;
#[cfg(feature = "crg_xtal")]
pub mod crg_xtal;
#[cfg(feature = "dcache")]
pub mod dcache;
#[cfg(feature = "dcdc")]
pub mod dcdc;
#[cfg(feature = "dcdc_boost")]
pub mod dcdc_boost;
#[cfg(feature = "dma")]
pub mod dma;
#[cfg(feature = "dsi2")]
pub mod dsi2;
#[cfg(feature = "dsidphy_reg")]
pub mod dsidphy_reg;
#[cfg(feature = "emmc")]
pub mod emmc;
#[cfg(feature = "gpadc")]
pub mod gpadc;
#[cfg(feature = "gpio")]
pub mod gpio;
#[cfg(feature = "gpreg")]
pub mod gpreg;
#[cfg(feature = "gpu_core")]
pub mod gpu_core;
#[cfg(feature = "gpu_reg")]
pub mod gpu_reg;
#[cfg(feature = "i2c")]
pub mod i2c;
#[cfg(feature = "i2c2")]
pub mod i2c2;
#[cfg(feature = "i2c3")]
pub mod i2c3;
#[cfg(feature = "i3c")]
pub mod i3c;
#[cfg(feature = "lcdc")]
pub mod lcdc;
#[cfg(feature = "memctrl")]
pub mod memctrl;
#[cfg(feature = "oqspif")]
pub mod oqspif;

#[cfg(feature = "aes_hash")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AesHash {
    ptr: *mut u8,
}
#[cfg(feature = "aes_hash")]
pub const AES_HASH: self::AesHash = self::AesHash {
    ptr: 0x30040000u32 as _,
};
#[cfg(feature = "anamisc_bif")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnamiscBif {
    ptr: *mut u8,
}
#[cfg(feature = "anamisc_bif")]
pub const ANAMISC_BIF: self::AnamiscBif = self::AnamiscBif {
    ptr: 0x50050600u32 as _,
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
    ptr: 0x51000600u32 as _,
};
#[cfg(feature = "chg_det")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDet {
    ptr: *mut u8,
}
#[cfg(feature = "chg_det")]
pub const CHG_DET: self::ChgDet = self::ChgDet {
    ptr: 0x50040300u32 as _,
};
#[cfg(feature = "crg_aud")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgAud {
    ptr: *mut u8,
}
#[cfg(feature = "crg_aud")]
pub const CRG_AUD: self::CrgAud = self::CrgAud {
    ptr: 0x50030000u32 as _,
};
#[cfg(feature = "crg_ctrl")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgCtrl {
    ptr: *mut u8,
}
#[cfg(feature = "crg_ctrl")]
pub const CRG_CTRL: self::CrgCtrl = self::CrgCtrl {
    ptr: 0x50060000u32 as _,
};
#[cfg(feature = "crg_gpu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgGpu {
    ptr: *mut u8,
}
#[cfg(feature = "crg_gpu")]
pub const CRG_GPU: self::CrgGpu = self::CrgGpu {
    ptr: 0x51001000u32 as _,
};
#[cfg(feature = "crg_snc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgSnc {
    ptr: *mut u8,
}
#[cfg(feature = "crg_snc")]
pub const CRG_SNC: self::CrgSnc = self::CrgSnc {
    ptr: 0x50020900u32 as _,
};
#[cfg(feature = "crg_sys")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgSys {
    ptr: *mut u8,
}
#[cfg(feature = "crg_sys")]
pub const CRG_SYS: self::CrgSys = self::CrgSys {
    ptr: 0x50040400u32 as _,
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
#[cfg(feature = "crg_vsys")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgVsys {
    ptr: *mut u8,
}
#[cfg(feature = "crg_vsys")]
pub const CRG_VSYS: self::CrgVsys = self::CrgVsys {
    ptr: 0x50000b00u32 as _,
};
#[cfg(feature = "crg_xtal")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrgXtal {
    ptr: *mut u8,
}
#[cfg(feature = "crg_xtal")]
pub const CRG_XTAL: self::CrgXtal = self::CrgXtal {
    ptr: 0x50050400u32 as _,
};
#[cfg(feature = "dcache")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcache {
    ptr: *mut u8,
}
#[cfg(feature = "dcache")]
pub const DCACHE: self::Dcache = self::Dcache {
    ptr: 0x30100000u32 as _,
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
#[cfg(feature = "dcdc_boost")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcBoost {
    ptr: *mut u8,
}
#[cfg(feature = "dcdc_boost")]
pub const DCDC_BOOST: self::DcdcBoost = self::DcdcBoost {
    ptr: 0x50000500u32 as _,
};
#[cfg(feature = "dma")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
#[cfg(feature = "dma")]
pub const DMA: self::Dma = self::Dma {
    ptr: 0x51000400u32 as _,
};
#[cfg(feature = "dsi2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2 {
    ptr: *mut u8,
}
#[cfg(feature = "dsi2")]
pub const DSI2: self::Dsi2 = self::Dsi2 {
    ptr: 0x51001300u32 as _,
};
#[cfg(feature = "dsidphy_reg")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DsidphyReg {
    ptr: *mut u8,
}
#[cfg(feature = "dsidphy_reg")]
pub const DSIDPHY_REG: self::DsidphyReg = self::DsidphyReg {
    ptr: 0x51001500u32 as _,
};
#[cfg(feature = "emmc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emmc {
    ptr: *mut u8,
}
#[cfg(feature = "emmc")]
pub const EMMC: self::Emmc = self::Emmc {
    ptr: 0x30058000u32 as _,
};
#[cfg(feature = "gpadc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpadc {
    ptr: *mut u8,
}
#[cfg(feature = "gpadc")]
pub const GPADC: self::Gpadc = self::Gpadc {
    ptr: 0x50020800u32 as _,
};
#[cfg(feature = "gpio")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
#[cfg(feature = "gpio")]
pub const GPIO: self::Gpio = self::Gpio {
    ptr: 0x50050100u32 as _,
};
#[cfg(feature = "gpreg")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpreg {
    ptr: *mut u8,
}
#[cfg(feature = "gpreg")]
pub const GPREG: self::Gpreg = self::Gpreg {
    ptr: 0x50040100u32 as _,
};
#[cfg(feature = "gpu_core")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpuCore {
    ptr: *mut u8,
}
#[cfg(feature = "gpu_core")]
pub const GPU_CORE: self::GpuCore = self::GpuCore {
    ptr: 0x51001200u32 as _,
};
#[cfg(feature = "gpu_reg")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpuReg {
    ptr: *mut u8,
}
#[cfg(feature = "gpu_reg")]
pub const GPU_REG: self::GpuReg = self::GpuReg {
    ptr: 0x51001100u32 as _,
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
#[cfg(feature = "i2c3")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3 {
    ptr: *mut u8,
}
#[cfg(feature = "i2c3")]
pub const I2C3: self::I2C3 = self::I2C3 {
    ptr: 0x50020500u32 as _,
};
#[cfg(feature = "i3c")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C {
    ptr: *mut u8,
}
#[cfg(feature = "i3c")]
pub const I3C: self::I3C = self::I3C {
    ptr: 0x50020c00u32 as _,
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
#[cfg(feature = "memctrl")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memctrl {
    ptr: *mut u8,
}
#[cfg(feature = "memctrl")]
pub const MEMCTRL: self::Memctrl = self::Memctrl {
    ptr: 0x50050000u32 as _,
};
#[cfg(feature = "oqspif")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oqspif {
    ptr: *mut u8,
}
#[cfg(feature = "oqspif")]
pub const OQSPIF: self::Oqspif = self::Oqspif {
    ptr: 0x36000000u32 as _,
};

pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
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
    fn CMAC2SYS();
    fn SNC2SYS();
    fn M33_CACHE_MRM();
    fn PDC_M33();
    fn KEY_WKUP_GPIO();
    fn VBUS();
    fn CHARGER_STATE();
    fn CHARGER_ERROR();
    fn DCDC_BOOST();
    fn PLL48_LOCK();
    fn CRYPTO();
    fn PLL_LOCK();
    fn XTAL32M_RDY();
    fn RFDIAG();
    fn GPIO_P0();
    fn GPIO_P1();
    fn GPIO_P2();
    fn TIMER();
    fn TIMER2();
    fn TIMER3();
    fn TIMER4();
    fn TIMER5();
    fn TIMER6();
    fn RTC();
    fn RTC_EVENT();
    fn CAPTIMER();
    fn ADC();
    fn ADC2();
    fn DMA();
    fn UART();
    fn UART2();
    fn UART3();
    fn SPI();
    fn SPI2();
    fn SPI3();
    fn I2C();
    fn I2C2();
    fn I2C3();
    fn I3C();
    fn USB();
    fn PCM();
    fn SRC_IN();
    fn SRC_OUT();
    fn SRC2_IN();
    fn SRC2_OUT();
    fn VAD();
    fn EMMC();
    fn SDIO();
    fn GPU();
    fn LCD();
    fn DSI();
    fn CHARGER_DET();
    fn DCACHE_MRM();
    fn CLK_CALIBRATION();
    fn VSYS_GEN();
    fn RESERVED55();
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 56] = [
    Vector { _handler: CMAC2SYS },
    Vector { _handler: SNC2SYS },
    Vector {
        _handler: M33_CACHE_MRM,
    },
    Vector { _handler: PDC_M33 },
    Vector {
        _handler: KEY_WKUP_GPIO,
    },
    Vector { _handler: VBUS },
    Vector {
        _handler: CHARGER_STATE,
    },
    Vector {
        _handler: CHARGER_ERROR,
    },
    Vector {
        _handler: DCDC_BOOST,
    },
    Vector {
        _handler: PLL48_LOCK,
    },
    Vector { _handler: CRYPTO },
    Vector { _handler: PLL_LOCK },
    Vector {
        _handler: XTAL32M_RDY,
    },
    Vector { _handler: RFDIAG },
    Vector { _handler: GPIO_P0 },
    Vector { _handler: GPIO_P1 },
    Vector { _handler: GPIO_P2 },
    Vector { _handler: TIMER },
    Vector { _handler: TIMER2 },
    Vector { _handler: TIMER3 },
    Vector { _handler: TIMER4 },
    Vector { _handler: TIMER5 },
    Vector { _handler: TIMER6 },
    Vector { _handler: RTC },
    Vector {
        _handler: RTC_EVENT,
    },
    Vector { _handler: CAPTIMER },
    Vector { _handler: ADC },
    Vector { _handler: ADC2 },
    Vector { _handler: DMA },
    Vector { _handler: UART },
    Vector { _handler: UART2 },
    Vector { _handler: UART3 },
    Vector { _handler: SPI },
    Vector { _handler: SPI2 },
    Vector { _handler: SPI3 },
    Vector { _handler: I2C },
    Vector { _handler: I2C2 },
    Vector { _handler: I2C3 },
    Vector { _handler: I3C },
    Vector { _handler: USB },
    Vector { _handler: PCM },
    Vector { _handler: SRC_IN },
    Vector { _handler: SRC_OUT },
    Vector { _handler: SRC2_IN },
    Vector { _handler: SRC2_OUT },
    Vector { _handler: VAD },
    Vector { _handler: EMMC },
    Vector { _handler: SDIO },
    Vector { _handler: GPU },
    Vector { _handler: LCD },
    Vector { _handler: DSI },
    Vector {
        _handler: CHARGER_DET,
    },
    Vector {
        _handler: DCACHE_MRM,
    },
    Vector {
        _handler: CLK_CALIBRATION,
    },
    Vector { _handler: VSYS_GEN },
    Vector {
        _handler: RESERVED55,
    },
];
#[doc = "Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "CMAC and mailbox interrupt request."]
    CMAC2SYS = 0,
    #[doc = "SNC M0+ interrupt to Cortex M33."]
    SNC2SYS = 1,
    #[doc = "CM33 Cache Miss Rate Monitor interrupt request."]
    M33_CACHE_MRM = 2,
    #[doc = "Wakeup IRQ from PDC to CM33."]
    PDC_M33 = 3,
    #[doc = "Debounced button press interrupt request."]
    KEY_WKUP_GPIO = 4,
    #[doc = "VBUS presence interrupt request."]
    VBUS = 5,
    #[doc = "Charger State interrupt request."]
    CHARGER_STATE = 6,
    #[doc = "Charger Error interrupt request."]
    CHARGER_ERROR = 7,
    #[doc = "DCDC boost controller interrupt request."]
    DCDC_BOOST = 8,
    #[doc = "PLL48 lock interrupt request. Indicates that the PLL48 is locked at 48MHz."]
    PLL48_LOCK = 9,
    #[doc = "Crypto interrupt request."]
    CRYPTO = 10,
    #[doc = "PLL lock interrupt request. Indicates that PLL is locked at 160MHz."]
    PLL_LOCK = 11,
    #[doc = "XTAL32M trimmed and ready interrupt request."]
    XTAL32M_RDY = 12,
    #[doc = "Baseband or Radio Diagnostics interrupt request."]
    RFDIAG = 13,
    #[doc = "GPIO port 0 toggle interrupt request."]
    GPIO_P0 = 14,
    #[doc = "GPIO port 1 toggle interrupt request."]
    GPIO_P1 = 15,
    #[doc = "GPIO port 2 toggle interrupt request."]
    GPIO_P2 = 16,
    #[doc = "TIMER interrupt request."]
    TIMER = 17,
    #[doc = "TIMER2 interrupt request."]
    TIMER2 = 18,
    #[doc = "TIMER3 interrupt request."]
    TIMER3 = 19,
    #[doc = "TIMER4 interrupt request."]
    TIMER4 = 20,
    #[doc = "TIMER5 interrupt request."]
    TIMER5 = 21,
    #[doc = "TIMER6 interrupt request."]
    TIMER6 = 22,
    #[doc = "RTC interrupt request."]
    RTC = 23,
    #[doc = "RTC event interrupt request."]
    RTC_EVENT = 24,
    #[doc = "GPIO triggered Timer Capture interrupt request."]
    CAPTIMER = 25,
    #[doc = "General Purpose Analog-Digital Converter interrupt request."]
    ADC = 26,
    #[doc = "Application Analog-Digital Converter interrupt request."]
    ADC2 = 27,
    #[doc = "General Purpose DMA interrupt request."]
    DMA = 28,
    #[doc = "UART interrupt request."]
    UART = 29,
    #[doc = "UART2 interrupt request."]
    UART2 = 30,
    #[doc = "UART3 interrupt request."]
    UART3 = 31,
    #[doc = "SPI interrupt request."]
    SPI = 32,
    #[doc = "SPI2 interrupt request."]
    SPI2 = 33,
    #[doc = "SPI3 interrupt request."]
    SPI3 = 34,
    #[doc = "I2C interrupt request."]
    I2C = 35,
    #[doc = "I2C2 interrupt request."]
    I2C2 = 36,
    #[doc = "I2C3 interrupt request."]
    I2C3 = 37,
    #[doc = "I3C interrupt request."]
    I3C = 38,
    #[doc = "USB interrupt request."]
    USB = 39,
    #[doc = "PCM interrupt request."]
    PCM = 40,
    #[doc = "SRC input interrupt request."]
    SRC_IN = 41,
    #[doc = "SRC output interrupt request."]
    SRC_OUT = 42,
    #[doc = "SRC2 input interrupt request."]
    SRC2_IN = 43,
    #[doc = "SRC2 output interrupt request."]
    SRC2_OUT = 44,
    #[doc = "VAD interrupt request."]
    VAD = 45,
    #[doc = "eMMC controller interrupt request."]
    EMMC = 46,
    #[doc = "SDIO controller interrupt request."]
    SDIO = 47,
    #[doc = "GPU interrupt request to Cortex M33."]
    GPU = 48,
    #[doc = "LCD controller interrupt request."]
    LCD = 49,
    #[doc = "DSI/D-PHY interrupt request."]
    DSI = 50,
    #[doc = "Charger detection interrupt request."]
    CHARGER_DET = 51,
    #[doc = "Data cache MRM interrupt request."]
    DCACHE_MRM = 52,
    #[doc = "Clock calibration interrupt request."]
    CLK_CALIBRATION = 53,
    #[doc = "VSYS generator interrupt request."]
    VSYS_GEN = 54,
    #[doc = "SoftWare interrupt request."]
    RESERVED55 = 55,
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
    #[cfg(feature = "aes_hash")]
    pub AES_HASH: self::AesHash,
    #[cfg(feature = "anamisc_bif")]
    pub ANAMISC_BIF: self::AnamiscBif,
    #[cfg(feature = "cache")]
    pub CACHE: self::Cache,
    #[cfg(feature = "charger")]
    pub CHARGER: self::Charger,
    #[cfg(feature = "chg_det")]
    pub CHG_DET: self::ChgDet,
    #[cfg(feature = "crg_aud")]
    pub CRG_AUD: self::CrgAud,
    #[cfg(feature = "crg_ctrl")]
    pub CRG_CTRL: self::CrgCtrl,
    #[cfg(feature = "crg_gpu")]
    pub CRG_GPU: self::CrgGpu,
    #[cfg(feature = "crg_snc")]
    pub CRG_SNC: self::CrgSnc,
    #[cfg(feature = "crg_sys")]
    pub CRG_SYS: self::CrgSys,
    #[cfg(feature = "crg_top")]
    pub CRG_TOP: self::CrgTop,
    #[cfg(feature = "crg_vsys")]
    pub CRG_VSYS: self::CrgVsys,
    #[cfg(feature = "crg_xtal")]
    pub CRG_XTAL: self::CrgXtal,
    #[cfg(feature = "dcache")]
    pub DCACHE: self::Dcache,
    #[cfg(feature = "dcdc")]
    pub DCDC: self::Dcdc,
    #[cfg(feature = "dcdc_boost")]
    pub DCDC_BOOST: self::DcdcBoost,
    #[cfg(feature = "dma")]
    pub DMA: self::Dma,
    #[cfg(feature = "dsi2")]
    pub DSI2: self::Dsi2,
    #[cfg(feature = "dsidphy_reg")]
    pub DSIDPHY_REG: self::DsidphyReg,
    #[cfg(feature = "emmc")]
    pub EMMC: self::Emmc,
    #[cfg(feature = "gpadc")]
    pub GPADC: self::Gpadc,
    #[cfg(feature = "gpio")]
    pub GPIO: self::Gpio,
    #[cfg(feature = "gpreg")]
    pub GPREG: self::Gpreg,
    #[cfg(feature = "gpu_core")]
    pub GPU_CORE: self::GpuCore,
    #[cfg(feature = "gpu_reg")]
    pub GPU_REG: self::GpuReg,
    #[cfg(feature = "i2c")]
    pub I2C: self::I2C,
    #[cfg(feature = "i2c2")]
    pub I2C2: self::I2C2,
    #[cfg(feature = "i2c3")]
    pub I2C3: self::I2C3,
    #[cfg(feature = "i3c")]
    pub I3C: self::I3C,
    #[cfg(feature = "lcdc")]
    pub LCDC: self::Lcdc,
    #[cfg(feature = "memctrl")]
    pub MEMCTRL: self::Memctrl,
    #[cfg(feature = "oqspif")]
    pub OQSPIF: self::Oqspif,
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
            #[cfg(feature = "aes_hash")]
            AES_HASH: crate::AES_HASH,
            #[cfg(feature = "anamisc_bif")]
            ANAMISC_BIF: crate::ANAMISC_BIF,
            #[cfg(feature = "cache")]
            CACHE: crate::CACHE,
            #[cfg(feature = "charger")]
            CHARGER: crate::CHARGER,
            #[cfg(feature = "chg_det")]
            CHG_DET: crate::CHG_DET,
            #[cfg(feature = "crg_aud")]
            CRG_AUD: crate::CRG_AUD,
            #[cfg(feature = "crg_ctrl")]
            CRG_CTRL: crate::CRG_CTRL,
            #[cfg(feature = "crg_gpu")]
            CRG_GPU: crate::CRG_GPU,
            #[cfg(feature = "crg_snc")]
            CRG_SNC: crate::CRG_SNC,
            #[cfg(feature = "crg_sys")]
            CRG_SYS: crate::CRG_SYS,
            #[cfg(feature = "crg_top")]
            CRG_TOP: crate::CRG_TOP,
            #[cfg(feature = "crg_vsys")]
            CRG_VSYS: crate::CRG_VSYS,
            #[cfg(feature = "crg_xtal")]
            CRG_XTAL: crate::CRG_XTAL,
            #[cfg(feature = "dcache")]
            DCACHE: crate::DCACHE,
            #[cfg(feature = "dcdc")]
            DCDC: crate::DCDC,
            #[cfg(feature = "dcdc_boost")]
            DCDC_BOOST: crate::DCDC_BOOST,
            #[cfg(feature = "dma")]
            DMA: crate::DMA,
            #[cfg(feature = "dsi2")]
            DSI2: crate::DSI2,
            #[cfg(feature = "dsidphy_reg")]
            DSIDPHY_REG: crate::DSIDPHY_REG,
            #[cfg(feature = "emmc")]
            EMMC: crate::EMMC,
            #[cfg(feature = "gpadc")]
            GPADC: crate::GPADC,
            #[cfg(feature = "gpio")]
            GPIO: crate::GPIO,
            #[cfg(feature = "gpreg")]
            GPREG: crate::GPREG,
            #[cfg(feature = "gpu_core")]
            GPU_CORE: crate::GPU_CORE,
            #[cfg(feature = "gpu_reg")]
            GPU_REG: crate::GPU_REG,
            #[cfg(feature = "i2c")]
            I2C: crate::I2C,
            #[cfg(feature = "i2c2")]
            I2C2: crate::I2C2,
            #[cfg(feature = "i2c3")]
            I2C3: crate::I2C3,
            #[cfg(feature = "i3c")]
            I3C: crate::I3C,
            #[cfg(feature = "lcdc")]
            LCDC: crate::LCDC,
            #[cfg(feature = "memctrl")]
            MEMCTRL: crate::MEMCTRL,
            #[cfg(feature = "oqspif")]
            OQSPIF: crate::OQSPIF,
        }
    }
}
