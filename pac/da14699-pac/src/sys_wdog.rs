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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:34 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"SYS_WDOG registers"]
unsafe impl ::core::marker::Send for super::SysWdog {}
unsafe impl ::core::marker::Sync for super::SysWdog {}
impl super::SysWdog {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn watchdog_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WatchdogCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WatchdogCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

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
    type DataType = u32;
}

pub type WatchdogCtrlReg = crate::RegValueT<WatchdogCtrlReg_SPEC>;

impl WatchdogCtrlReg {
    #[inline(always)]
    pub fn write_busy(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, WatchdogCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,WatchdogCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wdog_freeze_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, WatchdogCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,WatchdogCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
        <crate::RegValueT<WatchdogCtrlReg_SPEC> as RegisterValue<_>>::new(6)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WatchdogReg_SPEC;
impl crate::sealed::RegSpec for WatchdogReg_SPEC {
    type DataType = u32;
}

pub type WatchdogReg = crate::RegValueT<WatchdogReg_SPEC>;

impl WatchdogReg {
    #[inline(always)]
    pub fn wdog_wen(
        self,
    ) -> crate::common::RegisterField<14, 0x3ffff, 1, 0, u32, u32, WatchdogReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            14,
            0x3ffff,
            1,
            0,
            u32,
            u32,
            WatchdogReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wdog_val_neg(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, WatchdogReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,WatchdogReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wdog_val(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, WatchdogReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,WatchdogReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WatchdogReg {
    #[inline(always)]
    fn default() -> WatchdogReg {
        <crate::RegValueT<WatchdogReg_SPEC> as RegisterValue<_>>::new(8191)
    }
}
