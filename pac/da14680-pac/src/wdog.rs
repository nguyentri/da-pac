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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:08 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"WDOG registers"]
unsafe impl ::core::marker::Send for super::Wdog {}
unsafe impl ::core::marker::Sync for super::Wdog {}
impl super::Wdog {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Watchdog control register."]
    #[inline(always)]
    pub const fn watchdog_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WatchdogCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WatchdogCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Watchdog timer register."]
    #[inline(always)]
    pub const fn watchdog_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WatchdogReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WatchdogReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WatchdogCtrlReg_SPEC;
impl crate::sealed::RegSpec for WatchdogCtrlReg_SPEC {
    type DataType = u16;
}
#[doc = "Watchdog control register."]
pub type WatchdogCtrlReg = crate::RegValueT<WatchdogCtrlReg_SPEC>;

impl WatchdogCtrlReg {
    #[doc = "0 = Watchdog timer generates NMI at value 0, and WDOG (SYS) reset at <=-16. Timer can be frozen /resumed using\nSET_FREEZE_REG\\[FRZ_WDOG\\]/\nRESET_FREEZE_REG\\[FRZ_WDOG\\].\n1 = Watchdog timer generates a WDOG (SYS) reset at value 0 and can not be frozen by Software.\nNote that this bit can only be set to 1 by SW and only be reset with a WDOG (SYS) reset or SW reset.\nThe watchdog is always frozen when the Cortex-M0 is halted in DEBUG State."]
    #[inline(always)]
    pub fn nmi_rst(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, WatchdogCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,WatchdogCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WatchdogCtrlReg {
    #[inline(always)]
    fn default() -> WatchdogCtrlReg {
        <crate::RegValueT<WatchdogCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WatchdogReg_SPEC;
impl crate::sealed::RegSpec for WatchdogReg_SPEC {
    type DataType = u16;
}
#[doc = "Watchdog timer register."]
pub type WatchdogReg = crate::RegValueT<WatchdogReg_SPEC>;

impl WatchdogReg {
    #[doc = "0000.000 = Write enable for Watchdog timer\nelse Write disable. This filter prevents unintentional presetting the watchdog with a SW run-away."]
    #[inline(always)]
    pub fn wdog_wen(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, WatchdogReg_SPEC, crate::common::W> {
        crate::common::RegisterField::<9,0x7f,1,0,u8, WatchdogReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "0 = Watchdog timer value is positive.\n1 = Watchdog timer value is negative."]
    #[inline(always)]
    pub fn wdog_val_neg(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, WatchdogReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,WatchdogReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write: Watchdog timer reload value. Note that all bits 15-9 must be 0 to reload this register.\nRead: Actual Watchdog timer value. Decremented by 1 every 10.24 msec. Bit 8 indicates a negative counter value. 2, 1, 0, 1FF16, 1FE16 etc. An NMI or WDOG (SYS) reset is generated under the following conditions:\nIf WATCHDOG_CTRL_REG\\[NMI_RST\\] = 0 then\n    If WDOG_VAL = 0 -> NMI (Non Maskable Interrupt)\n    if WDOG_VAL = 1F016 -> WDOG reset -> reload FF16\nIf WATCHDOG_CTRL_REG\\[NMI_RST\\] = 1 then\n    if WDOG_VAL <= 0 -> WDOG reset -> reload FF16"]
    #[inline(always)]
    pub fn wdog_val(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, WatchdogReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, WatchdogReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WatchdogReg {
    #[inline(always)]
    fn default() -> WatchdogReg {
        <crate::RegValueT<WatchdogReg_SPEC> as RegisterValue<_>>::new(255)
    }
}
