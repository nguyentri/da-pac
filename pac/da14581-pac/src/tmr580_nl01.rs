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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:28 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"tmr580_nl01 registers"]
unsafe impl ::core::marker::Send for super::Tmr580Nl01 {}
unsafe impl ::core::marker::Sync for super::Tmr580Nl01 {}
impl super::Tmr580Nl01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn pwm2_duty_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm2DutyCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm2DutyCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm3_duty_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm3DutyCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm3DutyCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm4_duty_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm4DutyCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm4DutyCycle_SPEC, crate::common::RW>::from_ptr(
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
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn triple_pwm_frequency(
        &self,
    ) -> &'static crate::common::Reg<self::TriplePwmFrequency_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TriplePwmFrequency_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm2DutyCycle_SPEC;
impl crate::sealed::RegSpec for Pwm2DutyCycle_SPEC {
    type DataType = u16;
}

pub type Pwm2DutyCycle = crate::RegValueT<Pwm2DutyCycle_SPEC>;

impl Pwm2DutyCycle {
    #[inline(always)]
    pub fn duty_cycle(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        u16,
        u16,
        Pwm2DutyCycle_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm2DutyCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm2DutyCycle {
    #[inline(always)]
    fn default() -> Pwm2DutyCycle {
        <crate::RegValueT<Pwm2DutyCycle_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm3DutyCycle_SPEC;
impl crate::sealed::RegSpec for Pwm3DutyCycle_SPEC {
    type DataType = u16;
}

pub type Pwm3DutyCycle = crate::RegValueT<Pwm3DutyCycle_SPEC>;

impl Pwm3DutyCycle {
    #[inline(always)]
    pub fn duty_cycle(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        u16,
        u16,
        Pwm3DutyCycle_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm3DutyCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm3DutyCycle {
    #[inline(always)]
    fn default() -> Pwm3DutyCycle {
        <crate::RegValueT<Pwm3DutyCycle_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm4DutyCycle_SPEC;
impl crate::sealed::RegSpec for Pwm4DutyCycle_SPEC {
    type DataType = u16;
}

pub type Pwm4DutyCycle = crate::RegValueT<Pwm4DutyCycle_SPEC>;

impl Pwm4DutyCycle {
    #[inline(always)]
    pub fn duty_cycle(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        u16,
        u16,
        Pwm4DutyCycle_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm4DutyCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm4DutyCycle {
    #[inline(always)]
    fn default() -> Pwm4DutyCycle {
        <crate::RegValueT<Pwm4DutyCycle_SPEC> as RegisterValue<_>>::new(0)
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
        <crate::RegValueT<TriplePwmCtrlReg_SPEC> as RegisterValue<_>>::new(4)
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
