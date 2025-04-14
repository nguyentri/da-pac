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
#[doc = r"PWMLED registers"]
unsafe impl ::core::marker::Send for super::Pwmled {}
unsafe impl ::core::marker::Sync for super::Pwmled {}
impl super::Pwmled {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn pwmled_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PwmledCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PwmledCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwmled_duty_cycle_led1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PwmledDutyCycleLed1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PwmledDutyCycleLed1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwmled_duty_cycle_led2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PwmledDutyCycleLed2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PwmledDutyCycleLed2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pwmled_frequency_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PwmledFrequencyReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PwmledFrequencyReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwmledCtrlReg_SPEC;
impl crate::sealed::RegSpec for PwmledCtrlReg_SPEC {
    type DataType = u32;
}

pub type PwmledCtrlReg = crate::RegValueT<PwmledCtrlReg_SPEC>;

impl PwmledCtrlReg {
    #[inline(always)]
    pub fn led2_load_sel(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, PwmledCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,PwmledCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn led1_load_sel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, PwmledCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,PwmledCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn led2_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PwmledCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PwmledCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn led1_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, PwmledCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,PwmledCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn led_trim(
        self,
    ) -> crate::common::RegisterField<2, 0xf, 1, 0, u8, u8, PwmledCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0xf,1,0,u8,u8,PwmledCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sw_pause_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PwmledCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PwmledCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pwm_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PwmledCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PwmledCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PwmledCtrlReg {
    #[inline(always)]
    fn default() -> PwmledCtrlReg {
        <crate::RegValueT<PwmledCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwmledDutyCycleLed1Reg_SPEC;
impl crate::sealed::RegSpec for PwmledDutyCycleLed1Reg_SPEC {
    type DataType = u32;
}

pub type PwmledDutyCycleLed1Reg = crate::RegValueT<PwmledDutyCycleLed1Reg_SPEC>;

impl PwmledDutyCycleLed1Reg {
    #[inline(always)]
    pub fn led1_pwm_start_cycle(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        PwmledDutyCycleLed1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            PwmledDutyCycleLed1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn led1_pwm_end_cycle(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        PwmledDutyCycleLed1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            PwmledDutyCycleLed1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PwmledDutyCycleLed1Reg {
    #[inline(always)]
    fn default() -> PwmledDutyCycleLed1Reg {
        <crate::RegValueT<PwmledDutyCycleLed1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwmledDutyCycleLed2Reg_SPEC;
impl crate::sealed::RegSpec for PwmledDutyCycleLed2Reg_SPEC {
    type DataType = u32;
}

pub type PwmledDutyCycleLed2Reg = crate::RegValueT<PwmledDutyCycleLed2Reg_SPEC>;

impl PwmledDutyCycleLed2Reg {
    #[inline(always)]
    pub fn led2_pwm_start_cycle(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        PwmledDutyCycleLed2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            PwmledDutyCycleLed2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn led2_pwm_end_cycle(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        PwmledDutyCycleLed2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            PwmledDutyCycleLed2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PwmledDutyCycleLed2Reg {
    #[inline(always)]
    fn default() -> PwmledDutyCycleLed2Reg {
        <crate::RegValueT<PwmledDutyCycleLed2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwmledFrequencyReg_SPEC;
impl crate::sealed::RegSpec for PwmledFrequencyReg_SPEC {
    type DataType = u32;
}

pub type PwmledFrequencyReg = crate::RegValueT<PwmledFrequencyReg_SPEC>;

impl PwmledFrequencyReg {
    #[inline(always)]
    pub fn led_pwm_frequency(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        PwmledFrequencyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            PwmledFrequencyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PwmledFrequencyReg {
    #[inline(always)]
    fn default() -> PwmledFrequencyReg {
        <crate::RegValueT<PwmledFrequencyReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
