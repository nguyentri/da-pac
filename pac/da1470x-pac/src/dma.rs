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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:52 +0000

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

    #[doc = "Source address register of DMA channel 0"]
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

    #[doc = "Destination address register of DMA channel 0"]
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

    #[doc = "Control register of DMA channel 0"]
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

    #[doc = "Index pointer register of DMA channel 0"]
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

    #[doc = "Interrupt length register of DMA channel 0"]
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

    #[doc = "Transfer length register of DMA channel 0"]
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

    #[doc = "Source address register of DMA channel 1"]
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

    #[doc = "Destination address register of DMA channel 1"]
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

    #[doc = "Control register of DMA channel 1"]
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

    #[doc = "Index pointer register of DMA channel 1"]
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

    #[doc = "Interrupt length register of DMA channel 1"]
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

    #[doc = "Transfer length register of DMA channel 1"]
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

    #[doc = "Source address register of DMA channel 2"]
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

    #[doc = "Destination address register of DMA channel 2"]
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

    #[doc = "Control register of DMA channel 2"]
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

    #[doc = "Index pointer register of DMA channel 2"]
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

    #[doc = "Interrupt length register of DMA channel 2"]
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

    #[doc = "Transfer length register of DMA channel 2"]
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

    #[doc = "Source address register of DMA channel 3"]
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

    #[doc = "Destination address register of DMA channel 3"]
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

    #[doc = "Control register of DMA channel 3"]
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

    #[doc = "Index pointer register of DMA channel 3"]
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

    #[doc = "Interrupt length register of DMA channel 3"]
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

    #[doc = "Transfer length register of DMA channel 3"]
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

    #[doc = "Source address register of DMA channel 4"]
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

    #[doc = "Destination address register of DMA channel 4"]
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

    #[doc = "Control register of DMA channel 4"]
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

    #[doc = "Index pointer register of DMA channel 4"]
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

    #[doc = "Interrupt length register of DMA channel 4"]
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

    #[doc = "Transfer length register of DMA channel 4"]
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

    #[doc = "Source address register of DMA channel 5"]
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

    #[doc = "Destination address register of DMA channel 5"]
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

    #[doc = "Control register of DMA channel 5"]
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

    #[doc = "Index pointer register of DMA channel 5"]
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

    #[doc = "Interrupt length register of DMA channel 5"]
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

    #[doc = "Transfer length register of DMA channel 5"]
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

    #[doc = "Source address register of DMA channel 6"]
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

    #[doc = "Destination address register of DMA channel 6"]
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

    #[doc = "Control register of DMA channel 6"]
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

    #[doc = "Index pointer register of DMA channel 6"]
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

    #[doc = "Interrupt length register of DMA channel 6"]
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

    #[doc = "Transfer length register of DMA channel 6"]
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

    #[doc = "Source address register of DMA channel 7"]
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

    #[doc = "Destination address register of DMA channel 7"]
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

    #[doc = "Control register of DMA channel 7"]
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

    #[doc = "Index pointer register of DMA channel 7"]
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

    #[doc = "Interrupt length register of DMA channel 7"]
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

    #[doc = "Transfer length register of DMA channel 7"]
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

    #[doc = "DMA Interrupt clear register"]
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

    #[doc = "DMA Interrupt mask register"]
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

    #[doc = "DMA Interrupt status register"]
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

    #[doc = "DMA channels peripherals mapping register"]
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

    #[doc = "DMA Reset Interrupt mask register"]
    #[inline(always)]
    pub const fn dma_reset_int_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DmaResetIntMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DmaResetIntMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(276usize),
            )
        }
    }

    #[doc = "DMA Set Interrupt mask register"]
    #[inline(always)]
    pub const fn dma_set_int_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DmaSetIntMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DmaSetIntMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
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

#[doc = "Source address register of DMA channel 0"]
pub type Dma0AStartReg = crate::RegValueT<Dma0AStartReg_SPEC>;

