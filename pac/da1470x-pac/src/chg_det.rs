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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:41 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CHG_DET registers"]
unsafe impl ::core::marker::Send for super::ChgDet {}
unsafe impl ::core::marker::Sync for super::ChgDet {}
impl super::ChgDet {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn chg_det_adc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetAdcCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetAdcCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn chg_det_dcd_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetDcdTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetDcdTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn chg_det_fsm_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetFsmCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetFsmCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn chg_det_fsm_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetFsmStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetFsmStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn chg_det_irq_clear_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetIrqClearReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetIrqClearReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn chg_det_irq_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetIrqMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetIrqMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn chg_det_irq_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetIrqStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetIrqStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn chg_det_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn chg_det_sw_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetSwCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetSwCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn chg_det_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetAdcCtrlReg_SPEC;
impl crate::sealed::RegSpec for ChgDetAdcCtrlReg_SPEC {
    type DataType = u32;
}

pub type ChgDetAdcCtrlReg = crate::RegValueT<ChgDetAdcCtrlReg_SPEC>;

impl ChgDetAdcCtrlReg {
    #[inline(always)]
    pub fn adc_v30_sel(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChgDetAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ChgDetAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_dm_to_adc_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChgDetAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ChgDetAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_dp_to_adc_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetAdcCtrlReg {
    #[inline(always)]
    fn default() -> ChgDetAdcCtrlReg {
        <crate::RegValueT<ChgDetAdcCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetDcdTimerReg_SPEC;
impl crate::sealed::RegSpec for ChgDetDcdTimerReg_SPEC {
    type DataType = u32;
}

pub type ChgDetDcdTimerReg = crate::RegValueT<ChgDetDcdTimerReg_SPEC>;

impl ChgDetDcdTimerReg {
    #[inline(always)]
    pub fn dcd_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChgDetDcdTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChgDetDcdTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcd_timeout_thres(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChgDetDcdTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChgDetDcdTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChgDetDcdTimerReg {
    #[inline(always)]
    fn default() -> ChgDetDcdTimerReg {
        <crate::RegValueT<ChgDetDcdTimerReg_SPEC> as RegisterValue<_>>::new(39322200)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetFsmCtrlReg_SPEC;
impl crate::sealed::RegSpec for ChgDetFsmCtrlReg_SPEC {
    type DataType = u32;
}

pub type ChgDetFsmCtrlReg = crate::RegValueT<ChgDetFsmCtrlReg_SPEC>;

impl ChgDetFsmCtrlReg {
    #[inline(always)]
    pub fn chg_det_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetFsmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetFsmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetFsmCtrlReg {
    #[inline(always)]
    fn default() -> ChgDetFsmCtrlReg {
        <crate::RegValueT<ChgDetFsmCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetFsmStatusReg_SPEC;
impl crate::sealed::RegSpec for ChgDetFsmStatusReg_SPEC {
    type DataType = u32;
}

pub type ChgDetFsmStatusReg = crate::RegValueT<ChgDetFsmStatusReg_SPEC>;

impl ChgDetFsmStatusReg {
    #[inline(always)]
    pub fn chg_det_state(
        self,
    ) -> crate::common::RegisterField<9, 0xf, 1, 0, u8, u8, ChgDetFsmStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            9,
            0xf,
            1,
            0,
            u8,
            u8,
            ChgDetFsmStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn no_contact_detected(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn port_2p4amp_detected(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn port_2amp_detected(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn port_1amp_detected(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ps2_prop_port_detected(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcp_port_detected(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cdp_port_detected(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdp_port_detected(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn detection_completed(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetFsmStatusReg {
    #[inline(always)]
    fn default() -> ChgDetFsmStatusReg {
        <crate::RegValueT<ChgDetFsmStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetIrqClearReg_SPEC;
impl crate::sealed::RegSpec for ChgDetIrqClearReg_SPEC {
    type DataType = u32;
}

pub type ChgDetIrqClearReg = crate::RegValueT<ChgDetIrqClearReg_SPEC>;

impl ChgDetIrqClearReg {
    #[inline(always)]
    pub fn chg_det_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetIrqClearReg {
    #[inline(always)]
    fn default() -> ChgDetIrqClearReg {
        <crate::RegValueT<ChgDetIrqClearReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetIrqMaskReg_SPEC;
impl crate::sealed::RegSpec for ChgDetIrqMaskReg_SPEC {
    type DataType = u32;
}

pub type ChgDetIrqMaskReg = crate::RegValueT<ChgDetIrqMaskReg_SPEC>;

impl ChgDetIrqMaskReg {
    #[inline(always)]
    pub fn chg_det_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetIrqMaskReg {
    #[inline(always)]
    fn default() -> ChgDetIrqMaskReg {
        <crate::RegValueT<ChgDetIrqMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetIrqStatusReg_SPEC;
impl crate::sealed::RegSpec for ChgDetIrqStatusReg_SPEC {
    type DataType = u32;
}

pub type ChgDetIrqStatusReg = crate::RegValueT<ChgDetIrqStatusReg_SPEC>;

impl ChgDetIrqStatusReg {
    #[inline(always)]
    pub fn chg_det_irq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetIrqStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetIrqStatusReg {
    #[inline(always)]
    fn default() -> ChgDetIrqStatusReg {
        <crate::RegValueT<ChgDetIrqStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetStatusReg_SPEC;
impl crate::sealed::RegSpec for ChgDetStatusReg_SPEC {
    type DataType = u32;
}

pub type ChgDetStatusReg = crate::RegValueT<ChgDetStatusReg_SPEC>;

impl ChgDetStatusReg {
    #[inline(always)]
    pub fn usb_dm_val2(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChgDetStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,ChgDetStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_dp_val2(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChgDetStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,ChgDetStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_dm_val(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChgDetStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,ChgDetStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_dp_val(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChgDetStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,ChgDetStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_chg_det(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChgDetStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,ChgDetStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_dcp_det(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetStatusReg {
    #[inline(always)]
    fn default() -> ChgDetStatusReg {
        <crate::RegValueT<ChgDetStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetSwCtrlReg_SPEC;
impl crate::sealed::RegSpec for ChgDetSwCtrlReg_SPEC {
    type DataType = u32;
}

pub type ChgDetSwCtrlReg = crate::RegValueT<ChgDetSwCtrlReg_SPEC>;

impl ChgDetSwCtrlReg {
    #[inline(always)]
    pub fn idm_sink_on(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChgDetSwCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ChgDetSwCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn idp_sink_on(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChgDetSwCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ChgDetSwCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vdm_src_on(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChgDetSwCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ChgDetSwCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vdp_src_on(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChgDetSwCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ChgDetSwCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn idp_src_on(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChgDetSwCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ChgDetSwCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_charge_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetSwCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetSwCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetSwCtrlReg {
    #[inline(always)]
    fn default() -> ChgDetSwCtrlReg {
        <crate::RegValueT<ChgDetSwCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetTimerReg_SPEC;
impl crate::sealed::RegSpec for ChgDetTimerReg_SPEC {
    type DataType = u32;
}

pub type ChgDetTimerReg = crate::RegValueT<ChgDetTimerReg_SPEC>;

impl ChgDetTimerReg {
    #[inline(always)]
    pub fn chg_det_timer(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, ChgDetTimerReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,ChgDetTimerReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn chg_det_timer_thres(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, ChgDetTimerReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,ChgDetTimerReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetTimerReg {
    #[inline(always)]
    fn default() -> ChgDetTimerReg {
        <crate::RegValueT<ChgDetTimerReg_SPEC> as RegisterValue<_>>::new(3276850)
    }
}
