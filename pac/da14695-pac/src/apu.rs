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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:21 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"APU registers"]
unsafe impl ::core::marker::Send for super::Apu {}
unsafe impl ::core::marker::Sync for super::Apu {}
impl super::Apu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn apu_mux_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ApuMuxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ApuMuxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coef0a_set1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Coef0ASet1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Coef0ASet1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coef10_set1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Coef10Set1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Coef10Set1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coef32_set1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Coef32Set1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Coef32Set1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coef54_set1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Coef54Set1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Coef54Set1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coef76_set1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Coef76Set1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Coef76Set1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coef98_set1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Coef98Set1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Coef98Set1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pcm1_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Pcm1CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pcm1CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pcm1_in1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Pcm1In1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pcm1In1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pcm1_in2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Pcm1In2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pcm1In2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pcm1_out1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Pcm1Out1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pcm1Out1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pcm1_out2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Pcm1Out2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pcm1Out2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[inline(always)]
    pub const fn src1_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Src1CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Src1CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn src1_in1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Src1In1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Src1In1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn src1_in2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Src1In2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Src1In2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn src1_in_fs_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Src1InFsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Src1InFsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn src1_out1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Src1Out1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Src1Out1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn src1_out2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Src1Out2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Src1Out2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn src1_out_fs_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Src1OutFsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Src1OutFsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApuMuxReg_SPEC;
impl crate::sealed::RegSpec for ApuMuxReg_SPEC {
    type DataType = u32;
}

pub type ApuMuxReg = crate::RegValueT<ApuMuxReg_SPEC>;

