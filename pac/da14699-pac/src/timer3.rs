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
#[doc = r"TIMER3 registers"]
unsafe impl ::core::marker::Send for super::Timer3 {}
unsafe impl ::core::marker::Sync for super::Timer3 {}
impl super::Timer3 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn timer3_capture_gpio1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer3CaptureGpio1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer3CaptureGpio1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer3_capture_gpio2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer3CaptureGpio2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer3CaptureGpio2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer3_clear_irq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer3ClearIrqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer3ClearIrqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer3_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer3CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer3CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer3_gpio1_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer3Gpio1ConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer3Gpio1ConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer3_gpio2_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer3Gpio2ConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer3Gpio2ConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer3_prescaler_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer3PrescalerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer3PrescalerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer3_prescaler_val_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer3PrescalerValReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer3PrescalerValReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer3_pwm_dc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer3PwmDcReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer3PwmDcReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer3_pwm_freq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer3PwmFreqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer3PwmFreqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer3_reload_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer3ReloadReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer3ReloadReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer3_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer3StatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer3StatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer3_timer_val_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer3TimerValReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer3TimerValReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3CaptureGpio1Reg_SPEC;
impl crate::sealed::RegSpec for Timer3CaptureGpio1Reg_SPEC {
    type DataType = u32;
}

pub type Timer3CaptureGpio1Reg = crate::RegValueT<Timer3CaptureGpio1Reg_SPEC>;

