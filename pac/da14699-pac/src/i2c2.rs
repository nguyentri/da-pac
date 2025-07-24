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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:45 +0000

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

    #[doc = "I2C ACK General Call Register"]
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

    #[doc = "Clear ACTIVITY Interrupt Register"]
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

    #[doc = "Clear GEN_CALL Interrupt Register"]
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

    #[doc = "Clear Combined and Individual Interrupt Register"]
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

    #[doc = "Clear RD_REQ Interrupt Register"]
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

    #[doc = "Clear RX_DONE Interrupt Register"]
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

    #[doc = "Clear RX_OVER Interrupt Register"]
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

    #[doc = "Clear RX_UNDER Interrupt Register"]
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

    #[doc = "Clear START_DET Interrupt Register"]
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

    #[doc = "Clear STOP_DET Interrupt Register"]
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

    #[doc = "Clear TX_ABRT Interrupt Register"]
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

    #[doc = "Clear TX_OVER Interrupt Register"]
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

    #[doc = "Component Parameter Register"]
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

    #[doc = "I2C Component Type Register"]
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

    #[doc = "I2C Component Version Register"]
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

    #[doc = "I2C Control Register"]
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

    #[doc = "I2C Rx/Tx Data Buffer and Command Register"]
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

    #[doc = "DMA Control Register"]
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

    #[doc = "I2C Receive Data Level Register"]
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

    #[doc = "DMA Transmit Data Level Register"]
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

    #[doc = "I2C Enable Register"]
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

    #[doc = "I2C Enable Status Register"]
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

    #[doc = "Fast Speed I2C Clock SCL High Count Register"]
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

    #[doc = "Fast Speed I2C Clock SCL Low Count Register"]
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

    #[doc = "I2C High Speed Master Mode Code Address Register"]
    #[inline(always)]
    pub const fn i2c2_hs_maddr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2HsMaddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2HsMaddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "High Speed I2C Clock SCL High Count Register"]
    #[inline(always)]
    pub const fn i2c2_hs_scl_hcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2HsSclHcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2HsSclHcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "High Speed I2C Clock SCL Low Count Register"]
    #[inline(always)]
    pub const fn i2c2_hs_scl_lcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2HsSclLcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2HsSclLcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "I2C SS and FS spike suppression limit Size"]
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

    #[doc = "I2C HS spike suppression limit Size"]
    #[inline(always)]
    pub const fn i2c2_ic_hs_spklen_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I2C2IcHsSpklenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I2C2IcHsSpklenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "I2C Interrupt Mask Register"]
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

    #[doc = "I2C Interrupt Status Register"]
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

    #[doc = "I2C Raw Interrupt Status Register"]
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

    #[doc = "I2C Receive FIFO Level Register"]
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

    #[doc = "I2C Receive FIFO Threshold Register"]
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

    #[doc = "I2C Slave Address Register"]
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

    #[doc = "I2C SDA Hold Time Length Register"]
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

    #[doc = "I2C SDA Setup Register"]
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

    #[doc = "Standard Speed I2C Clock SCL High Count Register"]
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

    #[doc = "Standard Speed I2C Clock SCL Low Count Register"]
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

    #[doc = "I2C Status Register"]
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

    #[doc = "I2C Target Address Register"]
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

    #[doc = "I2C Transmit FIFO Level Register"]
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

    #[doc = "I2C Transmit Abort Source Register"]
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

    #[doc = "I2C Transmit FIFO Threshold Register"]
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
    type DataType = u32;
}

#[doc = "I2C ACK General Call Register"]
pub type I2C2AckGeneralCallReg = crate::RegValueT<I2C2AckGeneralCallReg_SPEC>;

impl I2C2AckGeneralCallReg {
    #[doc = "ACK General Call. When set to 1, I2C Ctrl responds with a ACK (by asserting ic_data_oe) when it receives a General Call. When set to 0, the controller does not generate General Call interrupts.\n1 = Generate ACK for a General Call\n0 = Generate NACK for General Call"]
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
    type DataType = u32;
}

#[doc = "Clear ACTIVITY Interrupt Register"]
pub type I2C2ClrActivityReg = crate::RegValueT<I2C2ClrActivityReg_SPEC>;

impl I2C2ClrActivityReg {
    #[doc = "Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore. If the I2C module is still active on the bus, the ACTIVITY interrupt bit continues to be set. It is automatically cleared by hardware if the module is disabled and if there is no further activity on the bus. The value read from this register to get status of the ACTIVITY interrupt (bit 8) of the IC_RAW_INTR_STAT register"]
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
    type DataType = u32;
}

#[doc = "Clear GEN_CALL Interrupt Register"]
pub type I2C2ClrGenCallReg = crate::RegValueT<I2C2ClrGenCallReg_SPEC>;

impl I2C2ClrGenCallReg {
    #[doc = "Read this register to clear the GEN_CALL interrupt (bit 11) of\nI2C_RAW_INTR_STAT register."]
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
    type DataType = u32;
}

#[doc = "Clear Combined and Individual Interrupt Register"]
pub type I2C2ClrIntrReg = crate::RegValueT<I2C2ClrIntrReg_SPEC>;

impl I2C2ClrIntrReg {
    #[doc = "Read this register to clear the combined interrupt, all individual interrupts, and the I2C_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts. Refer to Bit 9 of the I2C_TX_ABRT_SOURCE register for an exception to clearing I2C_TX_ABRT_SOURCE"]
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
    type DataType = u32;
}

#[doc = "Clear RD_REQ Interrupt Register"]
pub type I2C2ClrRdReqReg = crate::RegValueT<I2C2ClrRdReqReg_SPEC>;

impl I2C2ClrRdReqReg {
    #[doc = "Read this register to clear the RD_REQ interrupt (bit 5) of the I2C_RAW_INTR_STAT register."]
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
    type DataType = u32;
}

#[doc = "Clear RX_DONE Interrupt Register"]
pub type I2C2ClrRxDoneReg = crate::RegValueT<I2C2ClrRxDoneReg_SPEC>;

impl I2C2ClrRxDoneReg {
    #[doc = "Read this register to clear the RX_DONE interrupt (bit 7) of the\nI2C_RAW_INTR_STAT register."]
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
    type DataType = u32;
}

#[doc = "Clear RX_OVER Interrupt Register"]
pub type I2C2ClrRxOverReg = crate::RegValueT<I2C2ClrRxOverReg_SPEC>;

