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

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"TIMER0 registers"]
unsafe impl ::core::marker::Send for super::Timer0 {}
unsafe impl ::core::marker::Sync for super::Timer0 {}
impl super::Timer0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn pwm2_end_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm2EndCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm2EndCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm2_start_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm2StartCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm2StartCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm3_end_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm3EndCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm3EndCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm3_start_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm3StartCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm3StartCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm4_end_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm4EndCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm4EndCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm4_start_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm4StartCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm4StartCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm5_end_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm5EndCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm5EndCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm5_start_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm5StartCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm5StartCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm6_end_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm6EndCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm6EndCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm6_start_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm6StartCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm6StartCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm7_end_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm7EndCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm7EndCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwm7_start_cycle(
        &self,
    ) -> &'static crate::common::Reg<self::Pwm7StartCycle_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwm7StartCycle_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
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
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[inline(always)]
    pub const fn triple_pwm_frequency(
        &self,
    ) -> &'static crate::common::Reg<self::TriplePwmFrequency_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TriplePwmFrequency_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
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
pub struct Pwm5EndCycle_SPEC;
impl crate::sealed::RegSpec for Pwm5EndCycle_SPEC {
    type DataType = u16;
}

pub type Pwm5EndCycle = crate::RegValueT<Pwm5EndCycle_SPEC>;

impl Pwm5EndCycle {
    #[inline(always)]
    pub fn end_cycle(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, Pwm5EndCycle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm5EndCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm5EndCycle {
    #[inline(always)]
    fn default() -> Pwm5EndCycle {
        <crate::RegValueT<Pwm5EndCycle_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm5StartCycle_SPEC;
impl crate::sealed::RegSpec for Pwm5StartCycle_SPEC {
    type DataType = u16;
}

pub type Pwm5StartCycle = crate::RegValueT<Pwm5StartCycle_SPEC>;

impl Pwm5StartCycle {
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
        Pwm5StartCycle_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm5StartCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm5StartCycle {
    #[inline(always)]
    fn default() -> Pwm5StartCycle {
        <crate::RegValueT<Pwm5StartCycle_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm6EndCycle_SPEC;
impl crate::sealed::RegSpec for Pwm6EndCycle_SPEC {
    type DataType = u16;
}

pub type Pwm6EndCycle = crate::RegValueT<Pwm6EndCycle_SPEC>;

impl Pwm6EndCycle {
    #[inline(always)]
    pub fn end_cycle(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, Pwm6EndCycle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm6EndCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm6EndCycle {
    #[inline(always)]
    fn default() -> Pwm6EndCycle {
        <crate::RegValueT<Pwm6EndCycle_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm6StartCycle_SPEC;
impl crate::sealed::RegSpec for Pwm6StartCycle_SPEC {
    type DataType = u16;
}

pub type Pwm6StartCycle = crate::RegValueT<Pwm6StartCycle_SPEC>;

impl Pwm6StartCycle {
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
        Pwm6StartCycle_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm6StartCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm6StartCycle {
    #[inline(always)]
    fn default() -> Pwm6StartCycle {
        <crate::RegValueT<Pwm6StartCycle_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm7EndCycle_SPEC;
impl crate::sealed::RegSpec for Pwm7EndCycle_SPEC {
    type DataType = u16;
}

pub type Pwm7EndCycle = crate::RegValueT<Pwm7EndCycle_SPEC>;

impl Pwm7EndCycle {
    #[inline(always)]
    pub fn end_cycle(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, Pwm7EndCycle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm7EndCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm7EndCycle {
    #[inline(always)]
    fn default() -> Pwm7EndCycle {
        <crate::RegValueT<Pwm7EndCycle_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm7StartCycle_SPEC;
impl crate::sealed::RegSpec for Pwm7StartCycle_SPEC {
    type DataType = u16;
}

pub type Pwm7StartCycle = crate::RegValueT<Pwm7StartCycle_SPEC>;

impl Pwm7StartCycle {
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
        Pwm7StartCycle_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            Pwm7StartCycle_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwm7StartCycle {
    #[inline(always)]
    fn default() -> Pwm7StartCycle {
        <crate::RegValueT<Pwm7StartCycle_SPEC> as RegisterValue<_>>::new(0)
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
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Timer0OnReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Timer0OnReg_SPEC,crate::common::RW>::from_register(self,0)
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
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Timer0ReloadMReg_SPEC,
            crate::common::RW,
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
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Timer0ReloadNReg_SPEC,
            crate::common::RW,
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
    pub fn pwm_freq(
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
