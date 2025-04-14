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
#[doc = r"I2C3 registers"]
unsafe impl ::core::marker::Send for super::I2C3 {}
unsafe impl ::core::marker::Sync for super::I2C3 {}
impl super::I2C3 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn i2c3_ack_general_call_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3AckGeneralCallReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3AckGeneralCallReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_clr_activity_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3ClrActivityReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3ClrActivityReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_clr_gen_call_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3ClrGenCallReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3ClrGenCallReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_clr_intr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3ClrIntrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3ClrIntrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_clr_rd_req_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3ClrRdReqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3ClrRdReqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_clr_rx_done_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3ClrRxDoneReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3ClrRxDoneReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_clr_rx_over_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3ClrRxOverReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3ClrRxOverReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_clr_rx_under_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3ClrRxUnderReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3ClrRxUnderReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_clr_start_det_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3ClrStartDetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3ClrStartDetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_clr_stop_det_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3ClrStopDetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3ClrStopDetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_clr_tx_abrt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3ClrTxAbrtReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3ClrTxAbrtReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_clr_tx_over_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3ClrTxOverReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3ClrTxOverReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_con_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3ConReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3ConReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_data_cmd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3DataCmdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3DataCmdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_dma_cr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3DmaCrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3DmaCrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_dma_rdlr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3DmaRdlrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3DmaRdlrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_dma_tdlr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3DmaTdlrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3DmaTdlrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_enable_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3EnableReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3EnableReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_enable_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3EnableStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3EnableStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_fs_scl_hcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3FsSclHcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3FsSclHcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_fs_scl_lcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3FsSclLcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3FsSclLcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_hs_maddr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3HsMaddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3HsMaddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_hs_scl_hcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3HsSclHcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3HsSclHcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_hs_scl_lcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3HsSclLcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3HsSclLcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_ic_fs_spklen_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3IcFsSpklenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3IcFsSpklenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_ic_hs_spklen_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3IcHsSpklenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3IcHsSpklenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_intr_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3IntrMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3IntrMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_intr_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3IntrStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3IntrStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_raw_intr_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3RawIntrStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3RawIntrStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_rxflr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3RxflrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3RxflrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_rx_tl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3RxTlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3RxTlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_sar_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3SarReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3SarReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_sda_hold_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3SdaHoldReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3SdaHoldReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_sda_setup_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3SdaSetupReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3SdaSetupReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_ss_scl_hcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3SsSclHcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3SsSclHcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_ss_scl_lcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3SsSclLcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3SsSclLcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3StatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3StatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_tar_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3TarReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3TarReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_txflr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3TxflrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3TxflrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_tx_abrt_source_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3TxAbrtSourceReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3TxAbrtSourceReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i2c3_tx_tl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C3TxTlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C3TxTlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3AckGeneralCallReg_SPEC;
impl crate::sealed::RegSpec for I2C3AckGeneralCallReg_SPEC {
    type DataType = u32;
}

pub type I2C3AckGeneralCallReg = crate::RegValueT<I2C3AckGeneralCallReg_SPEC>;

