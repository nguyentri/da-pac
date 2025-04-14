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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:50 +0000

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
    pub const fn dma0_a_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma0AStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma0AStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma0_a_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma0AStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma0AStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma0_b_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma0BStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma0BStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma0_b_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma0BStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma0BStartlReg_SPEC, crate::common::RW>::from_ptr(
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
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma0_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma0IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma0IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
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
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma1_a_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma1AStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma1AStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma1_a_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma1AStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma1AStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma1_b_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma1BStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma1BStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma1_b_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma1BStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma1BStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma1_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma1CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma1CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma1_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma1IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma1IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma1_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma1IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma1IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma1_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma1LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma1LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma2_a_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma2AStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma2AStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma2_a_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma2AStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma2AStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma2_b_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma2BStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma2BStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma2_b_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma2BStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma2BStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma2_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma2CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma2CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma2_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma2IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma2IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma2_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma2IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma2IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma2_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma2LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma2LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma3_a_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma3AStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma3AStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma3_a_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma3AStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma3AStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma3_b_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma3BStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma3BStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma3_b_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma3BStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma3BStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma3_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma3CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma3CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma3_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma3IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma3IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(62usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma3_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma3IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma3IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma3_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma3LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma3LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(58usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma4_a_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma4AStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma4AStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma4_a_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma4AStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma4AStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma4_b_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma4BStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma4BStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(70usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma4_b_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma4BStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma4BStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma4_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma4CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma4CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma4_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma4IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma4IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(78usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma4_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma4IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma4IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma4_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma4LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma4LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(74usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma5_a_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma5AStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma5AStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(82usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma5_a_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma5AStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma5AStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma5_b_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma5BStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma5BStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(86usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma5_b_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma5BStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma5BStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma5_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma5CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma5CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma5_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma5IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma5IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(94usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma5_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma5IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma5IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma5_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma5LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma5LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(90usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma6_a_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma6AStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma6AStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(98usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma6_a_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma6AStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma6AStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma6_b_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma6BStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma6BStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(102usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma6_b_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma6BStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma6BStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma6_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma6CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma6CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma6_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma6IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma6IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(110usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma6_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma6IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma6IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma6_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma6LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma6LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(106usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma7_a_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma7AStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma7AStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(114usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma7_a_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma7AStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma7AStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma7_b_starth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma7BStarthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma7BStarthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(118usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma7_b_startl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma7BStartlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma7BStartlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma7_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma7CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma7CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma7_idx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma7IdxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma7IdxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(126usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma7_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma7IntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma7IntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma7_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dma7LenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dma7LenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(122usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma_clear_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DmaClearIntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DmaClearIntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma_int_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DmaIntStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DmaIntStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(130usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dma_req_mux_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DmaReqMuxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DmaReqMuxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0AStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma0AStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma0AStarthReg = crate::RegValueT<Dma0AStarthReg_SPEC>;

impl Dma0AStarthReg {
    #[inline(always)]
    pub fn dma0_a_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma0AStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma0AStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma0AStarthReg {
    #[inline(always)]
    fn default() -> Dma0AStarthReg {
        <crate::RegValueT<Dma0AStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0AStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma0AStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma0AStartlReg = crate::RegValueT<Dma0AStartlReg_SPEC>;

impl Dma0AStartlReg {
    #[inline(always)]
    pub fn dma0_a_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma0AStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma0AStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma0AStartlReg {
    #[inline(always)]
    fn default() -> Dma0AStartlReg {
        <crate::RegValueT<Dma0AStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0BStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma0BStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma0BStarthReg = crate::RegValueT<Dma0BStarthReg_SPEC>;

impl Dma0BStarthReg {
    #[inline(always)]
    pub fn dma0_b_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma0BStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma0BStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma0BStarthReg {
    #[inline(always)]
    fn default() -> Dma0BStarthReg {
        <crate::RegValueT<Dma0BStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0BStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma0BStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma0BStartlReg = crate::RegValueT<Dma0BStartlReg_SPEC>;

impl Dma0BStartlReg {
    #[inline(always)]
    pub fn dma0_b_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma0BStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma0BStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma0BStartlReg {
    #[inline(always)]
    fn default() -> Dma0BStartlReg {
        <crate::RegValueT<Dma0BStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma0CtrlReg_SPEC {
    type DataType = u16;
}

pub type Dma0CtrlReg = crate::RegValueT<Dma0CtrlReg_SPEC>;

impl Dma0CtrlReg {
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Dma0CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_enable(
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
        <crate::RegValueT<Dma0CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma0IdxReg_SPEC {
    type DataType = u16;
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
    type DataType = u16;
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
    type DataType = u16;
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
pub struct Dma1AStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma1AStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma1AStarthReg = crate::RegValueT<Dma1AStarthReg_SPEC>;

impl Dma1AStarthReg {
    #[inline(always)]
    pub fn dma1_a_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma1AStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma1AStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma1AStarthReg {
    #[inline(always)]
    fn default() -> Dma1AStarthReg {
        <crate::RegValueT<Dma1AStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1AStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma1AStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma1AStartlReg = crate::RegValueT<Dma1AStartlReg_SPEC>;

impl Dma1AStartlReg {
    #[inline(always)]
    pub fn dma1_a_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma1AStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma1AStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma1AStartlReg {
    #[inline(always)]
    fn default() -> Dma1AStartlReg {
        <crate::RegValueT<Dma1AStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1BStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma1BStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma1BStarthReg = crate::RegValueT<Dma1BStarthReg_SPEC>;

impl Dma1BStarthReg {
    #[inline(always)]
    pub fn dma1_b_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma1BStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma1BStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma1BStarthReg {
    #[inline(always)]
    fn default() -> Dma1BStarthReg {
        <crate::RegValueT<Dma1BStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1BStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma1BStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma1BStartlReg = crate::RegValueT<Dma1BStartlReg_SPEC>;

impl Dma1BStartlReg {
    #[inline(always)]
    pub fn dma1_b_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma1BStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma1BStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma1BStartlReg {
    #[inline(always)]
    fn default() -> Dma1BStartlReg {
        <crate::RegValueT<Dma1BStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma1CtrlReg_SPEC {
    type DataType = u16;
}

pub type Dma1CtrlReg = crate::RegValueT<Dma1CtrlReg_SPEC>;

impl Dma1CtrlReg {
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Dma1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_enable(
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
        <crate::RegValueT<Dma1CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma1IdxReg_SPEC {
    type DataType = u16;
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
    type DataType = u16;
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
    type DataType = u16;
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
pub struct Dma2AStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma2AStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma2AStarthReg = crate::RegValueT<Dma2AStarthReg_SPEC>;

impl Dma2AStarthReg {
    #[inline(always)]
    pub fn dma2_a_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma2AStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma2AStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma2AStarthReg {
    #[inline(always)]
    fn default() -> Dma2AStarthReg {
        <crate::RegValueT<Dma2AStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2AStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma2AStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma2AStartlReg = crate::RegValueT<Dma2AStartlReg_SPEC>;

impl Dma2AStartlReg {
    #[inline(always)]
    pub fn dma2_a_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma2AStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma2AStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma2AStartlReg {
    #[inline(always)]
    fn default() -> Dma2AStartlReg {
        <crate::RegValueT<Dma2AStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2BStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma2BStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma2BStarthReg = crate::RegValueT<Dma2BStarthReg_SPEC>;

impl Dma2BStarthReg {
    #[inline(always)]
    pub fn dma2_b_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma2BStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma2BStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma2BStarthReg {
    #[inline(always)]
    fn default() -> Dma2BStarthReg {
        <crate::RegValueT<Dma2BStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2BStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma2BStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma2BStartlReg = crate::RegValueT<Dma2BStartlReg_SPEC>;

impl Dma2BStartlReg {
    #[inline(always)]
    pub fn dma2_b_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma2BStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma2BStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma2BStartlReg {
    #[inline(always)]
    fn default() -> Dma2BStartlReg {
        <crate::RegValueT<Dma2BStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma2CtrlReg_SPEC {
    type DataType = u16;
}

pub type Dma2CtrlReg = crate::RegValueT<Dma2CtrlReg_SPEC>;

impl Dma2CtrlReg {
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Dma2CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_enable(
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
        <crate::RegValueT<Dma2CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma2IdxReg_SPEC {
    type DataType = u16;
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
    type DataType = u16;
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
    type DataType = u16;
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
pub struct Dma3AStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma3AStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma3AStarthReg = crate::RegValueT<Dma3AStarthReg_SPEC>;

impl Dma3AStarthReg {
    #[inline(always)]
    pub fn dma3_a_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma3AStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma3AStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma3AStarthReg {
    #[inline(always)]
    fn default() -> Dma3AStarthReg {
        <crate::RegValueT<Dma3AStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3AStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma3AStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma3AStartlReg = crate::RegValueT<Dma3AStartlReg_SPEC>;

impl Dma3AStartlReg {
    #[inline(always)]
    pub fn dma3_a_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma3AStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma3AStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma3AStartlReg {
    #[inline(always)]
    fn default() -> Dma3AStartlReg {
        <crate::RegValueT<Dma3AStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3BStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma3BStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma3BStarthReg = crate::RegValueT<Dma3BStarthReg_SPEC>;

impl Dma3BStarthReg {
    #[inline(always)]
    pub fn dma3_b_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma3BStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma3BStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma3BStarthReg {
    #[inline(always)]
    fn default() -> Dma3BStarthReg {
        <crate::RegValueT<Dma3BStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3BStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma3BStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma3BStartlReg = crate::RegValueT<Dma3BStartlReg_SPEC>;

impl Dma3BStartlReg {
    #[inline(always)]
    pub fn dma3_b_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma3BStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma3BStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma3BStartlReg {
    #[inline(always)]
    fn default() -> Dma3BStartlReg {
        <crate::RegValueT<Dma3BStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma3CtrlReg_SPEC {
    type DataType = u16;
}

pub type Dma3CtrlReg = crate::RegValueT<Dma3CtrlReg_SPEC>;

impl Dma3CtrlReg {
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Dma3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_enable(
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
        <crate::RegValueT<Dma3CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma3IdxReg_SPEC {
    type DataType = u16;
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
    type DataType = u16;
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
    type DataType = u16;
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
pub struct Dma4AStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma4AStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma4AStarthReg = crate::RegValueT<Dma4AStarthReg_SPEC>;

impl Dma4AStarthReg {
    #[inline(always)]
    pub fn dma4_a_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma4AStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma4AStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma4AStarthReg {
    #[inline(always)]
    fn default() -> Dma4AStarthReg {
        <crate::RegValueT<Dma4AStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma4AStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma4AStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma4AStartlReg = crate::RegValueT<Dma4AStartlReg_SPEC>;

impl Dma4AStartlReg {
    #[inline(always)]
    pub fn dma4_a_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma4AStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma4AStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma4AStartlReg {
    #[inline(always)]
    fn default() -> Dma4AStartlReg {
        <crate::RegValueT<Dma4AStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma4BStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma4BStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma4BStarthReg = crate::RegValueT<Dma4BStarthReg_SPEC>;

impl Dma4BStarthReg {
    #[inline(always)]
    pub fn dma4_b_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma4BStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma4BStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma4BStarthReg {
    #[inline(always)]
    fn default() -> Dma4BStarthReg {
        <crate::RegValueT<Dma4BStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma4BStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma4BStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma4BStartlReg = crate::RegValueT<Dma4BStartlReg_SPEC>;

impl Dma4BStartlReg {
    #[inline(always)]
    pub fn dma4_b_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma4BStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma4BStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma4BStartlReg {
    #[inline(always)]
    fn default() -> Dma4BStartlReg {
        <crate::RegValueT<Dma4BStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma4CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma4CtrlReg_SPEC {
    type DataType = u16;
}

pub type Dma4CtrlReg = crate::RegValueT<Dma4CtrlReg_SPEC>;

impl Dma4CtrlReg {
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Dma4CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_enable(
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
        <crate::RegValueT<Dma4CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma4IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma4IdxReg_SPEC {
    type DataType = u16;
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
    type DataType = u16;
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
    type DataType = u16;
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
pub struct Dma5AStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma5AStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma5AStarthReg = crate::RegValueT<Dma5AStarthReg_SPEC>;

impl Dma5AStarthReg {
    #[inline(always)]
    pub fn dma5_a_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma5AStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma5AStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma5AStarthReg {
    #[inline(always)]
    fn default() -> Dma5AStarthReg {
        <crate::RegValueT<Dma5AStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma5AStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma5AStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma5AStartlReg = crate::RegValueT<Dma5AStartlReg_SPEC>;

impl Dma5AStartlReg {
    #[inline(always)]
    pub fn dma5_a_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma5AStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma5AStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma5AStartlReg {
    #[inline(always)]
    fn default() -> Dma5AStartlReg {
        <crate::RegValueT<Dma5AStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma5BStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma5BStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma5BStarthReg = crate::RegValueT<Dma5BStarthReg_SPEC>;

impl Dma5BStarthReg {
    #[inline(always)]
    pub fn dma5_b_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma5BStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma5BStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma5BStarthReg {
    #[inline(always)]
    fn default() -> Dma5BStarthReg {
        <crate::RegValueT<Dma5BStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma5BStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma5BStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma5BStartlReg = crate::RegValueT<Dma5BStartlReg_SPEC>;

impl Dma5BStartlReg {
    #[inline(always)]
    pub fn dma5_b_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma5BStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma5BStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma5BStartlReg {
    #[inline(always)]
    fn default() -> Dma5BStartlReg {
        <crate::RegValueT<Dma5BStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma5CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma5CtrlReg_SPEC {
    type DataType = u16;
}

pub type Dma5CtrlReg = crate::RegValueT<Dma5CtrlReg_SPEC>;

impl Dma5CtrlReg {
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Dma5CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_enable(
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
        <crate::RegValueT<Dma5CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma5IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma5IdxReg_SPEC {
    type DataType = u16;
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
    type DataType = u16;
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
    type DataType = u16;
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
pub struct Dma6AStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma6AStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma6AStarthReg = crate::RegValueT<Dma6AStarthReg_SPEC>;

impl Dma6AStarthReg {
    #[inline(always)]
    pub fn dma6_a_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma6AStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma6AStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma6AStarthReg {
    #[inline(always)]
    fn default() -> Dma6AStarthReg {
        <crate::RegValueT<Dma6AStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma6AStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma6AStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma6AStartlReg = crate::RegValueT<Dma6AStartlReg_SPEC>;

impl Dma6AStartlReg {
    #[inline(always)]
    pub fn dma6_a_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma6AStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma6AStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma6AStartlReg {
    #[inline(always)]
    fn default() -> Dma6AStartlReg {
        <crate::RegValueT<Dma6AStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma6BStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma6BStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma6BStarthReg = crate::RegValueT<Dma6BStarthReg_SPEC>;

impl Dma6BStarthReg {
    #[inline(always)]
    pub fn dma6_b_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma6BStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma6BStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma6BStarthReg {
    #[inline(always)]
    fn default() -> Dma6BStarthReg {
        <crate::RegValueT<Dma6BStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma6BStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma6BStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma6BStartlReg = crate::RegValueT<Dma6BStartlReg_SPEC>;

impl Dma6BStartlReg {
    #[inline(always)]
    pub fn dma6_b_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma6BStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma6BStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma6BStartlReg {
    #[inline(always)]
    fn default() -> Dma6BStartlReg {
        <crate::RegValueT<Dma6BStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma6CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma6CtrlReg_SPEC {
    type DataType = u16;
}

pub type Dma6CtrlReg = crate::RegValueT<Dma6CtrlReg_SPEC>;

impl Dma6CtrlReg {
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Dma6CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_enable(
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
        <crate::RegValueT<Dma6CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma6IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma6IdxReg_SPEC {
    type DataType = u16;
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
    type DataType = u16;
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
    type DataType = u16;
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
pub struct Dma7AStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma7AStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma7AStarthReg = crate::RegValueT<Dma7AStarthReg_SPEC>;

impl Dma7AStarthReg {
    #[inline(always)]
    pub fn dma7_a_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma7AStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma7AStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma7AStarthReg {
    #[inline(always)]
    fn default() -> Dma7AStarthReg {
        <crate::RegValueT<Dma7AStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma7AStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma7AStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma7AStartlReg = crate::RegValueT<Dma7AStartlReg_SPEC>;

impl Dma7AStartlReg {
    #[inline(always)]
    pub fn dma7_a_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma7AStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma7AStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma7AStartlReg {
    #[inline(always)]
    fn default() -> Dma7AStartlReg {
        <crate::RegValueT<Dma7AStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma7BStarthReg_SPEC;
impl crate::sealed::RegSpec for Dma7BStarthReg_SPEC {
    type DataType = u16;
}

pub type Dma7BStarthReg = crate::RegValueT<Dma7BStarthReg_SPEC>;

impl Dma7BStarthReg {
    #[inline(always)]
    pub fn dma7_b_starth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma7BStarthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma7BStarthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma7BStarthReg {
    #[inline(always)]
    fn default() -> Dma7BStarthReg {
        <crate::RegValueT<Dma7BStarthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma7BStartlReg_SPEC;
impl crate::sealed::RegSpec for Dma7BStartlReg_SPEC {
    type DataType = u16;
}

pub type Dma7BStartlReg = crate::RegValueT<Dma7BStartlReg_SPEC>;

impl Dma7BStartlReg {
    #[inline(always)]
    pub fn dma7_b_startl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dma7BStartlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dma7BStartlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dma7BStartlReg {
    #[inline(always)]
    fn default() -> Dma7BStartlReg {
        <crate::RegValueT<Dma7BStartlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma7CtrlReg_SPEC;
impl crate::sealed::RegSpec for Dma7CtrlReg_SPEC {
    type DataType = u16;
}

pub type Dma7CtrlReg = crate::RegValueT<Dma7CtrlReg_SPEC>;

impl Dma7CtrlReg {
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Dma7CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_enable(
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
        <crate::RegValueT<Dma7CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma7IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma7IdxReg_SPEC {
    type DataType = u16;
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
    type DataType = u16;
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
    type DataType = u16;
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
    type DataType = u16;
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
pub struct DmaIntStatusReg_SPEC;
impl crate::sealed::RegSpec for DmaIntStatusReg_SPEC {
    type DataType = u16;
}

pub type DmaIntStatusReg = crate::RegValueT<DmaIntStatusReg_SPEC>;

impl DmaIntStatusReg {
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
    type DataType = u16;
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
        <crate::RegValueT<DmaReqMuxReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
