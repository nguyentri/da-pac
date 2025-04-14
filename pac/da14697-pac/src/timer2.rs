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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:28 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"TIMER2 registers"]
unsafe impl ::core::marker::Send for super::Timer2 {}
unsafe impl ::core::marker::Sync for super::Timer2 {}
impl super::Timer2 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn timer2_capture_gpio1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer2CaptureGpio1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer2CaptureGpio1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer2_capture_gpio2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer2CaptureGpio2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer2CaptureGpio2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer2_clear_irq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer2ClearIrqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer2ClearIrqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer2_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer2CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer2CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer2_gpio1_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer2Gpio1ConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer2Gpio1ConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer2_gpio2_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer2Gpio2ConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer2Gpio2ConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer2_prescaler_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer2PrescalerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer2PrescalerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer2_prescaler_val_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer2PrescalerValReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer2PrescalerValReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer2_pwm_dc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer2PwmDcReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer2PwmDcReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer2_pwm_freq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer2PwmFreqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer2PwmFreqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer2_reload_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer2ReloadReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer2ReloadReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer2_shotwidth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer2ShotwidthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer2ShotwidthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer2_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer2StatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer2StatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer2_timer_val_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer2TimerValReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer2TimerValReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2CaptureGpio1Reg_SPEC;
impl crate::sealed::RegSpec for Timer2CaptureGpio1Reg_SPEC {
    type DataType = u32;
}

pub type Timer2CaptureGpio1Reg = crate::RegValueT<Timer2CaptureGpio1Reg_SPEC>;