impl ApuMuxReg {
    #[inline(always)]
    pub fn pdm1_mux_in(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ApuMuxReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ApuMuxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pcm1_mux_in(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, ApuMuxReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,ApuMuxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src1_mux_in(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, ApuMuxReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,ApuMuxReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ApuMuxReg {
    #[inline(always)]
    fn default() -> ApuMuxReg {
        <crate::RegValueT<ApuMuxReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coef0ASet1Reg_SPEC;
impl crate::sealed::RegSpec for Coef0ASet1Reg_SPEC {
    type DataType = u32;
}

pub type Coef0ASet1Reg = crate::RegValueT<Coef0ASet1Reg_SPEC>;

impl Coef0ASet1Reg {
    #[inline(always)]
    pub fn src_coef10(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Coef0ASet1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Coef0ASet1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Coef0ASet1Reg {
    #[inline(always)]
    fn default() -> Coef0ASet1Reg {
        <crate::RegValueT<Coef0ASet1Reg_SPEC> as RegisterValue<_>>::new(16882)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coef10Set1Reg_SPEC;
impl crate::sealed::RegSpec for Coef10Set1Reg_SPEC {
    type DataType = u32;
}

pub type Coef10Set1Reg = crate::RegValueT<Coef10Set1Reg_SPEC>;

impl Coef10Set1Reg {
    #[inline(always)]
    pub fn src_coef1(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Coef10Set1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Coef10Set1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn src_coef0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Coef10Set1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Coef10Set1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Coef10Set1Reg {
    #[inline(always)]
    fn default() -> Coef10Set1Reg {
        <crate::RegValueT<Coef10Set1Reg_SPEC> as RegisterValue<_>>::new(2041156216)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coef32Set1Reg_SPEC;
impl crate::sealed::RegSpec for Coef32Set1Reg_SPEC {
    type DataType = u32;
}

pub type Coef32Set1Reg = crate::RegValueT<Coef32Set1Reg_SPEC>;

impl Coef32Set1Reg {
    #[inline(always)]
    pub fn src_coef3(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Coef32Set1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Coef32Set1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn src_coef2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Coef32Set1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Coef32Set1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Coef32Set1Reg {
    #[inline(always)]
    fn default() -> Coef32Set1Reg {
        <crate::RegValueT<Coef32Set1Reg_SPEC> as RegisterValue<_>>::new(1834388289)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coef54Set1Reg_SPEC;
impl crate::sealed::RegSpec for Coef54Set1Reg_SPEC {
    type DataType = u32;
}

pub type Coef54Set1Reg = crate::RegValueT<Coef54Set1Reg_SPEC>;

impl Coef54Set1Reg {
    #[inline(always)]
    pub fn src_coef5(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Coef54Set1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Coef54Set1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn src_coef4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Coef54Set1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Coef54Set1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Coef54Set1Reg {
    #[inline(always)]
    fn default() -> Coef54Set1Reg {
        <crate::RegValueT<Coef54Set1Reg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coef76Set1Reg_SPEC;
impl crate::sealed::RegSpec for Coef76Set1Reg_SPEC {
    type DataType = u32;
}

pub type Coef76Set1Reg = crate::RegValueT<Coef76Set1Reg_SPEC>;

impl Coef76Set1Reg {
    #[inline(always)]
    pub fn src_coef7(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Coef76Set1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Coef76Set1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn src_coef6(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Coef76Set1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Coef76Set1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Coef76Set1Reg {
    #[inline(always)]
    fn default() -> Coef76Set1Reg {
        <crate::RegValueT<Coef76Set1Reg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coef98Set1Reg_SPEC;
impl crate::sealed::RegSpec for Coef98Set1Reg_SPEC {
    type DataType = u32;
}

pub type Coef98Set1Reg = crate::RegValueT<Coef98Set1Reg_SPEC>;

impl Coef98Set1Reg {
    #[inline(always)]
    pub fn src_coef9(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Coef98Set1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Coef98Set1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn src_coef8(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Coef98Set1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Coef98Set1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Coef98Set1Reg {
    #[inline(always)]
    fn default() -> Coef98Set1Reg {
        <crate::RegValueT<Coef98Set1Reg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcm1CtrlReg_SPEC;
impl crate::sealed::RegSpec for Pcm1CtrlReg_SPEC {
    type DataType = u32;
}

pub type Pcm1CtrlReg = crate::RegValueT<Pcm1CtrlReg_SPEC>;

impl Pcm1CtrlReg {
    #[inline(always)]
    pub fn pcm_fsc_div(
        self,
    ) -> crate::common::RegisterField<20, 0xfff, 1, 0, u16, u16, Pcm1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xfff,1,0,u16,u16,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pcm_fsc_edge(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pcm_ch_del(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, u8, Pcm1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1f,1,0,u8,u8,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pcm_clk_bit(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pcm_fscinv(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pcm_clkinv(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pcm_ppod(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pcm_fscdel(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pcm_fsclen(
        self,
    ) -> crate::common::RegisterField<2, 0xf, 1, 0, u8, u8, Pcm1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0xf,1,0,u8,u8,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pcm_master(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pcm_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pcm1CtrlReg {
    #[inline(always)]
    fn default() -> Pcm1CtrlReg {
        <crate::RegValueT<Pcm1CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcm1In1Reg_SPEC;
impl crate::sealed::RegSpec for Pcm1In1Reg_SPEC {
    type DataType = u32;
}

pub type Pcm1In1Reg = crate::RegValueT<Pcm1In1Reg_SPEC>;

impl Pcm1In1Reg {
    #[inline(always)]
    pub fn pcm_in(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Pcm1In1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Pcm1In1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pcm1In1Reg {
    #[inline(always)]
    fn default() -> Pcm1In1Reg {
        <crate::RegValueT<Pcm1In1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcm1In2Reg_SPEC;
impl crate::sealed::RegSpec for Pcm1In2Reg_SPEC {
    type DataType = u32;
}

pub type Pcm1In2Reg = crate::RegValueT<Pcm1In2Reg_SPEC>;

impl Pcm1In2Reg {
    #[inline(always)]
    pub fn pcm_in(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Pcm1In2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Pcm1In2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pcm1In2Reg {
    #[inline(always)]
    fn default() -> Pcm1In2Reg {
        <crate::RegValueT<Pcm1In2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcm1Out1Reg_SPEC;
impl crate::sealed::RegSpec for Pcm1Out1Reg_SPEC {
    type DataType = u32;
}

pub type Pcm1Out1Reg = crate::RegValueT<Pcm1Out1Reg_SPEC>;

impl Pcm1Out1Reg {
    #[inline(always)]
    pub fn pcm_out(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Pcm1Out1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Pcm1Out1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pcm1Out1Reg {
    #[inline(always)]
    fn default() -> Pcm1Out1Reg {
        <crate::RegValueT<Pcm1Out1Reg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcm1Out2Reg_SPEC;
impl crate::sealed::RegSpec for Pcm1Out2Reg_SPEC {
    type DataType = u32;
}

pub type Pcm1Out2Reg = crate::RegValueT<Pcm1Out2Reg_SPEC>;

impl Pcm1Out2Reg {
    #[inline(always)]
    pub fn pcm_out(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Pcm1Out2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Pcm1Out2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pcm1Out2Reg {
    #[inline(always)]
    fn default() -> Pcm1Out2Reg {
        <crate::RegValueT<Pcm1Out2Reg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src1CtrlReg_SPEC;
impl crate::sealed::RegSpec for Src1CtrlReg_SPEC {
    type DataType = u32;
}

pub type Src1CtrlReg = crate::RegValueT<Src1CtrlReg_SPEC>;

impl Src1CtrlReg {
    #[inline(always)]
    pub fn src_pdm_do_del(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, Src1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_pdm_mode(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, u8, Src1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,u8,u8,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_pdm_di_del(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, u8, Src1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x3,1,0,u8,u8,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_out_flowclr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Src1CtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25,1,0,Src1CtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_in_flowclr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Src1CtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24,1,0,Src1CtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_out_unflow(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Src1CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,Src1CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_out_ovflow(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Src1CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22,1,0,Src1CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_in_unflow(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Src1CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,Src1CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_in_ovflow(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Src1CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,Src1CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_resync(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Src1CtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19,1,0,Src1CtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_out_ok(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Src1CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,Src1CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_out_us(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, Src1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_out_cal_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_out_amode(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_pdm_out_inv(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_fifo_direction(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_fifo_enable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_out_dsd_mode(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_in_dsd_mode(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_dither_disable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_in_ok(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Src1CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,Src1CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_in_ds(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Src1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_pdm_in_inv(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_in_cal_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_in_amode(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Src1CtrlReg {
    #[inline(always)]
    fn default() -> Src1CtrlReg {
        <crate::RegValueT<Src1CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src1In1Reg_SPEC;
impl crate::sealed::RegSpec for Src1In1Reg_SPEC {
    type DataType = u32;
}

pub type Src1In1Reg = crate::RegValueT<Src1In1Reg_SPEC>;

impl Src1In1Reg {
    #[inline(always)]
    pub fn src_in(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Src1In1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Src1In1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Src1In1Reg {
    #[inline(always)]
    fn default() -> Src1In1Reg {
        <crate::RegValueT<Src1In1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src1In2Reg_SPEC;
impl crate::sealed::RegSpec for Src1In2Reg_SPEC {
    type DataType = u32;
}

pub type Src1In2Reg = crate::RegValueT<Src1In2Reg_SPEC>;

impl Src1In2Reg {
    #[inline(always)]
    pub fn src_in(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Src1In2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Src1In2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Src1In2Reg {
    #[inline(always)]
    fn default() -> Src1In2Reg {
        <crate::RegValueT<Src1In2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src1InFsReg_SPEC;
impl crate::sealed::RegSpec for Src1InFsReg_SPEC {
    type DataType = u32;
}

pub type Src1InFsReg = crate::RegValueT<Src1InFsReg_SPEC>;

impl Src1InFsReg {
    #[inline(always)]
    pub fn src_in_fs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        Src1InFsReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Src1InFsReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Src1InFsReg {
    #[inline(always)]
    fn default() -> Src1InFsReg {
        <crate::RegValueT<Src1InFsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src1Out1Reg_SPEC;
impl crate::sealed::RegSpec for Src1Out1Reg_SPEC {
    type DataType = u32;
}

pub type Src1Out1Reg = crate::RegValueT<Src1Out1Reg_SPEC>;

impl Src1Out1Reg {
    #[inline(always)]
    pub fn src_out(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Src1Out1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Src1Out1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Src1Out1Reg {
    #[inline(always)]
    fn default() -> Src1Out1Reg {
        <crate::RegValueT<Src1Out1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src1Out2Reg_SPEC;
impl crate::sealed::RegSpec for Src1Out2Reg_SPEC {
    type DataType = u32;
}

pub type Src1Out2Reg = crate::RegValueT<Src1Out2Reg_SPEC>;

impl Src1Out2Reg {
    #[inline(always)]
    pub fn src_out(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Src1Out2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Src1Out2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Src1Out2Reg {
    #[inline(always)]
    fn default() -> Src1Out2Reg {
        <crate::RegValueT<Src1Out2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src1OutFsReg_SPEC;
impl crate::sealed::RegSpec for Src1OutFsReg_SPEC {
    type DataType = u32;
}

pub type Src1OutFsReg = crate::RegValueT<Src1OutFsReg_SPEC>;

impl Src1OutFsReg {
    #[inline(always)]
    pub fn src_out_fs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        Src1OutFsReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Src1OutFsReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Src1OutFsReg {
    #[inline(always)]
    fn default() -> Src1OutFsReg {
        <crate::RegValueT<Src1OutFsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
