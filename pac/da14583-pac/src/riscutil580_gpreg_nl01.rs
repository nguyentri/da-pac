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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:25:13 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"riscutil580_gpreg_nl01 registers"]
unsafe impl ::core::marker::Send for super::Riscutil580GpregNl01 {}
unsafe impl ::core::marker::Sync for super::Riscutil580GpregNl01 {}
impl super::Riscutil580GpregNl01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Various debug information register."]
    #[inline(always)]
    pub const fn debug_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DebugReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DebugReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "General purpose system control register."]
    #[inline(always)]
    pub const fn gp_control_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpControlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpControlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "General purpose system status register."]
    #[inline(always)]
    pub const fn gp_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Controls unfreezing of various timers/counters."]
    #[inline(always)]
    pub const fn reset_freeze_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ResetFreezeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ResetFreezeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Controls freezing of various timers/counters."]
    #[inline(always)]
    pub const fn set_freeze_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SetFreezeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SetFreezeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugReg_SPEC;
impl crate::sealed::RegSpec for DebugReg_SPEC {
    type DataType = u16;
}
#[doc = "Various debug information register."]
pub type DebugReg = crate::RegValueT<DebugReg_SPEC>;

impl DebugReg {
    #[doc = "Default \'1\', freezing of the on-chip timers is enabled when the Cortex-M0 is halted in DEBUG State.\nIf \'0\', freezing of the on-chip timers is depending on FREEZE_REG when the Cortex-M0 is halted in DEBUG State except the watchdog timer. The watchdog timer is always frozen when the Cortex-M0 is halted in DEBUG State."]
    #[inline(always)]
    pub fn debugs_freeze_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DebugReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, DebugReg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for DebugReg {
    #[inline(always)]
    fn default() -> DebugReg {
        <crate::RegValueT<DebugReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpControlReg_SPEC;
impl crate::sealed::RegSpec for GpControlReg_SPEC {
    type DataType = u16;
}
#[doc = "General purpose system control register."]
pub type GpControlReg = crate::RegValueT<GpControlReg_SPEC>;

impl GpControlReg {
    #[doc = "Select the mapping of the Exchange memory pages.\n0: EM size 0 kB, SysRAM size 42 kB\n1: EM size 2 kB, SysRAM size 48 kB\n2: EM size 3 kB, SysRAM size 47 kB\n3: EM size 4 kB, SysRAM size 46 kB\n4: EM size 5 kB, SysRAM size 45 kB\n5: EM size 6 kB, SysRAM size 44 kB\n6: EM size 7 kB, SysRAM size 43 kB\n7: EM size 8 kB, SysRAM size 42 kB\n8: Reserved\n9: EM size 4 kB, SysRAM size 40 kB\n10: EM size 5 kB, SysRAM size 40 kB\n11: EM size 6 kB, SysRAM size 40 kB\n12: EM size 7 kB, SysRAM size 40 kB\n13: EM size 8 kB, SysRAM size 40 kB\n14: EM size 9 kB, SysRAM size 40 kB\n15: EM size 10 kB, SysRAM size 40 kB\n16: Reserved\n17: EM size 6 kB, SysRAM size 38 kB\n18: EM size 7 kB, SysRAM size 38 kB\n19: EM size 8 kB, SysRAM size 38 kB\n20: EM size 9 kB, SysRAM size 38 kB\n21: EM size 10 kB, SysRAM size 38 kB\n22: EM size 11 kB, SysRAM size 38 kB\n23: EM size 12 kB, SysRAM size 38 kB\nother: Reserved."]
    #[inline(always)]
    pub fn em_map(
        self,
    ) -> crate::common::RegisterField<1, 0x1f, 1, 0, u8, GpControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1f,1,0,u8, GpControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the BLE wakes up. Must be kept high at least for 1 low power clock period. \nIf the BLE is in deep sleep state, then by setting this bit it will cause the wakeup LP IRQ to be asserted with a delay of 3 to 4 low power cycles."]
    #[inline(always)]
    pub fn ble_wakeup_req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, GpControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,GpControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpControlReg {
    #[inline(always)]
    fn default() -> GpControlReg {
        <crate::RegValueT<GpControlReg_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpStatusReg_SPEC;
impl crate::sealed::RegSpec for GpStatusReg_SPEC {
    type DataType = u16;
}
#[doc = "General purpose system status register."]
pub type GpStatusReg = crate::RegValueT<GpStatusReg_SPEC>;

impl GpStatusReg {
    #[doc = "If \'1\', it designates that the chip is in Calibration Phase i.e. the OTP has been initially programmed but no Calibration has occured."]
    #[inline(always)]
    pub fn cal_phase(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, GpStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,GpStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpStatusReg {
    #[inline(always)]
    fn default() -> GpStatusReg {
        <crate::RegValueT<GpStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetFreezeReg_SPEC;
impl crate::sealed::RegSpec for ResetFreezeReg_SPEC {
    type DataType = u16;
}
#[doc = "Controls unfreezing of various timers/counters."]
pub type ResetFreezeReg = crate::RegValueT<ResetFreezeReg_SPEC>;

impl ResetFreezeReg {
    #[doc = "If \'1\', the watchdog timer continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_wdog(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the the BLE master clock continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_bletim(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the SW Timer (TIMER0) continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the Wake Up Timer continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_wkuptim(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ResetFreezeReg {
    #[inline(always)]
    fn default() -> ResetFreezeReg {
        <crate::RegValueT<ResetFreezeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetFreezeReg_SPEC;
impl crate::sealed::RegSpec for SetFreezeReg_SPEC {
    type DataType = u16;
}
#[doc = "Controls freezing of various timers/counters."]
pub type SetFreezeReg = crate::RegValueT<SetFreezeReg_SPEC>;

impl SetFreezeReg {
    #[doc = "If \'1\', the watchdog timer is frozen, \'0\' is discarded. WATCHDOG_CTRL_REG\\[NMI_RST\\] must be \'0\' to allow the freeze function."]
    #[inline(always)]
    pub fn frz_wdog(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the BLE master clock is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_bletim(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the SW Timer (TIMER0) is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the Wake Up Timer is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_wkuptim(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SetFreezeReg {
    #[inline(always)]
    fn default() -> SetFreezeReg {
        <crate::RegValueT<SetFreezeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
