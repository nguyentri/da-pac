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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:56 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"TIMER1 registers"]
unsafe impl ::core::marker::Send for super::Timer1 {}
unsafe impl ::core::marker::Sync for super::Timer1 {}
impl super::Timer1 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn captim_capture_gpio1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimCaptureGpio1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimCaptureGpio1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn captim_capture_gpio2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimCaptureGpio2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimCaptureGpio2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn captim_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn captim_gpio1_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimGpio1ConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimGpio1ConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn captim_gpio2_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimGpio2ConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimGpio2ConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn captim_prescaler_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimPrescalerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimPrescalerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn captim_prescaler_val_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimPrescalerValReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimPrescalerValReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn captim_pwm_dc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimPwmDcReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimPwmDcReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn captim_pwm_freq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimPwmFreqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimPwmFreqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn captim_reload_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimReloadReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimReloadReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn captim_shotwidth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimShotwidthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimShotwidthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn captim_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn captim_timer_val_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimTimerValReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimTimerValReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimCaptureGpio1Reg_SPEC;
impl crate::sealed::RegSpec for CaptimCaptureGpio1Reg_SPEC {
    type DataType = u16;
}

pub type CaptimCaptureGpio1Reg = crate::RegValueT<CaptimCaptureGpio1Reg_SPEC>;

