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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:02 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"GP_TIMERS registers"]
unsafe impl ::core::marker::Send for super::GpTimers {}
unsafe impl ::core::marker::Sync for super::GpTimers {}
impl super::GpTimers {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn breath_cfg_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BreathCfgReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BreathCfgReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn breath_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BreathCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BreathCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[inline(always)]
    pub const fn breath_duty_max_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BreathDutyMaxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BreathDutyMaxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[inline(always)]
    pub const fn breath_duty_min_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BreathDutyMinReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BreathDutyMinReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm2_end_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm2EndCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm2EndCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm2_start_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm2StartCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm2StartCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm3_end_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm3EndCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm3EndCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm3_start_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm3StartCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm3StartCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm4_end_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm4EndCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm4EndCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm4_start_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm4StartCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm4StartCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer0_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer0CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer0CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer0_on_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer0OnReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer0OnReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer0_reload_m_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer0ReloadMReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer0ReloadMReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer0_reload_n_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer0ReloadNReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer0ReloadNReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn triple_pwm_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TriplePwmCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TriplePwmCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn triple_pwm_frequency(
        &self,
    ) -> &'static crate::common::Reg<self::TriplePwmFrequency_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TriplePwmFrequency_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BreathCfgReg_SPEC;
impl crate::sealed::RegSpec for BreathCfgReg_SPEC {
    type DataType = u16;
}

pub type BreathCfgReg = crate::RegValueT<BreathCfgReg_SPEC>;

impl BreathCfgReg {
    #[inline(always)]
    pub fn brth_step(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, BreathCfgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,BreathCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn brth_div(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, BreathCfgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,BreathCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BreathCfgReg {
    #[inline(always)]
    fn default() -> BreathCfgReg {
        <crate::RegValueT<BreathCfgReg_SPEC> as RegisterValue<_>>::new(511)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BreathCtrlReg_SPEC;
impl crate::sealed::RegSpec for BreathCtrlReg_SPEC {
    type DataType = u16;
}

pub type BreathCtrlReg = crate::RegValueT<BreathCtrlReg_SPEC>;

impl BreathCtrlReg {
    #[inline(always)]
    pub fn brth_pwm_pol(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BreathCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,BreathCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn brth_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BreathCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,BreathCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BreathCtrlReg {
    #[inline(always)]
    fn default() -> BreathCtrlReg {
        <crate::RegValueT<BreathCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BreathDutyMaxReg_SPEC;
impl crate::sealed::RegSpec for BreathDutyMaxReg_SPEC {
    type DataType = u16;
}

pub type BreathDutyMaxReg = crate::RegValueT<BreathDutyMaxReg_SPEC>;

impl BreathDutyMaxReg {
    #[inline(always)]
    pub fn brth_duty_max(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, BreathDutyMaxReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            BreathDutyMaxReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BreathDutyMaxReg {
    #[inline(always)]
    fn default() -> BreathDutyMaxReg {
        <crate::RegValueT<BreathDutyMaxReg_SPEC> as RegisterValue<_>>::new(10)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BreathDutyMinReg_SPEC;
impl crate::sealed::RegSpec for BreathDutyMinReg_SPEC {
    type DataType = u16;
}

pub type BreathDutyMinReg = crate::RegValueT<BreathDutyMinReg_SPEC>;

impl BreathDutyMinReg {
    #[inline(always)]
    pub fn brth_duty_min(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, BreathDutyMinReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            BreathDutyMinReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BreathDutyMinReg {
    #[inline(always)]
    fn default() -> BreathDutyMinReg {
        <crate::RegValueT<BreathDutyMinReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm2EndCycle_SPEC;
impl crate::sealed::RegSpec for Pwm2EndCycle_SPEC {
    type DataType = u16;
}

pub type Pwm2EndCycle = crate::RegValueT<Pwm2EndCycle_SPEC>;

impl Pwm2EndCycle {
    #[inline(always)]
    pub fn end_cycle(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, Pwm2EndCycle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm2EndCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm2EndCycle {
    #[inline(always)]
    fn default() -> Pwm2EndCycle {
        <crate::RegValueT<Pwm2EndCycle_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm2StartCycle_SPEC;
impl crate::sealed::RegSpec for Pwm2StartCycle_SPEC {
    type DataType = u16;
}

pub type Pwm2StartCycle = crate::RegValueT<Pwm2StartCycle_SPEC>;

impl Pwm2StartCycle {
    #[inline(always)]
    pub fn start_cycle(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        u16,
        u16,
        Pwm2StartCycle_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm2StartCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm2StartCycle {
    #[inline(always)]
    fn default() -> Pwm2StartCycle {
        <crate::RegValueT<Pwm2StartCycle_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm3EndCycle_SPEC;
impl crate::sealed::RegSpec for Pwm3EndCycle_SPEC {
    type DataType = u16;
}

pub type Pwm3EndCycle = crate::RegValueT<Pwm3EndCycle_SPEC>;

impl Pwm3EndCycle {
    #[inline(always)]
    pub fn end_cycle(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, Pwm3EndCycle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm3EndCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm3EndCycle {
    #[inline(always)]
    fn default() -> Pwm3EndCycle {
        <crate::RegValueT<Pwm3EndCycle_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm3StartCycle_SPEC;
impl crate::sealed::RegSpec for Pwm3StartCycle_SPEC {
    type DataType = u16;
}

pub type Pwm3StartCycle = crate::RegValueT<Pwm3StartCycle_SPEC>;

impl Pwm3StartCycle {
    #[inline(always)]
    pub fn start_cycle(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        u16,
        u16,
        Pwm3StartCycle_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm3StartCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm3StartCycle {
    #[inline(always)]
    fn default() -> Pwm3StartCycle {
        <crate::RegValueT<Pwm3StartCycle_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm4EndCycle_SPEC;
impl crate::sealed::RegSpec for Pwm4EndCycle_SPEC {
    type DataType = u16;
}

pub type Pwm4EndCycle = crate::RegValueT<Pwm4EndCycle_SPEC>;

impl Pwm4EndCycle {
    #[inline(always)]
    pub fn end_cycle(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, Pwm4EndCycle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm4EndCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm4EndCycle {
    #[inline(always)]
    fn default() -> Pwm4EndCycle {
        <crate::RegValueT<Pwm4EndCycle_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm4StartCycle_SPEC;
impl crate::sealed::RegSpec for Pwm4StartCycle_SPEC {
    type DataType = u16;
}

pub type Pwm4StartCycle = crate::RegValueT<Pwm4StartCycle_SPEC>;

impl Pwm4StartCycle {
    #[inline(always)]
    pub fn start_cycle(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        u16,
        u16,
        Pwm4StartCycle_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm4StartCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm4StartCycle {
    #[inline(always)]
    fn default() -> Pwm4StartCycle {
        <crate::RegValueT<Pwm4StartCycle_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0CtrlReg_SPEC;
impl crate::sealed::RegSpec for Timer0CtrlReg_SPEC {
    type DataType = u16;
}

pub type Timer0CtrlReg = crate::RegValueT<Timer0CtrlReg_SPEC>;

impl Timer0CtrlReg {
    #[inline(always)]
    pub fn pwm_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Timer0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Timer0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim0_clk_div(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Timer0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Timer0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim0_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Timer0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Timer0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim0_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Timer0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Timer0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Timer0CtrlReg {
    #[inline(always)]
    fn default() -> Timer0CtrlReg {
        <crate::RegValueT<Timer0CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0OnReg_SPEC;
impl crate::sealed::RegSpec for Timer0OnReg_SPEC {
    type DataType = u16;
}

pub type Timer0OnReg = crate::RegValueT<Timer0OnReg_SPEC>;

impl Timer0OnReg {
    #[inline(always)]
    pub fn tim0_on(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Timer0OnReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Timer0OnReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Timer0OnReg {
    #[inline(always)]
    fn default() -> Timer0OnReg {
        <crate::RegValueT<Timer0OnReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0ReloadMReg_SPEC;
impl crate::sealed::RegSpec for Timer0ReloadMReg_SPEC {
    type DataType = u16;
}

pub type Timer0ReloadMReg = crate::RegValueT<Timer0ReloadMReg_SPEC>;

impl Timer0ReloadMReg {
    #[inline(always)]
    pub fn tim0_m(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Timer0ReloadMReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Timer0ReloadMReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer0ReloadMReg {
    #[inline(always)]
    fn default() -> Timer0ReloadMReg {
        <crate::RegValueT<Timer0ReloadMReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0ReloadNReg_SPEC;
impl crate::sealed::RegSpec for Timer0ReloadNReg_SPEC {
    type DataType = u16;
}

pub type Timer0ReloadNReg = crate::RegValueT<Timer0ReloadNReg_SPEC>;

impl Timer0ReloadNReg {
    #[inline(always)]
    pub fn tim0_n(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Timer0ReloadNReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Timer0ReloadNReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer0ReloadNReg {
    #[inline(always)]
    fn default() -> Timer0ReloadNReg {
        <crate::RegValueT<Timer0ReloadNReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TriplePwmCtrlReg_SPEC;
impl crate::sealed::RegSpec for TriplePwmCtrlReg_SPEC {
    type DataType = u16;
}

pub type TriplePwmCtrlReg = crate::RegValueT<TriplePwmCtrlReg_SPEC>;

impl TriplePwmCtrlReg {
    #[inline(always)]
    pub fn triple_pwm_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, TriplePwmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,TriplePwmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hw_pause_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, TriplePwmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,TriplePwmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sw_pause_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, TriplePwmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,TriplePwmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn triple_pwm_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, TriplePwmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,TriplePwmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TriplePwmCtrlReg {
    #[inline(always)]
    fn default() -> TriplePwmCtrlReg {
        <crate::RegValueT<TriplePwmCtrlReg_SPEC> as RegisterValue<_>>::new(12)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TriplePwmFrequency_SPEC;
impl crate::sealed::RegSpec for TriplePwmFrequency_SPEC {
    type DataType = u16;
}

pub type TriplePwmFrequency = crate::RegValueT<TriplePwmFrequency_SPEC>;

impl TriplePwmFrequency {
    #[inline(always)]
    pub fn freq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        u16,
        u16,
        TriplePwmFrequency_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            TriplePwmFrequency_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TriplePwmFrequency {
    #[inline(always)]
    fn default() -> TriplePwmFrequency {
        <crate::RegValueT<TriplePwmFrequency_SPEC> as RegisterValue<_>>::new(0)
    }
}