impl Dma0AStartReg {
    #[doc = "Source start address of channel 0"]
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

#[doc = "Destination address register of DMA channel 0"]
pub type Dma0BStartReg = crate::RegValueT<Dma0BStartReg_SPEC>;

impl Dma0BStartReg {
    #[doc = "Destination start address of channel 0"]
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

#[doc = "Control register of DMA channel 0"]
pub type Dma0CtrlReg = crate::RegValueT<Dma0CtrlReg_SPEC>;

impl Dma0CtrlReg {
    #[doc = "0: DMA channel de-asserts the bus request upon completion of the write transfer (burst or single-shot)\n1: DMA channel keeps on requesting the bus upon completion of the write. This is effective only in Memory-to-Memory transfers (DREQ_MODE = \'0\') and results into requesting the bus continuously during the whole transfer, to speed-up its completion (default)."]
    #[inline(always)]
    pub fn dma_exclusive_access(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Ignores bus error response from the AHB bus, so DMA continues normally.\n1: Detects the bus response and tracks any bus error may occur during the transfer. If a bus error is detected, the channel completes the current read-write DMA cycle (either in burst or single transfers mode) and then closes the transfer, de-asserting DMA_ON bit automatically.\nIt is noted that the respective bus error detection status bit of DMA_INT_STATUS_REG is automatically cleared as soon as the channel is switched-on again, in order to perform a new transfer."]
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the DMA read/write bursts, according to the following configuration:\n00 = Bursts are disabled\n01 = Bursts of 4 are enabled\n10 = Bursts of 8 are enabled\n11 = Reserved"]
    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma0CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA operates with level-sensitive peripheral requests (default)\n1: DMA operates with (positive) edge-sensitive peripheral requests"]
    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA performs copy A1 to B1, A2 to B2, etc ...\n1: DMA performs copy of A1 to B1, B2, etc ...\nThis feature is useful for memory initialization to any value. Thus, BINC must be set to \'1\', while AINC is don\'t care, as only one fetch from A is done. This process cannot be interrupted by other DMA channels. It is also noted that DMA_INIT should not be used when DREQ_MODE=\'1\'."]
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Blocking mode, the DMA performs a fast back-to-back copy, disabling bus access for any bus master with lower priority.\n1: Interrupting mode, the DMA inserts a wait cycle after each store allowing the CPU to steal cycles or cache to perform a burst read. If DREQ_MODE=\'1\', DMA_IDLE is don\'t care."]
    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The priority level determines which DMA channel will be granted access for transferring data, in case more than one channels are active and request the bus at the same time. The greater the value, the higher the priority. In specific:\n000: lowest priority\n111: highest priority\nIf different channels with equal priority level values request the bus at the same time, an inherent priority mechanism is applied. According to this mechanism, if, for example, both the DMA0 and DMA1 channels have the same priority level, then DMA0 will first be granted access to the bus."]
    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma0CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Normal mode. The DMA channel stops after having completed the transfer of length determined by DMAx_LEN_REG. DMA_ON automatically deasserts when the transfer is completed.\n1: Circular mode (applicable only if DREQ_MODE = \'1\'). In this mode, DMA_ON never deasserts, as the DMA channel automatically resets DMAx_IDX_REG and starts a new transfer."]
    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of source address.\n0 = do not increment (source address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of destination address.\n0 = do not increment (destination address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel starts immediately\n1: DMA channel must be triggered by peripheral DMA request (see also the description of DMA_REQ_MUX_REG)"]
    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma0CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bus transfer width:\n00 = 1 Byte (suggested for peripherals like UART and 8-bit SPI)\n01 = 2 Bytes (suggested for peripherals like I2C and 16-bit SPI)\n10 = 4 Bytes (suggested for Memory-to-Memory transfers)\n11 = Reserved"]
    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma0CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma0CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel is off, clocks are disabled\n1: DMA channel is enabled. This bit will be automatically cleared after the completion of a transfer, if Circular mode is not enabled. In Circular mode, this bit stays set.\nNOTE: If DMA_ON is disabled by SW while the DMA channel is active, it cannot be enabled again until the channel has completed the last on-going read-write cycle and has stopped. Thus, the SW has to check that the reading of DMA0_CTRL_REG.DMA_ON returns 0, before setting again the specific bit-field."]
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
        <crate::RegValueT<Dma0CtrlReg_SPEC> as RegisterValue<_>>::new(98304)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma0IdxReg_SPEC {
    type DataType = u32;
}

#[doc = "Index pointer register of DMA channel 0"]
pub type Dma0IdxReg = crate::RegValueT<Dma0IdxReg_SPEC>;

impl Dma0IdxReg {
    #[doc = "This (read-only) register determines the data items already transferred by the DMA channel. Hence, if its value is 1, then the DMA channel has already copied one data item and it is currently performing the next copy. If its value is 2, then two items have already been copied, and so on.\nWhen the transfer is completed (so when DMA0_CTRL_REG\\[DMA_ON\\] has been cleared) and DMA0_CTRL_REG\\[CIRCULAR\\] is not set, the register keeps its (last) value (which should be equal to DMA0_LEN_REG) and it is automatically reset to 0 upon starting a new transfer. In Circular mode, the register is automatically initialized to 0 as soon as the DMA channel starts-over again."]
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

#[doc = "Interrupt length register of DMA channel 0"]
pub type Dma0IntReg = crate::RegValueT<Dma0IntReg_SPEC>;

impl Dma0IntReg {
    #[doc = "Number of transfers until an interrupt is generated. The interrupt is generated after a transfer, if and only if DMA0_INT_REG has reached DMA0_IDX_REG and before DMA0_IDX_REG is incremented. The interrupt enable bit of this this channel must be already enabled, to let the channel\'s controller generate the interrupt (see also DMA_INT_MASK_REG and the SET/RESET interrupt registers)."]
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

#[doc = "Transfer length register of DMA channel 0"]
pub type Dma0LenReg = crate::RegValueT<Dma0LenReg_SPEC>;

impl Dma0LenReg {
    #[doc = "DMA channel\'s transfer length. DMA0_LEN of value 0, 1, 2, ... results into an actual transfer length of 1, 2, 3, ..."]
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

#[doc = "Source address register of DMA channel 1"]
pub type Dma1AStartReg = crate::RegValueT<Dma1AStartReg_SPEC>;

impl Dma1AStartReg {
    #[doc = "Source start address of channel 1"]
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

#[doc = "Destination address register of DMA channel 1"]
pub type Dma1BStartReg = crate::RegValueT<Dma1BStartReg_SPEC>;

impl Dma1BStartReg {
    #[doc = "Destination start address of channel 1"]
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

#[doc = "Control register of DMA channel 1"]
pub type Dma1CtrlReg = crate::RegValueT<Dma1CtrlReg_SPEC>;

impl Dma1CtrlReg {
    #[doc = "0: DMA channel de-asserts the bus request upon completion of the write transfer (burst or single-shot)\n1: DMA channel keeps on requesting the bus upon completion of the write. This is effective only in Memory-to-Memory transfers (DREQ_MODE = \'0\') and results into requesting the bus continuously during the whole transfer, to speed-up its completion (default)."]
    #[inline(always)]
    pub fn dma_exclusive_access(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Ignores bus error response from the AHB bus, so DMA continues normally.\n1: Detects the bus response and tracks any bus error may occur during the transfer. If a bus error is detected, the channel completes the current read-write DMA cycle (either in burst or single transfers mode) and then closes the transfer, de-asserting DMA_ON bit automatically.\nIt is noted that the respective bus error detection status bit of DMA_INT_STATUS_REG is automatically cleared as soon as the channel is switched-on again, in order to perform a new transfer."]
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the DMA read/write bursts, according to the following configuration:\n00 = Bursts are disabled\n01 = Bursts of 4 are enabled\n10 = Bursts of 8 are enabled\n11 = Reserved"]
    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA operates with level-sensitive peripheral requests (default)\n1: DMA operates with (positive) edge-sensitive peripheral requests"]
    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA performs copy A1 to B1, A2 to B2, etc ...\n1: DMA performs copy of A1 to B1, B2, etc ...\nThis feature is useful for memory initialization to any value. Thus, BINC must be set to \'1\', while AINC is don\'t care, as only one fetch from A is done. This process cannot be interrupted by other DMA channels. It is also noted that DMA_INIT should not be used when DREQ_MODE=\'1\'."]
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Blocking mode, the DMA performs a fast back-to-back copy, disabling bus access for any bus master with lower priority.\n1: Interrupting mode, the DMA inserts a wait cycle after each store allowing the CPU to steal cycles or cache to perform a burst read. If DREQ_MODE=\'1\', DMA_IDLE is don\'t care."]
    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The priority level determines which DMA channel will be granted access for transferring data, in case more than one channels are active and request the bus at the same time. The greater the value, the higher the priority. In specific:\n000 = lowest priority\n111 = highest priority\nIf different channels with equal priority level values request the bus at the same time, an inherent priority mechanism is applied. According to this mechanism, if, for example, both the DMA0 and DMA1 channels have the same priority level, then DMA0 will first be granted access to the bus."]
    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Normal mode. The DMA channel stops after having completed the transfer of length determined by DMAx_LEN_REG. DMA_ON automatically deasserts when the transfer is completed.\n1: Circular mode (applicable only if DREQ_MODE = \'1\'). In this mode, DMA_ON never deasserts, as the DMA channel automatically resets DMAx_IDX_REG and starts a new transfer."]
    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of source address.\n0 = do not increment (source address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of destination address.\n0 = do not increment (destination address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel starts immediately\n1: DMA channel must be triggered by peripheral DMA request (see also the description of DMA_REQ_MUX_REG)"]
    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bus transfer width:\n00 = 1 Byte (suggested for peripherals like UART and 8-bit SPI)\n01 = 2 Bytes (suggested for peripherals like I2C and 16-bit SPI)\n10 = 4 Bytes (suggested for Memory-to-Memory transfers)\n11 = Reserved"]
    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel is off, clocks are disabled\n1: DMA channel is enabled. This bit will be automatically cleared after the completion of a transfer, if Circular mode is not enabled. In Circular mode, this bit stays set.\nNOTE: If DMA_ON is disabled by SW while the DMA channel is active, it cannot be enabled again until the channel has completed the last on-going read-write cycle and has stopped. Thus, the SW has to check that the reading of DMA1_CTRL_REG.DMA_ON returns 0, before setting again the specific bit-field."]
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
        <crate::RegValueT<Dma1CtrlReg_SPEC> as RegisterValue<_>>::new(98304)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma1IdxReg_SPEC {
    type DataType = u32;
}

#[doc = "Index pointer register of DMA channel 1"]
pub type Dma1IdxReg = crate::RegValueT<Dma1IdxReg_SPEC>;

impl Dma1IdxReg {
    #[doc = "This (read-only) register determines the data items already transferred by the DMA channel. Hence, if its value is 1, then the DMA channel has already copied one data item and it is currently performing the next copy. If its value is 2, then two items have already been copied, and so on.\nWhen the transfer is completed (so when DMA1_CTRL_REG\\[DMA_ON\\] has been cleared) and DMA1_CTRL_REG\\[CIRCULAR\\] is not set, the register keeps its (last) value (which should be equal to DMA1_LEN_REG) and it is automatically reset to 0 upon starting a new transfer. In Circular mode, the register is automatically initialized to 0 as soon as the DMA channel starts-over again."]
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

#[doc = "Interrupt length register of DMA channel 1"]
pub type Dma1IntReg = crate::RegValueT<Dma1IntReg_SPEC>;

impl Dma1IntReg {
    #[doc = "Number of transfers until an interrupt is generated. The interrupt is generated after a transfer, if and only if DMA1_INT_REG has reached DMA1_IDX_REG and before DMA1_IDX_REG is incremented. The interrupt enable bit of this this channel must be already enabled, to let the channel\'s controller generate the interrupt (see also DMA_INT_MASK_REG and the SET/RESET interrupt registers)."]
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

#[doc = "Transfer length register of DMA channel 1"]
pub type Dma1LenReg = crate::RegValueT<Dma1LenReg_SPEC>;

impl Dma1LenReg {
    #[doc = "DMA channel\'s transfer length. DMA1_LEN of value 0, 1, 2, ... results into an actual transfer length of 1, 2, 3, ..."]
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

#[doc = "Source address register of DMA channel 2"]
pub type Dma2AStartReg = crate::RegValueT<Dma2AStartReg_SPEC>;

impl Dma2AStartReg {
    #[doc = "Source start address of channel 2"]
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

#[doc = "Destination address register of DMA channel 2"]
pub type Dma2BStartReg = crate::RegValueT<Dma2BStartReg_SPEC>;

impl Dma2BStartReg {
    #[doc = "Destination start address of channel 2"]
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

#[doc = "Control register of DMA channel 2"]
pub type Dma2CtrlReg = crate::RegValueT<Dma2CtrlReg_SPEC>;

impl Dma2CtrlReg {
    #[doc = "0: DMA channel de-asserts the bus request upon completion of the write transfer (burst or single-shot)\n1: DMA channel keeps on requesting the bus upon completion of the write. This is effective only in Memory-to-Memory transfers (DREQ_MODE = \'0\') and results into requesting the bus continuously during the whole transfer, to speed-up its completion (default)."]
    #[inline(always)]
    pub fn dma_exclusive_access(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Ignores bus error response from the AHB bus, so DMA continues normally.\n1: Detects the bus response and tracks any bus error may occur during the transfer. If a bus error is detected, the channel completes the current read-write DMA cycle (either in burst or single transfers mode) and then closes the transfer, de-asserting DMA_ON bit automatically.\nIt is noted that the respective bus error detection status bit of DMA_INT_STATUS_REG is automatically cleared as soon as the channel is switched-on again, in order to perform a new transfer."]
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the DMA read/write bursts, according to the following configuration:\n00 = Bursts are disabled\n01 = Bursts of 4 are enabled\n10 = Bursts of 8 are enabled\n11 = Reserved"]
    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma2CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA operates with level-sensitive peripheral requests (default)\n1: DMA operates with (positive) edge-sensitive peripheral requests"]
    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA performs copy A1 to B1, A2 to B2, etc ...\n1: DMA performs copy of A1 to B1, B2, etc ...\nThis feature is useful for memory initialization to any value. Thus, BINC must be set to \'1\', while AINC is don\'t care, as only one fetch from A is done. This process cannot be interrupted by other DMA channels. It is also noted that DMA_INIT should not be used when DREQ_MODE=\'1\'."]
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Blocking mode, the DMA performs a fast back-to-back copy, disabling bus access for any bus master with lower priority.\n1: Interrupting mode, the DMA inserts a wait cycle after each store allowing the CPU to steal cycles or cache to perform a burst read. If DREQ_MODE=\'1\', DMA_IDLE is don\'t care."]
    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The priority level determines which DMA channel will be granted access for transferring data, in case more than one channels are active and request the bus at the same time. The greater the value, the higher the priority. In specific:\n000 = lowest priority\n111 = highest priority\nIf different channels with equal priority level values request the bus at the same time, an inherent priority mechanism is applied. According to this mechanism, if, for example, both the DMA0 and DMA1 channels have the same priority level, then DMA0 will first be granted access to the bus."]
    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma2CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Normal mode. The DMA channel stops after having completed the transfer of length determined by DMAx_LEN_REG. DMA_ON automatically deasserts when the transfer is completed.\n1: Circular mode (applicable only if DREQ_MODE = \'1\'). In this mode, DMA_ON never deasserts, as the DMA channel automatically resets DMAx_IDX_REG and starts a new transfer."]
    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of destination address.\n0 = do not increment (destination address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of destination address\n0 = do not increment\n1 = increment according value of BW"]
    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel starts immediately\n1: DMA channel must be triggered by peripheral DMA request (see also the description of DMA_REQ_MUX_REG)"]
    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bus transfer width:\n00 = 1 Byte (suggested for peripherals like UART and 8-bit SPI)\n01 = 2 Bytes (suggested for peripherals like I2C and 16-bit SPI)\n10 = 4 Bytes (suggested for Memory-to-Memory transfers)\n11 = Reserved"]
    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma2CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel is off, clocks are disabled\n1: DMA channel is enabled. This bit will be automatically cleared after the completion of a transfer, if Circular mode is not enabled. In Circular mode, this bit stays set.\nNOTE: If DMA_ON is disabled by SW while the DMA channel is active, it cannot be enabled again until the channel has completed the last on-going read-write cycle and has stopped. Thus, the SW has to check that the reading of DMA2_CTRL_REG.DMA_ON returns 0, before setting again the specific bit-field."]
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
        <crate::RegValueT<Dma2CtrlReg_SPEC> as RegisterValue<_>>::new(98304)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma2IdxReg_SPEC {
    type DataType = u32;
}

#[doc = "Index pointer register of DMA channel 2"]
pub type Dma2IdxReg = crate::RegValueT<Dma2IdxReg_SPEC>;

impl Dma2IdxReg {
    #[doc = "This (read-only) register determines the data items already transferred by the DMA channel. Hence, if its value is 1, then the DMA channel has already copied one data item and it is currently performing the next copy. If its value is 2, then two items have already been copied, and so on.\nWhen the transfer is completed (so when DMA2_CTRL_REG\\[DMA_ON\\] has been cleared) and DMA2_CTRL_REG\\[CIRCULAR\\] is not set, the register keeps its (last) value (which should be equal to DMA2_LEN_REG) and it is automatically reset to 0 upon starting a new transfer. In Circular mode, the register is automatically initialized to 0 as soon as the DMA channel starts-over again."]
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

#[doc = "Interrupt length register of DMA channel 2"]
pub type Dma2IntReg = crate::RegValueT<Dma2IntReg_SPEC>;

impl Dma2IntReg {
    #[doc = "Number of transfers until an interrupt is generated. The interrupt is generated after a transfer, if and only if DMA2_INT_REG has reached DMA2_IDX_REG and before DMA2_IDX_REG is incremented. The interrupt enable bit of this this channel must be already enabled, to let the channel\'s controller generate the interrupt (see also DMA_INT_MASK_REG and the SET/RESET interrupt registers)."]
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

#[doc = "Transfer length register of DMA channel 2"]
pub type Dma2LenReg = crate::RegValueT<Dma2LenReg_SPEC>;

impl Dma2LenReg {
    #[doc = "DMA channel\'s transfer length. DMA2_LEN of value 0, 1, 2, ... results into an actual transfer length of 1, 2, 3, ..."]
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

#[doc = "Source address register of DMA channel 3"]
pub type Dma3AStartReg = crate::RegValueT<Dma3AStartReg_SPEC>;

impl Dma3AStartReg {
    #[doc = "Source start address of channel 3"]
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

#[doc = "Destination address register of DMA channel 3"]
pub type Dma3BStartReg = crate::RegValueT<Dma3BStartReg_SPEC>;

impl Dma3BStartReg {
    #[doc = "Destination start address of channel 3"]
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

#[doc = "Control register of DMA channel 3"]
pub type Dma3CtrlReg = crate::RegValueT<Dma3CtrlReg_SPEC>;

impl Dma3CtrlReg {
    #[doc = "0: DMA channel de-asserts the bus request upon completion of the write transfer (burst or single-shot)\n1: DMA channel keeps on requesting the bus upon completion of the write. This is effective only in Memory-to-Memory transfers (DREQ_MODE = \'0\') and results into requesting the bus continuously during the whole transfer, to speed-up its completion (default)."]
    #[inline(always)]
    pub fn dma_exclusive_access(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Ignores bus error response from the AHB bus, so DMA continues normally.\n1: Detects the bus response and tracks any bus error may occur during the transfer. If a bus error is detected, the channel completes the current read-write DMA cycle (either in burst or single transfers mode) and then closes the transfer, de-asserting DMA_ON bit automatically.\nIt is noted that the respective bus error detection status bit of DMA_INT_STATUS_REG is automatically cleared as soon as the channel is switched-on again, in order to perform a new transfer."]
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the DMA read/write bursts, according to the following configuration:\n00 = Bursts are disabled\n01 = Bursts of 4 are enabled\n10 = Bursts of 8 are enabled\n11 = Reserved"]
    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA operates with level-sensitive peripheral requests (default)\n1: DMA operates with (positive) edge-sensitive peripheral requests"]
    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA performs copy A1 to B1, A2 to B2, etc ...\n1: DMA performs copy of A1 to B1, B2, etc ...\nThis feature is useful for memory initialization to any value. Thus, BINC must be set to \'1\', while AINC is don\'t care, as only one fetch from A is done. This process cannot be interrupted by other DMA channels. It is also noted that DMA_INIT should not be used when DREQ_MODE=\'1\'."]
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Blocking mode, the DMA performs a fast back-to-back copy, disabling bus access for any bus master with lower priority.\n1: Interrupting mode, the DMA inserts a wait cycle after each store allowing the CPU to steal cycles or cache to perform a burst read. If DREQ_MODE=\'1\', DMA_IDLE is don\'t care."]
    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The priority level determines which DMA channel will be granted access for transferring data, in case more than one channels are active and request the bus at the same time. The greater the value, the higher the priority. In specific:\n000 = lowest priority\n111 = highest priority\nIf different channels with equal priority level values request the bus at the same time, an inherent priority mechanism is applied. According to this mechanism, if, for example, both the DMA0 and DMA1 channels have the same priority level, then DMA0 will first be granted access to the bus."]
    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Normal mode. The DMA channel stops after having completed the transfer of length determined by DMAx_LEN_REG. DMA_ON automatically deasserts when the transfer is completed.\n1: Circular mode (applicable only if DREQ_MODE = \'1\'). In this mode, DMA_ON never deasserts, as the DMA channel automatically resets DMAx_IDX_REG and starts a new transfer."]
    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of source address.\n0 = do not increment (source address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of destination address.\n0 = do not increment (destination address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel starts immediately\n1: DMA channel must be triggered by peripheral DMA request (see also the description of DMA_REQ_MUX_REG)"]
    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma3CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bus transfer width:\n00 = 1 Byte (suggested for peripherals like UART and 8-bit SPI)\n01 = 2 Bytes (suggested for peripherals like I2C and 16-bit SPI)\n10 = 4 Bytes (suggested for Memory-to-Memory transfers)\n11 = Reserved"]
    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma3CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma3CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel is off, clocks are disabled\n1: DMA channel is enabled. This bit will be automatically cleared after the completion of a transfer, if Circular mode is not enabled. In Circular mode, this bit stays set.\nNOTE: If DMA_ON is disabled by SW while the DMA channel is active, it cannot be enabled again until the channel has completed the last on-going read-write cycle and has stopped. Thus, the SW has to check that the reading of DMA3_CTRL_REG.DMA_ON returns 0, before setting again the specific bit-field."]
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
        <crate::RegValueT<Dma3CtrlReg_SPEC> as RegisterValue<_>>::new(98304)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma3IdxReg_SPEC {
    type DataType = u32;
}

#[doc = "Index pointer register of DMA channel 3"]
pub type Dma3IdxReg = crate::RegValueT<Dma3IdxReg_SPEC>;

impl Dma3IdxReg {
    #[doc = "This (read-only) register determines the data items already transferred by the DMA channel. Hence, if its value is 1, then the DMA channel has already copied one data item and it is currently performing the next copy. If its value is 2, then two items have already been copied, and so on.\nWhen the transfer is completed (so when DMA3_CTRL_REG\\[DMA_ON\\] has been cleared) and DMA3_CTRL_REG\\[CIRCULAR\\] is not set, the register keeps its (last) value (which should be equal to DMA3_LEN_REG) and it is automatically reset to 0 upon starting a new transfer. In Circular mode, the register is automatically initialized to 0 as soon as the DMA channel starts-over again."]
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

#[doc = "Interrupt length register of DMA channel 3"]
pub type Dma3IntReg = crate::RegValueT<Dma3IntReg_SPEC>;

impl Dma3IntReg {
    #[doc = "Number of transfers until an interrupt is generated. The interrupt is generated after a transfer, if and only if DMA3_INT_REG has reached DMA3_IDX_REG and before DMA3_IDX_REG is incremented. The interrupt enable bit of this this channel must be already enabled, to let the channel\'s controller generate the interrupt (see also DMA_INT_MASK_REG and the SET/RESET interrupt registers)."]
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

#[doc = "Transfer length register of DMA channel 3"]
pub type Dma3LenReg = crate::RegValueT<Dma3LenReg_SPEC>;

impl Dma3LenReg {
    #[doc = "DMA channel\'s transfer length. DMA3_LEN of value 0, 1, 2, ... results into an actual transfer length of 1, 2, 3, ..."]
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

#[doc = "Source address register of DMA channel 4"]
pub type Dma4AStartReg = crate::RegValueT<Dma4AStartReg_SPEC>;

impl Dma4AStartReg {
    #[doc = "Source start address of channel 4"]
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

#[doc = "Destination address register of DMA channel 4"]
pub type Dma4BStartReg = crate::RegValueT<Dma4BStartReg_SPEC>;

impl Dma4BStartReg {
    #[doc = "Destination start address of channel 4"]
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

#[doc = "Control register of DMA channel 4"]
pub type Dma4CtrlReg = crate::RegValueT<Dma4CtrlReg_SPEC>;

impl Dma4CtrlReg {
    #[doc = "0: DMA channel de-asserts the bus request upon completion of the write transfer (burst or single-shot)\n1: DMA channel keeps on requesting the bus upon completion of the write. This is effective only in Memory-to-Memory transfers (DREQ_MODE = \'0\') and results into requesting the bus continuously during the whole transfer, to speed-up its completion (default)."]
    #[inline(always)]
    pub fn dma_exclusive_access(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Ignores bus error response from the AHB bus, so DMA continues normally.\n1: Detects the bus response and tracks any bus error may occur during the transfer. If a bus error is detected, the channel completes the current read-write DMA cycle (either in burst or single transfers mode) and then closes the transfer, de-asserting DMA_ON bit automatically.\nIt is noted that the respective bus error detection status bit of DMA_INT_STATUS_REG is automatically cleared as soon as the channel is switched-on again, in order to perform a new transfer."]
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the DMA read/write bursts, according to the following configuration:\n00 = Bursts are disabled\n01 = Bursts of 4 are enabled\n10 = Bursts of 8 are enabled\n11 = Reserved"]
    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma4CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA operates with level-sensitive peripheral requests (default)\n1: DMA operates with (positive) edge-sensitive peripheral requests"]
    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA performs copy A1 to B1, A2 to B2, etc ...\n1: DMA performs copy of A1 to B1, B2, etc ...\nThis feature is useful for memory initialization to any value. Thus, BINC must be set to \'1\', while AINC is don\'t care, as only one fetch from A is done. This process cannot be interrupted by other DMA channels. It is also noted that DMA_INIT should not be used when DREQ_MODE=\'1\'."]
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Blocking mode, the DMA performs a fast back-to-back copy, disabling bus access for any bus master with lower priority.\n1: Interrupting mode, the DMA inserts a wait cycle after each store allowing the CPU to steal cycles or cache to perform a burst read. If DREQ_MODE=\'1\', DMA_IDLE is don\'t care."]
    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The priority level determines which DMA channel will be granted access for transferring data, in case more than one channels are active and request the bus at the same time. The greater the value, the higher the priority. In specific:\n000 = lowest priority\n111 = highest priority\nIf different channels with equal priority level values request the bus at the same time, an inherent priority mechanism is applied. According to this mechanism, if, for example, both the DMA0 and DMA1 channels have the same priority level, then DMA0 will first be granted access to the bus."]
    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma4CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Normal mode. The DMA channel stops after having completed the transfer of length determined by DMAx_LEN_REG. DMA_ON automatically deasserts when the transfer is completed.\n1: Circular mode (applicable only if DREQ_MODE = \'1\'). In this mode, DMA_ON never deasserts, as the DMA channel automatically resets DMAx_IDX_REG and starts a new transfer."]
    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of source address.\n0 = do not increment (source address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of destination address.\n0 = do not increment (destination address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel starts immediately\n1: DMA channel must be triggered by peripheral DMA request (see also the description of DMA_REQ_MUX_REG)"]
    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bus transfer width:\n00 = 1 Byte (suggested for peripherals like UART and 8-bit SPI)\n01 = 2 Bytes (suggested for peripherals like I2C and 16-bit SPI)\n10 = 4 Bytes (suggested for Memory-to-Memory transfers)\n11 = Reserved"]
    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma4CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel is off, clocks are disabled\n1: DMA channel is enabled. This bit will be automatically cleared after the completion of a transfer, if Circular mode is not enabled. In Circular mode, this bit stays set.\nNOTE: If DMA_ON is disabled by SW while the DMA channel is active, it cannot be enabled again until the channel has completed the last on-going read-write cycle and has stopped. Thus, the SW has to check that the reading of DMA4_CTRL_REG.DMA_ON returns 0, before setting again the specific bit-field."]
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
        <crate::RegValueT<Dma4CtrlReg_SPEC> as RegisterValue<_>>::new(98304)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma4IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma4IdxReg_SPEC {
    type DataType = u32;
}

#[doc = "Index pointer register of DMA channel 4"]
pub type Dma4IdxReg = crate::RegValueT<Dma4IdxReg_SPEC>;

impl Dma4IdxReg {
    #[doc = "This (read-only) register determines the data items already transferred by the DMA channel. Hence, if its value is 1, then the DMA channel has already copied one data item and it is currently performing the next copy. If its value is 2, then two items have already been copied, and so on.\nWhen the transfer is completed (so when DMA4_CTRL_REG\\[DMA_ON\\] has been cleared) and DMA4_CTRL_REG\\[CIRCULAR\\] is not set, the register keeps its (last) value (which should be equal to DMA4_LEN_REG) and it is automatically reset to 0 upon starting a new transfer. In Circular mode, the register is automatically initialized to 0 as soon as the DMA channel starts-over again."]
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

#[doc = "Interrupt length register of DMA channel 4"]
pub type Dma4IntReg = crate::RegValueT<Dma4IntReg_SPEC>;

impl Dma4IntReg {
    #[doc = "Number of transfers until an interrupt is generated. The interrupt is generated after a transfer, if and only if DMA4_INT_REG has reached DMA4_IDX_REG and before DMA4_IDX_REG is incremented. The interrupt enable bit of this this channel must be already enabled, to let the channel\'s controller generate the interrupt (see also DMA_INT_MASK_REG and the SET/RESET interrupt registers)."]
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

#[doc = "Transfer length register of DMA channel 4"]
pub type Dma4LenReg = crate::RegValueT<Dma4LenReg_SPEC>;

impl Dma4LenReg {
    #[doc = "DMA channel\'s transfer length. DMA4_LEN of value 0, 1, 2, ... results into an actual transfer length of 1, 2, 3, ..."]
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

#[doc = "Source address register of DMA channel 5"]
pub type Dma5AStartReg = crate::RegValueT<Dma5AStartReg_SPEC>;

impl Dma5AStartReg {
    #[doc = "Source start address of channel 5"]
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

#[doc = "Destination address register of DMA channel 5"]
pub type Dma5BStartReg = crate::RegValueT<Dma5BStartReg_SPEC>;

impl Dma5BStartReg {
    #[doc = "Destination start address of channel 5"]
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

#[doc = "Control register of DMA channel 5"]
pub type Dma5CtrlReg = crate::RegValueT<Dma5CtrlReg_SPEC>;

impl Dma5CtrlReg {
    #[doc = "0: DMA channel de-asserts the bus request upon completion of the write transfer (burst or single-shot)\n1: DMA channel keeps on requesting the bus upon completion of the write. This is effective only in Memory-to-Memory transfers (DREQ_MODE = \'0\') and results into requesting the bus continuously during the whole transfer, to speed-up its completion (default)."]
    #[inline(always)]
    pub fn dma_exclusive_access(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Ignores bus error response from the AHB bus, so DMA continues normally.\n1: Detects the bus response and tracks any bus error may occur during the transfer. If a bus error is detected, the channel completes the current read-write DMA cycle (either in burst or single transfers mode) and then closes the transfer, de-asserting DMA_ON bit automatically.\nIt is noted that the respective bus error detection status bit of DMA_INT_STATUS_REG is automatically cleared as soon as the channel is switched-on again, in order to perform a new transfer."]
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the DMA read/write bursts, according to the following configuration:\n00 = Bursts are disabled\n01 = Bursts of 4 are enabled\n10 = Bursts of 8 are enabled\n11 = Reserved"]
    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma5CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA operates with level-sensitive peripheral requests (default)\n1: DMA operates with (positive) edge-sensitive peripheral requests"]
    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA performs copy A1 to B1, A2 to B2, etc ...\n1: DMA performs copy of A1 to B1, B2, etc ...\nThis feature is useful for memory initialization to any value. Thus, BINC must be set to \'1\', while AINC is don\'t care, as only one fetch from A is done. This process cannot be interrupted by other DMA channels. It is also noted that DMA_INIT should not be used when DREQ_MODE=\'1\'."]
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Blocking mode, the DMA performs a fast back-to-back copy, disabling bus access for any bus master with lower priority.\n1: Interrupting mode, the DMA inserts a wait cycle after each store allowing the CPU to steal cycles or cache to perform a burst read. If DREQ_MODE=\'1\', DMA_IDLE is don\'t care."]
    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The priority level determines which DMA channel will be granted access for transferring data, in case more than one channels are active and request the bus at the same time. The greater the value, the higher the priority. In specific:\n000 = lowest priority\n111 = highest priority\nIf different channels with equal priority level values request the bus at the same time, an inherent priority mechanism is applied. According to this mechanism, if, for example, both the DMA0 and DMA1 channels have the same priority level, then DMA0 will first be granted access to the bus."]
    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma5CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Normal mode. The DMA channel stops after having completed the transfer of length determined by DMAx_LEN_REG. DMA_ON automatically deasserts when the transfer is completed.\n1: Circular mode (applicable only if DREQ_MODE = \'1\'). In this mode, DMA_ON never deasserts, as the DMA channel automatically resets DMAx_IDX_REG and starts a new transfer."]
    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of source address.\n0 = do not increment (source address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of destination address.\n0 = do not increment (destination address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel starts immediately\n1: DMA channel must be triggered by peripheral DMA request (see also the description of DMA_REQ_MUX_REG)"]
    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma5CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bus transfer width:\n00 = 1 Byte (suggested for peripherals like UART and 8-bit SPI)\n01 = 2 Bytes (suggested for peripherals like I2C and 16-bit SPI)\n10 = 4 Bytes (suggested for Memory-to-Memory transfers)\n11 = Reserved"]
    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma5CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma5CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel is off, clocks are disabled\n1: DMA channel is enabled. This bit will be automatically cleared after the completion of a transfer, if Circular mode is not enabled. In Circular mode, this bit stays set.\nNOTE: If DMA_ON is disabled by SW while the DMA channel is active, it cannot be enabled again until the channel has completed the last on-going read-write cycle and has stopped. Thus, the SW has to check that the reading of DMA5_CTRL_REG.DMA_ON returns 0, before setting again the specific bit-field."]
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
        <crate::RegValueT<Dma5CtrlReg_SPEC> as RegisterValue<_>>::new(98304)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma5IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma5IdxReg_SPEC {
    type DataType = u32;
}

#[doc = "Index pointer register of DMA channel 5"]
pub type Dma5IdxReg = crate::RegValueT<Dma5IdxReg_SPEC>;

impl Dma5IdxReg {
    #[doc = "This (read-only) register determines the data items already transferred by the DMA channel. Hence, if its value is 1, then the DMA channel has already copied one data item and it is currently performing the next copy. If its value is 2, then two items have already been copied, and so on.\nWhen the transfer is completed (so when DMA5_CTRL_REG\\[DMA_ON\\] has been cleared) and DMA5_CTRL_REG\\[CIRCULAR\\] is not set, the register keeps its (last) value (which should be equal to DMA5_LEN_REG) and it is automatically reset to 0 upon starting a new transfer. In Circular mode, the register is automatically initialized to 0 as soon as the DMA channel starts-over again."]
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

#[doc = "Interrupt length register of DMA channel 5"]
pub type Dma5IntReg = crate::RegValueT<Dma5IntReg_SPEC>;

impl Dma5IntReg {
    #[doc = "Number of transfers until an interrupt is generated. The interrupt is generated after a transfer, if and only if DMA5_INT_REG has reached DMA5_IDX_REG and before DMA5_IDX_REG is incremented. The interrupt enable bit of this this channel must be already enabled, to let the channel\'s controller generate the interrupt (see also DMA_INT_MASK_REG and the SET/RESET interrupt registers)."]
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

#[doc = "Transfer length register of DMA channel 5"]
pub type Dma5LenReg = crate::RegValueT<Dma5LenReg_SPEC>;

impl Dma5LenReg {
    #[doc = "DMA channel\'s transfer length. DMA5_LEN of value 0, 1, 2, ... results into an actual transfer length of 1, 2, 3, ..."]
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

#[doc = "Source address register of DMA channel 6"]
pub type Dma6AStartReg = crate::RegValueT<Dma6AStartReg_SPEC>;

impl Dma6AStartReg {
    #[doc = "Source start address of channel 6"]
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

#[doc = "Destination address register of DMA channel 6"]
pub type Dma6BStartReg = crate::RegValueT<Dma6BStartReg_SPEC>;

impl Dma6BStartReg {
    #[doc = "Destination start address of channel 6"]
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

#[doc = "Control register of DMA channel 6"]
pub type Dma6CtrlReg = crate::RegValueT<Dma6CtrlReg_SPEC>;

impl Dma6CtrlReg {
    #[doc = "0: DMA channel de-asserts the bus request upon completion of the write transfer (burst or single-shot)\n1: DMA channel keeps on requesting the bus upon completion of the write. This is effective only in Memory-to-Memory transfers (DREQ_MODE = \'0\') and results into requesting the bus continuously during the whole transfer, to speed-up its completion (default)."]
    #[inline(always)]
    pub fn dma_exclusive_access(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Ignores bus error response from the AHB bus, so DMA continues normally.\n1: Detects the bus response and tracks any bus error may occur during the transfer. If a bus error is detected, the channel completes the current read-write DMA cycle (either in burst or single transfers mode) and then closes the transfer, de-asserting DMA_ON bit automatically.\nIt is noted that the respective bus error detection status bit of DMA_INT_STATUS_REG is automatically cleared as soon as the channel is switched-on again, in order to perform a new transfer."]
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the DMA read/write bursts, according to the following configuration:\n00 = Bursts are disabled\n01 = Bursts of 4 are enabled\n10 = Bursts of 8 are enabled\n11 = Reserved"]
    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma6CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA operates with level-sensitive peripheral requests (default)\n1: DMA operates with (positive) edge-sensitive peripheral requests"]
    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA performs copy A1 to B1, A2 to B2, etc ...\n1: DMA performs copy of A1 to B1, B2, etc ...\nThis feature is useful for memory initialization to any value. Thus, BINC must be set to \'1\', while AINC is don\'t care, as only one fetch from A is done. This process cannot be interrupted by other DMA channels. It is also noted that DMA_INIT should not be used when DREQ_MODE=\'1\'."]
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Blocking mode, the DMA performs a fast back-to-back copy, disabling bus access for any bus master with lower priority.\n1: Interrupting mode, the DMA inserts a wait cycle after each store allowing the CPU to steal cycles or cache to perform a burst read. If DREQ_MODE=\'1\', DMA_IDLE is don\'t care."]
    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The priority level determines which DMA channel will be granted access for transferring data, in case more than one channels are active and request the bus at the same time. The greater the value, the higher the priority. In specific:\n000 = lowest priority\n111 = highest priority\nIf different channels with equal priority level values request the bus at the same time, an inherent priority mechanism is applied. According to this mechanism, if, for example, both the DMA0 and DMA1 channels have the same priority level, then DMA0 will first be granted access to the bus."]
    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma6CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Normal mode. The DMA channel stops after having completed the transfer of length determined by DMAx_LEN_REG. DMA_ON automatically deasserts when the transfer is completed.\n1: Circular mode (applicable only if DREQ_MODE = \'1\'). In this mode, DMA_ON never deasserts, as the DMA channel automatically resets DMAx_IDX_REG and starts a new transfer."]
    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of source address.\n0 = do not increment (source address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of destination address.\n0 = do not increment (destination address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel starts immediately\n1: DMA channel must be triggered by peripheral DMA request (see also the description of DMA_REQ_MUX_REG)"]
    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma6CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bus transfer width:\n00 = 1 Byte (suggested for peripherals like UART and 8-bit SPI)\n01 = 2 Bytes (suggested for peripherals like I2C and 16-bit SPI)\n10 = 4 Bytes (suggested for Memory-to-Memory transfers)\n11 = Reserved"]
    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma6CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma6CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel is off, clocks are disabled\n1: DMA channel is enabled. This bit will be automatically cleared after the completion of a transfer, if Circular mode is not enabled. In Circular mode, this bit stays set.\nNOTE: If DMA_ON is disabled by SW while the DMA channel is active, it cannot be enabled again until the channel has completed the last on-going read-write cycle and has stopped. Thus, the SW has to check that the reading of DMA6_CTRL_REG.DMA_ON returns 0, before setting again the specific bit-field."]
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
        <crate::RegValueT<Dma6CtrlReg_SPEC> as RegisterValue<_>>::new(98304)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma6IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma6IdxReg_SPEC {
    type DataType = u32;
}

#[doc = "Index pointer register of DMA channel 6"]
pub type Dma6IdxReg = crate::RegValueT<Dma6IdxReg_SPEC>;

impl Dma6IdxReg {
    #[doc = "This (read-only) register determines the data items already transferred by the DMA channel. Hence, if its value is 1, then the DMA channel has already copied one data item and it is currently performing the next copy. If its value is 2, then two items have already been copied, and so on.\nWhen the transfer is completed (so when DMA6_CTRL_REG\\[DMA_ON\\] has been cleared) and DMA6_CTRL_REG\\[CIRCULAR\\] is not set, the register keeps its (last) value (which should be equal to DMA6_LEN_REG) and it is automatically reset to 0 upon starting a new transfer. In Circular mode, the register is automatically initialized to 0 as soon as the DMA channel starts-over again."]
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

#[doc = "Interrupt length register of DMA channel 6"]
pub type Dma6IntReg = crate::RegValueT<Dma6IntReg_SPEC>;

impl Dma6IntReg {
    #[doc = "Number of transfers until an interrupt is generated. The interrupt is generated after a transfer, if and only if DMA6_INT_REG has reached DMA6_IDX_REG and before DMA6_IDX_REG is incremented. The interrupt enable bit of this this channel must be already enabled, to let the channel\'s controller generate the interrupt (see also DMA_INT_MASK_REG and the SET/RESET interrupt registers)."]
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

#[doc = "Transfer length register of DMA channel 6"]
pub type Dma6LenReg = crate::RegValueT<Dma6LenReg_SPEC>;

impl Dma6LenReg {
    #[doc = "DMA channel\'s transfer length. DMA6_LEN of value 0, 1, 2, ... results into an actual transfer length of 1, 2, 3, ..."]
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

#[doc = "Source address register of DMA channel 7"]
pub type Dma7AStartReg = crate::RegValueT<Dma7AStartReg_SPEC>;

impl Dma7AStartReg {
    #[doc = "Source start address of channel 7"]
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

#[doc = "Destination address register of DMA channel 7"]
pub type Dma7BStartReg = crate::RegValueT<Dma7BStartReg_SPEC>;

impl Dma7BStartReg {
    #[doc = "Destination start address of channel 7"]
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

#[doc = "Control register of DMA channel 7"]
pub type Dma7CtrlReg = crate::RegValueT<Dma7CtrlReg_SPEC>;

impl Dma7CtrlReg {
    #[doc = "0: DMA channel de-asserts the bus request upon completion of the write transfer (burst or single-shot)\n1: DMA channel keeps on requesting the bus upon completion of the write. This is effective only in Memory-to-Memory transfers (DREQ_MODE = \'0\') and results into requesting the bus continuously during the whole transfer, to speed-up its completion (default)."]
    #[inline(always)]
    pub fn dma_exclusive_access(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Ignores bus error response from the AHB bus, so DMA continues normally.\n1: Detects the bus response and tracks any bus error may occur during the transfer. If a bus error is detected, the channel completes the current read-write DMA cycle (either in burst or single transfers mode) and then closes the transfer, de-asserting DMA_ON bit automatically.\nIt is noted that the respective bus error detection status bit of DMA_INT_STATUS_REG is automatically cleared as soon as the channel is switched-on again, in order to perform a new transfer.\nNOTE: This bit-field is overruled to \'1\' when channel 5 is configured as trusted channel (in Secure Boot mode)."]
    #[inline(always)]
    pub fn bus_error_detect(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the DMA read/write bursts, according to the following configuration:\n00 = Bursts are disabled\n01 = Bursts of 4 are enabled\n10 = Bursts of 8 are enabled\n11 = Reserved"]
    #[inline(always)]
    pub fn burst_mode(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Dma7CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA operates with level-sensitive peripheral requests (default)\n1: DMA operates with (positive) edge-sensitive peripheral requests"]
    #[inline(always)]
    pub fn req_sense(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA performs copy A1 to B1, A2 to B2, etc ...\n1: DMA performs copy of A1 to B1, B2, etc ...\nThis feature is useful for memory initialization to any value. Thus, BINC must be set to \'1\', while AINC is don\'t care, as only one fetch from A is done. This process cannot be interrupted by other DMA channels. It is also noted that DMA_INIT should not be used when DREQ_MODE=\'1\'.\nNOTE: This bit-field is overruled to \'0\' when channel 5 is configured as trusted channel (in Secure Boot mode)."]
    #[inline(always)]
    pub fn dma_init(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Blocking mode, the DMA performs a fast back-to-back copy, disabling bus access for any bus master with lower priority.\n1: Interrupting mode, the DMA inserts a wait cycle after each store allowing the CPU to steal cycles or cache to perform a burst read. If DREQ_MODE=\'1\', DMA_IDLE is don\'t care.\nNOTE: This bit-field is overruled to \'0\' when channel 5 is configured as trusted channel (in Secure Boot mode)."]
    #[inline(always)]
    pub fn dma_idle(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The priority level determines which DMA channel will be granted access for transferring data, in case more than one channels are active and request the bus at the same time. The greater the value, the higher the priority. In specific:\n000 = lowest priority\n111 = highest priority\nIf different channels with equal priority level values request the bus at the same time, an inherent priority mechanism is applied. According to this mechanism, if, for example, both the DMA0 and DMA1 channels have the same priority level, then DMA0 will first be granted access to the bus."]
    #[inline(always)]
    pub fn dma_prio(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, Dma7CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Normal mode. The DMA channel stops after having completed the transfer of length determined by DMAx_LEN_REG. DMA_ON automatically deasserts when the transfer is completed.\n1: Circular mode (applicable only if DREQ_MODE = \'1\'). In this mode, DMA_ON never deasserts, as the DMA channel automatically resets DMAx_IDX_REG and starts a new transfer."]
    #[inline(always)]
    pub fn circular(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of source address.\n0 = do not increment (source address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn ainc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable increment of destination address.\n0 = do not increment (destination address stays the same during the transfer)\n1 = increment according to the value of BW bit-field (by 1, when BW=\"00\" ; by 2, when BW=\"01\" ; by 4, when BW=\"10\")"]
    #[inline(always)]
    pub fn binc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel starts immediately\n1: DMA channel must be triggered by peripheral DMA request (see also the description of DMA_REQ_MUX_REG)\nNOTE: This bit-field is overruled to \'0\' when channel 5 is configured as trusted channel (in Secure Boot mode)."]
    #[inline(always)]
    pub fn dreq_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dma7CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bus transfer width:\n00 = 1 Byte (suggested for peripherals like UART and 8-bit SPI)\n01 = 2 Bytes (suggested for peripherals like I2C and 16-bit SPI)\n10 = 4 Bytes (suggested for Memory-to-Memory transfers)\n11 = Reserved"]
    #[inline(always)]
    pub fn bw(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dma7CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dma7CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DMA channel is off, clocks are disabled\n1: DMA channel is enabled. This bit will be automatically cleared after the completion of a transfer, if Circular mode is not enabled. In Circular mode, this bit stays set.\nNOTE: If DMA_ON is disabled by SW while the DMA channel is active, it cannot be enabled again until the channel has completed the last on-going read-write cycle and has stopped. Thus, the SW has to check that the reading of DMA7_CTRL_REG.DMA_ON returns 0, before setting again the specific bit-field."]
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
        <crate::RegValueT<Dma7CtrlReg_SPEC> as RegisterValue<_>>::new(98304)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma7IdxReg_SPEC;
impl crate::sealed::RegSpec for Dma7IdxReg_SPEC {
    type DataType = u32;
}

#[doc = "Index pointer register of DMA channel 7"]
pub type Dma7IdxReg = crate::RegValueT<Dma7IdxReg_SPEC>;

impl Dma7IdxReg {
    #[doc = "This (read-only) register determines the data items already transferred by the DMA channel. Hence, if its value is 1, then the DMA channel has already copied one data item and it is currently performing the next copy. If its value is 2, then two items have already been copied, and so on.\nWhen the transfer is completed (so when DMA7_CTRL_REG\\[DMA_ON\\] has been cleared) and DMA7_CTRL_REG\\[CIRCULAR\\] is not set, the register keeps its (last) value (which should be equal to DMA7_LEN_REG) and it is automatically reset to 0 upon starting a new transfer. In Circular mode, the register is automatically initialized to 0 as soon as the DMA channel starts-over again."]
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

#[doc = "Interrupt length register of DMA channel 7"]
pub type Dma7IntReg = crate::RegValueT<Dma7IntReg_SPEC>;

impl Dma7IntReg {
    #[doc = "Number of transfers until an interrupt is generated. The interrupt is generated after a transfer, if and only if DMA7_INT_REG has reached DMA7_IDX_REG and before DMA7_IDX_REG is incremented. The interrupt enable bit of this this channel must be already enabled, to let the channel\'s controller generate the interrupt (see also DMA_INT_MASK_REG and the SET/RESET interrupt registers)."]
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

#[doc = "Transfer length register of DMA channel 7"]
pub type Dma7LenReg = crate::RegValueT<Dma7LenReg_SPEC>;

impl Dma7LenReg {
    #[doc = "DMA channel\'s transfer length. DMA7_LEN of value 0, 1, 2, ... results into an actual transfer length of 1, 2, 3, ..."]
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

#[doc = "DMA Interrupt clear register"]
pub type DmaClearIntReg = crate::RegValueT<DmaClearIntReg_SPEC>;

impl DmaClearIntReg {
    #[doc = "Writing a 1 will reset the status bit of DMA_INT_STATUS_REG for channel 7 ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn dma_rst_irq_ch7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Writing a 1 will reset the status bit of DMA_INT_STATUS_REG for channel 6 ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn dma_rst_irq_ch6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Writing a 1 will reset the status bit of DMA_INT_STATUS_REG for channel 5 ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn dma_rst_irq_ch5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Writing a 1 will reset the status bit of DMA_INT_STATUS_REG for channel 4 ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn dma_rst_irq_ch4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Writing a 1 will reset the status bit of DMA_INT_STATUS_REG for channel 3 ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn dma_rst_irq_ch3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Writing a 1 will reset the status bit of DMA_INT_STATUS_REG for channel 2 ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn dma_rst_irq_ch2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Writing a 1 will reset the status bit of DMA_INT_STATUS_REG for channel 1 ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn dma_rst_irq_ch1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DmaClearIntReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,DmaClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Writing a 1 will reset the status bit of DMA_INT_STATUS_REG for channel 0 ; writing a 0 will have no effect"]
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

#[doc = "DMA Interrupt mask register"]
pub type DmaIntMaskReg = crate::RegValueT<DmaIntMaskReg_SPEC>;

impl DmaIntMaskReg {
    #[doc = "0: disable interrupts on channel 7\n1: enable interrupts on channel 7"]
    #[inline(always)]
    pub fn dma_irq_enable7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: disable interrupts on channel 6\n1: enable interrupts on channel 6"]
    #[inline(always)]
    pub fn dma_irq_enable6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: disable interrupts on channel 5\n1: enable interrupts on channel 5"]
    #[inline(always)]
    pub fn dma_irq_enable5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: disable interrupts on channel 4\n1: enable interrupts on channel 4"]
    #[inline(always)]
    pub fn dma_irq_enable4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: disable interrupts on channel 3\n1: enable interrupts on channel 3"]
    #[inline(always)]
    pub fn dma_irq_enable3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: disable interrupts on channel 2\n1: enable interrupts on channel 2"]
    #[inline(always)]
    pub fn dma_irq_enable2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: disable interrupts on channel 1\n1: enable interrupts on channel 1"]
    #[inline(always)]
    pub fn dma_irq_enable1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DmaIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DmaIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: disable interrupts on channel 0\n1: enable interrupts on channel 0"]
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

#[doc = "DMA Interrupt status register"]
pub type DmaIntStatusReg = crate::RegValueT<DmaIntStatusReg_SPEC>;

impl DmaIntStatusReg {
    #[doc = "0: No bus error response is detected for channel 7\n1: Bus error response detected for channel 7\nNOTE: This bit-field is auto-clear and it is initialized to \'0\' as soon as a new transfer is started. It is also noted that when channel 7 is configured as \"trusted\" (in Secure Boot mode), this bit-field is overruled to \'0\', masking the bus error status reported to the user."]
    #[inline(always)]
    pub fn dma_bus_err7(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: No bus error response is detected for channel 6\n1: Bus error response detected for channel 6"]
    #[inline(always)]
    pub fn dma_bus_err6(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: No bus error response is detected for channel 5\n1: Bus error response detected for channel 5"]
    #[inline(always)]
    pub fn dma_bus_err5(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: No bus error response is detected for channel 4\n1: Bus error response detected for channel 4\nNOTE: This bit-field is auto-clear and it is initialized to \'0\' as soon as a new transfer is started."]
    #[inline(always)]
    pub fn dma_bus_err4(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: No bus error response is detected for channel 3\n1: Bus error response detected for channel 3\nNOTE: This bit-field is auto-clear and it is initialized to \'0\' as soon as a new transfer is started."]
    #[inline(always)]
    pub fn dma_bus_err3(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: No bus error response is detected for channel 2\n1: Bus error response detected for channel 2\nNOTE: This bit-field is auto-clear and it is initialized to \'0\' as soon as a new transfer is started."]
    #[inline(always)]
    pub fn dma_bus_err2(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: No bus error response is detected for channel 1\n1: Bus error response detected for channel 1\nNOTE: This bit-field is auto-clear and it is initialized to \'0\' as soon as a new transfer is started."]
    #[inline(always)]
    pub fn dma_bus_err1(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: No bus error response is detected for channel 0\n1: Bus error response detected for channel 0\nNOTE: This bit-field is auto-clear and it is initialized to \'0\' as soon as a new transfer is started."]
    #[inline(always)]
    pub fn dma_bus_err0(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: IRQ on channel 7 is not set\n1: IRQ on channel 7 is set"]
    #[inline(always)]
    pub fn dma_irq_ch7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: IRQ on channel 6 is not set\n1: IRQ on channel 6 is set"]
    #[inline(always)]
    pub fn dma_irq_ch6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: IRQ on channel 5 is not set\n1: IRQ on channel 5 is set"]
    #[inline(always)]
    pub fn dma_irq_ch5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: IRQ on channel 4 is not set\n1: IRQ on channel 4 is set"]
    #[inline(always)]
    pub fn dma_irq_ch4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: IRQ on channel 3 is not set\n1: IRQ on channel 3 is set"]
    #[inline(always)]
    pub fn dma_irq_ch3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: IRQ on channel 2 is not set\n1: IRQ on channel 2 is set"]
    #[inline(always)]
    pub fn dma_irq_ch2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: IRQ on channel 1 is not set\n1: IRQ on channel 1 is set"]
    #[inline(always)]
    pub fn dma_irq_ch1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DmaIntStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,DmaIntStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: IRQ on channel 0 is not set\n1: IRQ on channel 0 is set"]
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

#[doc = "DMA channels peripherals mapping register"]
pub type DmaReqMuxReg = crate::RegValueT<DmaReqMuxReg_SPEC>;

impl DmaReqMuxReg {
    #[doc = "Select which combination of peripherals are mapped on the DMA channels. The peripherals are mapped as pairs on two channels.\nThe first DMA request is mapped on channel 6 and the second on channel 7. Please refer to the description of DMA01_SEL bit-field for the exact peripherals\' mapping.\nNOTE: When channel DMA7 is configured as secure channel, it cannot support any peripheral requests, since DREQ_MODE is disabled automatically, overruling the corresponding bit-field of DMA7_CTRL_REG."]
    #[inline(always)]
    pub fn dma67_sel(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, DmaReqMuxReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,DmaReqMuxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select which combination of peripherals are mapped on the DMA channels. The peripherals are mapped as pairs on two channels.\nThe first DMA request is mapped on channel 4 and the second on channel 5. Please refer to the description of DMA01_SEL bit-field for the exact peripherals\' mapping."]
    #[inline(always)]
    pub fn dma45_sel(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, DmaReqMuxReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,DmaReqMuxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select which combination of peripherals are mapped on the DMA channels. The peripherals are mapped as pairs on two channels.\nThe first DMA request is mapped on channel 2 and the second on channel 3. Please refer to the description of DMA01_SEL bit-field for the exact peripherals\' mapping."]
    #[inline(always)]
    pub fn dma23_sel(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, DmaReqMuxReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,DmaReqMuxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select which combination of peripherals are mapped on the DMA channels. The peripherals are mapped as pairs on two channels.\nThe first DMA request is mapped on channel 0 and the second on channel 1 and the peripherals supported are listed below. Note that in the following list, the \"rx\" implies a Peripheral-to-Memory transfer and the \"tx\" a Memory-to-Peripheral transfer.\n0x0: SPI_rx / SPI_tx\n0x1: SP2_rx / SPI2_tx\n0x2: UART_rx / UART_tx\n0x3: UART2_rx / UART2_tx\n0x4: I2C_rx / I2C_tx\n0x5: I2C2_rx / I2C2_tx\n0x6: USB_rx / USB_tx\n0x7: UART3_rx / UART3_tx\n0x8: PCM / PCM\n0x9: SRC_rx / SRC_tx\n0xA: SPI3_rx / SPI3_tx\n0xB: I2C3_rx / I2C3_tx\n0xC: GP_ADC / APP_ADC\n0xD: SRC2_rx / SRC2_tx\n0xE: I3C_rx / I3C_tx\n0xF: None\n\nNOTE: If any of the two available peripheral selector fields (DMA01_SEL, DMA23_SEL) have the same value, the lesser significant selector has higher priority and will control the DMA acknowledge signal driven to the selected peripheral. Hence, if DMA01_SEL = DMA23_SEL, the channels 0 and 1 will provide the Rx and Tx DMA acknowledge signals for the selected peripheral.\nConsequently, it is suggested to assign the intended peripheral value to a unique selector field. Exception is the SRC, for which the mapping of the same peripheral option to more than one channel pairs may be intended (e.g. for stereo mode)."]
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

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaResetIntMaskReg_SPEC;
impl crate::sealed::RegSpec for DmaResetIntMaskReg_SPEC {
    type DataType = u32;
}

#[doc = "DMA Reset Interrupt mask register"]
pub type DmaResetIntMaskReg = crate::RegValueT<DmaResetIntMaskReg_SPEC>;

impl DmaResetIntMaskReg {
    #[doc = "Writing a \'1\' will disable the IRQs in the DMA channel 7, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_reset_irq_enable7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DmaResetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,DmaResetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' will disable the IRQs in the DMA channel 6, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_reset_irq_enable6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DmaResetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,DmaResetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' will disable the IRQs in the DMA channel 5, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_reset_irq_enable5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DmaResetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,DmaResetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' will disable the IRQs in the DMA channel 4, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_reset_irq_enable4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DmaResetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,DmaResetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' will disable the IRQs in the DMA channel 3, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_reset_irq_enable3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DmaResetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,DmaResetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' will disable the IRQs in the DMA channel 2, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_reset_irq_enable2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DmaResetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DmaResetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' will disable the IRQs in the DMA channel 1, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_reset_irq_enable1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DmaResetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DmaResetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' will disable the IRQs in the DMA channel 0, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_reset_irq_enable0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DmaResetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DmaResetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DmaResetIntMaskReg {
    #[inline(always)]
    fn default() -> DmaResetIntMaskReg {
        <crate::RegValueT<DmaResetIntMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaSetIntMaskReg_SPEC;
impl crate::sealed::RegSpec for DmaSetIntMaskReg_SPEC {
    type DataType = u32;
}

#[doc = "DMA Set Interrupt mask register"]
pub type DmaSetIntMaskReg = crate::RegValueT<DmaSetIntMaskReg_SPEC>;

impl DmaSetIntMaskReg {
    #[doc = "Writing a \'1\' will enable the IRQs in the DMA channel 7, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_set_irq_enable7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DmaSetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,DmaSetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' will enable the IRQs in the DMA channel 6, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_set_irq_enable6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DmaSetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,DmaSetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' will enable the IRQs in the DMA channel 5, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_set_irq_enable5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DmaSetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,DmaSetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' will enable the IRQs in the DMA channel 4, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_set_irq_enable4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DmaSetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,DmaSetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' will enable the IRQs in the DMA channel 3, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_set_irq_enable3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DmaSetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,DmaSetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' will enable the IRQs in the DMA channel 2, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_set_irq_enable2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DmaSetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DmaSetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' will enable the IRQs in the DMA channel 1, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_set_irq_enable1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DmaSetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DmaSetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' will enable the IRQs in the DMA channel 0, writing a \'0\' has no effect. Reading returns always \'0\'."]
    #[inline(always)]
    pub fn dma_set_irq_enable0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DmaSetIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DmaSetIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DmaSetIntMaskReg {
    #[inline(always)]
    fn default() -> DmaSetIntMaskReg {
        <crate::RegValueT<DmaSetIntMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
