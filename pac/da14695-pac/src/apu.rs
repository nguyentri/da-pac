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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:31 +0000

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

    #[doc = "APU mux register"]
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

    #[doc = "SRC coefficient 10 set 1"]
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

    #[doc = "SRC coefficient 1,0 set 1"]
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

    #[doc = "SRC coefficient 3,2 set 1"]
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

    #[doc = "SRC coefficient 5,4 set 1"]
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

    #[doc = "SRC coefficient 7,6 set 1"]
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

    #[doc = "SRC coefficient 9,8 set 1"]
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

    #[doc = "PCM1 Control register"]
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

    #[doc = "PCM1 data in 1"]
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

    #[doc = "PCM1 data in 2"]
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

    #[doc = "PCM1 data out 1"]
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

    #[doc = "PCM1 data out 2"]
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

    #[doc = "SRC1 control register"]
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

    #[doc = "SRC1 data in 1"]
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

    #[doc = "SRC1 data in 2"]
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

    #[doc = "SRC1 Sample input rate"]
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

    #[doc = "SRC1 data out 1"]
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

    #[doc = "SRC1 data out 2"]
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

    #[doc = "SRC1 Sample output rate"]
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

#[doc = "APU mux register"]
pub type ApuMuxReg = crate::RegValueT<ApuMuxReg_SPEC>;