impl CaptimCaptureGpio1Reg {
    #[inline(always)]
    pub fn captim_capture_gpio1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        CaptimCaptureGpio1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            CaptimCaptureGpio1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimCaptureGpio1Reg {
    #[inline(always)]
    fn default() -> CaptimCaptureGpio1Reg {
        <crate::RegValueT<CaptimCaptureGpio1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimCaptureGpio2Reg_SPEC;
impl crate::sealed::RegSpec for CaptimCaptureGpio2Reg_SPEC {
    type DataType = u16;
}

pub type CaptimCaptureGpio2Reg = crate::RegValueT<CaptimCaptureGpio2Reg_SPEC>;

impl CaptimCaptureGpio2Reg {
    #[inline(always)]
    pub fn captim_capture_gpio2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        CaptimCaptureGpio2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            CaptimCaptureGpio2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimCaptureGpio2Reg {
    #[inline(always)]
    fn default() -> CaptimCaptureGpio2Reg {
        <crate::RegValueT<CaptimCaptureGpio2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimCtrlReg_SPEC;
impl crate::sealed::RegSpec for CaptimCtrlReg_SPEC {
    type DataType = u16;
}

pub type CaptimCtrlReg = crate::RegValueT<CaptimCtrlReg_SPEC>;

impl CaptimCtrlReg {
    #[inline(always)]
    pub fn captim_sys_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn captim_free_run_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn captim_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn captim_in2_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn captim_in1_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn captim_count_down_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn captim_oneshot_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn captim_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CaptimCtrlReg {
    #[inline(always)]
    fn default() -> CaptimCtrlReg {
        <crate::RegValueT<CaptimCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimGpio1ConfReg_SPEC;
impl crate::sealed::RegSpec for CaptimGpio1ConfReg_SPEC {
    type DataType = u16;
}

pub type CaptimGpio1ConfReg = crate::RegValueT<CaptimGpio1ConfReg_SPEC>;

impl CaptimGpio1ConfReg {
    #[inline(always)]
    pub fn captim_gpio1_conf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        CaptimGpio1ConfReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            CaptimGpio1ConfReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimGpio1ConfReg {
    #[inline(always)]
    fn default() -> CaptimGpio1ConfReg {
        <crate::RegValueT<CaptimGpio1ConfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimGpio2ConfReg_SPEC;
impl crate::sealed::RegSpec for CaptimGpio2ConfReg_SPEC {
    type DataType = u16;
}

pub type CaptimGpio2ConfReg = crate::RegValueT<CaptimGpio2ConfReg_SPEC>;

impl CaptimGpio2ConfReg {
    #[inline(always)]
    pub fn captim_gpio2_conf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        CaptimGpio2ConfReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            CaptimGpio2ConfReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimGpio2ConfReg {
    #[inline(always)]
    fn default() -> CaptimGpio2ConfReg {
        <crate::RegValueT<CaptimGpio2ConfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimPrescalerReg_SPEC;
impl crate::sealed::RegSpec for CaptimPrescalerReg_SPEC {
    type DataType = u16;
}

pub type CaptimPrescalerReg = crate::RegValueT<CaptimPrescalerReg_SPEC>;

impl CaptimPrescalerReg {
    #[inline(always)]
    pub fn captim_prescaler(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        CaptimPrescalerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            CaptimPrescalerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimPrescalerReg {
    #[inline(always)]
    fn default() -> CaptimPrescalerReg {
        <crate::RegValueT<CaptimPrescalerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimPrescalerValReg_SPEC;
impl crate::sealed::RegSpec for CaptimPrescalerValReg_SPEC {
    type DataType = u16;
}

pub type CaptimPrescalerValReg = crate::RegValueT<CaptimPrescalerValReg_SPEC>;

impl CaptimPrescalerValReg {
    #[inline(always)]
    pub fn captim_prescaler_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        CaptimPrescalerValReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            CaptimPrescalerValReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimPrescalerValReg {
    #[inline(always)]
    fn default() -> CaptimPrescalerValReg {
        <crate::RegValueT<CaptimPrescalerValReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimPwmDcReg_SPEC;
impl crate::sealed::RegSpec for CaptimPwmDcReg_SPEC {
    type DataType = u16;
}

pub type CaptimPwmDcReg = crate::RegValueT<CaptimPwmDcReg_SPEC>;

impl CaptimPwmDcReg {
    #[inline(always)]
    pub fn captim_pwm_dc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        CaptimPwmDcReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            CaptimPwmDcReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimPwmDcReg {
    #[inline(always)]
    fn default() -> CaptimPwmDcReg {
        <crate::RegValueT<CaptimPwmDcReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimPwmFreqReg_SPEC;
impl crate::sealed::RegSpec for CaptimPwmFreqReg_SPEC {
    type DataType = u16;
}

pub type CaptimPwmFreqReg = crate::RegValueT<CaptimPwmFreqReg_SPEC>;

impl CaptimPwmFreqReg {
    #[inline(always)]
    pub fn captim_pwm_freq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        CaptimPwmFreqReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            CaptimPwmFreqReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimPwmFreqReg {
    #[inline(always)]
    fn default() -> CaptimPwmFreqReg {
        <crate::RegValueT<CaptimPwmFreqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimReloadReg_SPEC;
impl crate::sealed::RegSpec for CaptimReloadReg_SPEC {
    type DataType = u16;
}

pub type CaptimReloadReg = crate::RegValueT<CaptimReloadReg_SPEC>;

impl CaptimReloadReg {
    #[inline(always)]
    pub fn captim_reload(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        CaptimReloadReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            CaptimReloadReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimReloadReg {
    #[inline(always)]
    fn default() -> CaptimReloadReg {
        <crate::RegValueT<CaptimReloadReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimShotwidthReg_SPEC;
impl crate::sealed::RegSpec for CaptimShotwidthReg_SPEC {
    type DataType = u16;
}

pub type CaptimShotwidthReg = crate::RegValueT<CaptimShotwidthReg_SPEC>;

impl CaptimShotwidthReg {
    #[inline(always)]
    pub fn captim_shotwidth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        CaptimShotwidthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            CaptimShotwidthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimShotwidthReg {
    #[inline(always)]
    fn default() -> CaptimShotwidthReg {
        <crate::RegValueT<CaptimShotwidthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimStatusReg_SPEC;
impl crate::sealed::RegSpec for CaptimStatusReg_SPEC {
    type DataType = u16;
}

pub type CaptimStatusReg = crate::RegValueT<CaptimStatusReg_SPEC>;

impl CaptimStatusReg {
    #[inline(always)]
    pub fn captim_oneshot_phase(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, CaptimStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,CaptimStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn captim_in2_state(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CaptimStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,CaptimStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn captim_in1_state(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CaptimStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,CaptimStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CaptimStatusReg {
    #[inline(always)]
    fn default() -> CaptimStatusReg {
        <crate::RegValueT<CaptimStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimTimerValReg_SPEC;
impl crate::sealed::RegSpec for CaptimTimerValReg_SPEC {
    type DataType = u16;
}

pub type CaptimTimerValReg = crate::RegValueT<CaptimTimerValReg_SPEC>;

impl CaptimTimerValReg {
    #[inline(always)]
    pub fn captim_timer_value(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        CaptimTimerValReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            CaptimTimerValReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimTimerValReg {
    #[inline(always)]
    fn default() -> CaptimTimerValReg {
        <crate::RegValueT<CaptimTimerValReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