impl Timer3CaptureGpio1Reg {
    #[inline(always)]
    pub fn tim_capture_gpio1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        Timer3CaptureGpio1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Timer3CaptureGpio1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer3CaptureGpio1Reg {
    #[inline(always)]
    fn default() -> Timer3CaptureGpio1Reg {
        <crate::RegValueT<Timer3CaptureGpio1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3CaptureGpio2Reg_SPEC;
impl crate::sealed::RegSpec for Timer3CaptureGpio2Reg_SPEC {
    type DataType = u32;
}

pub type Timer3CaptureGpio2Reg = crate::RegValueT<Timer3CaptureGpio2Reg_SPEC>;

impl Timer3CaptureGpio2Reg {
    #[inline(always)]
    pub fn tim_capture_gpio2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        Timer3CaptureGpio2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Timer3CaptureGpio2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer3CaptureGpio2Reg {
    #[inline(always)]
    fn default() -> Timer3CaptureGpio2Reg {
        <crate::RegValueT<Timer3CaptureGpio2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3ClearIrqReg_SPEC;
impl crate::sealed::RegSpec for Timer3ClearIrqReg_SPEC {
    type DataType = u32;
}

pub type Timer3ClearIrqReg = crate::RegValueT<Timer3ClearIrqReg_SPEC>;

impl Timer3ClearIrqReg {
    #[inline(always)]
    pub fn tim_clear_irq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Timer3ClearIrqReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Timer3ClearIrqReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Timer3ClearIrqReg {
    #[inline(always)]
    fn default() -> Timer3ClearIrqReg {
        <crate::RegValueT<Timer3ClearIrqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3CtrlReg_SPEC;
impl crate::sealed::RegSpec for Timer3CtrlReg_SPEC {
    type DataType = u32;
}

pub type Timer3CtrlReg = crate::RegValueT<Timer3CtrlReg_SPEC>;

impl Timer3CtrlReg {
    #[inline(always)]
    pub fn tim_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Timer3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Timer3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_sys_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Timer3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Timer3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_free_run_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Timer3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Timer3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Timer3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Timer3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_in2_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Timer3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Timer3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_in1_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Timer3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Timer3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_count_down_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Timer3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Timer3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Timer3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Timer3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Timer3CtrlReg {
    #[inline(always)]
    fn default() -> Timer3CtrlReg {
        <crate::RegValueT<Timer3CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3Gpio1ConfReg_SPEC;
impl crate::sealed::RegSpec for Timer3Gpio1ConfReg_SPEC {
    type DataType = u32;
}

pub type Timer3Gpio1ConfReg = crate::RegValueT<Timer3Gpio1ConfReg_SPEC>;

impl Timer3Gpio1ConfReg {
    #[inline(always)]
    pub fn tim_gpio1_conf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        Timer3Gpio1ConfReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            Timer3Gpio1ConfReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer3Gpio1ConfReg {
    #[inline(always)]
    fn default() -> Timer3Gpio1ConfReg {
        <crate::RegValueT<Timer3Gpio1ConfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3Gpio2ConfReg_SPEC;
impl crate::sealed::RegSpec for Timer3Gpio2ConfReg_SPEC {
    type DataType = u32;
}

pub type Timer3Gpio2ConfReg = crate::RegValueT<Timer3Gpio2ConfReg_SPEC>;

impl Timer3Gpio2ConfReg {
    #[inline(always)]
    pub fn tim_gpio2_conf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        Timer3Gpio2ConfReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            Timer3Gpio2ConfReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer3Gpio2ConfReg {
    #[inline(always)]
    fn default() -> Timer3Gpio2ConfReg {
        <crate::RegValueT<Timer3Gpio2ConfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3PrescalerReg_SPEC;
impl crate::sealed::RegSpec for Timer3PrescalerReg_SPEC {
    type DataType = u32;
}

pub type Timer3PrescalerReg = crate::RegValueT<Timer3PrescalerReg_SPEC>;

impl Timer3PrescalerReg {
    #[inline(always)]
    pub fn tim_prescaler(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        Timer3PrescalerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            Timer3PrescalerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer3PrescalerReg {
    #[inline(always)]
    fn default() -> Timer3PrescalerReg {
        <crate::RegValueT<Timer3PrescalerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3PrescalerValReg_SPEC;
impl crate::sealed::RegSpec for Timer3PrescalerValReg_SPEC {
    type DataType = u32;
}

pub type Timer3PrescalerValReg = crate::RegValueT<Timer3PrescalerValReg_SPEC>;

impl Timer3PrescalerValReg {
    #[inline(always)]
    pub fn tim_prescaler_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        Timer3PrescalerValReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            Timer3PrescalerValReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer3PrescalerValReg {
    #[inline(always)]
    fn default() -> Timer3PrescalerValReg {
        <crate::RegValueT<Timer3PrescalerValReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3PwmDcReg_SPEC;
impl crate::sealed::RegSpec for Timer3PwmDcReg_SPEC {
    type DataType = u32;
}

pub type Timer3PwmDcReg = crate::RegValueT<Timer3PwmDcReg_SPEC>;

impl Timer3PwmDcReg {
    #[inline(always)]
    pub fn tim_pwm_dc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Timer3PwmDcReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Timer3PwmDcReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer3PwmDcReg {
    #[inline(always)]
    fn default() -> Timer3PwmDcReg {
        <crate::RegValueT<Timer3PwmDcReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3PwmFreqReg_SPEC;
impl crate::sealed::RegSpec for Timer3PwmFreqReg_SPEC {
    type DataType = u32;
}

pub type Timer3PwmFreqReg = crate::RegValueT<Timer3PwmFreqReg_SPEC>;

impl Timer3PwmFreqReg {
    #[inline(always)]
    pub fn tim_pwm_freq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Timer3PwmFreqReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Timer3PwmFreqReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer3PwmFreqReg {
    #[inline(always)]
    fn default() -> Timer3PwmFreqReg {
        <crate::RegValueT<Timer3PwmFreqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3ReloadReg_SPEC;
impl crate::sealed::RegSpec for Timer3ReloadReg_SPEC {
    type DataType = u32;
}

pub type Timer3ReloadReg = crate::RegValueT<Timer3ReloadReg_SPEC>;

impl Timer3ReloadReg {
    #[inline(always)]
    pub fn tim_reload(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        Timer3ReloadReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Timer3ReloadReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer3ReloadReg {
    #[inline(always)]
    fn default() -> Timer3ReloadReg {
        <crate::RegValueT<Timer3ReloadReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3StatusReg_SPEC;
impl crate::sealed::RegSpec for Timer3StatusReg_SPEC {
    type DataType = u32;
}

pub type Timer3StatusReg = crate::RegValueT<Timer3StatusReg_SPEC>;

impl Timer3StatusReg {
    #[inline(always)]
    pub fn tim_oneshot_phase(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Timer3StatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Timer3StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_in2_state(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Timer3StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Timer3StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_in1_state(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Timer3StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Timer3StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Timer3StatusReg {
    #[inline(always)]
    fn default() -> Timer3StatusReg {
        <crate::RegValueT<Timer3StatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3TimerValReg_SPEC;
impl crate::sealed::RegSpec for Timer3TimerValReg_SPEC {
    type DataType = u32;
}

pub type Timer3TimerValReg = crate::RegValueT<Timer3TimerValReg_SPEC>;

impl Timer3TimerValReg {
    #[inline(always)]
    pub fn tim_timer_value(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        Timer3TimerValReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Timer3TimerValReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer3TimerValReg {
    #[inline(always)]
    fn default() -> Timer3TimerValReg {
        <crate::RegValueT<Timer3TimerValReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