impl Timer2CaptureGpio1Reg {
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
        Timer2CaptureGpio1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Timer2CaptureGpio1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer2CaptureGpio1Reg {
    #[inline(always)]
    fn default() -> Timer2CaptureGpio1Reg {
        <crate::RegValueT<Timer2CaptureGpio1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2CaptureGpio2Reg_SPEC;
impl crate::sealed::RegSpec for Timer2CaptureGpio2Reg_SPEC {
    type DataType = u32;
}

pub type Timer2CaptureGpio2Reg = crate::RegValueT<Timer2CaptureGpio2Reg_SPEC>;

impl Timer2CaptureGpio2Reg {
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
        Timer2CaptureGpio2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Timer2CaptureGpio2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer2CaptureGpio2Reg {
    #[inline(always)]
    fn default() -> Timer2CaptureGpio2Reg {
        <crate::RegValueT<Timer2CaptureGpio2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2ClearIrqReg_SPEC;
impl crate::sealed::RegSpec for Timer2ClearIrqReg_SPEC {
    type DataType = u32;
}

pub type Timer2ClearIrqReg = crate::RegValueT<Timer2ClearIrqReg_SPEC>;

impl Timer2ClearIrqReg {
    #[inline(always)]
    pub fn tim_clear_irq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Timer2ClearIrqReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Timer2ClearIrqReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Timer2ClearIrqReg {
    #[inline(always)]
    fn default() -> Timer2ClearIrqReg {
        <crate::RegValueT<Timer2ClearIrqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2CtrlReg_SPEC;
impl crate::sealed::RegSpec for Timer2CtrlReg_SPEC {
    type DataType = u32;
}

pub type Timer2CtrlReg = crate::RegValueT<Timer2CtrlReg_SPEC>;

impl Timer2CtrlReg {
    #[inline(always)]
    pub fn tim_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Timer2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Timer2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_sys_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Timer2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Timer2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_free_run_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Timer2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Timer2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Timer2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Timer2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_in2_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Timer2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Timer2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_in1_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Timer2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Timer2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_count_down_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Timer2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Timer2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_oneshot_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Timer2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Timer2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Timer2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Timer2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Timer2CtrlReg {
    #[inline(always)]
    fn default() -> Timer2CtrlReg {
        <crate::RegValueT<Timer2CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2Gpio1ConfReg_SPEC;
impl crate::sealed::RegSpec for Timer2Gpio1ConfReg_SPEC {
    type DataType = u32;
}

pub type Timer2Gpio1ConfReg = crate::RegValueT<Timer2Gpio1ConfReg_SPEC>;

impl Timer2Gpio1ConfReg {
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
        Timer2Gpio1ConfReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            Timer2Gpio1ConfReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer2Gpio1ConfReg {
    #[inline(always)]
    fn default() -> Timer2Gpio1ConfReg {
        <crate::RegValueT<Timer2Gpio1ConfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2Gpio2ConfReg_SPEC;
impl crate::sealed::RegSpec for Timer2Gpio2ConfReg_SPEC {
    type DataType = u32;
}

pub type Timer2Gpio2ConfReg = crate::RegValueT<Timer2Gpio2ConfReg_SPEC>;

impl Timer2Gpio2ConfReg {
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
        Timer2Gpio2ConfReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            Timer2Gpio2ConfReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer2Gpio2ConfReg {
    #[inline(always)]
    fn default() -> Timer2Gpio2ConfReg {
        <crate::RegValueT<Timer2Gpio2ConfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2PrescalerReg_SPEC;
impl crate::sealed::RegSpec for Timer2PrescalerReg_SPEC {
    type DataType = u32;
}

pub type Timer2PrescalerReg = crate::RegValueT<Timer2PrescalerReg_SPEC>;

impl Timer2PrescalerReg {
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
        Timer2PrescalerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            Timer2PrescalerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer2PrescalerReg {
    #[inline(always)]
    fn default() -> Timer2PrescalerReg {
        <crate::RegValueT<Timer2PrescalerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2PrescalerValReg_SPEC;
impl crate::sealed::RegSpec for Timer2PrescalerValReg_SPEC {
    type DataType = u32;
}

pub type Timer2PrescalerValReg = crate::RegValueT<Timer2PrescalerValReg_SPEC>;

impl Timer2PrescalerValReg {
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
        Timer2PrescalerValReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            Timer2PrescalerValReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer2PrescalerValReg {
    #[inline(always)]
    fn default() -> Timer2PrescalerValReg {
        <crate::RegValueT<Timer2PrescalerValReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2PwmDcReg_SPEC;
impl crate::sealed::RegSpec for Timer2PwmDcReg_SPEC {
    type DataType = u32;
}

pub type Timer2PwmDcReg = crate::RegValueT<Timer2PwmDcReg_SPEC>;

impl Timer2PwmDcReg {
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
        Timer2PwmDcReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Timer2PwmDcReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer2PwmDcReg {
    #[inline(always)]
    fn default() -> Timer2PwmDcReg {
        <crate::RegValueT<Timer2PwmDcReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2PwmFreqReg_SPEC;
impl crate::sealed::RegSpec for Timer2PwmFreqReg_SPEC {
    type DataType = u32;
}

pub type Timer2PwmFreqReg = crate::RegValueT<Timer2PwmFreqReg_SPEC>;

impl Timer2PwmFreqReg {
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
        Timer2PwmFreqReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Timer2PwmFreqReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer2PwmFreqReg {
    #[inline(always)]
    fn default() -> Timer2PwmFreqReg {
        <crate::RegValueT<Timer2PwmFreqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2ReloadReg_SPEC;
impl crate::sealed::RegSpec for Timer2ReloadReg_SPEC {
    type DataType = u32;
}

pub type Timer2ReloadReg = crate::RegValueT<Timer2ReloadReg_SPEC>;

impl Timer2ReloadReg {
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
        Timer2ReloadReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Timer2ReloadReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer2ReloadReg {
    #[inline(always)]
    fn default() -> Timer2ReloadReg {
        <crate::RegValueT<Timer2ReloadReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2ShotwidthReg_SPEC;
impl crate::sealed::RegSpec for Timer2ShotwidthReg_SPEC {
    type DataType = u32;
}

pub type Timer2ShotwidthReg = crate::RegValueT<Timer2ShotwidthReg_SPEC>;

impl Timer2ShotwidthReg {
    #[inline(always)]
    pub fn tim_shotwidth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        Timer2ShotwidthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Timer2ShotwidthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer2ShotwidthReg {
    #[inline(always)]
    fn default() -> Timer2ShotwidthReg {
        <crate::RegValueT<Timer2ShotwidthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2StatusReg_SPEC;
impl crate::sealed::RegSpec for Timer2StatusReg_SPEC {
    type DataType = u32;
}

pub type Timer2StatusReg = crate::RegValueT<Timer2StatusReg_SPEC>;

impl Timer2StatusReg {
    #[inline(always)]
    pub fn tim_oneshot_phase(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Timer2StatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Timer2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_in2_state(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Timer2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Timer2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_in1_state(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Timer2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Timer2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Timer2StatusReg {
    #[inline(always)]
    fn default() -> Timer2StatusReg {
        <crate::RegValueT<Timer2StatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2TimerValReg_SPEC;
impl crate::sealed::RegSpec for Timer2TimerValReg_SPEC {
    type DataType = u32;
}

pub type Timer2TimerValReg = crate::RegValueT<Timer2TimerValReg_SPEC>;

impl Timer2TimerValReg {
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
        Timer2TimerValReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Timer2TimerValReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer2TimerValReg {
    #[inline(always)]
    fn default() -> Timer2TimerValReg {
        <crate::RegValueT<Timer2TimerValReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
