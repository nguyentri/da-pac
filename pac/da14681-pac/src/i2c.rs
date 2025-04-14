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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:56 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"I2C registers"]
unsafe impl ::core::marker::Send for super::I2C {}
unsafe impl ::core::marker::Sync for super::I2C {}
impl super::I2C {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn i2c_ack_general_call_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CAckGeneralCallReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CAckGeneralCallReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_clr_activity_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CClrActivityReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CClrActivityReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_clr_gen_call_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CClrGenCallReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CClrGenCallReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_clr_intr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CClrIntrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CClrIntrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_clr_rd_req_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CClrRdReqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CClrRdReqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_clr_rx_done_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CClrRxDoneReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CClrRxDoneReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_clr_rx_over_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CClrRxOverReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CClrRxOverReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_clr_rx_under_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CClrRxUnderReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CClrRxUnderReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_clr_start_det_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CClrStartDetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CClrStartDetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_clr_stop_det_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CClrStopDetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CClrStopDetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_clr_tx_abrt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CClrTxAbrtReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CClrTxAbrtReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_clr_tx_over_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CClrTxOverReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CClrTxOverReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_comp2_version(
        &self,
    ) -> &'static crate::common::Reg<self::I2CComp2Version_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CComp2Version_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(250usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_comp_param1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CCompParam1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CCompParam1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_comp_param2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CCompParam2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CCompParam2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(246usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_comp_type2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CCompType2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CCompType2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(254usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_comp_type_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CCompTypeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CCompTypeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(252usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_comp_version_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CCompVersionReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CCompVersionReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_con_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CConReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CConReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_data_cmd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CDataCmdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CDataCmdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_dma_cr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CDmaCrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CDmaCrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_dma_rdlr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CDmaRdlrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CDmaRdlrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_dma_tdlr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CDmaTdlrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CDmaTdlrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_enable_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CEnableReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CEnableReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_enable_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CEnableStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CEnableStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_fs_scl_hcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CFsSclHcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CFsSclHcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_fs_scl_lcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CFsSclLcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CFsSclLcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_hs_maddr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CHsMaddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CHsMaddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_ic_fs_spklen_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CIcFsSpklenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CIcFsSpklenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_intr_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CIntrMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CIntrMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_intr_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CIntrStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CIntrStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_raw_intr_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CRawIntrStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CRawIntrStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_rxflr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CRxflrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CRxflrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_rx_tl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CRxTlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CRxTlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_sar_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CSarReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CSarReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_sda_hold_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CSdaHoldReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CSdaHoldReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_sda_setup_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CSdaSetupReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CSdaSetupReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_ss_scl_hcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CSsSclHcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CSsSclHcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_ss_scl_lcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CSsSclLcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CSsSclLcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_tar_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CTarReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CTarReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_txflr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CTxflrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CTxflrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_tx_abrt_source_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CTxAbrtSourceReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CTxAbrtSourceReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c_tx_tl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2CTxTlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2CTxTlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CAckGeneralCallReg_SPEC;
impl crate::sealed::RegSpec for I2CAckGeneralCallReg_SPEC {
    type DataType = u16;
}

pub type I2CAckGeneralCallReg = crate::RegValueT<I2CAckGeneralCallReg_SPEC>;

impl I2CAckGeneralCallReg {
    #[inline(always)]
    pub fn ack_gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CAckGeneralCallReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,I2CAckGeneralCallReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CAckGeneralCallReg {
    #[inline(always)]
    fn default() -> I2CAckGeneralCallReg {
        <crate::RegValueT<I2CAckGeneralCallReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CClrActivityReg_SPEC;
impl crate::sealed::RegSpec for I2CClrActivityReg_SPEC {
    type DataType = u16;
}

pub type I2CClrActivityReg = crate::RegValueT<I2CClrActivityReg_SPEC>;

impl I2CClrActivityReg {
    #[inline(always)]
    pub fn clr_activity(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CClrActivityReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CClrActivityReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CClrActivityReg {
    #[inline(always)]
    fn default() -> I2CClrActivityReg {
        <crate::RegValueT<I2CClrActivityReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CClrGenCallReg_SPEC;
impl crate::sealed::RegSpec for I2CClrGenCallReg_SPEC {
    type DataType = u16;
}

pub type I2CClrGenCallReg = crate::RegValueT<I2CClrGenCallReg_SPEC>;

impl I2CClrGenCallReg {
    #[inline(always)]
    pub fn clr_gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CClrGenCallReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CClrGenCallReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CClrGenCallReg {
    #[inline(always)]
    fn default() -> I2CClrGenCallReg {
        <crate::RegValueT<I2CClrGenCallReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CClrIntrReg_SPEC;
impl crate::sealed::RegSpec for I2CClrIntrReg_SPEC {
    type DataType = u16;
}

pub type I2CClrIntrReg = crate::RegValueT<I2CClrIntrReg_SPEC>;

impl I2CClrIntrReg {
    #[inline(always)]
    pub fn clr_intr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CClrIntrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CClrIntrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CClrIntrReg {
    #[inline(always)]
    fn default() -> I2CClrIntrReg {
        <crate::RegValueT<I2CClrIntrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CClrRdReqReg_SPEC;
impl crate::sealed::RegSpec for I2CClrRdReqReg_SPEC {
    type DataType = u16;
}

pub type I2CClrRdReqReg = crate::RegValueT<I2CClrRdReqReg_SPEC>;

impl I2CClrRdReqReg {
    #[inline(always)]
    pub fn clr_rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CClrRdReqReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CClrRdReqReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CClrRdReqReg {
    #[inline(always)]
    fn default() -> I2CClrRdReqReg {
        <crate::RegValueT<I2CClrRdReqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CClrRxDoneReg_SPEC;
impl crate::sealed::RegSpec for I2CClrRxDoneReg_SPEC {
    type DataType = u16;
}

pub type I2CClrRxDoneReg = crate::RegValueT<I2CClrRxDoneReg_SPEC>;

impl I2CClrRxDoneReg {
    #[inline(always)]
    pub fn clr_rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CClrRxDoneReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CClrRxDoneReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CClrRxDoneReg {
    #[inline(always)]
    fn default() -> I2CClrRxDoneReg {
        <crate::RegValueT<I2CClrRxDoneReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CClrRxOverReg_SPEC;
impl crate::sealed::RegSpec for I2CClrRxOverReg_SPEC {
    type DataType = u16;
}

pub type I2CClrRxOverReg = crate::RegValueT<I2CClrRxOverReg_SPEC>;

impl I2CClrRxOverReg {
    #[inline(always)]
    pub fn clr_rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CClrRxOverReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CClrRxOverReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CClrRxOverReg {
    #[inline(always)]
    fn default() -> I2CClrRxOverReg {
        <crate::RegValueT<I2CClrRxOverReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CClrRxUnderReg_SPEC;
impl crate::sealed::RegSpec for I2CClrRxUnderReg_SPEC {
    type DataType = u16;
}

pub type I2CClrRxUnderReg = crate::RegValueT<I2CClrRxUnderReg_SPEC>;

impl I2CClrRxUnderReg {
    #[inline(always)]
    pub fn clr_rx_under(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CClrRxUnderReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CClrRxUnderReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CClrRxUnderReg {
    #[inline(always)]
    fn default() -> I2CClrRxUnderReg {
        <crate::RegValueT<I2CClrRxUnderReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CClrStartDetReg_SPEC;
impl crate::sealed::RegSpec for I2CClrStartDetReg_SPEC {
    type DataType = u16;
}

pub type I2CClrStartDetReg = crate::RegValueT<I2CClrStartDetReg_SPEC>;

impl I2CClrStartDetReg {
    #[inline(always)]
    pub fn clr_start_det(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CClrStartDetReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CClrStartDetReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CClrStartDetReg {
    #[inline(always)]
    fn default() -> I2CClrStartDetReg {
        <crate::RegValueT<I2CClrStartDetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CClrStopDetReg_SPEC;
impl crate::sealed::RegSpec for I2CClrStopDetReg_SPEC {
    type DataType = u16;
}

pub type I2CClrStopDetReg = crate::RegValueT<I2CClrStopDetReg_SPEC>;

impl I2CClrStopDetReg {
    #[inline(always)]
    pub fn clr_activity(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CClrStopDetReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CClrStopDetReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CClrStopDetReg {
    #[inline(always)]
    fn default() -> I2CClrStopDetReg {
        <crate::RegValueT<I2CClrStopDetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CClrTxAbrtReg_SPEC;
impl crate::sealed::RegSpec for I2CClrTxAbrtReg_SPEC {
    type DataType = u16;
}

pub type I2CClrTxAbrtReg = crate::RegValueT<I2CClrTxAbrtReg_SPEC>;

impl I2CClrTxAbrtReg {
    #[inline(always)]
    pub fn clr_tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CClrTxAbrtReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CClrTxAbrtReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CClrTxAbrtReg {
    #[inline(always)]
    fn default() -> I2CClrTxAbrtReg {
        <crate::RegValueT<I2CClrTxAbrtReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CClrTxOverReg_SPEC;
impl crate::sealed::RegSpec for I2CClrTxOverReg_SPEC {
    type DataType = u16;
}

pub type I2CClrTxOverReg = crate::RegValueT<I2CClrTxOverReg_SPEC>;

impl I2CClrTxOverReg {
    #[inline(always)]
    pub fn clr_tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CClrTxOverReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CClrTxOverReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CClrTxOverReg {
    #[inline(always)]
    fn default() -> I2CClrTxOverReg {
        <crate::RegValueT<I2CClrTxOverReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CComp2Version_SPEC;
impl crate::sealed::RegSpec for I2CComp2Version_SPEC {
    type DataType = u16;
}

pub type I2CComp2Version = crate::RegValueT<I2CComp2Version_SPEC>;

impl I2CComp2Version {
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
        I2CComp2Version_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2CComp2Version_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2CComp2Version {
    #[inline(always)]
    fn default() -> I2CComp2Version {
        <crate::RegValueT<I2CComp2Version_SPEC> as RegisterValue<_>>::new(12594)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CCompParam1Reg_SPEC;
impl crate::sealed::RegSpec for I2CCompParam1Reg_SPEC {
    type DataType = u16;
}

pub type I2CCompParam1Reg = crate::RegValueT<I2CCompParam1Reg_SPEC>;

impl I2CCompParam1Reg {
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
        I2CCompParam1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2CCompParam1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2CCompParam1Reg {
    #[inline(always)]
    fn default() -> I2CCompParam1Reg {
        <crate::RegValueT<I2CCompParam1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CCompParam2Reg_SPEC;
impl crate::sealed::RegSpec for I2CCompParam2Reg_SPEC {
    type DataType = u16;
}

pub type I2CCompParam2Reg = crate::RegValueT<I2CCompParam2Reg_SPEC>;

impl I2CCompParam2Reg {
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
        I2CCompParam2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2CCompParam2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2CCompParam2Reg {
    #[inline(always)]
    fn default() -> I2CCompParam2Reg {
        <crate::RegValueT<I2CCompParam2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CCompType2Reg_SPEC;
impl crate::sealed::RegSpec for I2CCompType2Reg_SPEC {
    type DataType = u16;
}

pub type I2CCompType2Reg = crate::RegValueT<I2CCompType2Reg_SPEC>;

impl I2CCompType2Reg {
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
        I2CCompType2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2CCompType2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2CCompType2Reg {
    #[inline(always)]
    fn default() -> I2CCompType2Reg {
        <crate::RegValueT<I2CCompType2Reg_SPEC> as RegisterValue<_>>::new(17495)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CCompTypeReg_SPEC;
impl crate::sealed::RegSpec for I2CCompTypeReg_SPEC {
    type DataType = u16;
}

pub type I2CCompTypeReg = crate::RegValueT<I2CCompTypeReg_SPEC>;

impl I2CCompTypeReg {
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
        I2CCompTypeReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2CCompTypeReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2CCompTypeReg {
    #[inline(always)]
    fn default() -> I2CCompTypeReg {
        <crate::RegValueT<I2CCompTypeReg_SPEC> as RegisterValue<_>>::new(320)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CCompVersionReg_SPEC;
impl crate::sealed::RegSpec for I2CCompVersionReg_SPEC {
    type DataType = u16;
}

pub type I2CCompVersionReg = crate::RegValueT<I2CCompVersionReg_SPEC>;

impl I2CCompVersionReg {
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
        I2CCompVersionReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2CCompVersionReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2CCompVersionReg {
    #[inline(always)]
    fn default() -> I2CCompVersionReg {
        <crate::RegValueT<I2CCompVersionReg_SPEC> as RegisterValue<_>>::new(12330)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CConReg_SPEC;
impl crate::sealed::RegSpec for I2CConReg_SPEC {
    type DataType = u16;
}

pub type I2CConReg = crate::RegValueT<I2CConReg_SPEC>;

impl I2CConReg {
    #[inline(always)]
    pub fn i2c_slave_disable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2CConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,I2CConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_restart_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2CConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I2CConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_10bitaddr_master(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2CConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,I2CConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_10bitaddr_slave(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2CConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I2CConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_speed(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, I2CConReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,I2CConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_master_mode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I2CConReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CConReg {
    #[inline(always)]
    fn default() -> I2CConReg {
        <crate::RegValueT<I2CConReg_SPEC> as RegisterValue<_>>::new(125)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CDataCmdReg_SPEC;
impl crate::sealed::RegSpec for I2CDataCmdReg_SPEC {
    type DataType = u16;
}

pub type I2CDataCmdReg = crate::RegValueT<I2CDataCmdReg_SPEC>;

impl I2CDataCmdReg {
    #[inline(always)]
    pub fn cmd(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2CDataCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,I2CDataCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dat(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, I2CDataCmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,I2CDataCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CDataCmdReg {
    #[inline(always)]
    fn default() -> I2CDataCmdReg {
        <crate::RegValueT<I2CDataCmdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CDmaCrReg_SPEC;
impl crate::sealed::RegSpec for I2CDmaCrReg_SPEC {
    type DataType = u16;
}

pub type I2CDmaCrReg = crate::RegValueT<I2CDmaCrReg_SPEC>;

impl I2CDmaCrReg {
    #[inline(always)]
    pub fn tdmae(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2CDmaCrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I2CDmaCrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rdmae(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CDmaCrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I2CDmaCrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CDmaCrReg {
    #[inline(always)]
    fn default() -> I2CDmaCrReg {
        <crate::RegValueT<I2CDmaCrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CDmaRdlrReg_SPEC;
impl crate::sealed::RegSpec for I2CDmaRdlrReg_SPEC {
    type DataType = u16;
}

pub type I2CDmaRdlrReg = crate::RegValueT<I2CDmaRdlrReg_SPEC>;

impl I2CDmaRdlrReg {
    #[inline(always)]
    pub fn dmardl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, I2CDmaRdlrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,I2CDmaRdlrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CDmaRdlrReg {
    #[inline(always)]
    fn default() -> I2CDmaRdlrReg {
        <crate::RegValueT<I2CDmaRdlrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CDmaTdlrReg_SPEC;
impl crate::sealed::RegSpec for I2CDmaTdlrReg_SPEC {
    type DataType = u16;
}

pub type I2CDmaTdlrReg = crate::RegValueT<I2CDmaTdlrReg_SPEC>;

impl I2CDmaTdlrReg {
    #[inline(always)]
    pub fn dmatdl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, I2CDmaTdlrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,I2CDmaTdlrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CDmaTdlrReg {
    #[inline(always)]
    fn default() -> I2CDmaTdlrReg {
        <crate::RegValueT<I2CDmaTdlrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CEnableReg_SPEC;
impl crate::sealed::RegSpec for I2CEnableReg_SPEC {
    type DataType = u16;
}

pub type I2CEnableReg = crate::RegValueT<I2CEnableReg_SPEC>;

impl I2CEnableReg {
    #[inline(always)]
    pub fn i2c_abort(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2CEnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I2CEnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ctrl_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CEnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I2CEnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CEnableReg {
    #[inline(always)]
    fn default() -> I2CEnableReg {
        <crate::RegValueT<I2CEnableReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CEnableStatusReg_SPEC;
impl crate::sealed::RegSpec for I2CEnableStatusReg_SPEC {
    type DataType = u16;
}

pub type I2CEnableStatusReg = crate::RegValueT<I2CEnableStatusReg_SPEC>;

impl I2CEnableStatusReg {
    #[inline(always)]
    pub fn slv_rx_data_lost(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2CEnableStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2CEnableStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slv_disabled_while_busy(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2CEnableStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2CEnableStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ic_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CEnableStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CEnableStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CEnableStatusReg {
    #[inline(always)]
    fn default() -> I2CEnableStatusReg {
        <crate::RegValueT<I2CEnableStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CFsSclHcntReg_SPEC;
impl crate::sealed::RegSpec for I2CFsSclHcntReg_SPEC {
    type DataType = u16;
}

pub type I2CFsSclHcntReg = crate::RegValueT<I2CFsSclHcntReg_SPEC>;

impl I2CFsSclHcntReg {
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
        I2CFsSclHcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2CFsSclHcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2CFsSclHcntReg {
    #[inline(always)]
    fn default() -> I2CFsSclHcntReg {
        <crate::RegValueT<I2CFsSclHcntReg_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CFsSclLcntReg_SPEC;
impl crate::sealed::RegSpec for I2CFsSclLcntReg_SPEC {
    type DataType = u16;
}

pub type I2CFsSclLcntReg = crate::RegValueT<I2CFsSclLcntReg_SPEC>;

impl I2CFsSclLcntReg {
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
        I2CFsSclLcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2CFsSclLcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2CFsSclLcntReg {
    #[inline(always)]
    fn default() -> I2CFsSclLcntReg {
        <crate::RegValueT<I2CFsSclLcntReg_SPEC> as RegisterValue<_>>::new(23)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CHsMaddrReg_SPEC;
impl crate::sealed::RegSpec for I2CHsMaddrReg_SPEC {
    type DataType = u16;
}

pub type I2CHsMaddrReg = crate::RegValueT<I2CHsMaddrReg_SPEC>;

impl I2CHsMaddrReg {
    #[inline(always)]
    pub fn iic_hs_mar(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, I2CHsMaddrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,I2CHsMaddrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CHsMaddrReg {
    #[inline(always)]
    fn default() -> I2CHsMaddrReg {
        <crate::RegValueT<I2CHsMaddrReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CIcFsSpklenReg_SPEC;
impl crate::sealed::RegSpec for I2CIcFsSpklenReg_SPEC {
    type DataType = u16;
}

pub type I2CIcFsSpklenReg = crate::RegValueT<I2CIcFsSpklenReg_SPEC>;

impl I2CIcFsSpklenReg {
    #[inline(always)]
    pub fn ic_fs_spklen(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, I2CIcFsSpklenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I2CIcFsSpklenReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2CIcFsSpklenReg {
    #[inline(always)]
    fn default() -> I2CIcFsSpklenReg {
        <crate::RegValueT<I2CIcFsSpklenReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CIntrMaskReg_SPEC;
impl crate::sealed::RegSpec for I2CIntrMaskReg_SPEC {
    type DataType = u16;
}

pub type I2CIntrMaskReg = crate::RegValueT<I2CIntrMaskReg_SPEC>;

impl I2CIntrMaskReg {
    #[inline(always)]
    pub fn m_gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2CIntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,I2CIntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_start_det(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2CIntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,I2CIntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_stop_det(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2CIntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,I2CIntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_activity(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2CIntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,I2CIntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2CIntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,I2CIntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2CIntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,I2CIntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2CIntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I2CIntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_tx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2CIntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,I2CIntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2CIntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I2CIntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rx_full(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2CIntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,I2CIntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2CIntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I2CIntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rx_under(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CIntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I2CIntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CIntrMaskReg {
    #[inline(always)]
    fn default() -> I2CIntrMaskReg {
        <crate::RegValueT<I2CIntrMaskReg_SPEC> as RegisterValue<_>>::new(2303)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CIntrStatReg_SPEC;
impl crate::sealed::RegSpec for I2CIntrStatReg_SPEC {
    type DataType = u16;
}

pub type I2CIntrStatReg = crate::RegValueT<I2CIntrStatReg_SPEC>;

impl I2CIntrStatReg {
    #[inline(always)]
    pub fn r_gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2CIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,I2CIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_start_det(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2CIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,I2CIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_stop_det(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2CIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,I2CIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_activity(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2CIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,I2CIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2CIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,I2CIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2CIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2CIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2CIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2CIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_tx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2CIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2CIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2CIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2CIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rx_full(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2CIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2CIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2CIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2CIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rx_under(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CIntrStatReg {
    #[inline(always)]
    fn default() -> I2CIntrStatReg {
        <crate::RegValueT<I2CIntrStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CRawIntrStatReg_SPEC;
impl crate::sealed::RegSpec for I2CRawIntrStatReg_SPEC {
    type DataType = u16;
}

pub type I2CRawIntrStatReg = crate::RegValueT<I2CRawIntrStatReg_SPEC>;

impl I2CRawIntrStatReg {
    #[inline(always)]
    pub fn gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2CRawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,I2CRawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn start_det(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2CRawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,I2CRawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn stop_det(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2CRawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,I2CRawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn activity(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2CRawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,I2CRawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2CRawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,I2CRawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2CRawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2CRawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2CRawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2CRawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2CRawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2CRawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2CRawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2CRawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_full(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2CRawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2CRawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2CRawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2CRawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_under(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CRawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CRawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CRawIntrStatReg {
    #[inline(always)]
    fn default() -> I2CRawIntrStatReg {
        <crate::RegValueT<I2CRawIntrStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CRxflrReg_SPEC;
impl crate::sealed::RegSpec for I2CRxflrReg_SPEC {
    type DataType = u16;
}

pub type I2CRxflrReg = crate::RegValueT<I2CRxflrReg_SPEC>;

impl I2CRxflrReg {
    #[inline(always)]
    pub fn rxflr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, I2CRxflrReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,I2CRxflrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CRxflrReg {
    #[inline(always)]
    fn default() -> I2CRxflrReg {
        <crate::RegValueT<I2CRxflrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CRxTlReg_SPEC;
impl crate::sealed::RegSpec for I2CRxTlReg_SPEC {
    type DataType = u16;
}

pub type I2CRxTlReg = crate::RegValueT<I2CRxTlReg_SPEC>;

impl I2CRxTlReg {
    #[inline(always)]
    pub fn rx_tl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, I2CRxTlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,I2CRxTlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CRxTlReg {
    #[inline(always)]
    fn default() -> I2CRxTlReg {
        <crate::RegValueT<I2CRxTlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CSarReg_SPEC;
impl crate::sealed::RegSpec for I2CSarReg_SPEC {
    type DataType = u16;
}

pub type I2CSarReg = crate::RegValueT<I2CSarReg_SPEC>;

impl I2CSarReg {
    #[inline(always)]
    pub fn ic_sar(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, I2CSarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,I2CSarReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CSarReg {
    #[inline(always)]
    fn default() -> I2CSarReg {
        <crate::RegValueT<I2CSarReg_SPEC> as RegisterValue<_>>::new(85)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CSdaHoldReg_SPEC;
impl crate::sealed::RegSpec for I2CSdaHoldReg_SPEC {
    type DataType = u16;
}

pub type I2CSdaHoldReg = crate::RegValueT<I2CSdaHoldReg_SPEC>;

impl I2CSdaHoldReg {
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
        I2CSdaHoldReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2CSdaHoldReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2CSdaHoldReg {
    #[inline(always)]
    fn default() -> I2CSdaHoldReg {
        <crate::RegValueT<I2CSdaHoldReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CSdaSetupReg_SPEC;
impl crate::sealed::RegSpec for I2CSdaSetupReg_SPEC {
    type DataType = u16;
}

pub type I2CSdaSetupReg = crate::RegValueT<I2CSdaSetupReg_SPEC>;

impl I2CSdaSetupReg {
    #[inline(always)]
    pub fn sda_setup(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, I2CSdaSetupReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,I2CSdaSetupReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CSdaSetupReg {
    #[inline(always)]
    fn default() -> I2CSdaSetupReg {
        <crate::RegValueT<I2CSdaSetupReg_SPEC> as RegisterValue<_>>::new(100)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CSsSclHcntReg_SPEC;
impl crate::sealed::RegSpec for I2CSsSclHcntReg_SPEC {
    type DataType = u16;
}

pub type I2CSsSclHcntReg = crate::RegValueT<I2CSsSclHcntReg_SPEC>;

impl I2CSsSclHcntReg {
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
        I2CSsSclHcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2CSsSclHcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2CSsSclHcntReg {
    #[inline(always)]
    fn default() -> I2CSsSclHcntReg {
        <crate::RegValueT<I2CSsSclHcntReg_SPEC> as RegisterValue<_>>::new(72)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CSsSclLcntReg_SPEC;
impl crate::sealed::RegSpec for I2CSsSclLcntReg_SPEC {
    type DataType = u16;
}

pub type I2CSsSclLcntReg = crate::RegValueT<I2CSsSclLcntReg_SPEC>;

impl I2CSsSclLcntReg {
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
        I2CSsSclLcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2CSsSclLcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2CSsSclLcntReg {
    #[inline(always)]
    fn default() -> I2CSsSclLcntReg {
        <crate::RegValueT<I2CSsSclLcntReg_SPEC> as RegisterValue<_>>::new(79)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CStatusReg_SPEC;
impl crate::sealed::RegSpec for I2CStatusReg_SPEC {
    type DataType = u16;
}

pub type I2CStatusReg = crate::RegValueT<I2CStatusReg_SPEC>;

impl I2CStatusReg {
    #[inline(always)]
    pub fn slv_activity(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2CStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2CStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mst_activity(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2CStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2CStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2CStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2CStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfne(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2CStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2CStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tfe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2CStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2CStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tfnf(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2CStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2CStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_activity(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CStatusReg {
    #[inline(always)]
    fn default() -> I2CStatusReg {
        <crate::RegValueT<I2CStatusReg_SPEC> as RegisterValue<_>>::new(6)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CTarReg_SPEC;
impl crate::sealed::RegSpec for I2CTarReg_SPEC {
    type DataType = u16;
}

pub type I2CTarReg = crate::RegValueT<I2CTarReg_SPEC>;

impl I2CTarReg {
    #[inline(always)]
    pub fn special(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2CTarReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,I2CTarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gc_or_start(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2CTarReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,I2CTarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ic_tar(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, I2CTarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,I2CTarReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CTarReg {
    #[inline(always)]
    fn default() -> I2CTarReg {
        <crate::RegValueT<I2CTarReg_SPEC> as RegisterValue<_>>::new(85)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CTxflrReg_SPEC;
impl crate::sealed::RegSpec for I2CTxflrReg_SPEC {
    type DataType = u16;
}

pub type I2CTxflrReg = crate::RegValueT<I2CTxflrReg_SPEC>;

impl I2CTxflrReg {
    #[inline(always)]
    pub fn txflr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, I2CTxflrReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,I2CTxflrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CTxflrReg {
    #[inline(always)]
    fn default() -> I2CTxflrReg {
        <crate::RegValueT<I2CTxflrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CTxAbrtSourceReg_SPEC;
impl crate::sealed::RegSpec for I2CTxAbrtSourceReg_SPEC {
    type DataType = u16;
}

pub type I2CTxAbrtSourceReg = crate::RegValueT<I2CTxAbrtSourceReg_SPEC>;

impl I2CTxAbrtSourceReg {
    #[inline(always)]
    pub fn abrt_slvrd_intx(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_slv_arblost(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_slvflush_txfifo(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn arb_lost(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_master_dis(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_10b_rd_norstrt(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_sbyte_norstrt(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_hs_norstrt(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_sbyte_ackdet(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_hs_ackdet(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_gcall_read(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_gcall_noack(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_txdata_noack(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_10addr2_noack(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_10addr1_noack(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_7b_addr_noack(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2CTxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2CTxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CTxAbrtSourceReg {
    #[inline(always)]
    fn default() -> I2CTxAbrtSourceReg {
        <crate::RegValueT<I2CTxAbrtSourceReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CTxTlReg_SPEC;
impl crate::sealed::RegSpec for I2CTxTlReg_SPEC {
    type DataType = u16;
}

pub type I2CTxTlReg = crate::RegValueT<I2CTxTlReg_SPEC>;

impl I2CTxTlReg {
    #[inline(always)]
    pub fn tx_tl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, I2CTxTlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,I2CTxTlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2CTxTlReg {
    #[inline(always)]
    fn default() -> I2CTxTlReg {
        <crate::RegValueT<I2CTxTlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
