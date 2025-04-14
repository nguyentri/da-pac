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
#[doc = r"quadec580_gr01 registers"]
unsafe impl ::core::marker::Send for super::Quadec580Gr01 {}
unsafe impl ::core::marker::Sync for super::Quadec580Gr01 {}
impl super::Quadec580Gr01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn qdec_clockdiv_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QdecClockdivReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QdecClockdivReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qdec_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QdecCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QdecCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qdec_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QdecCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QdecCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qdec_xcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QdecXcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QdecXcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qdec_ycnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QdecYcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QdecYcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qdec_zcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QdecZcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QdecZcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdecClockdivReg_SPEC;
impl crate::sealed::RegSpec for QdecClockdivReg_SPEC {
    type DataType = u16;
}

pub type QdecClockdivReg = crate::RegValueT<QdecClockdivReg_SPEC>;

impl QdecClockdivReg {
    #[inline(always)]
    pub fn clock_divider(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        QdecClockdivReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            QdecClockdivReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QdecClockdivReg {
    #[inline(always)]
    fn default() -> QdecClockdivReg {
        <crate::RegValueT<QdecClockdivReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdecCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for QdecCtrl2Reg_SPEC {
    type DataType = u16;
}

pub type QdecCtrl2Reg = crate::RegValueT<QdecCtrl2Reg_SPEC>;

impl QdecCtrl2Reg {
    #[inline(always)]
    pub fn chz_port_sel(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, QdecCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,QdecCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn chy_port_sel(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, QdecCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,QdecCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn chx_port_sel(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, QdecCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,QdecCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QdecCtrl2Reg {
    #[inline(always)]
    fn default() -> QdecCtrl2Reg {
        <crate::RegValueT<QdecCtrl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdecCtrlReg_SPEC;
impl crate::sealed::RegSpec for QdecCtrlReg_SPEC {
    type DataType = u16;
}

pub type QdecCtrlReg = crate::RegValueT<QdecCtrlReg_SPEC>;

impl QdecCtrlReg {
    #[inline(always)]
    pub fn qd_irq_thres(
        self,
    ) -> crate::common::RegisterField<3, 0x7f, 1, 0, u8, u8, QdecCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7f,1,0,u8,u8,QdecCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qd_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, QdecCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,QdecCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qd_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, QdecCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,QdecCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qd_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, QdecCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,QdecCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QdecCtrlReg {
    #[inline(always)]
    fn default() -> QdecCtrlReg {
        <crate::RegValueT<QdecCtrlReg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdecXcntReg_SPEC;
impl crate::sealed::RegSpec for QdecXcntReg_SPEC {
    type DataType = u16;
}

pub type QdecXcntReg = crate::RegValueT<QdecXcntReg_SPEC>;

impl QdecXcntReg {
    #[inline(always)]
    pub fn x_counter(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, QdecXcntReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,QdecXcntReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for QdecXcntReg {
    #[inline(always)]
    fn default() -> QdecXcntReg {
        <crate::RegValueT<QdecXcntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdecYcntReg_SPEC;
impl crate::sealed::RegSpec for QdecYcntReg_SPEC {
    type DataType = u16;
}

pub type QdecYcntReg = crate::RegValueT<QdecYcntReg_SPEC>;

impl QdecYcntReg {
    #[inline(always)]
    pub fn y_counter(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, QdecYcntReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,QdecYcntReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for QdecYcntReg {
    #[inline(always)]
    fn default() -> QdecYcntReg {
        <crate::RegValueT<QdecYcntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdecZcntReg_SPEC;
impl crate::sealed::RegSpec for QdecZcntReg_SPEC {
    type DataType = u16;
}

pub type QdecZcntReg = crate::RegValueT<QdecZcntReg_SPEC>;

impl QdecZcntReg {
    #[inline(always)]
    pub fn z_counter(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, QdecZcntReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,QdecZcntReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for QdecZcntReg {
    #[inline(always)]
    fn default() -> QdecZcntReg {
        <crate::RegValueT<QdecZcntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