impl I2C2ClrRxOverReg {
    #[doc = "Read this register to clear the RX_OVER interrupt (bit 1) of the\nI2C_RAW_INTR_STAT register."]
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
    type DataType = u32;
}

#[doc = "Clear RX_UNDER Interrupt Register"]
pub type I2C2ClrRxUnderReg = crate::RegValueT<I2C2ClrRxUnderReg_SPEC>;

impl I2C2ClrRxUnderReg {
    #[doc = "Read this register to clear the RX_UNDER interrupt (bit 0) of the\nI2C_RAW_INTR_STAT register."]
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
    type DataType = u32;
}

#[doc = "Clear START_DET Interrupt Register"]
pub type I2C2ClrStartDetReg = crate::RegValueT<I2C2ClrStartDetReg_SPEC>;

impl I2C2ClrStartDetReg {
    #[doc = "Read this register to clear the START_DET interrupt (bit 10) of the IC_RAW_INTR_STAT register."]
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
    type DataType = u32;
}

#[doc = "Clear STOP_DET Interrupt Register"]
pub type I2C2ClrStopDetReg = crate::RegValueT<I2C2ClrStopDetReg_SPEC>;

impl I2C2ClrStopDetReg {
    #[doc = "Read this register to clear the STOP_DET interrupt (bit 9) of the IC_RAW_INTR_STAT register."]
    #[inline(always)]
    pub fn clr_stop_det(
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
    type DataType = u32;
}

#[doc = "Clear TX_ABRT Interrupt Register"]
pub type I2C2ClrTxAbrtReg = crate::RegValueT<I2C2ClrTxAbrtReg_SPEC>;

impl I2C2ClrTxAbrtReg {
    #[doc = "Read this register to clear the TX_ABRT interrupt (bit 6) of the\nIC_RAW_INTR_STAT register, and the I2C_TX_ABRT_SOURCE register. This also releases the TX FIFO from the flushed/reset state, allowing more writes to the TX FIFO. Refer to Bit 9 of the I2C_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE."]
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
    type DataType = u32;
}

#[doc = "Clear TX_OVER Interrupt Register"]
pub type I2C2ClrTxOverReg = crate::RegValueT<I2C2ClrTxOverReg_SPEC>;

impl I2C2ClrTxOverReg {
    #[doc = "Read this register to clear the TX_OVER interrupt (bit 3) of the I2C_RAW_INTR_STAT register."]
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
pub struct I2C2CompParam1Reg_SPEC;
impl crate::sealed::RegSpec for I2C2CompParam1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Component Parameter Register"]
pub type I2C2CompParam1Reg = crate::RegValueT<I2C2CompParam1Reg_SPEC>;

impl I2C2CompParam1Reg {
    #[doc = "This is a constant read-only register that contains encoded information about the component\'s parameter settings."]
    #[inline(always)]
    pub fn ic_comp_param1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I2C2CompParam1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
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
pub struct I2C2CompTypeReg_SPEC;
impl crate::sealed::RegSpec for I2C2CompTypeReg_SPEC {
    type DataType = u32;
}

#[doc = "I2C Component Type Register"]
pub type I2C2CompTypeReg = crate::RegValueT<I2C2CompTypeReg_SPEC>;

impl I2C2CompTypeReg {
    #[inline(always)]
    pub fn ic_comp_type(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I2C2CompTypeReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I2C2CompTypeReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2CompTypeReg {
    #[inline(always)]
    fn default() -> I2C2CompTypeReg {
        <crate::RegValueT<I2C2CompTypeReg_SPEC> as RegisterValue<_>>::new(1146552640)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2CompVersionReg_SPEC;
impl crate::sealed::RegSpec for I2C2CompVersionReg_SPEC {
    type DataType = u32;
}

#[doc = "I2C Component Version Register"]
pub type I2C2CompVersionReg = crate::RegValueT<I2C2CompVersionReg_SPEC>;

impl I2C2CompVersionReg {
    #[inline(always)]
    pub fn ic_comp_version(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I2C2CompVersionReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I2C2CompVersionReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2CompVersionReg {
    #[inline(always)]
    fn default() -> I2C2CompVersionReg {
        <crate::RegValueT<I2C2CompVersionReg_SPEC> as RegisterValue<_>>::new(842019370)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2ConReg_SPEC;
impl crate::sealed::RegSpec for I2C2ConReg_SPEC {
    type DataType = u32;
}

#[doc = "I2C Control Register"]
pub type I2C2ConReg = crate::RegValueT<I2C2ConReg_SPEC>;

impl I2C2ConReg {
    #[doc = "In Master mode:\n1 = issues the STOP_DET interrupt only when master is active.\n0 = issues the STOP_DET irrespective of whether master is active or not."]
    #[inline(always)]
    pub fn i2c_stop_det_if_master_active(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C2ConReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,I2C2ConReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH\n1 = Hold bus when RX_FIFO is full\n0 = Overflow when RX_FIFO is full"]
    #[inline(always)]
    pub fn i2c_rx_fifo_full_hld_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C2ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,I2C2ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register.\n1 = Controlled generation of TX_EMPTY interrupt\n0 = Default behaviour of TX_EMPTY interrupt"]
    #[inline(always)]
    pub fn i2c_tx_empty_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C2ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,I2C2ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1 = slave issues STOP_DET intr only if addressed\n0 = slave issues STOP_DET intr always\nDuring a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1\'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR)."]
    #[inline(always)]
    pub fn i2c_stop_det_ifaddressed(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C2ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,I2C2ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave enabled or disabled after reset is applied, which means software does not have to configure the slave.\n0=slave is enabled\n1=slave is disabled\nSoftware should ensure that if this bit is written with \'0\', then bit 0 should also be written with a \'0\'."]
    #[inline(always)]
    pub fn i2c_slave_disable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C2ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,I2C2ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Determines whether RESTART conditions may be sent when acting as a master\n0= disable\n1=enable"]
    #[inline(always)]
    pub fn i2c_restart_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C2ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I2C2ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Controls whether the controller starts its transfers in 7- or 10-bit addressing mode when acting as a master.\n0= 7-bit addressing\n1= 10-bit addressing"]
    #[inline(always)]
    pub fn i2c_10bitaddr_master(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C2ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,I2C2ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When acting as a slave, this bit controls whether the controller responds to 7- or 10-bit addresses.\n0= 7-bit addressing\n1= 10-bit addressing"]
    #[inline(always)]
    pub fn i2c_10bitaddr_slave(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C2ConReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I2C2ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits control at which speed the controller operates.\n1= standard mode (100 kbit/s)\n2= fast mode (400 kbit/s)\n3= high speed mode"]
    #[inline(always)]
    pub fn i2c_speed(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, I2C2ConReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,I2C2ConReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This bit controls whether the controller master is enabled.\n0= master disabled\n1= master enabled\nSoftware should ensure that if this bit is written with \'1\' then bit 6 should also be written with a \'1\'."]
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
        <crate::RegValueT<I2C2ConReg_SPEC> as RegisterValue<_>>::new(127)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2DataCmdReg_SPEC;
impl crate::sealed::RegSpec for I2C2DataCmdReg_SPEC {
    type DataType = u32;
}

#[doc = "I2C Rx/Tx Data Buffer and Command Register"]
pub type I2C2DataCmdReg = crate::RegValueT<I2C2DataCmdReg_SPEC>;

impl I2C2DataCmdReg {
    #[doc = "This bit controls whether a RESTART is issued before the byte is sent or received. \n1 = If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.\n0 = If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead."]
    #[inline(always)]
    pub fn i2c_restart(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C2DataCmdReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10,1,0,I2C2DataCmdReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "This bit controls whether a STOP is issued after the byte is sent or received.\n1 = STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus.\n0 = STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO."]
    #[inline(always)]
    pub fn i2c_stop(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C2DataCmdReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9,1,0,I2C2DataCmdReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "This bit controls whether a read or a write is performed. This bit does not control the direction when the I2C Ctrl acts as a slave. It controls only the direction when it acts as a master.\n1 = Read\n0 = Write\nWhen a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a \"don\'t care\" because writes to this register are not required. In slave-transmitter mode, a \"0\" indicates that CPU data is to be transmitted and as DAT or IC_DATA_CMD\\[7:0\\]. When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the I2C_RAW_INTR_STAT_REG), unless bit 11 (SPECIAL) in the I2C_TAR register has been cleared.\nIf a \"1\" is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.\nNOTE: It is possible that while attempting a master I2C read transfer on the controller, a RD_REQ interrupt may have occurred simultaneously due to a remote I2C master addressing the controller. In this type of scenario, it ignores the I2C_DATA_CMD write, generates a TX_ABRT interrupt, and waits to service the RD_REQ interrupt"]
    #[inline(always)]
    pub fn i2c_cmd(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C2DataCmdReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8,1,0,I2C2DataCmdReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the controller. However, when you read this register, these bits return the value of data received on the controller\'s interface."]
    #[inline(always)]
    pub fn i2c_dat(
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
    type DataType = u32;
}

#[doc = "DMA Control Register"]
pub type I2C2DmaCrReg = crate::RegValueT<I2C2DmaCrReg_SPEC>;

impl I2C2DmaCrReg {
    #[doc = "Transmit DMA Enable. //This bit enables/disables the transmit FIFO DMA channel.\n0 = Transmit DMA disabled\n1 = Transmit DMA enabled"]
    #[inline(always)]
    pub fn tdmae(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2DmaCrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2DmaCrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel.\n0 = Receive DMA disabled\n1 = Receive DMA enabled"]
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
    type DataType = u32;
}

#[doc = "I2C Receive Data Level Register"]
pub type I2C2DmaRdlrReg = crate::RegValueT<I2C2DmaRdlrReg_SPEC>;

impl I2C2DmaRdlrReg {
    #[doc = "Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO."]
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
    type DataType = u32;
}

#[doc = "DMA Transmit Data Level Register"]
pub type I2C2DmaTdlrReg = crate::RegValueT<I2C2DmaTdlrReg_SPEC>;

impl I2C2DmaTdlrReg {
    #[doc = "Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1."]
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
    type DataType = u32;
}

#[doc = "I2C Enable Register"]
pub type I2C2EnableReg = crate::RegValueT<I2C2EnableReg_SPEC>;

impl I2C2EnableReg {
    #[doc = "In Master mode:\n1 = Blocks the transmission of data on I2C bus even if Tx FIFO has data to transmit.\n0.= The transmission of data starts on I2C bus automatically, as soon as the first data is available in the Tx FIFO."]
    #[inline(always)]
    pub fn i2c_tx_cmd_block(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C2EnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,I2C2EnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation."]
    #[inline(always)]
    pub fn i2c_abort(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2EnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2EnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Controls whether the controller is enabled.\n0 = Disables the controller (TX and RX FIFOs are held in an erased state)\n1 = Enables the controller\nSoftware can disable the controller while it is active. However, it is important that care be taken to ensure that the controller is disabled properly. When the controller is disabled, the following occurs:\n* The TX FIFO and RX FIFO get flushed.\n* Status bits in the IC_INTR_STAT register are still active until the controller goes into IDLE state.\nIf the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the controller stops the current transfer at the end of the current byte and does not acknowledge the transfer.\nThere is a two ic_clk delay when enabling or disabling the controller"]
    #[inline(always)]
    pub fn i2c_en(
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
    type DataType = u32;
}

#[doc = "I2C Enable Status Register"]
pub type I2C2EnableStatusReg = crate::RegValueT<I2C2EnableStatusReg_SPEC>;

impl I2C2EnableStatusReg {
    #[doc = "Slave Received Data Lost. This bit indicates if a Slave-Receiver operation has been aborted with at least one data byte received from an I2C transfer due to the setting of IC_ENABLE from 1 to 0. When read as 1, the controller is deemed to have been actively engaged in an aborted I2C transfer (with matching address) and the data phase of the I2C transfer has been entered, even though a data byte has been responded with a NACK. NOTE: If the remote I2C master terminates the transfer with a STOP condition before the controller has a chance to NACK a transfer, and IC_ENABLE has been set to 0, then this bit is also set to 1. When read as 0, the controller is deemed to have been disabled without being actively involved in the data phase of a Slave-Receiver transfer.\nNOTE: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.\n1 = Slave RX Data is lost\n0 = Slave RX Data is not lost"]
    #[inline(always)]
    pub fn slv_rx_data_lost(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C2EnableStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C2EnableStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Slave Disabled While Busy (Transmit, Receive). This bit indicates if a potential or active Slave operation has been aborted due to the setting of the IC_ENABLE register from 1 to 0. This bit is set when the CPU writes a 0 to the IC_ENABLE register while:\n(a) I2C Ctrl is receiving the address byte of the Slave-Transmitter operation from a remote master; OR,\n(b) address and data bytes of the Slave-Receiver operation from a remote master. When read as 1, the controller is deemed to have forced a NACK during any part of an I2C transfer, irrespective of whether the I2C address matches the slave address set in I2C Ctrl (IC_SAR register) OR if the transfer is completed before IC_ENABLE is set to 0 but has not taken effect.\nNOTE: If the remote I2C master terminates the transfer with a STOP condition before the the controller has a chance to NACK a transfer, and IC_ENABLE has been set to 0, then this bit will also be set to 1.\nWhen read as 0, the controller is deemed to have been disabled when there is master activity, or when the I2C bus is idle.\nNOTE: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.\n1 =Slave is disabled when it is active\n0 =Slave is disabled when it is idle"]
    #[inline(always)]
    pub fn slv_disabled_while_busy(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2EnableStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2EnableStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "ic_en Status. This bit always reflects the value driven on the output port ic_en. When read as 1, the controller is deemed to be in an enabled state.\nWhen read as 0, the controller is deemed completely inactive.\nNOTE: The CPU can safely read this bit anytime. When this bit is read as 0, the CPU can safely read SLV_RX_DATA_LOST (bit 2) and SLV_DISABLED_WHILE_BUSY (bit 1).\n1 = I2C enabled\n0 =I2C disabled"]
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
    type DataType = u32;
}

#[doc = "Fast Speed I2C Clock SCL High Count Register"]
pub type I2C2FsSclHcntReg = crate::RegValueT<I2C2FsSclHcntReg_SPEC>;

impl I2C2FsSclHcntReg {
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for fast speed. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. This register can be written only when the I2C interface is disabled, which corresponds to the I2C_ENABLE register being set to 0. Writes at other times have no effect.\nThe minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set."]
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
        <crate::RegValueT<I2C2FsSclHcntReg_SPEC> as RegisterValue<_>>::new(26)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2FsSclLcntReg_SPEC;
impl crate::sealed::RegSpec for I2C2FsSclLcntReg_SPEC {
    type DataType = u32;
}

#[doc = "Fast Speed I2C Clock SCL Low Count Register"]
pub type I2C2FsSclLcntReg = crate::RegValueT<I2C2FsSclLcntReg_SPEC>;

impl I2C2FsSclLcntReg {
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low-period count for fast speed. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. This register can be written only when the I2C interface is disabled, which corresponds to the I2C_ENABLE register being set to 0. Writes at other times have no effect.\nThe minimum valid value is 8; hardware prevents values less than this being written, and if attempted results in 8 being set. For designs with APB_DATA_WIDTH = 8 the order of programming is important to ensure the correct operation of the controller. The lower byte must be programmed first. Then the upper byte is programmed."]
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
        <crate::RegValueT<I2C2FsSclLcntReg_SPEC> as RegisterValue<_>>::new(50)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2HsMaddrReg_SPEC;
impl crate::sealed::RegSpec for I2C2HsMaddrReg_SPEC {
    type DataType = u32;
}

#[doc = "I2C High Speed Master Mode Code Address Register"]
pub type I2C2HsMaddrReg = crate::RegValueT<I2C2HsMaddrReg_SPEC>;

impl I2C2HsMaddrReg {
    #[doc = "This bit field holds the value of the I2C HS mode master code. HS-mode master codes are reserved 8-bit codes (00001xxx) that are not used for slave addressing or other purposes. Each master has its unique master code; up to eight high-speed mode masters can be present on the same I2C bus system. Valid values are from 0 to 7. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\] register being set to 0. Writes at other times have no effect."]
    #[inline(always)]
    pub fn i2c_ic_hs_mar(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, I2C2HsMaddrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,I2C2HsMaddrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I2C2HsMaddrReg {
    #[inline(always)]
    fn default() -> I2C2HsMaddrReg {
        <crate::RegValueT<I2C2HsMaddrReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2HsSclHcntReg_SPEC;
impl crate::sealed::RegSpec for I2C2HsSclHcntReg_SPEC {
    type DataType = u32;
}

#[doc = "High Speed I2C Clock SCL High Count Register"]
pub type I2C2HsSclHcntReg = crate::RegValueT<I2C2HsSclHcntReg_SPEC>;

impl I2C2HsSclHcntReg {
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high period count for high speed.refer to \"IC_CLK Frequency Configuration\".\nThe SCL High time depends on the loading of the bus. For 100pF loading, the SCL High time is 60ns; for 400pF loading, the SCL High time is 120ns. This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE != high.\nThis register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\] register being set to 0. Writes at other times have no effect.\nThe minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed."]
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
        I2C2HsSclHcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C2HsSclHcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2HsSclHcntReg {
    #[inline(always)]
    fn default() -> I2C2HsSclHcntReg {
        <crate::RegValueT<I2C2HsSclHcntReg_SPEC> as RegisterValue<_>>::new(6)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2HsSclLcntReg_SPEC;
impl crate::sealed::RegSpec for I2C2HsSclLcntReg_SPEC {
    type DataType = u32;
}

#[doc = "High Speed I2C Clock SCL Low Count Register"]
pub type I2C2HsSclLcntReg = crate::RegValueT<I2C2HsSclLcntReg_SPEC>;

impl I2C2HsSclLcntReg {
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for high speed. For more information, refer to \"IC_CLK Frequency Configuration\".\nThe SCL low time depends on the loading of the bus. For 100pF loading, the SCL low time is 160ns; for 400pF loading, the SCL low time is 320ns. This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE != high.\nThis register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\] register being set to 0. Writes at other times have no effect.\nThe minimum valid value is 8; hardware prevents values less than this being written, and if attempted results in 8 being set. For designs with APB_DATA_WIDTH == 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed. If the value is less than 8 then the count value gets changed to 8."]
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
        I2C2HsSclLcntReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I2C2HsSclLcntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2HsSclLcntReg {
    #[inline(always)]
    fn default() -> I2C2HsSclLcntReg {
        <crate::RegValueT<I2C2HsSclLcntReg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2IcFsSpklenReg_SPEC;
impl crate::sealed::RegSpec for I2C2IcFsSpklenReg_SPEC {
    type DataType = u32;
}

#[doc = "I2C SS and FS spike suppression limit Size"]
pub type I2C2IcFsSpklenReg = crate::RegValueT<I2C2IcFsSpklenReg_SPEC>;

impl I2C2IcFsSpklenReg {
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set."]
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
pub struct I2C2IcHsSpklenReg_SPEC;
impl crate::sealed::RegSpec for I2C2IcHsSpklenReg_SPEC {
    type DataType = u32;
}

#[doc = "I2C HS spike suppression limit Size"]
pub type I2C2IcHsSpklenReg = crate::RegValueT<I2C2IcHsSpklenReg_SPEC>;

impl I2C2IcHsSpklenReg {
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\] register being set to 0. Writes at other times have no effect.\nThe minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set."]
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
        I2C2IcHsSpklenReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I2C2IcHsSpklenReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I2C2IcHsSpklenReg {
    #[inline(always)]
    fn default() -> I2C2IcHsSpklenReg {
        <crate::RegValueT<I2C2IcHsSpklenReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2IntrMaskReg_SPEC;
impl crate::sealed::RegSpec for I2C2IntrMaskReg_SPEC {
    type DataType = u32;
}

#[doc = "I2C Interrupt Mask Register"]
pub type I2C2IntrMaskReg = crate::RegValueT<I2C2IntrMaskReg_SPEC>;

impl I2C2IntrMaskReg {
    #[doc = "M_SCL_STUCK_AT_LOW Register field Reserved bits"]
    #[inline(always)]
    pub fn m_scl_stuck_at_low(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,I2C2IntrMaskReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_master_on_hold(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_restart_det(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_start_det(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_stop_det(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_activity(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_tx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_rx_full(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
    #[inline(always)]
    pub fn m_rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2IntrMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2IntrMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits mask their corresponding interrupt status bits in the I2C_INTR_STAT register."]
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
    type DataType = u32;
}

#[doc = "I2C Interrupt Status Register"]
pub type I2C2IntrStatReg = crate::RegValueT<I2C2IntrStatReg_SPEC>;

impl I2C2IntrStatReg {
    #[doc = "1 = R_SCL_STUCK_AT_LOW interrupt is active\n0 = R_SCL_STUCK_AT_LOW interrupt is inactive"]
    #[inline(always)]
    pub fn r_scl_stuck_at_low(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates whether master is holding the bus and TX FIFO is empty. Enabled only when I2C_DYNAMIC_TAR_UPDATE=1 and IC_EMPTYFIFO_HOLD_MASTER_EN=1."]
    #[inline(always)]
    pub fn r_master_on_hold(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates whether a RESTART condition has occurred on the I2C interface when DW_apb_i2c is operating in Slave mode and the slave is being addressed.\nEnabled only when IC_SLV_RESTART_DET_EN=1.\nNote: However, in high-speed mode or during a START BYTE transfer, the RESTART comes before the address field as per the I2C protocol. In this case, the slave is not the addressed slave when the RESTART is issued, therefore DW_apb_i2c does not generate the RESTART_DET interrupt."]
    #[inline(always)]
    pub fn r_restart_det(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Set only when a General Call address is received and it is acknowledged. It stays set until it is cleared either by disabling controller or when the CPU reads bit 0 of the I2C_CLR_GEN_CALL register. The controller stores the received data in the Rx buffer."]
    #[inline(always)]
    pub fn r_gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether controller is operating in slave or master mode."]
    #[inline(always)]
    pub fn r_start_det(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates whether a STOP condition has occurred on the I2C interface regardless of whether controller is operating in slave or master mode."]
    #[inline(always)]
    pub fn r_stop_det(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit captures I2C Ctrl activity and stays set until it is cleared. There are four ways to clear it:\n=> Disabling the I2C Ctrl\n=> Reading the IC_CLR_ACTIVITY register\n=> Reading the IC_CLR_INTR register\n=> System reset\nOnce this bit is set, it stays set unless one of the four methods is used to clear it. Even if the controller module is idle, this bit remains set until cleared, indicating that there was activity on the bus."]
    #[inline(always)]
    pub fn r_activity(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When the controller is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done."]
    #[inline(always)]
    pub fn r_rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit indicates if the controller, as an I2C transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a \"transmit abort\".\nWhen this bit is set to 1, the I2C_TX_ABRT_SOURCE register indicates the reason why the transmit abort takes places.\nNOTE: The controller flushes/resets/empties the TX FIFO whenever this bit is set. The TX FIFO remains in this flushed state until the register I2C_CLR_TX_ABRT is read. Once this read is performed, the TX FIFO is then ready to accept more data bytes from the APB interface."]
    #[inline(always)]
    pub fn r_tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit is set to 1 when the controller is acting as a slave and another I2C master is attempting to read data from the controller. The controller holds the I2C bus in a wait state (SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the I2C_DATA_CMD register. This bit is set to 0 just after the processor reads the I2C_CLR_RD_REQ register"]
    #[inline(always)]
    pub fn r_rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit is set to 1 when the transmit buffer is at or below the threshold value set in the I2C_TX_TL register. It is automatically cleared by hardware when the buffer level goes above the threshold. When the IC_ENABLE bit 0 is 0, the TX FIFO is flushed and held in reset. There the TX FIFO looks like it has no data within it, so this bit is set to 1, provided there is activity in the master or slave state machines. When there is no longer activity, then with ic_en=0, this bit is set to 0."]
    #[inline(always)]
    pub fn r_tx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Set during transmit if the transmit buffer is filled to 32 and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register. When the module is disabled, this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared"]
    #[inline(always)]
    pub fn r_tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Set when the receive buffer reaches or goes above the RX_TL threshold in the I2C_RX_TL register. It is automatically cleared by hardware when buffer level goes below the threshold. If the module is disabled (I2C_ENABLE\\[0\\]=0), the RX FIFO is flushed and held in reset; therefore the RX FIFO is not full. So this bit is cleared once the I2C_ENABLE bit 0 is programmed with a 0, regardless of the activity that continues."]
    #[inline(always)]
    pub fn r_rx_full(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Set if the receive buffer is completely filled to 32 and an additional byte is received from an external I2C device. The controller acknowledges this, but any data bytes received after the FIFO is full are lost. If the module is disabled (I2C_ENABLE\\[0\\]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared."]
    #[inline(always)]
    pub fn r_rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2IntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2IntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register. If the module is disabled (I2C_ENABLE\\[0\\]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared."]
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
    type DataType = u32;
}

#[doc = "I2C Raw Interrupt Status Register"]
pub type I2C2RawIntrStatReg = crate::RegValueT<I2C2RawIntrStatReg_SPEC>;

impl I2C2RawIntrStatReg {
    #[doc = "CL_STUCK_AT_LOW Register field Reserved bits"]
    #[inline(always)]
    pub fn scl_stuck_at_low(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "ndicates whether master is holding the bus and TX FIFO is empty. Enabled only when I2C_DYNAMIC_TAR_UPDATE=1 and IC_EMPTYFIFO_HOLD_MASTER_EN=1."]
    #[inline(always)]
    pub fn master_on_hold(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates whether a RESTART condition has occurred on the I2C interface when DW_apb_i2c is operating in Slave mode and the slave is being addressed.\nEnabled only when IC_SLV_RESTART_DET_EN=1.\nNote: However, in high-speed mode or during a START BYTE transfer, the RESTART comes before the address field as per the I2C protocol. In this case, the slave is not the addressed slave when the RESTART is issued, therefore DW_apb_i2c does not generate the RESTART_DET interrupt."]
    #[inline(always)]
    pub fn restart_det(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Set only when a General Call address is received and it is acknowledged. It stays set until it is cleared either by disabling controller or when the CPU reads bit 0 of the I2C_CLR_GEN_CALL register. I2C Ctrl stores the received data in the Rx buffer."]
    #[inline(always)]
    pub fn gen_call(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether controller is operating in slave or master mode."]
    #[inline(always)]
    pub fn start_det(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates whether a STOP condition has occurred on the I2C interface regardless of whether controller is operating in slave or master mode."]
    #[inline(always)]
    pub fn stop_det(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit captures I2C Ctrl activity and stays set until it is cleared. There are four ways to clear it:\n=> Disabling the I2C Ctrl\n=> Reading the IC_CLR_ACTIVITY register\n=> Reading the IC_CLR_INTR register\n=> System reset\nOnce this bit is set, it stays set unless one of the four methods is used to clear it. Even if the controller module is idle, this bit remains set until cleared, indicating that there was activity on the bus."]
    #[inline(always)]
    pub fn activity(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When the controller is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done."]
    #[inline(always)]
    pub fn rx_done(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit indicates if the controller, as an I2C transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a \"transmit abort\".\nWhen this bit is set to 1, the I2C_TX_ABRT_SOURCE register indicates the reason why the transmit abort takes places.\nNOTE: The controller flushes/resets/empties the TX FIFO whenever this bit is set. The TX FIFO remains in this flushed state until the register I2C_CLR_TX_ABRT is read. Once this read is performed, the TX FIFO is then ready to accept more data bytes from the APB interface."]
    #[inline(always)]
    pub fn tx_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit is set to 1 when I2C Ctrl is acting as a slave and another I2C master is attempting to read data from the controller. The controller holds the I2C bus in a wait state (SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the I2C_DATA_CMD register. This bit is set to 0 just after the processor reads the I2C_CLR_RD_REQ register"]
    #[inline(always)]
    pub fn rd_req(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit is set to 1 when the transmit buffer is at or below the threshold value set in the I2C_TX_TL register. It is automatically cleared by hardware when the buffer level goes above the threshold. When the IC_ENABLE bit 0 is 0, the TX FIFO is flushed and held in reset. There the TX FIFO looks like it has no data within it, so this bit is set to 1, provided there is activity in the master or slave state machines. When there is no longer activity, then with ic_en=0, this bit is set to 0."]
    #[inline(always)]
    pub fn tx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Set during transmit if the transmit buffer is filled to 32 and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register. When the module is disabled, this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared"]
    #[inline(always)]
    pub fn tx_over(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Set when the receive buffer reaches or goes above the RX_TL threshold in the I2C_RX_TL register. It is automatically cleared by hardware when buffer level goes below the threshold. If the module is disabled (I2C_ENABLE\\[0\\]=0), the RX FIFO is flushed and held in reset; therefore the RX FIFO is not full. So this bit is cleared once the I2C_ENABLE bit 0 is programmed with a 0, regardless of the activity that continues."]
    #[inline(always)]
    pub fn rx_full(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Set if the receive buffer is completely filled to 32 and an additional byte is received from an external I2C device. The controller acknowledges this, but any data bytes received after the FIFO is full are lost. If the module is disabled (I2C_ENABLE\\[0\\]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared."]
    #[inline(always)]
    pub fn rx_over(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2RawIntrStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2RawIntrStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register. If the module is disabled (I2C_ENABLE\\[0\\]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared."]
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
    type DataType = u32;
}

#[doc = "I2C Receive FIFO Level Register"]
pub type I2C2RxflrReg = crate::RegValueT<I2C2RxflrReg_SPEC>;

impl I2C2RxflrReg {
    #[doc = "Receive FIFO Level. Contains the number of valid data entries in the receive FIFO. Size is constrained by the RXFLR value"]
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
    type DataType = u32;
}

#[doc = "I2C Receive FIFO Threshold Register"]
pub type I2C2RxTlReg = crate::RegValueT<I2C2RxTlReg_SPEC>;

impl I2C2RxTlReg {
    #[doc = "Receive FIFO Threshold Level Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in I2C_RAW_INTR_STAT register). The valid range is 0-31, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 31 sets the threshold for 32 entries."]
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
    type DataType = u32;
}

#[doc = "I2C Slave Address Register"]
pub type I2C2SarReg = crate::RegValueT<I2C2SarReg_SPEC>;

impl I2C2SarReg {
    #[doc = "The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR\\[6:0\\] is used. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect.\nWrites to this register succeed only when IC_ENABLE\\[0\\] is set to 0."]
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
    type DataType = u32;
}

#[doc = "I2C SDA Hold Time Length Register"]
pub type I2C2SdaHoldReg = crate::RegValueT<I2C2SdaHoldReg_SPEC>;

impl I2C2SdaHoldReg {
    #[doc = "Sets the required SDA hold time in units of ic_clk period, when receiver."]
    #[inline(always)]
    pub fn i2c_sda_rx_hold(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, I2C2SdaHoldReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,I2C2SdaHoldReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets the required SDA hold time in units of ic_clk period, when transmitter."]
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
    type DataType = u32;
}

#[doc = "I2C SDA Setup Register"]
pub type I2C2SdaSetupReg = crate::RegValueT<I2C2SdaSetupReg_SPEC>;

impl I2C2SdaSetupReg {
    #[doc = "SDA Setup.\nThis register controls the amount of time delay (number of I2C clock periods) between the rising edge of SCL and SDA changing by holding SCL low when I2C block services a read request while operating as a slave-transmitter. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2.\nIt is recommended that if the required delay is 1000ns, then for an I2C frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11.Writes to this register succeed only when IC_ENABLE\\[0\\] = 0."]
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
    type DataType = u32;
}

#[doc = "Standard Speed I2C Clock SCL High Count Register"]
pub type I2C2SsSclHcntReg = crate::RegValueT<I2C2SsSclHcntReg_SPEC>;

impl I2C2SsSclHcntReg {
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other\ntimes have no effect.\nThe minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set.\nNOTE: This register must not be programmed to a value higher than 65525, because the controller uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10."]
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
        <crate::RegValueT<I2C2SsSclHcntReg_SPEC> as RegisterValue<_>>::new(145)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2SsSclLcntReg_SPEC;
impl crate::sealed::RegSpec for I2C2SsSclLcntReg_SPEC {
    type DataType = u32;
}

#[doc = "Standard Speed I2C Clock SCL Low Count Register"]
pub type I2C2SsSclLcntReg = crate::RegValueT<I2C2SsSclLcntReg_SPEC>;

impl I2C2SsSclLcntReg {
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed.\nThis register can be written only when the I2C interface is disabled which corresponds to the I2C_ENABLE register being set to 0. Writes at other times have no effect.\nThe minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set."]
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
        <crate::RegValueT<I2C2SsSclLcntReg_SPEC> as RegisterValue<_>>::new(171)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C2StatusReg_SPEC;
impl crate::sealed::RegSpec for I2C2StatusReg_SPEC {
    type DataType = u32;
}

#[doc = "I2C Status Register"]
pub type I2C2StatusReg = crate::RegValueT<I2C2StatusReg_SPEC>;

impl I2C2StatusReg {
    #[doc = "This bit indicates the BUS Hold in Slave mode due to Rx FIFO is Full and an additional byte has been received\n1 = Slave holds the bus due to Rx FIFO is full\n0 = Slave is not holding the bus or Bus hold is not due to Rx FIFO is full"]
    #[inline(always)]
    pub fn lv_hold_rx_fifo_full(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit indicates the BUS Hold in Slave mode for the Read request when the Tx FIFO is empty. The Bus is in hold until the Tx FIFO has data to Transmit for the read request.\n1 = Slave holds the bus due to Tx FIFO is empty\n0 = Slave is not holding the bus or Bus hold is not due to Tx FIFO is empty"]
    #[inline(always)]
    pub fn slv_hold_tx_fifo_empty(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit indicates the BUS Hold in Master mode due to Rx FIFO is Full and additional byte has been received\n1 = Master holds the bus due to Rx FIFO is full\n0 = Master is not holding the bus or Bus hold is not due to Rx FIFO is full"]
    #[inline(always)]
    pub fn mst_hold_rx_fifo_full(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "the DW_apb_i2c master stalls the write transfer when Tx FIFO is empty, and the the last byte does not have the Stop bit set. This bit indicates the BUS hold when the master holds the bus because of the Tx FIFO being empty, and the the previous transferred command does not have the Stop bit set.\n1 =Master holds the bus due to Tx FIFO is empty\n0 =Master is not holding the bus or Bus hold is not due to Tx FIFO is empty"]
    #[inline(always)]
    pub fn mst_hold_tx_fifo_empty(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not in the IDLE state, this bit is set.\n0 = Slave FSM is in IDLE state so the Slave part of the controller is not Active\n1 = Slave FSM is not in IDLE state so the Slave part of the controller is Active"]
    #[inline(always)]
    pub fn slv_activity(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Master FSM Activity Status. When the Master Finite State Machine (FSM) is not in the IDLE state, this bit is set.\n0 = Master FSM is in IDLE state so the Master part of the controller is not Active\n1 = Master FSM is not in IDLE state so the Master part of the controller is Active"]
    #[inline(always)]
    pub fn mst_activity(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared.\n0 = Receive FIFO is not full\n1 = Receive FIFO is full"]
    #[inline(always)]
    pub fn rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty.\n0 = Receive FIFO is empty\n1 = Receive FIFO is not empty"]
    #[inline(always)]
    pub fn rfne(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt.\n0 = Transmit FIFO is not empty\n1 = Transmit FIFO is empty"]
    #[inline(always)]
    pub fn tfe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full.\n0 = Transmit FIFO is full\n1 = Transmit FIFO is not full"]
    #[inline(always)]
    pub fn tfnf(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "I2C Activity Status."]
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
    type DataType = u32;
}

#[doc = "I2C Target Address Register"]
pub type I2C2TarReg = crate::RegValueT<I2C2TarReg_SPEC>;

impl I2C2TarReg {
    #[doc = "On read\nThis bit indicates whether software performs a General Call or START BYTE command.\n0 = ignore bit 10 GC_OR_START and use IC_TAR normally\n1 = perform special I2C command as specified in GC_OR_START\nbit\nOn write\n1 = Enables programming of GENERAL_CALL or START_BYTE transmission\n0 = Disables programming of GENERAL_CALL or START_BYTE transmission\nWrites to this register succeed only when IC_ENABLE\\[0\\] is set to 0."]
    #[inline(always)]
    pub fn special(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C2TarReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,I2C2TarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "On read\nIf bit 11 (SPECIAL) is set to 1, then this bit indicates whether a General Call or START byte command is to be performed by the controller.\n0 = General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The controller remains in General Call mode until the SPECIAL bit value (bit 11) is cleared.\n1 = START BYTE\nOn write\n1 = START byte transmission\n0 = GENERAL_CALL byte transmission\nWrites to this register succeed only when IC_ENABLE\\[0\\] is set to 0."]
    #[inline(always)]
    pub fn gc_or_start(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C2TarReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,I2C2TarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits.\nNote: If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave\nWrites to this register succeed only when IC_ENABLE\\[0\\] is set to 0."]
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
    type DataType = u32;
}

#[doc = "I2C Transmit FIFO Level Register"]
pub type I2C2TxflrReg = crate::RegValueT<I2C2TxflrReg_SPEC>;

impl I2C2TxflrReg {
    #[doc = "Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO. Size is constrained by the TXFLR value"]
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
    type DataType = u32;
}

#[doc = "I2C Transmit Abort Source Register"]
pub type I2C2TxAbrtSourceReg = crate::RegValueT<I2C2TxAbrtSourceReg_SPEC>;

impl I2C2TxAbrtSourceReg {
    #[doc = "Master-Transmitter : This is a master-mode-only bit. Master has detected the transfer abort (IC_ENABLE\\[1\\])"]
    #[inline(always)]
    pub fn abrt_user_abrt(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<16,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Slave-Transmitter : When the processor side responds to a slave mode request for data to be transmitted to a remote master and user writes a 1 in CMD (bit 8) of 2IC_DATA_CMD register\n1 = Slave trying to transmit to remote master in read mode\n0 = Slave trying to transmit to remote master in read mode- scenario not present"]
    #[inline(always)]
    pub fn abrt_slvrd_intx(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<15,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Slave-Transmitter : Slave lost the bus while transmitting data to a remote master. I2C_TX_ABRT_SOURCE\\[12\\] is set at the same time. Note: Even though the slave never \"owns\" the bus, something could go wrong on the bus. This is a fail safe check. For instance, during a data transmission at the low-to-high transition of SCL, if what is on the data bus is not what is supposed to be transmitted, then the controller no longer own the bus.\n1 = Slave lost arbitration to remote master\n0 = Slave lost arbitration to remote master- scenario not present"]
    #[inline(always)]
    pub fn abrt_slv_arblost(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<14,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Slave-Transmitter : Slave has received a read command and some data exists in the TX FIFO so the slave issues a TX_ABRT interrupt to flush old data in TX FIFO.\n1 = Slave flushes existing data in TX-FIFO upon getting read command\n0 = Slave flushes existing data in TX-FIFO upon getting read command- scenario not present"]
    #[inline(always)]
    pub fn abrt_slvflush_txfifo(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<13,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Master-Transmitter or Slave-Transmitter : Master has lost arbitration, or if I2C_TX_ABRT_SOURCE\\[14\\] is also set, then the slave transmitter has lost arbitration. Note: I2C can be both master and slave at the same time.\n1 = Master or Slave-Transmitter lost arbitration\n0 = Master or Slave-Transmitter lost arbitration- scenario not present"]
    #[inline(always)]
    pub fn arb_lost(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<12,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Master-Transmitter or Master-Receiver : User tries to initiate a Master operation with the Master mode disabled.\n1 = User intitating master operation when MASTER disable\n0 = User initiating master operation when MASTER disabled- scenario not present"]
    #[inline(always)]
    pub fn abrt_master_dis(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<11,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Master-Receiver : The restart is disabled (IC_RESTART_EN bit (I2C_CON\\[5\\]) = 0) and the master sends a read command in 10-bit addressing mode.\n1 =Master trying to read in 10Bit addressing mode when RESTART disabled\n0 =Master not trying to read in 10Bit addressing mode when RESTART disabled"]
    #[inline(always)]
    pub fn abrt_10b_rd_norstrt(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Master : To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; restart must be enabled (I2C_CON\\[5\\]=1), the SPECIAL bit must be cleared (I2C_TAR\\[11\\]), or the GC_OR_START bit must be cleared (I2C_TAR\\[10\\]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, bit 9 clears for one cycle and then gets re-asserted. 1: The restart is disabled (IC_RESTART_EN bit (I2C_CON\\[5\\]) = 0) and the user is trying to send a START Byte. \n1 = User trying to send START byte when RESTART disabled\n0 = User trying to send START byte when RESTART disabled- scenario not present"]
    #[inline(always)]
    pub fn abrt_sbyte_norstrt(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Master-Transmitter or Master-Receiver : The restart is disabled (IC_RESTART_EN bit (I2C_CON\\[5\\]) = 0) and the user is trying to use the master to transfer data in High Speed mode\n1 = User trying to switch Master to HS mode when RESTART disabled\n0 = User trying to switch Master to HS mode when RESTART disabled- scenario not present"]
    #[inline(always)]
    pub fn abrt_hs_norstrt(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Master : Master has sent a START Byte and the START Byte was acknowledged (wrong behavior). \n1 = ACK detected for START byte\n0 = ACK detected for START byte- scenario not present"]
    #[inline(always)]
    pub fn abrt_sbyte_ackdet(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Master : Master is in High Speed mode and the High Speed Master code was acknowledged (wrong behavior).\n1 = HS Master code ACKed in HS Mode\n0 = HS Master code ACKed in HS Mode- scenario not present"]
    #[inline(always)]
    pub fn abrt_hs_ackdet(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Master-Transmitter : The controller in master mode sent a General Call but the user programmed the byte following the General Call to be a read from the bus (IC_DATA_CMD\\[9\\] is set to 1).\n1 = GCALL is followed by read from bus\n0 = GCALL is followed by read from bus-scenario not present"]
    #[inline(always)]
    pub fn abrt_gcall_read(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Master-Transmitter : the controller in master mode sent a General Call and no slave on the bus acknowledged the General Call.\n1 = GCALL not ACKed by any slave\n0 = GCALL not ACKed by any slave-scenario not present"]
    #[inline(always)]
    pub fn abrt_gcall_noack(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Master-Transmitter : This is a master-mode only bit. Master has received an acknowledgement for the address, but when it sent data byte(s) following the address, it did not receive an acknowledge from the remote slave(s).\n1 = Transmitted data not ACKed by addressed slave\n0 = Transmitted data non-ACKed by addressed slave-scenario not present"]
    #[inline(always)]
    pub fn abrt_txdata_noack(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Master-Transmitter or Master-Receiver : Master is in 10-bit address mode and the second address byte of the 10-bit address was not acknowledged by any slave.\n1= Byte 2 of 10Bit Address not ACKed by any slave\n0 = This abort is not generated"]
    #[inline(always)]
    pub fn abrt_10addr2_noack(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Master-Transmitter or Master-Receiver : Master is in 10-bit address mode and the first 10-bit address byte was not acknowledged by any slave.\n1 =Byte 1 of 10Bit Address not ACKed by any slave\n0 =This abort is not generated"]
    #[inline(always)]
    pub fn abrt_10addr1_noack(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I2C2TxAbrtSourceReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I2C2TxAbrtSourceReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Master-Transmitter or Master-Receiver : Master is in 7-bit addressing mode and the address sent was not acknowledged by any slave.\n1 =This abort is generated because of NOACK for 7-bit address\n0 =This abort is not generated"]
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
    type DataType = u32;
}

#[doc = "I2C Transmit FIFO Threshold Register"]
pub type I2C2TxTlReg = crate::RegValueT<I2C2TxTlReg_SPEC>;

impl I2C2TxTlReg {
    #[doc = "Transmit FIFO Threshold Level Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in I2C_RAW_INTR_STAT register). The valid range is 0-31, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 31 sets the threshold for 32 entries.."]
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
