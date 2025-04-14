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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:15 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"SNC registers"]
unsafe impl ::core::marker::Send for super::Snc {}
unsafe impl ::core::marker::Sync for super::Snc {}
impl super::Snc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn snc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn snc_lp_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncLpTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncLpTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn snc_pc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncPcReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncPcReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn snc_r1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncR1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncR1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn snc_r2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncR2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncR2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn snc_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn snc_tmp1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncTmp1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncTmp1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn snc_tmp2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncTmp2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncTmp2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncCtrlReg_SPEC;
impl crate::sealed::RegSpec for SncCtrlReg_SPEC {
    type DataType = u32;
}

pub type SncCtrlReg = crate::RegValueT<SncCtrlReg_SPEC>;

impl SncCtrlReg {
    #[inline(always)]
    pub fn snc_irq_ack(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc_irq_config(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, SncCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc_branch_loop_init(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc_reset(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bus_error_detect_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc_sw_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SncCtrlReg {
    #[inline(always)]
    fn default() -> SncCtrlReg {
        <crate::RegValueT<SncCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncLpTimerReg_SPEC;
impl crate::sealed::RegSpec for SncLpTimerReg_SPEC {
    type DataType = u32;
}

pub type SncLpTimerReg = crate::RegValueT<SncLpTimerReg_SPEC>;

impl SncLpTimerReg {
    #[inline(always)]
    pub fn lp_timer(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, SncLpTimerReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,SncLpTimerReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SncLpTimerReg {
    #[inline(always)]
    fn default() -> SncLpTimerReg {
        <crate::RegValueT<SncLpTimerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncPcReg_SPEC;
impl crate::sealed::RegSpec for SncPcReg_SPEC {
    type DataType = u32;
}

pub type SncPcReg = crate::RegValueT<SncPcReg_SPEC>;

impl SncPcReg {
    #[inline(always)]
    pub fn pc_reg(
        self,
    ) -> crate::common::RegisterField<2, 0x1ffff, 1, 0, u32, u32, SncPcReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1ffff,1,0,u32,u32,SncPcReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SncPcReg {
    #[inline(always)]
    fn default() -> SncPcReg {
        <crate::RegValueT<SncPcReg_SPEC> as RegisterValue<_>>::new(536870912)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncR1Reg_SPEC;
impl crate::sealed::RegSpec for SncR1Reg_SPEC {
    type DataType = u32;
}

pub type SncR1Reg = crate::RegValueT<SncR1Reg_SPEC>;

impl SncR1Reg {
    #[inline(always)]
    pub fn r1_reg(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SncR1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SncR1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SncR1Reg {
    #[inline(always)]
    fn default() -> SncR1Reg {
        <crate::RegValueT<SncR1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncR2Reg_SPEC;
impl crate::sealed::RegSpec for SncR2Reg_SPEC {
    type DataType = u32;
}

pub type SncR2Reg = crate::RegValueT<SncR2Reg_SPEC>;

impl SncR2Reg {
    #[inline(always)]
    pub fn r2_reg(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SncR2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SncR2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SncR2Reg {
    #[inline(always)]
    fn default() -> SncR2Reg {
        <crate::RegValueT<SncR2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncStatusReg_SPEC;
impl crate::sealed::RegSpec for SncStatusReg_SPEC {
    type DataType = u32;
}

pub type SncStatusReg = crate::RegValueT<SncStatusReg_SPEC>;

impl SncStatusReg {
    #[inline(always)]
    pub fn snc_pc_loaded(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SncStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,SncStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc_is_stopped(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SncStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,SncStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hard_fault_status(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SncStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,SncStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bus_error_status(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SncStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SncStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc_done_status(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SncStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,SncStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gr_flag(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SncStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SncStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eq_flag(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SncStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SncStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SncStatusReg {
    #[inline(always)]
    fn default() -> SncStatusReg {
        <crate::RegValueT<SncStatusReg_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncTmp1Reg_SPEC;
impl crate::sealed::RegSpec for SncTmp1Reg_SPEC {
    type DataType = u32;
}

pub type SncTmp1Reg = crate::RegValueT<SncTmp1Reg_SPEC>;

impl SncTmp1Reg {
    #[inline(always)]
    pub fn tmp1_reg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        SncTmp1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            SncTmp1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SncTmp1Reg {
    #[inline(always)]
    fn default() -> SncTmp1Reg {
        <crate::RegValueT<SncTmp1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncTmp2Reg_SPEC;
impl crate::sealed::RegSpec for SncTmp2Reg_SPEC {
    type DataType = u32;
}

pub type SncTmp2Reg = crate::RegValueT<SncTmp2Reg_SPEC>;

impl SncTmp2Reg {
    #[inline(always)]
    pub fn tmp2_reg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        SncTmp2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            SncTmp2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SncTmp2Reg {
    #[inline(always)]
    fn default() -> SncTmp2Reg {
        <crate::RegValueT<SncTmp2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}
