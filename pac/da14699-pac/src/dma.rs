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
#[doc = r"DMA registers"]
unsafe impl ::core::marker::Send for super::Dma {}
unsafe impl ::core::marker::Sync for super::Dma {}
impl super::Dma {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn dma0_a_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma0AStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma0AStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma0_b_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma0BStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma0BStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma0_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma0CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma0CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma0_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma0IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma0IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma0_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma0IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma0IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma0_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma0LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma0LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma1_a_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma1AStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma1AStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma1_b_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma1BStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma1BStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma1_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma1CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma1CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma1_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma1IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma1IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma1_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma1IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma1IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma1_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma1LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma1LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma2_a_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma2AStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma2AStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma2_b_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma2BStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma2BStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma2_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma2CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma2CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma2_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma2IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma2IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma2_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma2IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma2IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma2_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma2LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma2LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma3_a_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma3AStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma3AStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma3_b_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma3BStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma3BStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma3_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma3CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma3CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma3_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma3IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma3IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma3_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma3IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma3IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma3_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma3LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma3LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma4_a_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma4AStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma4AStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma4_b_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma4BStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma4BStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma4_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma4CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma4CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma4_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma4IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma4IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma4_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma4IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma4IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma4_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma4LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma4LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma5_a_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma5AStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma5AStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma5_b_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma5BStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma5BStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma5_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma5CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma5CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma5_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma5IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma5IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(180usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma5_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma5IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma5IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma5_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma5LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma5LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma6_a_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma6AStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma6AStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma6_b_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma6BStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma6BStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma6_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma6CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma6CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma6_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma6IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma6IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma6_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma6IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma6IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma6_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma6LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma6LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma7_a_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma7AStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma7AStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma7_b_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma7BStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma7BStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma7_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma7CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma7CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma7_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma7IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma7IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma7_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma7IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma7IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma7_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma7LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma7LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(236usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma_clear_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DmaClearIntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DmaClearIntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma_int_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DmaIntMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DmaIntMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma_int_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DmaIntStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DmaIntStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma_req_mux_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DmaReqMuxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DmaReqMuxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0AStartReg_SPEC;
impl crate::sealed::RegSpec for Dma0AStartReg_SPEC {
    type DataType = u32;
}

pub type Dma0AStartReg = crate::RegValueT<Dma0AStartReg_SPEC>;

impl Dma0AStartReg {
    #[inline(always)]
    pub fn dma0_a_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma0AStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma0AStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma0AStartReg {
    #[inline(always)]
    fn default() -> Dma0AStartReg {
        <crate::RegValueT<Dma0AStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0BStartReg_SPEC;
impl crate::sealed::RegSpec for Dma0BStartReg_SPEC {
    type DataType = u32;
}

pub type Dma0BStartReg = crate::RegValueT<Dma0BStartReg_SPEC>;

impl Dma0BStartReg {
    #[inline(always)]
    pub fn dma0_b_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma0BStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma0BStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma0BStartReg {
    #[inline(always)]
    fn default() -> Dma0BStartReg {
        <crate::RegValueT<Dma0BStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma0CtrlReg_SPEC {
    type DataType = u32;
}

pub type Dma0CtrlReg = crate::RegValueT<Dma0CtrlReg_SPEC>;

impl Dma0CtrlReg {
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma0CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma0CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma0CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma0CtrlReg {
    #[inline(always)]
    fn default() -> Dma0CtrlReg {
        <crate::RegValueT<Dma0CtrlReg_SPEC> as RegisterValue<_>>::new(32768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma0IdxReg_SPEC {
    type DataType = u32;
}

pub type Dma0IdxReg = crate::RegValueT<Dma0IdxReg_SPEC>;

impl Dma0IdxReg {
    #[inline(always)]
    pub fn dma0_idx(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma0IdxReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma0IdxReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma0IdxReg {
    #[inline(always)]
    fn default() -> Dma0IdxReg {
        <crate::RegValueT<Dma0IdxReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0IntReg_SPEC;
impl crate::sealed::RegSpec for Dma0IntReg_SPEC {
    type DataType = u32;
}

pub type Dma0IntReg = crate::RegValueT<Dma0IntReg_SPEC>;

impl Dma0IntReg {
    #[inline(always)]
    pub fn dma0_int(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma0IntReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma0IntReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma0IntReg {
    #[inline(always)]
    fn default() -> Dma0IntReg {
        <crate::RegValueT<Dma0IntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0LenReg_SPEC;
impl crate::sealed::RegSpec for Dma0LenReg_SPEC {
    type DataType = u32;
}

pub type Dma0LenReg = crate::RegValueT<Dma0LenReg_SPEC>;

impl Dma0LenReg {
    #[inline(always)]
    pub fn dma0_len(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma0LenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma0LenReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma0LenReg {
    #[inline(always)]
    fn default() -> Dma0LenReg {
        <crate::RegValueT<Dma0LenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1AStartReg_SPEC;
impl crate::sealed::RegSpec for Dma1AStartReg_SPEC {
    type DataType = u32;
}

pub type Dma1AStartReg = crate::RegValueT<Dma1AStartReg_SPEC>;

impl Dma1AStartReg {
    #[inline(always)]
    pub fn dma1_a_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma1AStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma1AStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma1AStartReg {
    #[inline(always)]
    fn default() -> Dma1AStartReg {
        <crate::RegValueT<Dma1AStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1BStartReg_SPEC;
impl crate::sealed::RegSpec for Dma1BStartReg_SPEC {
    type DataType = u32;
}

pub type Dma1BStartReg = crate::RegValueT<Dma1BStartReg_SPEC>;

impl Dma1BStartReg {
    #[inline(always)]
    pub fn dma1_b_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma1BStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma1BStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma1BStartReg {
    #[inline(always)]
    fn default() -> Dma1BStartReg {
        <crate::RegValueT<Dma1BStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma1CtrlReg_SPEC {
    type DataType = u32;
}

pub type Dma1CtrlReg = crate::RegValueT<Dma1CtrlReg_SPEC>;

impl Dma1CtrlReg {
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma1CtrlReg {
    #[inline(always)]
    fn default() -> Dma1CtrlReg {
        <crate::RegValueT<Dma1CtrlReg_SPEC> as RegisterValue<_>>::new(32768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma1IdxReg_SPEC {
    type DataType = u32;
}

pub type Dma1IdxReg = crate::RegValueT<Dma1IdxReg_SPEC>;

impl Dma1IdxReg {
    #[inline(always)]
    pub fn dma1_idx(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma1IdxReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma1IdxReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma1IdxReg {
    #[inline(always)]
    fn default() -> Dma1IdxReg {
        <crate::RegValueT<Dma1IdxReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1IntReg_SPEC;
impl crate::sealed::RegSpec for Dma1IntReg_SPEC {
    type DataType = u32;
}

pub type Dma1IntReg = crate::RegValueT<Dma1IntReg_SPEC>;

impl Dma1IntReg {
    #[inline(always)]
    pub fn dma1_int(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma1IntReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma1IntReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma1IntReg {
    #[inline(always)]
    fn default() -> Dma1IntReg {
        <crate::RegValueT<Dma1IntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1LenReg_SPEC;
impl crate::sealed::RegSpec for Dma1LenReg_SPEC {
    type DataType = u32;
}

pub type Dma1LenReg = crate::RegValueT<Dma1LenReg_SPEC>;

impl Dma1LenReg {
    #[inline(always)]
    pub fn dma1_len(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma1LenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma1LenReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma1LenReg {
    #[inline(always)]
    fn default() -> Dma1LenReg {
        <crate::RegValueT<Dma1LenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2AStartReg_SPEC;
impl crate::sealed::RegSpec for Dma2AStartReg_SPEC {
    type DataType = u32;
}

pub type Dma2AStartReg = crate::RegValueT<Dma2AStartReg_SPEC>;

impl Dma2AStartReg {
    #[inline(always)]
    pub fn dma2_a_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma2AStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma2AStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma2AStartReg {
    #[inline(always)]
    fn default() -> Dma2AStartReg {
        <crate::RegValueT<Dma2AStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2BStartReg_SPEC;
impl crate::sealed::RegSpec for Dma2BStartReg_SPEC {
    type DataType = u32;
}

pub type Dma2BStartReg = crate::RegValueT<Dma2BStartReg_SPEC>;

impl Dma2BStartReg {
    #[inline(always)]
    pub fn dma2_b_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma2BStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma2BStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma2BStartReg {
    #[inline(always)]
    fn default() -> Dma2BStartReg {
        <crate::RegValueT<Dma2BStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma2CtrlReg_SPEC {
    type DataType = u32;
}

pub type Dma2CtrlReg = crate::RegValueT<Dma2CtrlReg_SPEC>;

impl Dma2CtrlReg {
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma2CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma2CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma2CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma2CtrlReg {
    #[inline(always)]
    fn default() -> Dma2CtrlReg {
        <crate::RegValueT<Dma2CtrlReg_SPEC> as RegisterValue<_>>::new(32768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma2IdxReg_SPEC {
    type DataType = u32;
}

pub type Dma2IdxReg = crate::RegValueT<Dma2IdxReg_SPEC>;

impl Dma2IdxReg {
    #[inline(always)]
    pub fn dma2_idx(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma2IdxReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma2IdxReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma2IdxReg {
    #[inline(always)]
    fn default() -> Dma2IdxReg {
        <crate::RegValueT<Dma2IdxReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2IntReg_SPEC;
impl crate::sealed::RegSpec for Dma2IntReg_SPEC {
    type DataType = u32;
}

pub type Dma2IntReg = crate::RegValueT<Dma2IntReg_SPEC>;

impl Dma2IntReg {
    #[inline(always)]
    pub fn dma2_int(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma2IntReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma2IntReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma2IntReg {
    #[inline(always)]
    fn default() -> Dma2IntReg {
        <crate::RegValueT<Dma2IntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2LenReg_SPEC;
impl crate::sealed::RegSpec for Dma2LenReg_SPEC {
    type DataType = u32;
}

pub type Dma2LenReg = crate::RegValueT<Dma2LenReg_SPEC>;

impl Dma2LenReg {
    #[inline(always)]
    pub fn dma2_len(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma2LenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma2LenReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma2LenReg {
    #[inline(always)]
    fn default() -> Dma2LenReg {
        <crate::RegValueT<Dma2LenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3AStartReg_SPEC;
impl crate::sealed::RegSpec for Dma3AStartReg_SPEC {
    type DataType = u32;
}

pub type Dma3AStartReg = crate::RegValueT<Dma3AStartReg_SPEC>;

impl Dma3AStartReg {
    #[inline(always)]
    pub fn dma3_a_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma3AStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma3AStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma3AStartReg {
    #[inline(always)]
    fn default() -> Dma3AStartReg {
        <crate::RegValueT<Dma3AStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3BStartReg_SPEC;
impl crate::sealed::RegSpec for Dma3BStartReg_SPEC {
    type DataType = u32;
}

pub type Dma3BStartReg = crate::RegValueT<Dma3BStartReg_SPEC>;

impl Dma3BStartReg {
    #[inline(always)]
    pub fn dma3_b_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma3BStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma3BStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma3BStartReg {
    #[inline(always)]
    fn default() -> Dma3BStartReg {
        <crate::RegValueT<Dma3BStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma3CtrlReg_SPEC {
    type DataType = u32;
}

pub type Dma3CtrlReg = crate::RegValueT<Dma3CtrlReg_SPEC>;

impl Dma3CtrlReg {
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma3CtrlReg {
    #[inline(always)]
    fn default() -> Dma3CtrlReg {
        <crate::RegValueT<Dma3CtrlReg_SPEC> as RegisterValue<_>>::new(32768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma3IdxReg_SPEC {
    type DataType = u32;
}

pub type Dma3IdxReg = crate::RegValueT<Dma3IdxReg_SPEC>;

impl Dma3IdxReg {
    #[inline(always)]
    pub fn dma3_idx(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma3IdxReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma3IdxReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma3IdxReg {
    #[inline(always)]
    fn default() -> Dma3IdxReg {
        <crate::RegValueT<Dma3IdxReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3IntReg_SPEC;
impl crate::sealed::RegSpec for Dma3IntReg_SPEC {
    type DataType = u32;
}

pub type Dma3IntReg = crate::RegValueT<Dma3IntReg_SPEC>;

impl Dma3IntReg {
    #[inline(always)]
    pub fn dma3_int(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma3IntReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma3IntReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma3IntReg {
    #[inline(always)]
    fn default() -> Dma3IntReg {
        <crate::RegValueT<Dma3IntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3LenReg_SPEC;
impl crate::sealed::RegSpec for Dma3LenReg_SPEC {
    type DataType = u32;
}

pub type Dma3LenReg = crate::RegValueT<Dma3LenReg_SPEC>;

impl Dma3LenReg {
    #[inline(always)]
    pub fn dma3_len(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma3LenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma3LenReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma3LenReg {
    #[inline(always)]
    fn default() -> Dma3LenReg {
        <crate::RegValueT<Dma3LenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma4AStartReg_SPEC;
impl crate::sealed::RegSpec for Dma4AStartReg_SPEC {
    type DataType = u32;
}

pub type Dma4AStartReg = crate::RegValueT<Dma4AStartReg_SPEC>;

impl Dma4AStartReg {
    #[inline(always)]
    pub fn dma4_a_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma4AStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma4AStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma4AStartReg {
    #[inline(always)]
    fn default() -> Dma4AStartReg {
        <crate::RegValueT<Dma4AStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma4BStartReg_SPEC;
impl crate::sealed::RegSpec for Dma4BStartReg_SPEC {
    type DataType = u32;
}

pub type Dma4BStartReg = crate::RegValueT<Dma4BStartReg_SPEC>;

impl Dma4BStartReg {
    #[inline(always)]
    pub fn dma4_b_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma4BStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma4BStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma4BStartReg {
    #[inline(always)]
    fn default() -> Dma4BStartReg {
        <crate::RegValueT<Dma4BStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma4CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma4CtrlReg_SPEC {
    type DataType = u32;
}

pub type Dma4CtrlReg = crate::RegValueT<Dma4CtrlReg_SPEC>;

impl Dma4CtrlReg {
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma4CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma4CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma4CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma4CtrlReg {
    #[inline(always)]
    fn default() -> Dma4CtrlReg {
        <crate::RegValueT<Dma4CtrlReg_SPEC> as RegisterValue<_>>::new(32768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma4IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma4IdxReg_SPEC {
    type DataType = u32;
}

pub type Dma4IdxReg = crate::RegValueT<Dma4IdxReg_SPEC>;

impl Dma4IdxReg {
    #[inline(always)]
    pub fn dma4_idx(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma4IdxReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma4IdxReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma4IdxReg {
    #[inline(always)]
    fn default() -> Dma4IdxReg {
        <crate::RegValueT<Dma4IdxReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma4IntReg_SPEC;
impl crate::sealed::RegSpec for Dma4IntReg_SPEC {
    type DataType = u32;
}

pub type Dma4IntReg = crate::RegValueT<Dma4IntReg_SPEC>;

impl Dma4IntReg {
    #[inline(always)]
    pub fn dma4_int(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma4IntReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma4IntReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma4IntReg {
    #[inline(always)]
    fn default() -> Dma4IntReg {
        <crate::RegValueT<Dma4IntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma4LenReg_SPEC;
impl crate::sealed::RegSpec for Dma4LenReg_SPEC {
    type DataType = u32;
}

pub type Dma4LenReg = crate::RegValueT<Dma4LenReg_SPEC>;

impl Dma4LenReg {
    #[inline(always)]
    pub fn dma4_len(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma4LenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma4LenReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma4LenReg {
    #[inline(always)]
    fn default() -> Dma4LenReg {
        <crate::RegValueT<Dma4LenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma5AStartReg_SPEC;
impl crate::sealed::RegSpec for Dma5AStartReg_SPEC {
    type DataType = u32;
}

pub type Dma5AStartReg = crate::RegValueT<Dma5AStartReg_SPEC>;

impl Dma5AStartReg {
    #[inline(always)]
    pub fn dma5_a_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma5AStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma5AStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma5AStartReg {
    #[inline(always)]
    fn default() -> Dma5AStartReg {
        <crate::RegValueT<Dma5AStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma5BStartReg_SPEC;
impl crate::sealed::RegSpec for Dma5BStartReg_SPEC {
    type DataType = u32;
}

pub type Dma5BStartReg = crate::RegValueT<Dma5BStartReg_SPEC>;

impl Dma5BStartReg {
    #[inline(always)]
    pub fn dma5_b_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma5BStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma5BStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma5BStartReg {
    #[inline(always)]
    fn default() -> Dma5BStartReg {
        <crate::RegValueT<Dma5BStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma5CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma5CtrlReg_SPEC {
    type DataType = u32;
}

pub type Dma5CtrlReg = crate::RegValueT<Dma5CtrlReg_SPEC>;

impl Dma5CtrlReg {
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma5CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma5CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma5CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma5CtrlReg {
    #[inline(always)]
    fn default() -> Dma5CtrlReg {
        <crate::RegValueT<Dma5CtrlReg_SPEC> as RegisterValue<_>>::new(32768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma5IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma5IdxReg_SPEC {
    type DataType = u32;
}

pub type Dma5IdxReg = crate::RegValueT<Dma5IdxReg_SPEC>;

impl Dma5IdxReg {
    #[inline(always)]
    pub fn dma5_idx(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma5IdxReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma5IdxReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma5IdxReg {
    #[inline(always)]
    fn default() -> Dma5IdxReg {
        <crate::RegValueT<Dma5IdxReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma5IntReg_SPEC;
impl crate::sealed::RegSpec for Dma5IntReg_SPEC {
    type DataType = u32;
}

pub type Dma5IntReg = crate::RegValueT<Dma5IntReg_SPEC>;

impl Dma5IntReg {
    #[inline(always)]
    pub fn dma5_int(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma5IntReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma5IntReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma5IntReg {
    #[inline(always)]
    fn default() -> Dma5IntReg {
        <crate::RegValueT<Dma5IntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma5LenReg_SPEC;
impl crate::sealed::RegSpec for Dma5LenReg_SPEC {
    type DataType = u32;
}

pub type Dma5LenReg = crate::RegValueT<Dma5LenReg_SPEC>;

impl Dma5LenReg {
    #[inline(always)]
    pub fn dma5_len(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma5LenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma5LenReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma5LenReg {
    #[inline(always)]
    fn default() -> Dma5LenReg {
        <crate::RegValueT<Dma5LenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma6AStartReg_SPEC;
impl crate::sealed::RegSpec for Dma6AStartReg_SPEC {
    type DataType = u32;
}

pub type Dma6AStartReg = crate::RegValueT<Dma6AStartReg_SPEC>;

impl Dma6AStartReg {
    #[inline(always)]
    pub fn dma6_a_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma6AStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma6AStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma6AStartReg {
    #[inline(always)]
    fn default() -> Dma6AStartReg {
        <crate::RegValueT<Dma6AStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma6BStartReg_SPEC;
impl crate::sealed::RegSpec for Dma6BStartReg_SPEC {
    type DataType = u32;
}

pub type Dma6BStartReg = crate::RegValueT<Dma6BStartReg_SPEC>;

impl Dma6BStartReg {
    #[inline(always)]
    pub fn dma6_b_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma6BStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma6BStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma6BStartReg {
    #[inline(always)]
    fn default() -> Dma6BStartReg {
        <crate::RegValueT<Dma6BStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma6CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma6CtrlReg_SPEC {
    type DataType = u32;
}

pub type Dma6CtrlReg = crate::RegValueT<Dma6CtrlReg_SPEC>;

impl Dma6CtrlReg {
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma6CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma6CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma6CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma6CtrlReg {
    #[inline(always)]
    fn default() -> Dma6CtrlReg {
        <crate::RegValueT<Dma6CtrlReg_SPEC> as RegisterValue<_>>::new(32768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma6IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma6IdxReg_SPEC {
    type DataType = u32;
}

pub type Dma6IdxReg = crate::RegValueT<Dma6IdxReg_SPEC>;

impl Dma6IdxReg {
    #[inline(always)]
    pub fn dma6_idx(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma6IdxReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma6IdxReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma6IdxReg {
    #[inline(always)]
    fn default() -> Dma6IdxReg {
        <crate::RegValueT<Dma6IdxReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma6IntReg_SPEC;
impl crate::sealed::RegSpec for Dma6IntReg_SPEC {
    type DataType = u32;
}

pub type Dma6IntReg = crate::RegValueT<Dma6IntReg_SPEC>;

impl Dma6IntReg {
    #[inline(always)]
    pub fn dma6_int(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma6IntReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma6IntReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma6IntReg {
    #[inline(always)]
    fn default() -> Dma6IntReg {
        <crate::RegValueT<Dma6IntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma6LenReg_SPEC;
impl crate::sealed::RegSpec for Dma6LenReg_SPEC {
    type DataType = u32;
}

pub type Dma6LenReg = crate::RegValueT<Dma6LenReg_SPEC>;

impl Dma6LenReg {
    #[inline(always)]
    pub fn dma6_len(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma6LenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma6LenReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma6LenReg {
    #[inline(always)]
    fn default() -> Dma6LenReg {
        <crate::RegValueT<Dma6LenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma7AStartReg_SPEC;
impl crate::sealed::RegSpec for Dma7AStartReg_SPEC {
    type DataType = u32;
}

pub type Dma7AStartReg = crate::RegValueT<Dma7AStartReg_SPEC>;

impl Dma7AStartReg {
    #[inline(always)]
    pub fn dma7_a_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma7AStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma7AStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma7AStartReg {
    #[inline(always)]
    fn default() -> Dma7AStartReg {
        <crate::RegValueT<Dma7AStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma7BStartReg_SPEC;
impl crate::sealed::RegSpec for Dma7BStartReg_SPEC {
    type DataType = u32;
}

pub type Dma7BStartReg = crate::RegValueT<Dma7BStartReg_SPEC>;

impl Dma7BStartReg {
    #[inline(always)]
    pub fn dma7_b_start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dma7BStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dma7BStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma7BStartReg {
    #[inline(always)]
    fn default() -> Dma7BStartReg {
        <crate::RegValueT<Dma7BStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma7CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma7CtrlReg_SPEC {
    type DataType = u32;
}

pub type Dma7CtrlReg = crate::RegValueT<Dma7CtrlReg_SPEC>;

impl Dma7CtrlReg {
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma7CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma7CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma7CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma7CtrlReg {
    #[inline(always)]
    fn default() -> Dma7CtrlReg {
        <crate::RegValueT<Dma7CtrlReg_SPEC> as RegisterValue<_>>::new(32768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma7IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma7IdxReg_SPEC {
    type DataType = u32;
}

pub type Dma7IdxReg = crate::RegValueT<Dma7IdxReg_SPEC>;

impl Dma7IdxReg {
    #[inline(always)]
    pub fn dma7_idx(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma7IdxReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma7IdxReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma7IdxReg {
    #[inline(always)]
    fn default() -> Dma7IdxReg {
        <crate::RegValueT<Dma7IdxReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma7IntReg_SPEC;
impl crate::sealed::RegSpec for Dma7IntReg_SPEC {
    type DataType = u32;
}

pub type Dma7IntReg = crate::RegValueT<Dma7IntReg_SPEC>;

impl Dma7IntReg {
    #[inline(always)]
    pub fn dma7_int(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma7IntReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma7IntReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma7IntReg {
    #[inline(always)]
    fn default() -> Dma7IntReg {
        <crate::RegValueT<Dma7IntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma7LenReg_SPEC;
impl crate::sealed::RegSpec for Dma7LenReg_SPEC {
    type DataType = u32;
}

pub type Dma7LenReg = crate::RegValueT<Dma7LenReg_SPEC>;

impl Dma7LenReg {
    #[inline(always)]
    pub fn dma7_len(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dma7LenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dma7LenReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dma7LenReg {
    #[inline(always)]
    fn default() -> Dma7LenReg {
        <crate::RegValueT<Dma7LenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaClearIntReg_SPEC;
impl crate::sealed::RegSpec for DmaClearIntReg_SPEC {
    type DataType = u32;
}

pub type DmaClearIntReg = crate::RegValueT<DmaClearIntReg_SPEC>;

impl DmaClearIntReg {
    #[inline(always)]
    pub fn dma_rst_irq_ch7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_rst_irq_ch6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_rst_irq_ch5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_rst_irq_ch4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_rst_irq_ch3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_rst_irq_ch2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_rst_irq_ch1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_rst_irq_ch0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for DmaClearIntReg {
    #[inline(always)]
    fn default() -> DmaClearIntReg {
        <crate::RegValueT<DmaClearIntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaIntMaskReg_SPEC;
impl crate::sealed::RegSpec for DmaIntMaskReg_SPEC {
    type DataType = u32;
}

pub type DmaIntMaskReg = crate::RegValueT<DmaIntMaskReg_SPEC>;

impl DmaIntMaskReg {
    #[inline(always)]
    pub fn dma_irq_enable7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_enable6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_enable5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_enable4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_enable3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_enable2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_enable1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_enable0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DmaIntMaskReg {
    #[inline(always)]
    fn default() -> DmaIntMaskReg {
        <crate::RegValueT<DmaIntMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaIntStatusReg_SPEC;
impl crate::sealed::RegSpec for DmaIntStatusReg_SPEC {
    type DataType = u32;
}

pub type DmaIntStatusReg = crate::RegValueT<DmaIntStatusReg_SPEC>;

impl DmaIntStatusReg {
    #[inline(always)]
    pub fn dma_bus_err7(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_bus_err6(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_bus_err5(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_bus_err4(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_bus_err3(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_bus_err2(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_bus_err1(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_bus_err0(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_ch7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_ch6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_ch5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_ch4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_ch3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_ch2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_ch1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_irq_ch0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DmaIntStatusReg {
    #[inline(always)]
    fn default() -> DmaIntStatusReg {
        <crate::RegValueT<DmaIntStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaReqMuxReg_SPEC;
impl crate::sealed::RegSpec for DmaReqMuxReg_SPEC {
    type DataType = u32;
}

pub type DmaReqMuxReg = crate::RegValueT<DmaReqMuxReg_SPEC>;

impl DmaReqMuxReg {
    #[inline(always)]
    pub fn dma67_sel(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, DmaReqMuxReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,DmaReqMuxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma45_sel(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, DmaReqMuxReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,DmaReqMuxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma23_sel(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, DmaReqMuxReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,DmaReqMuxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma01_sel(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, DmaReqMuxReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,DmaReqMuxReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DmaReqMuxReg {
    #[inline(always)]
    fn default() -> DmaReqMuxReg {
        <crate::RegValueT<DmaReqMuxReg_SPEC> as RegisterValue<_>>::new(65535)
    }
}
