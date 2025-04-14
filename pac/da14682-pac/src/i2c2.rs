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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:02 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"I2C2 registers"]
unsafe impl ::core::marker::Send for super::I2C2 {}
unsafe impl ::core::marker::Sync for super::I2C2 {}
impl super::I2C2 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn i2c2_ack_general_call_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2AckGeneralCallReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2AckGeneralCallReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_clr_activity_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2ClrActivityReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2ClrActivityReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_clr_gen_call_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2ClrGenCallReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2ClrGenCallReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_clr_intr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2ClrIntrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2ClrIntrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_clr_rd_req_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2ClrRdReqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2ClrRdReqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_clr_rx_done_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2ClrRxDoneReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2ClrRxDoneReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_clr_rx_over_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2ClrRxOverReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2ClrRxOverReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_clr_rx_under_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2ClrRxUnderReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2ClrRxUnderReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_clr_start_det_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2ClrStartDetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2ClrStartDetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_clr_stop_det_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2ClrStopDetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2ClrStopDetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_clr_tx_abrt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2ClrTxAbrtReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2ClrTxAbrtReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_clr_tx_over_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2ClrTxOverReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2ClrTxOverReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_comp2_version(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2Comp2Version_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2Comp2Version_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(250usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_comp_param1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2CompParam1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2CompParam1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_comp_param2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2CompParam2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2CompParam2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(246usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_comp_type2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2CompType2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2CompType2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(254usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_comp_type_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2CompTypeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2CompTypeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(252usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_comp_version_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2CompVersionReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2CompVersionReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_con_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2ConReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2ConReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_data_cmd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2DataCmdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2DataCmdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_dma_cr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2DmaCrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2DmaCrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_dma_rdlr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2DmaRdlrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2DmaRdlrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_dma_tdlr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2DmaTdlrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2DmaTdlrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_enable_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2EnableReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2EnableReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_enable_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2EnableStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2EnableStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_fs_scl_hcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2FsSclHcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2FsSclHcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_fs_scl_lcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2FsSclLcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2FsSclLcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_ic_fs_spklen_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2IcFsSpklenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2IcFsSpklenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_intr_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2IntrMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2IntrMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_intr_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2IntrStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2IntrStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_raw_intr_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2RawIntrStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2RawIntrStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_rxflr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2RxflrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2RxflrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_rx_tl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2RxTlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2RxTlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_sar_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2SarReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2SarReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_sda_hold_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2SdaHoldReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2SdaHoldReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_sda_setup_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2SdaSetupReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2SdaSetupReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_ss_scl_hcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2SsSclHcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2SsSclHcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_ss_scl_lcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2SsSclLcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2SsSclLcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2StatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2StatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_tar_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2TarReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2TarReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_txflr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2TxflrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2TxflrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_tx_abrt_source_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2TxAbrtSourceReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2TxAbrtSourceReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c2_tx_tl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2TxTlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2TxTlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2AckGeneralCallReg_SPEC;
impl crate::sealed::RegSpec for I2C2AckGeneralCallReg_SPEC {
    type DataType = u16;
}

pub type I2C2AckGeneralCallReg = crate::RegValueT<I2C2AckGeneralCallReg_SPEC>;

impl I2C2AckGeneralCallReg {
    #[inline(always)]
    pub fn ack_gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2AckGeneralCallReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,I2C2AckGeneralCallReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2AckGeneralCallReg {
    #[inline(always)]
    fn default() -> I2C2AckGeneralCallReg {
        <crate::RegValueT<I2C2AckGeneralCallReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2ClrActivityReg_SPEC;
impl crate::sealed::RegSpec for I2C2ClrActivityReg_SPEC {
    type DataType = u16;
}

pub type I2C2ClrActivityReg = crate::RegValueT<I2C2ClrActivityReg_SPEC>;

impl I2C2ClrActivityReg {
    #[inline(always)]
    pub fn clr_activity(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2ClrActivityReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2ClrActivityReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2ClrActivityReg {
    #[inline(always)]
    fn default() -> I2C2ClrActivityReg {
        <crate::RegValueT<I2C2ClrActivityReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2ClrGenCallReg_SPEC;
impl crate::sealed::RegSpec for I2C2ClrGenCallReg_SPEC {
    type DataType = u16;
}

pub type I2C2ClrGenCallReg = crate::RegValueT<I2C2ClrGenCallReg_SPEC>;

impl I2C2ClrGenCallReg {
    #[inline(always)]
    pub fn clr_gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2ClrGenCallReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2ClrGenCallReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2ClrGenCallReg {
    #[inline(always)]
    fn default() -> I2C2ClrGenCallReg {
        <crate::RegValueT<I2C2ClrGenCallReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2ClrIntrReg_SPEC;
impl crate::sealed::RegSpec for I2C2ClrIntrReg_SPEC {
    type DataType = u16;
}

pub type I2C2ClrIntrReg = crate::RegValueT<I2C2ClrIntrReg_SPEC>;

impl I2C2ClrIntrReg {
    #[inline(always)]
    pub fn clr_intr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2ClrIntrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2ClrIntrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2ClrIntrReg {
    #[inline(always)]
    fn default() -> I2C2ClrIntrReg {
        <crate::RegValueT<I2C2ClrIntrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2ClrRdReqReg_SPEC;
impl crate::sealed::RegSpec for I2C2ClrRdReqReg_SPEC {
    type DataType = u16;
}

pub type I2C2ClrRdReqReg = crate::RegValueT<I2C2ClrRdReqReg_SPEC>;

impl I2C2ClrRdReqReg {
    #[inline(always)]
    pub fn clr_rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2ClrRdReqReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2ClrRdReqReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2ClrRdReqReg {
    #[inline(always)]
    fn default() -> I2C2ClrRdReqReg {
        <crate::RegValueT<I2C2ClrRdReqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2ClrRxDoneReg_SPEC;
impl crate::sealed::RegSpec for I2C2ClrRxDoneReg_SPEC {
    type DataType = u16;
}

pub type I2C2ClrRxDoneReg = crate::RegValueT<I2C2ClrRxDoneReg_SPEC>;

impl I2C2ClrRxDoneReg {
    #[inline(always)]
    pub fn clr_rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2ClrRxDoneReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2ClrRxDoneReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2ClrRxDoneReg {
    #[inline(always)]
    fn default() -> I2C2ClrRxDoneReg {
        <crate::RegValueT<I2C2ClrRxDoneReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2ClrRxOverReg_SPEC;
impl crate::sealed::RegSpec for I2C2ClrRxOverReg_SPEC {
    type DataType = u16;
}

pub type I2C2ClrRxOverReg = crate::RegValueT<I2C2ClrRxOverReg_SPEC>;

impl I2C2ClrRxOverReg {
    #[inline(always)]
    pub fn clr_rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2ClrRxOverReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2ClrRxOverReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2ClrRxOverReg {
    #[inline(always)]
    fn default() -> I2C2ClrRxOverReg {
        <crate::RegValueT<I2C2ClrRxOverReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2ClrRxUnderReg_SPEC;
impl crate::sealed::RegSpec for I2C2ClrRxUnderReg_SPEC {
    type DataType = u16;
}

pub type I2C2ClrRxUnderReg = crate::RegValueT<I2C2ClrRxUnderReg_SPEC>;

impl I2C2ClrRxUnderReg {
    #[inline(always)]
    pub fn clr_rx_under(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2ClrRxUnderReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2ClrRxUnderReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2ClrRxUnderReg {
    #[inline(always)]
    fn default() -> I2C2ClrRxUnderReg {
        <crate::RegValueT<I2C2ClrRxUnderReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2ClrStartDetReg_SPEC;
impl crate::sealed::RegSpec for I2C2ClrStartDetReg_SPEC {
    type DataType = u16;
}

pub type I2C2ClrStartDetReg = crate::RegValueT<I2C2ClrStartDetReg_SPEC>;

impl I2C2ClrStartDetReg {
    #[inline(always)]
    pub fn clr_start_det(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2ClrStartDetReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2ClrStartDetReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2ClrStartDetReg {
    #[inline(always)]
    fn default() -> I2C2ClrStartDetReg {
        <crate::RegValueT<I2C2ClrStartDetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2ClrStopDetReg_SPEC;
impl crate::sealed::RegSpec for I2C2ClrStopDetReg_SPEC {
    type DataType = u16;
}

pub type I2C2ClrStopDetReg = crate::RegValueT<I2C2ClrStopDetReg_SPEC>;

impl I2C2ClrStopDetReg {
    #[inline(always)]
    pub fn clr_activity(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2ClrStopDetReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2ClrStopDetReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2ClrStopDetReg {
    #[inline(always)]
    fn default() -> I2C2ClrStopDetReg {
        <crate::RegValueT<I2C2ClrStopDetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2ClrTxAbrtReg_SPEC;
impl crate::sealed::RegSpec for I2C2ClrTxAbrtReg_SPEC {
    type DataType = u16;
}

pub type I2C2ClrTxAbrtReg = crate::RegValueT<I2C2ClrTxAbrtReg_SPEC>;

impl I2C2ClrTxAbrtReg {
    #[inline(always)]
    pub fn clr_tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2ClrTxAbrtReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2ClrTxAbrtReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2ClrTxAbrtReg {
    #[inline(always)]
    fn default() -> I2C2ClrTxAbrtReg {
        <crate::RegValueT<I2C2ClrTxAbrtReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2ClrTxOverReg_SPEC;
impl crate::sealed::RegSpec for I2C2ClrTxOverReg_SPEC {
    type DataType = u16;
}

pub type I2C2ClrTxOverReg = crate::RegValueT<I2C2ClrTxOverReg_SPEC>;

impl I2C2ClrTxOverReg {
    #[inline(always)]
    pub fn clr_tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2ClrTxOverReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2ClrTxOverReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2ClrTxOverReg {
    #[inline(always)]
    fn default() -> I2C2ClrTxOverReg {
        <crate::RegValueT<I2C2ClrTxOverReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2Comp2Version_SPEC;
impl crate::sealed::RegSpec for I2C2Comp2Version_SPEC {
    type DataType = u16;
}

pub type I2C2Comp2Version = crate::RegValueT<I2C2Comp2Version_SPEC>;

impl I2C2Comp2Version {
    #[inline(always)]
    pub fn ic_comp2_version(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I2C2Comp2Version_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C2Comp2Version_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2Comp2Version {
    #[inline(always)]
    fn default() -> I2C2Comp2Version {
        <crate::RegValueT<I2C2Comp2Version_SPEC> as RegisterValue<_>>::new(12594)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2CompParam1Reg_SPEC;
impl crate::sealed::RegSpec for I2C2CompParam1Reg_SPEC {
    type DataType = u16;
}

pub type I2C2CompParam1Reg = crate::RegValueT<I2C2CompParam1Reg_SPEC>;

impl I2C2CompParam1Reg {
    #[inline(always)]
    pub fn ic_comp_param1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I2C2CompParam1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C2CompParam1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2CompParam1Reg {
    #[inline(always)]
    fn default() -> I2C2CompParam1Reg {
        <crate::RegValueT<I2C2CompParam1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2CompParam2Reg_SPEC;
impl crate::sealed::RegSpec for I2C2CompParam2Reg_SPEC {
    type DataType = u16;
}

pub type I2C2CompParam2Reg = crate::RegValueT<I2C2CompParam2Reg_SPEC>;

impl I2C2CompParam2Reg {
    #[inline(always)]
    pub fn ic_comp_param2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I2C2CompParam2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C2CompParam2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2CompParam2Reg {
    #[inline(always)]
    fn default() -> I2C2CompParam2Reg {
        <crate::RegValueT<I2C2CompParam2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2CompType2Reg_SPEC;
impl crate::sealed::RegSpec for I2C2CompType2Reg_SPEC {
    type DataType = u16;
}

pub type I2C2CompType2Reg = crate::RegValueT<I2C2CompType2Reg_SPEC>;

impl I2C2CompType2Reg {
    #[inline(always)]
    pub fn ic_comp2_type(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I2C2CompType2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C2CompType2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2CompType2Reg {
    #[inline(always)]
    fn default() -> I2C2CompType2Reg {
        <crate::RegValueT<I2C2CompType2Reg_SPEC> as RegisterValue<_>>::new(17495)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2CompTypeReg_SPEC;
impl crate::sealed::RegSpec for I2C2CompTypeReg_SPEC {
    type DataType = u16;
}

pub type I2C2CompTypeReg = crate::RegValueT<I2C2CompTypeReg_SPEC>;

impl I2C2CompTypeReg {
    #[inline(always)]
    pub fn ic_comp_type(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I2C2CompTypeReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C2CompTypeReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2CompTypeReg {
    #[inline(always)]
    fn default() -> I2C2CompTypeReg {
        <crate::RegValueT<I2C2CompTypeReg_SPEC> as RegisterValue<_>>::new(320)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2CompVersionReg_SPEC;
impl crate::sealed::RegSpec for I2C2CompVersionReg_SPEC {
    type DataType = u16;
}

pub type I2C2CompVersionReg = crate::RegValueT<I2C2CompVersionReg_SPEC>;

impl I2C2CompVersionReg {
    #[inline(always)]
    pub fn ic_comp_version(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I2C2CompVersionReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C2CompVersionReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2CompVersionReg {
    #[inline(always)]
    fn default() -> I2C2CompVersionReg {
        <crate::RegValueT<I2C2CompVersionReg_SPEC> as RegisterValue<_>>::new(12330)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2ConReg_SPEC;
impl crate::sealed::RegSpec for I2C2ConReg_SPEC {
    type DataType = u16;
}

pub type I2C2ConReg = crate::RegValueT<I2C2ConReg_SPEC>;

impl I2C2ConReg {
    #[inline(always)]
    pub fn i2c_slave_disable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C2ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,I2C2ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_restart_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C2ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I2C2ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_10bitaddr_master(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C2ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,I2C2ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_10bitaddr_slave(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C2ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I2C2ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_speed(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, I2C2ConReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,I2C2ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_master_mode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2ConReg {
    #[inline(always)]
    fn default() -> I2C2ConReg {
        <crate::RegValueT<I2C2ConReg_SPEC> as RegisterValue<_>>::new(125)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2DataCmdReg_SPEC;
impl crate::sealed::RegSpec for I2C2DataCmdReg_SPEC {
    type DataType = u16;
}

pub type I2C2DataCmdReg = crate::RegValueT<I2C2DataCmdReg_SPEC>;

impl I2C2DataCmdReg {
    #[inline(always)]
    pub fn restart(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C2DataCmdReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10,1,0,I2C2DataCmdReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn stop(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C2DataCmdReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9,1,0,I2C2DataCmdReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C2DataCmdReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8,1,0,I2C2DataCmdReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dat(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, I2C2DataCmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,I2C2DataCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2DataCmdReg {
    #[inline(always)]
    fn default() -> I2C2DataCmdReg {
        <crate::RegValueT<I2C2DataCmdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2DmaCrReg_SPEC;
impl crate::sealed::RegSpec for I2C2DmaCrReg_SPEC {
    type DataType = u16;
}

pub type I2C2DmaCrReg = crate::RegValueT<I2C2DmaCrReg_SPEC>;

impl I2C2DmaCrReg {
    #[inline(always)]
    pub fn tdmae(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2DmaCrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2DmaCrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rdmae(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2DmaCrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2DmaCrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2DmaCrReg {
    #[inline(always)]
    fn default() -> I2C2DmaCrReg {
        <crate::RegValueT<I2C2DmaCrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2DmaRdlrReg_SPEC;
impl crate::sealed::RegSpec for I2C2DmaRdlrReg_SPEC {
    type DataType = u16;
}

pub type I2C2DmaRdlrReg = crate::RegValueT<I2C2DmaRdlrReg_SPEC>;

impl I2C2DmaRdlrReg {
    #[inline(always)]
    pub fn dmardl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, I2C2DmaRdlrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,I2C2DmaRdlrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2DmaRdlrReg {
    #[inline(always)]
    fn default() -> I2C2DmaRdlrReg {
        <crate::RegValueT<I2C2DmaRdlrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2DmaTdlrReg_SPEC;
impl crate::sealed::RegSpec for I2C2DmaTdlrReg_SPEC {
    type DataType = u16;
}

pub type I2C2DmaTdlrReg = crate::RegValueT<I2C2DmaTdlrReg_SPEC>;

impl I2C2DmaTdlrReg {
    #[inline(always)]
    pub fn dmatdl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, I2C2DmaTdlrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,I2C2DmaTdlrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2DmaTdlrReg {
    #[inline(always)]
    fn default() -> I2C2DmaTdlrReg {
        <crate::RegValueT<I2C2DmaTdlrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2EnableReg_SPEC;
impl crate::sealed::RegSpec for I2C2EnableReg_SPEC {
    type DataType = u16;
}

pub type I2C2EnableReg = crate::RegValueT<I2C2EnableReg_SPEC>;

impl I2C2EnableReg {
    #[inline(always)]
    pub fn i2c_abort(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2EnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2EnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ctrl_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2EnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2EnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2EnableReg {
    #[inline(always)]
    fn default() -> I2C2EnableReg {
        <crate::RegValueT<I2C2EnableReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2EnableStatusReg_SPEC;
impl crate::sealed::RegSpec for I2C2EnableStatusReg_SPEC {
    type DataType = u16;
}

pub type I2C2EnableStatusReg = crate::RegValueT<I2C2EnableStatusReg_SPEC>;

impl I2C2EnableStatusReg {
    #[inline(always)]
    pub fn slv_rx_data_lost(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C2EnableStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C2EnableStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slv_disabled_while_busy(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2EnableStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2EnableStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ic_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2EnableStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2EnableStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2EnableStatusReg {
    #[inline(always)]
    fn default() -> I2C2EnableStatusReg {
        <crate::RegValueT<I2C2EnableStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2FsSclHcntReg_SPEC;
impl crate::sealed::RegSpec for I2C2FsSclHcntReg_SPEC {
    type DataType = u16;
}

pub type I2C2FsSclHcntReg = crate::RegValueT<I2C2FsSclHcntReg_SPEC>;

impl I2C2FsSclHcntReg {
    #[inline(always)]
    pub fn ic_fs_scl_hcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I2C2FsSclHcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C2FsSclHcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2FsSclHcntReg {
    #[inline(always)]
    fn default() -> I2C2FsSclHcntReg {
        <crate::RegValueT<I2C2FsSclHcntReg_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2FsSclLcntReg_SPEC;
impl crate::sealed::RegSpec for I2C2FsSclLcntReg_SPEC {
    type DataType = u16;
}

pub type I2C2FsSclLcntReg = crate::RegValueT<I2C2FsSclLcntReg_SPEC>;

impl I2C2FsSclLcntReg {
    #[inline(always)]
    pub fn ic_fs_scl_lcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I2C2FsSclLcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C2FsSclLcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2FsSclLcntReg {
    #[inline(always)]
    fn default() -> I2C2FsSclLcntReg {
        <crate::RegValueT<I2C2FsSclLcntReg_SPEC> as RegisterValue<_>>::new(23)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2IcFsSpklenReg_SPEC;
impl crate::sealed::RegSpec for I2C2IcFsSpklenReg_SPEC {
    type DataType = u16;
}

pub type I2C2IcFsSpklenReg = crate::RegValueT<I2C2IcFsSpklenReg_SPEC>;

impl I2C2IcFsSpklenReg {
    #[inline(always)]
    pub fn ic_fs_spklen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I2C2IcFsSpklenReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I2C2IcFsSpklenReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2IcFsSpklenReg {
    #[inline(always)]
    fn default() -> I2C2IcFsSpklenReg {
        <crate::RegValueT<I2C2IcFsSpklenReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2IntrMaskReg_SPEC;
impl crate::sealed::RegSpec for I2C2IntrMaskReg_SPEC {
    type DataType = u16;
}

pub type I2C2IntrMaskReg = crate::RegValueT<I2C2IntrMaskReg_SPEC>;

impl I2C2IntrMaskReg {
    #[inline(always)]
    pub fn m_gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_start_det(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_stop_det(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_activity(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_tx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rx_full(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rx_under(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2IntrMaskReg {
    #[inline(always)]
    fn default() -> I2C2IntrMaskReg {
        <crate::RegValueT<I2C2IntrMaskReg_SPEC> as RegisterValue<_>>::new(2303)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2IntrStatReg_SPEC;
impl crate::sealed::RegSpec for I2C2IntrStatReg_SPEC {
    type DataType = u16;
}

pub type I2C2IntrStatReg = crate::RegValueT<I2C2IntrStatReg_SPEC>;

impl I2C2IntrStatReg {
    #[inline(always)]
    pub fn r_gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_start_det(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_stop_det(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_activity(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_tx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rx_full(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rx_under(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2IntrStatReg {
    #[inline(always)]
    fn default() -> I2C2IntrStatReg {
        <crate::RegValueT<I2C2IntrStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2RawIntrStatReg_SPEC;
impl crate::sealed::RegSpec for I2C2RawIntrStatReg_SPEC {
    type DataType = u16;
}

pub type I2C2RawIntrStatReg = crate::RegValueT<I2C2RawIntrStatReg_SPEC>;

impl I2C2RawIntrStatReg {
    #[inline(always)]
    pub fn gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn start_det(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn stop_det(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn activity(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_full(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_under(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2RawIntrStatReg {
    #[inline(always)]
    fn default() -> I2C2RawIntrStatReg {
        <crate::RegValueT<I2C2RawIntrStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2RxflrReg_SPEC;
impl crate::sealed::RegSpec for I2C2RxflrReg_SPEC {
    type DataType = u16;
}

pub type I2C2RxflrReg = crate::RegValueT<I2C2RxflrReg_SPEC>;

impl I2C2RxflrReg {
    #[inline(always)]
    pub fn rxflr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, I2C2RxflrReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,I2C2RxflrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2RxflrReg {
    #[inline(always)]
    fn default() -> I2C2RxflrReg {
        <crate::RegValueT<I2C2RxflrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2RxTlReg_SPEC;
impl crate::sealed::RegSpec for I2C2RxTlReg_SPEC {
    type DataType = u16;
}

pub type I2C2RxTlReg = crate::RegValueT<I2C2RxTlReg_SPEC>;

impl I2C2RxTlReg {
    #[inline(always)]
    pub fn rx_tl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, I2C2RxTlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,I2C2RxTlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2RxTlReg {
    #[inline(always)]
    fn default() -> I2C2RxTlReg {
        <crate::RegValueT<I2C2RxTlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2SarReg_SPEC;
impl crate::sealed::RegSpec for I2C2SarReg_SPEC {
    type DataType = u16;
}

pub type I2C2SarReg = crate::RegValueT<I2C2SarReg_SPEC>;

impl I2C2SarReg {
    #[inline(always)]
    pub fn ic_sar(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, I2C2SarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,I2C2SarReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2SarReg {
    #[inline(always)]
    fn default() -> I2C2SarReg {
        <crate::RegValueT<I2C2SarReg_SPEC> as RegisterValue<_>>::new(85)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2SdaHoldReg_SPEC;
impl crate::sealed::RegSpec for I2C2SdaHoldReg_SPEC {
    type DataType = u16;
}

pub type I2C2SdaHoldReg = crate::RegValueT<I2C2SdaHoldReg_SPEC>;

impl I2C2SdaHoldReg {
    #[inline(always)]
    pub fn ic_sda_hold(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I2C2SdaHoldReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C2SdaHoldReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2SdaHoldReg {
    #[inline(always)]
    fn default() -> I2C2SdaHoldReg {
        <crate::RegValueT<I2C2SdaHoldReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2SdaSetupReg_SPEC;
impl crate::sealed::RegSpec for I2C2SdaSetupReg_SPEC {
    type DataType = u16;
}

pub type I2C2SdaSetupReg = crate::RegValueT<I2C2SdaSetupReg_SPEC>;

impl I2C2SdaSetupReg {
    #[inline(always)]
    pub fn sda_setup(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, I2C2SdaSetupReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,I2C2SdaSetupReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2SdaSetupReg {
    #[inline(always)]
    fn default() -> I2C2SdaSetupReg {
        <crate::RegValueT<I2C2SdaSetupReg_SPEC> as RegisterValue<_>>::new(100)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2SsSclHcntReg_SPEC;
impl crate::sealed::RegSpec for I2C2SsSclHcntReg_SPEC {
    type DataType = u16;
}

pub type I2C2SsSclHcntReg = crate::RegValueT<I2C2SsSclHcntReg_SPEC>;

impl I2C2SsSclHcntReg {
    #[inline(always)]
    pub fn ic_ss_scl_hcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I2C2SsSclHcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C2SsSclHcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2SsSclHcntReg {
    #[inline(always)]
    fn default() -> I2C2SsSclHcntReg {
        <crate::RegValueT<I2C2SsSclHcntReg_SPEC> as RegisterValue<_>>::new(72)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2SsSclLcntReg_SPEC;
impl crate::sealed::RegSpec for I2C2SsSclLcntReg_SPEC {
    type DataType = u16;
}

pub type I2C2SsSclLcntReg = crate::RegValueT<I2C2SsSclLcntReg_SPEC>;

impl I2C2SsSclLcntReg {
    #[inline(always)]
    pub fn ic_ss_scl_lcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I2C2SsSclLcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C2SsSclLcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2SsSclLcntReg {
    #[inline(always)]
    fn default() -> I2C2SsSclLcntReg {
        <crate::RegValueT<I2C2SsSclLcntReg_SPEC> as RegisterValue<_>>::new(79)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2StatusReg_SPEC;
impl crate::sealed::RegSpec for I2C2StatusReg_SPEC {
    type DataType = u16;
}

pub type I2C2StatusReg = crate::RegValueT<I2C2StatusReg_SPEC>;

impl I2C2StatusReg {
    #[inline(always)]
    pub fn slv_activity(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mst_activity(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfne(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tfe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tfnf(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_activity(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2StatusReg {
    #[inline(always)]
    fn default() -> I2C2StatusReg {
        <crate::RegValueT<I2C2StatusReg_SPEC> as RegisterValue<_>>::new(6)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2TarReg_SPEC;
impl crate::sealed::RegSpec for I2C2TarReg_SPEC {
    type DataType = u16;
}

pub type I2C2TarReg = crate::RegValueT<I2C2TarReg_SPEC>;

impl I2C2TarReg {
    #[inline(always)]
    pub fn special(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C2TarReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,I2C2TarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gc_or_start(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C2TarReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,I2C2TarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ic_tar(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, I2C2TarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,I2C2TarReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2TarReg {
    #[inline(always)]
    fn default() -> I2C2TarReg {
        <crate::RegValueT<I2C2TarReg_SPEC> as RegisterValue<_>>::new(85)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2TxflrReg_SPEC;
impl crate::sealed::RegSpec for I2C2TxflrReg_SPEC {
    type DataType = u16;
}

pub type I2C2TxflrReg = crate::RegValueT<I2C2TxflrReg_SPEC>;

impl I2C2TxflrReg {
    #[inline(always)]
    pub fn txflr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, I2C2TxflrReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,I2C2TxflrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2TxflrReg {
    #[inline(always)]
    fn default() -> I2C2TxflrReg {
        <crate::RegValueT<I2C2TxflrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2TxAbrtSourceReg_SPEC;
impl crate::sealed::RegSpec for I2C2TxAbrtSourceReg_SPEC {
    type DataType = u16;
}

pub type I2C2TxAbrtSourceReg = crate::RegValueT<I2C2TxAbrtSourceReg_SPEC>;

impl I2C2TxAbrtSourceReg {
    #[inline(always)]
    pub fn abrt_slvrd_intx(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<15,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_slv_arblost(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<14,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_slvflush_txfifo(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<13,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn arb_lost(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<12,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_master_dis(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<11,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_10b_rd_norstrt(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_sbyte_norstrt(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_hs_norstrt(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_sbyte_ackdet(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_hs_ackdet(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_gcall_read(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_gcall_noack(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_txdata_noack(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_10addr2_noack(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_10addr1_noack(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_7b_addr_noack(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2TxAbrtSourceReg {
    #[inline(always)]
    fn default() -> I2C2TxAbrtSourceReg {
        <crate::RegValueT<I2C2TxAbrtSourceReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2TxTlReg_SPEC;
impl crate::sealed::RegSpec for I2C2TxTlReg_SPEC {
    type DataType = u16;
}

pub type I2C2TxTlReg = crate::RegValueT<I2C2TxTlReg_SPEC>;

impl I2C2TxTlReg {
    #[inline(always)]
    pub fn tx_tl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, I2C2TxTlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,I2C2TxTlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2TxTlReg {
    #[inline(always)]
    fn default() -> I2C2TxTlReg {
        <crate::RegValueT<I2C2TxTlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