impl ApuMuxReg {
    #[doc = "PDM1 input mux\n0 = SRC1_MUX_IN\n1 = PDM input"]
    #[inline(always)]
    pub fn pdm1_mux_in(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ApuMuxReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ApuMuxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PCM1 input mux\n0 = off\n1 = SRC1 output\n2 = PCM output registers"]
    #[inline(always)]
    pub fn pcm1_mux_in(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, ApuMuxReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,ApuMuxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SRC1 input mux\n0 = off\n1 = PCM output\n2 = SRC1 input registers"]
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

#[doc = "SRC coefficient 10 set 1"]
pub type Coef0ASet1Reg = crate::RegValueT<Coef0ASet1Reg_SPEC>;

impl Coef0ASet1Reg {
    #[doc = "coefficient 10"]
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

#[doc = "SRC coefficient 1,0 set 1"]
pub type Coef10Set1Reg = crate::RegValueT<Coef10Set1Reg_SPEC>;

impl Coef10Set1Reg {
    #[doc = "coefficient 1"]
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

    #[doc = "coefficient 0"]
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

#[doc = "SRC coefficient 3,2 set 1"]
pub type Coef32Set1Reg = crate::RegValueT<Coef32Set1Reg_SPEC>;

impl Coef32Set1Reg {
    #[doc = "coefficient 3"]
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

    #[doc = "coefficient 2"]
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

#[doc = "SRC coefficient 5,4 set 1"]
pub type Coef54Set1Reg = crate::RegValueT<Coef54Set1Reg_SPEC>;

impl Coef54Set1Reg {
    #[doc = "coefficient 5"]
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

    #[doc = "coefficient 4"]
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

#[doc = "SRC coefficient 7,6 set 1"]
pub type Coef76Set1Reg = crate::RegValueT<Coef76Set1Reg_SPEC>;

impl Coef76Set1Reg {
    #[doc = "coefficient 7"]
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

    #[doc = "coefficient 6"]
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

#[doc = "SRC coefficient 9,8 set 1"]
pub type Coef98Set1Reg = crate::RegValueT<Coef98Set1Reg_SPEC>;

impl Coef98Set1Reg {
    #[doc = "coefficient 9"]
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

    #[doc = "coefficient 8"]
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

#[doc = "PCM1 Control register"]
pub type Pcm1CtrlReg = crate::RegValueT<Pcm1CtrlReg_SPEC>;

impl Pcm1CtrlReg {
    #[doc = "PCM Framesync divider, Values 7-0xFFF. To divide by N, write N-1. (Minimum value N-1=7 for 8 bits PCM_FSC)\nNote if PCM_CLK_BIT=1, N must always be even"]
    #[inline(always)]
    pub fn pcm_fsc_div(
        self,
    ) -> crate::common::RegisterField<20, 0xfff, 1, 0, u16, u16, Pcm1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xfff,1,0,u16,u16,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: shift channels 1, 2, 3, 4, 5, 6, 7, 8 after PCM_FSC edge\n1: shift channels 1, 2, 3, 4 after PCM_FSC edge shift channels 5, 6, 7, 8 after opposite PCM_FSC edge"]
    #[inline(always)]
    pub fn pcm_fsc_edge(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Channel delay in multiples of 8 bits"]
    #[inline(always)]
    pub fn pcm_ch_del(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, u8, Pcm1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1f,1,0,u8,u8,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0:One clock cycle per data bit\n1:Two cloc cycles per data bit"]
    #[inline(always)]
    pub fn pcm_clk_bit(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: PCM FSC\n1: PCM FSC inverted"]
    #[inline(always)]
    pub fn pcm_fscinv(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0:PCM CLK\n1:PCM CLK inverted"]
    #[inline(always)]
    pub fn pcm_clkinv(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0:PCM DO push pull\n1:PCM DO open drain"]
    #[inline(always)]
    pub fn pcm_ppod(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0:PCM FSC starts one cycle before MSB bit\n1:PCM FSC starts at the same time as MSB bit"]
    #[inline(always)]
    pub fn pcm_fscdel(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0:PCM FSC length equal to 1 data bit\nN:PCM FSC length equal to N*8"]
    #[inline(always)]
    pub fn pcm_fsclen(
        self,
    ) -> crate::common::RegisterField<2, 0xf, 1, 0, u8, u8, Pcm1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0xf,1,0,u8,u8,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0:PCM interface in slave mode\n1:PCM interface in master mode"]
    #[inline(always)]
    pub fn pcm_master(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Pcm1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Pcm1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0:PCM interface disabled\n1:PCM interface enabled"]
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

#[doc = "PCM1 data in 1"]
pub type Pcm1In1Reg = crate::RegValueT<Pcm1In1Reg_SPEC>;

impl Pcm1In1Reg {
    #[doc = "PCM1_IN1 bits 31-0"]
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

#[doc = "PCM1 data in 2"]
pub type Pcm1In2Reg = crate::RegValueT<Pcm1In2Reg_SPEC>;

impl Pcm1In2Reg {
    #[doc = "PCM1_IN2 bits 31-0"]
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

#[doc = "PCM1 data out 1"]
pub type Pcm1Out1Reg = crate::RegValueT<Pcm1Out1Reg_SPEC>;

impl Pcm1Out1Reg {
    #[doc = "PCM1_OUT1 bits 31-0"]
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

#[doc = "PCM1 data out 2"]
pub type Pcm1Out2Reg = crate::RegValueT<Pcm1Out2Reg_SPEC>;

impl Pcm1Out2Reg {
    #[doc = "PCM1_OUT2 bits 31-0"]
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

#[doc = "SRC1 control register"]
pub type Src1CtrlReg = crate::RegValueT<Src1CtrlReg_SPEC>;

impl Src1CtrlReg {
    #[doc = "PDM_DO output delay line (typical)\n0: no delay\n1: 8 ns\n2: 12 ns\n3: 16 ns"]
    #[inline(always)]
    pub fn src_pdm_do_del(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, Src1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PDM Output mode selection on PDM_DO1\n00: No output\n01: Right channel (data from SRC1_IN_REG)\n10: Left channel (data from SRC2_IN_REG)\n11: Left and Right channel"]
    #[inline(always)]
    pub fn src_pdm_mode(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, u8, Src1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,u8,u8,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PDM_DI input delay line (typical)\n0: no delay\n1: 4 ns\n2: 8 ns\n3: 12 ns"]
    #[inline(always)]
    pub fn src_pdm_di_del(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, u8, Src1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x3,1,0,u8,u8,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a 1 clears the SRC1_OUT Overflow/underflow bits 23-22. No more over/underflow indications while bit is 1. Keep 1 until the over/under flow bit is cleared"]
    #[inline(always)]
    pub fn src_out_flowclr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Src1CtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25,1,0,Src1CtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Writing a 1 clears the SRC1_IN Overflow/underflow bits 21-20. No more over/underflow indications while bit is 1. Keep 1 until the over/under flow bit is cleared"]
    #[inline(always)]
    pub fn src_in_flowclr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Src1CtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24,1,0,Src1CtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "1 = SRC1_OUT Underflow occurred"]
    #[inline(always)]
    pub fn src_out_unflow(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Src1CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,Src1CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1 = SRC1_OUT Overflow occurred"]
    #[inline(always)]
    pub fn src_out_ovflow(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Src1CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22,1,0,Src1CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1 = SRC1_IN Underflow occurred"]
    #[inline(always)]
    pub fn src_in_unflow(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Src1CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,Src1CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1 = SRC1_IN Overflow occurred"]
    #[inline(always)]
    pub fn src_in_ovflow(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Src1CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,Src1CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1 = SRC will restart synchronisation"]
    #[inline(always)]
    pub fn src_resync(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Src1CtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19,1,0,Src1CtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "SRC1_OUT Status\n0: acquisition in progress\n1: acquisition ready (In manual mode this bit is always 1)"]
    #[inline(always)]
    pub fn src_out_ok(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Src1CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,Src1CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SRC1_OUT UpSampling IIR filters setting\n00: for sample rates up-to 48kHz\n01: for sample rates of 96kHz\n10: reserved\n11: for sample rates of 192kHz"]
    #[inline(always)]
    pub fn src_out_us(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, Src1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SRC1_OUT1 upsampiling filter bypass\n0:Do not bypass\n1:Bypass filter"]
    #[inline(always)]
    pub fn src_out_cal_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SRC1_OUT1 Automatic Conversion mode\n0:Manual mode\n1:Automatic mode"]
    #[inline(always)]
    pub fn src_out_amode(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Swap the left and the right output PDM channel"]
    #[inline(always)]
    pub fn src_pdm_out_inv(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = SRC fifo is used to store samples from memory to SRC\n1 = SRC fifo is used to store sample from SRC to memory"]
    #[inline(always)]
    pub fn src_fifo_direction(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = fifo disable. On each src request, one sample is serviced\n1 = fifo enable. Fifo is used to store samples from / to src\nSRC supports only DMA burst size 4 when fifo is enable else no burst"]
    #[inline(always)]
    pub fn src_fifo_enable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = SRC1 OUT PDM mode\n1 = SRC1 OUT DSD mode"]
    #[inline(always)]
    pub fn src_out_dsd_mode(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: SRC1 IN PDM mode\n1: SRC1 IN DSD mode"]
    #[inline(always)]
    pub fn src_in_dsd_mode(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Dithering feature\n0: Enable\n1: Disable"]
    #[inline(always)]
    pub fn src_dither_disable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SRC1_IN status\n0: Acquisition in progress\n1: Acquisition ready"]
    #[inline(always)]
    pub fn src_in_ok(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Src1CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,Src1CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SRC1_IN UpSampling IIR filters setting\n00: for sample rates up-to 48kHz\n01: for sample rates of 96kHz\n10: reserved\n11: for sample rates of 192kHz"]
    #[inline(always)]
    pub fn src_in_ds(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Src1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Swap the left and the right input PDM channel"]
    #[inline(always)]
    pub fn src_pdm_in_inv(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SRC1_IN upsampeling filter bypass\n0: Do not bypass\n1: Bypass filter"]
    #[inline(always)]
    pub fn src_in_cal_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SRC1_IN Automatic conversion mode\n0: Manual mode\n1: Automatic mode"]
    #[inline(always)]
    pub fn src_in_amode(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Src1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Src1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SRC1_IN and SRC1_OUT enable\n0: disabled\n1: enabled"]
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

#[doc = "SRC1 data in 1"]
pub type Src1In1Reg = crate::RegValueT<Src1In1Reg_SPEC>;

impl Src1In1Reg {
    #[doc = "SRC1_IN1"]
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

#[doc = "SRC1 data in 2"]
pub type Src1In2Reg = crate::RegValueT<Src1In2Reg_SPEC>;

impl Src1In2Reg {
    #[doc = "SRC1_IN2"]
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

#[doc = "SRC1 Sample input rate"]
pub type Src1InFsReg = crate::RegValueT<Src1InFsReg_SPEC>;

impl Src1InFsReg {
    #[doc = "SRC_IN Sample rate\nSRC_IN_FS = SRC_DIV*4096*Sample_rate/100\nSample_rate upper limit is 192kHz. For 96kHz and 192kHz SRC_CTRLx_REG\\[SRC_IN_DS\\] must be set as shown below:\n(for SRC_DIV=1)\nSample_rate SRC_IN_FS SRC_IN_DS Audio bandwidth\n8000 Hz 0x050000 0 4000 Hz\n11025 Hz 0x06E400 0 5512 Hz\n16000 Hz 0x0A0000 0 8000 Hz\n22050 Hz 0x0DC800 0 11025 Hz\n32000 Hz 0x140000 0 16000 Hz\n44100 Hz 0x1B9000 0 22050 Hz\n48000 Hz 0x1E0000 0 24000 Hz\n96000 Hz 0x1E0000 1 24000 Hz\n192000 Hz 0x1E0000 3 24000 Hz\n\nIn manual SRC mode, SRC_IN_FS can be set and adjusted to the desired sample rate at any time.\nIn automatic mode the SRC returns the final sample rate as soon as SRC_IN_OK. Note that SRC_DS is not calculated in automatic mode and must be set manually automatic mode with Sample_rate of 96 and 192kHz."]
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

#[doc = "SRC1 data out 1"]
pub type Src1Out1Reg = crate::RegValueT<Src1Out1Reg_SPEC>;

impl Src1Out1Reg {
    #[doc = "SRC1_OUT1"]
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

#[doc = "SRC1 data out 2"]
pub type Src1Out2Reg = crate::RegValueT<Src1Out2Reg_SPEC>;

impl Src1Out2Reg {
    #[doc = "SRC1_OUT2"]
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

#[doc = "SRC1 Sample output rate"]
pub type Src1OutFsReg = crate::RegValueT<Src1OutFsReg_SPEC>;

impl Src1OutFsReg {
    #[doc = "SRC_OUT Sample rate\nSRC_OUT_FS = SRC_DIV*4096*Sample_rate/100\nSample_rate upper limit is 192kHz. For 96kHz and 192kHz SRC_CTRLx_REG\\[SRC_DS\\] must be set as shown below:\n(for SRC_DIV=1)\nSample_rate SRC_OUT_FS  SRC_OUT_DS Audio bandwidth\n8000 Hz     0x050000    0          4000 Hz\n11025 Hz    0x06E400    0          5512 Hz\n16000 Hz    0x0A0000    0          8000 Hz\n22050 Hz    0x0DC800    0          11025 Hz\n32000 Hz    0x140000    0          16000 Hz\n44100 Hz    0x1B9000    0          22050 Hz\n48000 Hz    0x1E0000    0          24000 Hz\n96000 Hz    0x1E0000    1          24000 Hz\n192000 Hz   0x1E0000    3          24000 Hz\n\nIn manual SRC mode, SRC_OUT_FS can be set and adjusted to the desired sample rate at any time.\nIn automatic mode the SRC returns the final sample rate as soon as SRC_OUT_OK. Note that SRC_DS is not calculated in automatic mode and must be set manually automatic mode with Sample_rate of 96 and 192kHz."]
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