impl I2C3AckGeneralCallReg {
    #[inline(always)]
    pub fn ack_gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3AckGeneralCallReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,I2C3AckGeneralCallReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3AckGeneralCallReg {
    #[inline(always)]
    fn default() -> I2C3AckGeneralCallReg {
        <crate::RegValueT<I2C3AckGeneralCallReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3ClrActivityReg_SPEC;
impl crate::sealed::RegSpec for I2C3ClrActivityReg_SPEC {
    type DataType = u32;
}

pub type I2C3ClrActivityReg = crate::RegValueT<I2C3ClrActivityReg_SPEC>;

impl I2C3ClrActivityReg {
    #[inline(always)]
    pub fn clr_activity(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3ClrActivityReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3ClrActivityReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3ClrActivityReg {
    #[inline(always)]
    fn default() -> I2C3ClrActivityReg {
        <crate::RegValueT<I2C3ClrActivityReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3ClrGenCallReg_SPEC;
impl crate::sealed::RegSpec for I2C3ClrGenCallReg_SPEC {
    type DataType = u32;
}

pub type I2C3ClrGenCallReg = crate::RegValueT<I2C3ClrGenCallReg_SPEC>;

impl I2C3ClrGenCallReg {
    #[inline(always)]
    pub fn clr_gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3ClrGenCallReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3ClrGenCallReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3ClrGenCallReg {
    #[inline(always)]
    fn default() -> I2C3ClrGenCallReg {
        <crate::RegValueT<I2C3ClrGenCallReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3ClrIntrReg_SPEC;
impl crate::sealed::RegSpec for I2C3ClrIntrReg_SPEC {
    type DataType = u32;
}

pub type I2C3ClrIntrReg = crate::RegValueT<I2C3ClrIntrReg_SPEC>;

impl I2C3ClrIntrReg {
    #[inline(always)]
    pub fn clr_intr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3ClrIntrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3ClrIntrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3ClrIntrReg {
    #[inline(always)]
    fn default() -> I2C3ClrIntrReg {
        <crate::RegValueT<I2C3ClrIntrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3ClrRdReqReg_SPEC;
impl crate::sealed::RegSpec for I2C3ClrRdReqReg_SPEC {
    type DataType = u32;
}

pub type I2C3ClrRdReqReg = crate::RegValueT<I2C3ClrRdReqReg_SPEC>;

impl I2C3ClrRdReqReg {
    #[inline(always)]
    pub fn clr_rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3ClrRdReqReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3ClrRdReqReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3ClrRdReqReg {
    #[inline(always)]
    fn default() -> I2C3ClrRdReqReg {
        <crate::RegValueT<I2C3ClrRdReqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3ClrRxDoneReg_SPEC;
impl crate::sealed::RegSpec for I2C3ClrRxDoneReg_SPEC {
    type DataType = u32;
}

pub type I2C3ClrRxDoneReg = crate::RegValueT<I2C3ClrRxDoneReg_SPEC>;

impl I2C3ClrRxDoneReg {
    #[inline(always)]
    pub fn clr_rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3ClrRxDoneReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3ClrRxDoneReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3ClrRxDoneReg {
    #[inline(always)]
    fn default() -> I2C3ClrRxDoneReg {
        <crate::RegValueT<I2C3ClrRxDoneReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3ClrRxOverReg_SPEC;
impl crate::sealed::RegSpec for I2C3ClrRxOverReg_SPEC {
    type DataType = u32;
}

pub type I2C3ClrRxOverReg = crate::RegValueT<I2C3ClrRxOverReg_SPEC>;

impl I2C3ClrRxOverReg {
    #[inline(always)]
    pub fn clr_rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3ClrRxOverReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3ClrRxOverReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3ClrRxOverReg {
    #[inline(always)]
    fn default() -> I2C3ClrRxOverReg {
        <crate::RegValueT<I2C3ClrRxOverReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3ClrRxUnderReg_SPEC;
impl crate::sealed::RegSpec for I2C3ClrRxUnderReg_SPEC {
    type DataType = u32;
}

pub type I2C3ClrRxUnderReg = crate::RegValueT<I2C3ClrRxUnderReg_SPEC>;

impl I2C3ClrRxUnderReg {
    #[inline(always)]
    pub fn clr_rx_under(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3ClrRxUnderReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3ClrRxUnderReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3ClrRxUnderReg {
    #[inline(always)]
    fn default() -> I2C3ClrRxUnderReg {
        <crate::RegValueT<I2C3ClrRxUnderReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3ClrStartDetReg_SPEC;
impl crate::sealed::RegSpec for I2C3ClrStartDetReg_SPEC {
    type DataType = u32;
}

pub type I2C3ClrStartDetReg = crate::RegValueT<I2C3ClrStartDetReg_SPEC>;

impl I2C3ClrStartDetReg {
    #[inline(always)]
    pub fn clr_start_det(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3ClrStartDetReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3ClrStartDetReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3ClrStartDetReg {
    #[inline(always)]
    fn default() -> I2C3ClrStartDetReg {
        <crate::RegValueT<I2C3ClrStartDetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3ClrStopDetReg_SPEC;
impl crate::sealed::RegSpec for I2C3ClrStopDetReg_SPEC {
    type DataType = u32;
}

pub type I2C3ClrStopDetReg = crate::RegValueT<I2C3ClrStopDetReg_SPEC>;

impl I2C3ClrStopDetReg {
    #[inline(always)]
    pub fn clr_stop_det(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3ClrStopDetReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3ClrStopDetReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3ClrStopDetReg {
    #[inline(always)]
    fn default() -> I2C3ClrStopDetReg {
        <crate::RegValueT<I2C3ClrStopDetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3ClrTxAbrtReg_SPEC;
impl crate::sealed::RegSpec for I2C3ClrTxAbrtReg_SPEC {
    type DataType = u32;
}

pub type I2C3ClrTxAbrtReg = crate::RegValueT<I2C3ClrTxAbrtReg_SPEC>;

impl I2C3ClrTxAbrtReg {
    #[inline(always)]
    pub fn clr_tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3ClrTxAbrtReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3ClrTxAbrtReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3ClrTxAbrtReg {
    #[inline(always)]
    fn default() -> I2C3ClrTxAbrtReg {
        <crate::RegValueT<I2C3ClrTxAbrtReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3ClrTxOverReg_SPEC;
impl crate::sealed::RegSpec for I2C3ClrTxOverReg_SPEC {
    type DataType = u32;
}

pub type I2C3ClrTxOverReg = crate::RegValueT<I2C3ClrTxOverReg_SPEC>;

impl I2C3ClrTxOverReg {
    #[inline(always)]
    pub fn clr_tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3ClrTxOverReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3ClrTxOverReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3ClrTxOverReg {
    #[inline(always)]
    fn default() -> I2C3ClrTxOverReg {
        <crate::RegValueT<I2C3ClrTxOverReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3ConReg_SPEC;
impl crate::sealed::RegSpec for I2C3ConReg_SPEC {
    type DataType = u32;
}

pub type I2C3ConReg = crate::RegValueT<I2C3ConReg_SPEC>;

impl I2C3ConReg {
    #[inline(always)]
    pub fn i2c_stop_det_if_master_active(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C3ConReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,I2C3ConReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_rx_fifo_full_hld_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C3ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,I2C3ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_tx_empty_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C3ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,I2C3ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_stop_det_ifaddressed(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C3ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,I2C3ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_slave_disable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C3ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,I2C3ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_restart_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C3ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I2C3ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_10bitaddr_master(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C3ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,I2C3ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_10bitaddr_slave(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C3ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I2C3ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_speed(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, I2C3ConReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,I2C3ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_master_mode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3ConReg {
    #[inline(always)]
    fn default() -> I2C3ConReg {
        <crate::RegValueT<I2C3ConReg_SPEC> as RegisterValue<_>>::new(127)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3DataCmdReg_SPEC;
impl crate::sealed::RegSpec for I2C3DataCmdReg_SPEC {
    type DataType = u32;
}

pub type I2C3DataCmdReg = crate::RegValueT<I2C3DataCmdReg_SPEC>;

impl I2C3DataCmdReg {
    #[inline(always)]
    pub fn i2c_restart(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C3DataCmdReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10,1,0,I2C3DataCmdReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_stop(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C3DataCmdReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9,1,0,I2C3DataCmdReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_cmd(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C3DataCmdReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8,1,0,I2C3DataCmdReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_dat(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, I2C3DataCmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,I2C3DataCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3DataCmdReg {
    #[inline(always)]
    fn default() -> I2C3DataCmdReg {
        <crate::RegValueT<I2C3DataCmdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3DmaCrReg_SPEC;
impl crate::sealed::RegSpec for I2C3DmaCrReg_SPEC {
    type DataType = u32;
}

pub type I2C3DmaCrReg = crate::RegValueT<I2C3DmaCrReg_SPEC>;

impl I2C3DmaCrReg {
    #[inline(always)]
    pub fn tdmae(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C3DmaCrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I2C3DmaCrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rdmae(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3DmaCrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3DmaCrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3DmaCrReg {
    #[inline(always)]
    fn default() -> I2C3DmaCrReg {
        <crate::RegValueT<I2C3DmaCrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3DmaRdlrReg_SPEC;
impl crate::sealed::RegSpec for I2C3DmaRdlrReg_SPEC {
    type DataType = u32;
}

pub type I2C3DmaRdlrReg = crate::RegValueT<I2C3DmaRdlrReg_SPEC>;

impl I2C3DmaRdlrReg {
    #[inline(always)]
    pub fn dmardl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, I2C3DmaRdlrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,I2C3DmaRdlrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3DmaRdlrReg {
    #[inline(always)]
    fn default() -> I2C3DmaRdlrReg {
        <crate::RegValueT<I2C3DmaRdlrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3DmaTdlrReg_SPEC;
impl crate::sealed::RegSpec for I2C3DmaTdlrReg_SPEC {
    type DataType = u32;
}

pub type I2C3DmaTdlrReg = crate::RegValueT<I2C3DmaTdlrReg_SPEC>;

impl I2C3DmaTdlrReg {
    #[inline(always)]
    pub fn dmatdl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, I2C3DmaTdlrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,I2C3DmaTdlrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3DmaTdlrReg {
    #[inline(always)]
    fn default() -> I2C3DmaTdlrReg {
        <crate::RegValueT<I2C3DmaTdlrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3EnableReg_SPEC;
impl crate::sealed::RegSpec for I2C3EnableReg_SPEC {
    type DataType = u32;
}

pub type I2C3EnableReg = crate::RegValueT<I2C3EnableReg_SPEC>;

impl I2C3EnableReg {
    #[inline(always)]
    pub fn i2c_tx_cmd_block(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C3EnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,I2C3EnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_abort(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C3EnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I2C3EnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3EnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3EnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3EnableReg {
    #[inline(always)]
    fn default() -> I2C3EnableReg {
        <crate::RegValueT<I2C3EnableReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3EnableStatusReg_SPEC;
impl crate::sealed::RegSpec for I2C3EnableStatusReg_SPEC {
    type DataType = u32;
}

pub type I2C3EnableStatusReg = crate::RegValueT<I2C3EnableStatusReg_SPEC>;

impl I2C3EnableStatusReg {
    #[inline(always)]
    pub fn slv_rx_data_lost(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C3EnableStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C3EnableStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slv_disabled_while_busy(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C3EnableStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C3EnableStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ic_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3EnableStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3EnableStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3EnableStatusReg {
    #[inline(always)]
    fn default() -> I2C3EnableStatusReg {
        <crate::RegValueT<I2C3EnableStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3FsSclHcntReg_SPEC;
impl crate::sealed::RegSpec for I2C3FsSclHcntReg_SPEC {
    type DataType = u32;
}

pub type I2C3FsSclHcntReg = crate::RegValueT<I2C3FsSclHcntReg_SPEC>;

impl I2C3FsSclHcntReg {
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
        I2C3FsSclHcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C3FsSclHcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C3FsSclHcntReg {
    #[inline(always)]
    fn default() -> I2C3FsSclHcntReg {
        <crate::RegValueT<I2C3FsSclHcntReg_SPEC> as RegisterValue<_>>::new(26)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3FsSclLcntReg_SPEC;
impl crate::sealed::RegSpec for I2C3FsSclLcntReg_SPEC {
    type DataType = u32;
}

pub type I2C3FsSclLcntReg = crate::RegValueT<I2C3FsSclLcntReg_SPEC>;

impl I2C3FsSclLcntReg {
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
        I2C3FsSclLcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C3FsSclLcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C3FsSclLcntReg {
    #[inline(always)]
    fn default() -> I2C3FsSclLcntReg {
        <crate::RegValueT<I2C3FsSclLcntReg_SPEC> as RegisterValue<_>>::new(50)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3HsMaddrReg_SPEC;
impl crate::sealed::RegSpec for I2C3HsMaddrReg_SPEC {
    type DataType = u32;
}

pub type I2C3HsMaddrReg = crate::RegValueT<I2C3HsMaddrReg_SPEC>;

impl I2C3HsMaddrReg {
    #[inline(always)]
    pub fn i2c_ic_hs_mar(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, I2C3HsMaddrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,I2C3HsMaddrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3HsMaddrReg {
    #[inline(always)]
    fn default() -> I2C3HsMaddrReg {
        <crate::RegValueT<I2C3HsMaddrReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3HsSclHcntReg_SPEC;
impl crate::sealed::RegSpec for I2C3HsSclHcntReg_SPEC {
    type DataType = u32;
}

pub type I2C3HsSclHcntReg = crate::RegValueT<I2C3HsSclHcntReg_SPEC>;

impl I2C3HsSclHcntReg {
    #[inline(always)]
    pub fn ic_hs_scl_hcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I2C3HsSclHcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C3HsSclHcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C3HsSclHcntReg {
    #[inline(always)]
    fn default() -> I2C3HsSclHcntReg {
        <crate::RegValueT<I2C3HsSclHcntReg_SPEC> as RegisterValue<_>>::new(6)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3HsSclLcntReg_SPEC;
impl crate::sealed::RegSpec for I2C3HsSclLcntReg_SPEC {
    type DataType = u32;
}

pub type I2C3HsSclLcntReg = crate::RegValueT<I2C3HsSclLcntReg_SPEC>;

impl I2C3HsSclLcntReg {
    #[inline(always)]
    pub fn ic_hs_scl_lcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I2C3HsSclLcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C3HsSclLcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C3HsSclLcntReg {
    #[inline(always)]
    fn default() -> I2C3HsSclLcntReg {
        <crate::RegValueT<I2C3HsSclLcntReg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3IcFsSpklenReg_SPEC;
impl crate::sealed::RegSpec for I2C3IcFsSpklenReg_SPEC {
    type DataType = u32;
}

pub type I2C3IcFsSpklenReg = crate::RegValueT<I2C3IcFsSpklenReg_SPEC>;

impl I2C3IcFsSpklenReg {
    #[inline(always)]
    pub fn i2c_fs_spklen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I2C3IcFsSpklenReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I2C3IcFsSpklenReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C3IcFsSpklenReg {
    #[inline(always)]
    fn default() -> I2C3IcFsSpklenReg {
        <crate::RegValueT<I2C3IcFsSpklenReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3IcHsSpklenReg_SPEC;
impl crate::sealed::RegSpec for I2C3IcHsSpklenReg_SPEC {
    type DataType = u32;
}

pub type I2C3IcHsSpklenReg = crate::RegValueT<I2C3IcHsSpklenReg_SPEC>;

impl I2C3IcHsSpklenReg {
    #[inline(always)]
    pub fn i2c_hs_spklen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I2C3IcHsSpklenReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I2C3IcHsSpklenReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C3IcHsSpklenReg {
    #[inline(always)]
    fn default() -> I2C3IcHsSpklenReg {
        <crate::RegValueT<I2C3IcHsSpklenReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3IntrMaskReg_SPEC;
impl crate::sealed::RegSpec for I2C3IntrMaskReg_SPEC {
    type DataType = u32;
}

pub type I2C3IntrMaskReg = crate::RegValueT<I2C3IntrMaskReg_SPEC>;

impl I2C3IntrMaskReg {
    #[inline(always)]
    pub fn m_scl_stuck_at_low(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,I2C3IntrMaskReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_master_on_hold(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,I2C3IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_restart_det(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,I2C3IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,I2C3IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_start_det(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,I2C3IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_stop_det(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,I2C3IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_activity(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,I2C3IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,I2C3IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,I2C3IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I2C3IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_tx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,I2C3IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I2C3IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rx_full(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,I2C3IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I2C3IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn m_rx_under(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3IntrMaskReg {
    #[inline(always)]
    fn default() -> I2C3IntrMaskReg {
        <crate::RegValueT<I2C3IntrMaskReg_SPEC> as RegisterValue<_>>::new(2303)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3IntrStatReg_SPEC;
impl crate::sealed::RegSpec for I2C3IntrStatReg_SPEC {
    type DataType = u32;
}

pub type I2C3IntrStatReg = crate::RegValueT<I2C3IntrStatReg_SPEC>;

impl I2C3IntrStatReg {
    #[inline(always)]
    pub fn r_scl_stuck_at_low(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_master_on_hold(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_restart_det(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_start_det(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_stop_det(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_activity(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_tx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rx_full(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_rx_under(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3IntrStatReg {
    #[inline(always)]
    fn default() -> I2C3IntrStatReg {
        <crate::RegValueT<I2C3IntrStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3RawIntrStatReg_SPEC;
impl crate::sealed::RegSpec for I2C3RawIntrStatReg_SPEC {
    type DataType = u32;
}

pub type I2C3RawIntrStatReg = crate::RegValueT<I2C3RawIntrStatReg_SPEC>;

impl I2C3RawIntrStatReg {
    #[inline(always)]
    pub fn scl_stuck_at_low(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn master_on_hold(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn restart_det(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn start_det(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn stop_det(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn activity(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_full(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_under(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3RawIntrStatReg {
    #[inline(always)]
    fn default() -> I2C3RawIntrStatReg {
        <crate::RegValueT<I2C3RawIntrStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3RxflrReg_SPEC;
impl crate::sealed::RegSpec for I2C3RxflrReg_SPEC {
    type DataType = u32;
}

pub type I2C3RxflrReg = crate::RegValueT<I2C3RxflrReg_SPEC>;

impl I2C3RxflrReg {
    #[inline(always)]
    pub fn rxflr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, I2C3RxflrReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,I2C3RxflrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3RxflrReg {
    #[inline(always)]
    fn default() -> I2C3RxflrReg {
        <crate::RegValueT<I2C3RxflrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3RxTlReg_SPEC;
impl crate::sealed::RegSpec for I2C3RxTlReg_SPEC {
    type DataType = u32;
}

pub type I2C3RxTlReg = crate::RegValueT<I2C3RxTlReg_SPEC>;

impl I2C3RxTlReg {
    #[inline(always)]
    pub fn rx_tl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, I2C3RxTlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,I2C3RxTlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3RxTlReg {
    #[inline(always)]
    fn default() -> I2C3RxTlReg {
        <crate::RegValueT<I2C3RxTlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3SarReg_SPEC;
impl crate::sealed::RegSpec for I2C3SarReg_SPEC {
    type DataType = u32;
}

pub type I2C3SarReg = crate::RegValueT<I2C3SarReg_SPEC>;

impl I2C3SarReg {
    #[inline(always)]
    pub fn ic_sar(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, I2C3SarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,I2C3SarReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3SarReg {
    #[inline(always)]
    fn default() -> I2C3SarReg {
        <crate::RegValueT<I2C3SarReg_SPEC> as RegisterValue<_>>::new(85)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3SdaHoldReg_SPEC;
impl crate::sealed::RegSpec for I2C3SdaHoldReg_SPEC {
    type DataType = u32;
}

pub type I2C3SdaHoldReg = crate::RegValueT<I2C3SdaHoldReg_SPEC>;

impl I2C3SdaHoldReg {
    #[inline(always)]
    pub fn i2c_sda_rx_hold(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, I2C3SdaHoldReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,I2C3SdaHoldReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_sda_tx_hold(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I2C3SdaHoldReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C3SdaHoldReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C3SdaHoldReg {
    #[inline(always)]
    fn default() -> I2C3SdaHoldReg {
        <crate::RegValueT<I2C3SdaHoldReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3SdaSetupReg_SPEC;
impl crate::sealed::RegSpec for I2C3SdaSetupReg_SPEC {
    type DataType = u32;
}

pub type I2C3SdaSetupReg = crate::RegValueT<I2C3SdaSetupReg_SPEC>;

impl I2C3SdaSetupReg {
    #[inline(always)]
    pub fn sda_setup(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, I2C3SdaSetupReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,I2C3SdaSetupReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3SdaSetupReg {
    #[inline(always)]
    fn default() -> I2C3SdaSetupReg {
        <crate::RegValueT<I2C3SdaSetupReg_SPEC> as RegisterValue<_>>::new(100)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3SsSclHcntReg_SPEC;
impl crate::sealed::RegSpec for I2C3SsSclHcntReg_SPEC {
    type DataType = u32;
}

pub type I2C3SsSclHcntReg = crate::RegValueT<I2C3SsSclHcntReg_SPEC>;

impl I2C3SsSclHcntReg {
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
        I2C3SsSclHcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C3SsSclHcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C3SsSclHcntReg {
    #[inline(always)]
    fn default() -> I2C3SsSclHcntReg {
        <crate::RegValueT<I2C3SsSclHcntReg_SPEC> as RegisterValue<_>>::new(145)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3SsSclLcntReg_SPEC;
impl crate::sealed::RegSpec for I2C3SsSclLcntReg_SPEC {
    type DataType = u32;
}

pub type I2C3SsSclLcntReg = crate::RegValueT<I2C3SsSclLcntReg_SPEC>;

impl I2C3SsSclLcntReg {
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
        I2C3SsSclLcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C3SsSclLcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C3SsSclLcntReg {
    #[inline(always)]
    fn default() -> I2C3SsSclLcntReg {
        <crate::RegValueT<I2C3SsSclLcntReg_SPEC> as RegisterValue<_>>::new(171)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3StatusReg_SPEC;
impl crate::sealed::RegSpec for I2C3StatusReg_SPEC {
    type DataType = u32;
}

pub type I2C3StatusReg = crate::RegValueT<I2C3StatusReg_SPEC>;

impl I2C3StatusReg {
    #[inline(always)]
    pub fn lv_hold_rx_fifo_full(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C3StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,I2C3StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slv_hold_tx_fifo_empty(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C3StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,I2C3StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mst_hold_rx_fifo_full(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C3StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,I2C3StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mst_hold_tx_fifo_empty(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C3StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,I2C3StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slv_activity(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C3StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2C3StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mst_activity(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C3StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2C3StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C3StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2C3StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfne(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C3StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2C3StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tfe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C3StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C3StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tfnf(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C3StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C3StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_activity(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3StatusReg {
    #[inline(always)]
    fn default() -> I2C3StatusReg {
        <crate::RegValueT<I2C3StatusReg_SPEC> as RegisterValue<_>>::new(6)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3TarReg_SPEC;
impl crate::sealed::RegSpec for I2C3TarReg_SPEC {
    type DataType = u32;
}

pub type I2C3TarReg = crate::RegValueT<I2C3TarReg_SPEC>;

impl I2C3TarReg {
    #[inline(always)]
    pub fn special(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C3TarReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,I2C3TarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gc_or_start(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C3TarReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,I2C3TarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ic_tar(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, I2C3TarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,I2C3TarReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3TarReg {
    #[inline(always)]
    fn default() -> I2C3TarReg {
        <crate::RegValueT<I2C3TarReg_SPEC> as RegisterValue<_>>::new(85)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3TxflrReg_SPEC;
impl crate::sealed::RegSpec for I2C3TxflrReg_SPEC {
    type DataType = u32;
}

pub type I2C3TxflrReg = crate::RegValueT<I2C3TxflrReg_SPEC>;

impl I2C3TxflrReg {
    #[inline(always)]
    pub fn txflr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, I2C3TxflrReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,I2C3TxflrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3TxflrReg {
    #[inline(always)]
    fn default() -> I2C3TxflrReg {
        <crate::RegValueT<I2C3TxflrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3TxAbrtSourceReg_SPEC;
impl crate::sealed::RegSpec for I2C3TxAbrtSourceReg_SPEC {
    type DataType = u32;
}

pub type I2C3TxAbrtSourceReg = crate::RegValueT<I2C3TxAbrtSourceReg_SPEC>;

impl I2C3TxAbrtSourceReg {
    #[inline(always)]
    pub fn abrt_user_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<16,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_slvrd_intx(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<15,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_slv_arblost(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<14,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_slvflush_txfifo(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<13,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn arb_lost(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<12,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_master_dis(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<11,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_10b_rd_norstrt(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_sbyte_norstrt(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_hs_norstrt(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_sbyte_ackdet(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_hs_ackdet(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_gcall_read(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_gcall_noack(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_txdata_noack(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_10addr2_noack(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_10addr1_noack(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abrt_7b_addr_noack(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I2C3TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I2C3TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3TxAbrtSourceReg {
    #[inline(always)]
    fn default() -> I2C3TxAbrtSourceReg {
        <crate::RegValueT<I2C3TxAbrtSourceReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C3TxTlReg_SPEC;
impl crate::sealed::RegSpec for I2C3TxTlReg_SPEC {
    type DataType = u32;
}

pub type I2C3TxTlReg = crate::RegValueT<I2C3TxTlReg_SPEC>;

impl I2C3TxTlReg {
    #[inline(always)]
    pub fn tx_tl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, I2C3TxTlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,I2C3TxTlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C3TxTlReg {
    #[inline(always)]
    fn default() -> I2C3TxTlReg {
        <crate::RegValueT<I2C3TxTlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
