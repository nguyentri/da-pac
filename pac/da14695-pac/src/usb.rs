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
#[doc = r"USB registers"]
unsafe impl ::core::marker::Send for super::Usb {}
unsafe impl ::core::marker::Sync for super::Usb {}
impl super::Usb {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Alternate Event Register"]
    #[inline(always)]
    pub const fn usb_altev_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbAltevReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbAltevReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Alternate Mask Register"]
    #[inline(always)]
    pub const fn usb_altmsk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbAltmskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbAltmskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "USB Charger Control Register"]
    #[inline(always)]
    pub const fn usb_charger_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbChargerCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbChargerCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(424usize),
            )
        }
    }

    #[doc = "USB Charger Status Register"]
    #[inline(always)]
    pub const fn usb_charger_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbChargerStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbChargerStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(428usize),
            )
        }
    }

    #[doc = "USB DMA control register"]
    #[inline(always)]
    pub const fn usb_dma_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbDmaCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbDmaCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(416usize),
            )
        }
    }

    #[doc = "EP0 INNAK and OUTNAK Register"]
    #[inline(always)]
    pub const fn usb_ep0_nak_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbEp0NakReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbEp0NakReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Endpoint Control 0 Register"]
    #[inline(always)]
    pub const fn usb_epc0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbEpc0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbEpc0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Endpoint Control Register 1"]
    #[inline(always)]
    pub const fn usb_epc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbEpc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbEpc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "Endpoint Control Register 2"]
    #[inline(always)]
    pub const fn usb_epc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbEpc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbEpc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[doc = "Endpoint Control Register 3"]
    #[inline(always)]
    pub const fn usb_epc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbEpc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbEpc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "Endpoint Control Register 4"]
    #[inline(always)]
    pub const fn usb_epc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbEpc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbEpc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "Endpoint Control Register 5"]
    #[inline(always)]
    pub const fn usb_epc5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbEpc5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbEpc5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[doc = "Endpoint Control Register 6"]
    #[inline(always)]
    pub const fn usb_epc6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbEpc6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbEpc6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[doc = "Function Address Register"]
    #[inline(always)]
    pub const fn usb_far_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbFarReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbFarReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Frame Number High Byte Register"]
    #[inline(always)]
    pub const fn usb_fnh_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbFnhReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbFnhReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "Frame Number Low Byte Register"]
    #[inline(always)]
    pub const fn usb_fnl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbFnlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbFnlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "FIFO Warning Event Register"]
    #[inline(always)]
    pub const fn usb_fwev_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbFwevReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbFwevReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "FIFO Warning Mask Register"]
    #[inline(always)]
    pub const fn usb_fwmsk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbFwmskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbFwmskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Main Event Register"]
    #[inline(always)]
    pub const fn usb_maev_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbMaevReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbMaevReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Main Mask Register"]
    #[inline(always)]
    pub const fn usb_mamsk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbMamskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbMamskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Main Control Register)"]
    #[inline(always)]
    pub const fn usb_mctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbMctrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbMctrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "NAK Event Register"]
    #[inline(always)]
    pub const fn usb_nakev_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbNakevReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbNakevReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "NAK Mask Register"]
    #[inline(always)]
    pub const fn usb_nakmsk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbNakmskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbNakmskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Node Functional State Register"]
    #[inline(always)]
    pub const fn usb_nfsr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbNfsrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbNfsrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Receive Command 0 Register"]
    #[inline(always)]
    pub const fn usb_rxc0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbRxc0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbRxc0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[doc = "Receive Command Register 1"]
    #[inline(always)]
    pub const fn usb_rxc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbRxc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbRxc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(188usize),
            )
        }
    }

    #[doc = "Receive Command Register 2"]
    #[inline(always)]
    pub const fn usb_rxc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbRxc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbRxc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(220usize),
            )
        }
    }

    #[doc = "Receive Command Register 3"]
    #[inline(always)]
    pub const fn usb_rxc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbRxc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbRxc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(252usize),
            )
        }
    }

    #[doc = "Receive Data 0 Register"]
    #[inline(always)]
    pub const fn usb_rxd0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbRxd0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbRxd0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "Receive Data Register,1"]
    #[inline(always)]
    pub const fn usb_rxd1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbRxd1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbRxd1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(180usize),
            )
        }
    }

    #[doc = "Receive Data Register 2"]
    #[inline(always)]
    pub const fn usb_rxd2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbRxd2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbRxd2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[doc = "Receive Data Register 3"]
    #[inline(always)]
    pub const fn usb_rxd3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbRxd3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbRxd3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[doc = "Receive Event Register"]
    #[inline(always)]
    pub const fn usb_rxev_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbRxevReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbRxevReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Receive Mask Register"]
    #[inline(always)]
    pub const fn usb_rxmsk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbRxmskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbRxmskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Receive Status 0 Register"]
    #[inline(always)]
    pub const fn usb_rxs0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbRxs0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbRxs0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "Receive Status Register 1"]
    #[inline(always)]
    pub const fn usb_rxs1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbRxs1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbRxs1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[doc = "Receive Status Register 2"]
    #[inline(always)]
    pub const fn usb_rxs2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbRxs2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbRxs2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }

    #[doc = "Receive Status Register 3"]
    #[inline(always)]
    pub const fn usb_rxs3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbRxs3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbRxs3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }

    #[doc = "Transceiver configuration Register"]
    #[inline(always)]
    pub const fn usb_tcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Transmit command 0 Register"]
    #[inline(always)]
    pub const fn usb_txc0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTxc0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTxc0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "Transmit Command Register 1"]
    #[inline(always)]
    pub const fn usb_txc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTxc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTxc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[doc = "Transmit Command Register 2"]
    #[inline(always)]
    pub const fn usb_txc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTxc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTxc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[doc = "Transmit Command Register 3"]
    #[inline(always)]
    pub const fn usb_txc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTxc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTxc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(236usize),
            )
        }
    }

    #[doc = "Transmit Data 0 Register"]
    #[inline(always)]
    pub const fn usb_txd0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTxd0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTxd0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "Transmit Data Register 1"]
    #[inline(always)]
    pub const fn usb_txd1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTxd1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTxd1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "Transmit Data Register 2"]
    #[inline(always)]
    pub const fn usb_txd2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTxd2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTxd2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[doc = "Transmit Data Register 3"]
    #[inline(always)]
    pub const fn usb_txd3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTxd3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTxd3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[doc = "Transmit Event Register"]
    #[inline(always)]
    pub const fn usb_txev_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTxevReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTxevReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Transmit Mask Register"]
    #[inline(always)]
    pub const fn usb_txmsk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTxmskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTxmskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Transmit Status 0 Register"]
    #[inline(always)]
    pub const fn usb_txs0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTxs0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTxs0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "Transmit Status Register 1"]
    #[inline(always)]
    pub const fn usb_txs1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTxs1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTxs1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[doc = "Transmit Status Register 2"]
    #[inline(always)]
    pub const fn usb_txs2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTxs2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTxs2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[doc = "Transmit Status Register 3"]
    #[inline(always)]
    pub const fn usb_txs3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbTxs3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbTxs3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[doc = "USB test Register (for test purpose only)"]
    #[inline(always)]
    pub const fn usb_utr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbUtrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbUtrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Transceiver 2.0 Configuration and Diagnostics Register(for test purpose only)"]
    #[inline(always)]
    pub const fn usb_ux20cdr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbUx20CdrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbUx20CdrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = "Transceiver diagnostic Register (for test purpose only)"]
    #[inline(always)]
    pub const fn usb_xcvdiag_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbXcvdiagReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbXcvdiagReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbAltevReg_SPEC;
impl crate::sealed::RegSpec for UsbAltevReg_SPEC {
    type DataType = u32;
}

#[doc = "Alternate Event Register"]
pub type UsbAltevReg = crate::RegValueT<UsbAltevReg_SPEC>;

impl UsbAltevReg {
    #[doc = "Resume\nResume signalling is detected on the USB when the device is in Suspend state (NFS in the NFSR register is set to SUSPEND), and a non IDLE signal is present on the USB, indicating that this device should begin it\'s wake-up sequence and enter Operational state. This bit is cleared when the register is read."]
    #[inline(always)]
    pub fn usb_resume(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbAltevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbAltevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Reset\nThis bit is set to 1, when 2.5 us of SEO have been detected on the upstream port. In response, the functional state should be reset (NFS in the NFSR register is set to RESET), where it must remain for at least 100 us. The functional state can then return to Operational state. This bit is cleared when the register is read"]
    #[inline(always)]
    pub fn usb_reset(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbAltevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbAltevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Suspend Detect 5 ms\nThis bit is set to 1 after 5 ms of IDLE have been detected on the upstream port, indicating that this device is permitted to perform a remote wake-up operation. The resume may be initiated under firmware control by writing the resume value to the NFSR register. This bit is cleared when the register is read."]
    #[inline(always)]
    pub fn usb_sd5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbAltevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbAltevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Suspend Detect 3 ms\nThis bit is set to 1 after 3 ms of IDLE have been detected on the upstream port, indicating that the device should be suspended. The suspend occurs under firmware control by writing the suspend value to the Node Functional State (NFSR) register. This bit is cleared when the register is read."]
    #[inline(always)]
    pub fn usb_sd3(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbAltevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbAltevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "End of Packet\nA valid EOP sequence was been detected on the USB. It is used when this device has initiated a Remote wake-up sequence to indicate that the Resume sequence has been acknowledged and completed by the host. This bit is cleared when the register is read."]
    #[inline(always)]
    pub fn usb_eop(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbAltevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbAltevReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbAltevReg {
    #[inline(always)]
    fn default() -> UsbAltevReg {
        <crate::RegValueT<UsbAltevReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbAltmskReg_SPEC;
impl crate::sealed::RegSpec for UsbAltmskReg_SPEC {
    type DataType = u32;
}

#[doc = "Alternate Mask Register"]
pub type UsbAltmskReg = crate::RegValueT<UsbAltmskReg_SPEC>;

impl UsbAltmskReg {
    #[doc = "A bit set to 1 in this register enables automatic setting of the ALT bit in the MAEV register when the respective event in the ALTEV register occurs. Otherwise, setting MAEV.ALT bit is disabled.\nSame Bit Definition as ALTEV Register"]
    #[inline(always)]
    pub fn usb_m_resume(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbAltmskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbAltmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as ALTEV Register"]
    #[inline(always)]
    pub fn usb_m_reset(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbAltmskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbAltmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as ALTEV Register"]
    #[inline(always)]
    pub fn usb_m_sd5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbAltmskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbAltmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as ALTEV Register"]
    #[inline(always)]
    pub fn usb_m_sd3(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbAltmskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbAltmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as ALTEV Register"]
    #[inline(always)]
    pub fn usb_m_eop(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbAltmskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbAltmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbAltmskReg {
    #[inline(always)]
    fn default() -> UsbAltmskReg {
        <crate::RegValueT<UsbAltmskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbChargerCtrlReg_SPEC;
impl crate::sealed::RegSpec for UsbChargerCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "USB Charger Control Register"]
pub type UsbChargerCtrlReg = crate::RegValueT<UsbChargerCtrlReg_SPEC>;

impl UsbChargerCtrlReg {
    #[doc = "0 = Disable\n1 = Enable the Idm_sink to USBm"]
    #[inline(always)]
    pub fn idm_sink_on(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Disable\n1 = Enable the Idp_sink to USBp"]
    #[inline(always)]
    pub fn idp_sink_on(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Disable\n1 = Enable Vdm_src to USBm and USB_DCP_DET status bit."]
    #[inline(always)]
    pub fn vdm_src_on(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Disable\n1 = Enable the Vdp_src to USB_CHG_DET status bit."]
    #[inline(always)]
    pub fn vdp_src_on(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Disable\n1 = Enable the Idp_src and Rdm_dwn."]
    #[inline(always)]
    pub fn idp_src_on(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Disable USB charger detect circuit.\n1 = Enable USB charger detect circuit."]
    #[inline(always)]
    pub fn usb_charge_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbChargerCtrlReg {
    #[inline(always)]
    fn default() -> UsbChargerCtrlReg {
        <crate::RegValueT<UsbChargerCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbChargerStatReg_SPEC;
impl crate::sealed::RegSpec for UsbChargerStatReg_SPEC {
    type DataType = u32;
}

#[doc = "USB Charger Status Register"]
pub type UsbChargerStatReg = crate::RegValueT<UsbChargerStatReg_SPEC>;

impl UsbChargerStatReg {
    #[doc = "0 = USBm <2.3V\n1 = USBm >2.5V"]
    #[inline(always)]
    pub fn usb_dm_val2(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbChargerStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbChargerStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: USBp < 2.3V\n1: USBp > 2.5V"]
    #[inline(always)]
    pub fn usb_dp_val2(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbChargerStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,UsbChargerStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = USBm < 0.8V\n1 = USBm > 1.5V (PS2 or Proprietary Charger)"]
    #[inline(always)]
    pub fn usb_dm_val(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbChargerStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,UsbChargerStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = USBp < 0.8V\n1 = USBp > 1.5V"]
    #[inline(always)]
    pub fn usb_dp_val(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbChargerStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,UsbChargerStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = Standard downstream or nothing connected.\n1 = Charging Downstream Port (CDP) or Dedicated Charging."]
    #[inline(always)]
    pub fn usb_chg_det(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbChargerStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,UsbChargerStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = Charging downstream port is detected.\n1 = Dedicated charger is detected.\nControl bit VDM_SRC_ON must be set to validate this status bit.\nNote: This register shows the actual status."]
    #[inline(always)]
    pub fn usb_dcp_det(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbChargerStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,UsbChargerStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbChargerStatReg {
    #[inline(always)]
    fn default() -> UsbChargerStatReg {
        <crate::RegValueT<UsbChargerStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbDmaCtrlReg_SPEC;
impl crate::sealed::RegSpec for UsbDmaCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "USB DMA control register"]
pub type UsbDmaCtrlReg = crate::RegValueT<UsbDmaCtrlReg_SPEC>;

impl UsbDmaCtrlReg {
    #[doc = "0 = USB DMA control off. (Normal operation)\n1 = USB_DMA on. DMA channels 0 and 1 are connected by\nUSB Endpoint according bits USB_DMA_TX and USB_DMA_RX"]
    #[inline(always)]
    pub fn usb_dma_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbDmaCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbDmaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "000 = DMA channels 1 is connected Tx USB Endpoint 1\n001 = DMA channels 1 is connected Tx USB Endpoint 3\n010 = DMA channels 1 is connected Tx USB Endpoint 5\n100, 1xx = Reserved"]
    #[inline(always)]
    pub fn usb_dma_tx(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, UsbDmaCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,UsbDmaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "000 = DMA channels 0 is connected Rx USB Endpoint 2\n001 = DMA channels 0 is connected Rx USB Endpoint 4\n010 = DMA channels 0 is connected Rx USB Endpoint 6\n100, 1xx = Reserved"]
    #[inline(always)]
    pub fn usb_dma_rx(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, UsbDmaCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,UsbDmaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbDmaCtrlReg {
    #[inline(always)]
    fn default() -> UsbDmaCtrlReg {
        <crate::RegValueT<UsbDmaCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbEp0NakReg_SPEC;
impl crate::sealed::RegSpec for UsbEp0NakReg_SPEC {
    type DataType = u32;
}

#[doc = "EP0 INNAK and OUTNAK Register"]
pub type UsbEp0NakReg = crate::RegValueT<UsbEp0NakReg_SPEC>;

impl UsbEp0NakReg {
    #[doc = "End point 0 OUT NAK\nThis bit n is set to 1 when a NAK handshake is generated for an enabled address/endpoint combination (AD_EN in the FAR register is set to 1) in response to an OUT token. This bit is not set if NAK is generated as result of an overrun condition. It is cleared when the register is read."]
    #[inline(always)]
    pub fn usb_ep0_outnak(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbEp0NakReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,UsbEp0NakReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "End point 0 IN NAK\nThis bit is set to 1 when a NAK handshake is generated for an enabled address/endpoint combination (AD_EN in the FAR register is set to 1) in response to an IN token. This bit is cleared when the register is read."]
    #[inline(always)]
    pub fn usb_ep0_innak(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbEp0NakReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,UsbEp0NakReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbEp0NakReg {
    #[inline(always)]
    fn default() -> UsbEp0NakReg {
        <crate::RegValueT<UsbEp0NakReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbEpc0Reg_SPEC;
impl crate::sealed::RegSpec for UsbEpc0Reg_SPEC {
    type DataType = u32;
}

#[doc = "Endpoint Control 0 Register"]
pub type UsbEpc0Reg = crate::RegValueT<UsbEpc0Reg_SPEC>;

impl UsbEpc0Reg {
    #[doc = "Stall\nSetting this bit to 1 causes the chip to generate STALL handshakes under the following conditions:\n- The transmit FIFO is enabled and an IN token is received.\n- The receive FIFO is enabled and an OUT token is received.\nNote: A SETUP token does not cause a STALL handshake to be generated when this bit is set.\nUpon transmitting the STALL handshake, the RX_LAST and the TX_DONE bits in the respective Receive/Transmit Status registers are set to 1."]
    #[inline(always)]
    pub fn usb_stall(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbEpc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbEpc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Default Address\nWhen set to 1, the device responds to the default address regardless of the contents of FAR6-0/EP03-0 fields. When an IN packet is transmitted for the endpoint, the DEF bit is automatically cleared to 0.\nThis bit aids in the transition from default address to assigned address. The transition from the default address 00000000000b to an address assigned during bus enumeration may not occur in the middle of the SET_ADDRESS control sequence. This is necessary to complete the control sequence. However, the address must change immediately after this sequence finishes in order to avoid errors when another control sequence immediately follows the SET_ADDRESS command.\nOn USB reset, the firmware has 10 ms for set-up, and should write 8016 to the FAR register and 0016 to the EPC0 register. On receipt of a SET_ADDRESS command, the firmware must write 4016 to the EPC0 register and (8016 or <assigned_function_address>) to the FAR register. It must then queue a zero length IN packet to complete the status phase of the SET_ADDRESS control sequence."]
    #[inline(always)]
    pub fn usb_def(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbEpc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbEpc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint Address\nThis field holds the 4-bit Endpoint address. For Endpoint 0, these bits are hardwired to 0000b. Writing a 1 to any of the EP bits is ignored."]
    #[inline(always)]
    pub fn usb_ep(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, UsbEpc0Reg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,UsbEpc0Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbEpc0Reg {
    #[inline(always)]
    fn default() -> UsbEpc0Reg {
        <crate::RegValueT<UsbEpc0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbEpc1Reg_SPEC;
impl crate::sealed::RegSpec for UsbEpc1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Endpoint Control Register 1"]
pub type UsbEpc1Reg = crate::RegValueT<UsbEpc1Reg_SPEC>;

impl UsbEpc1Reg {
    #[doc = "Stall\nSetting this bit to 1 causes the chip to generate STALL handshakes under the following conditions:\nThe transmit FIFO is enabled and an IN token is received.\nThe receive FIFO is enabled and an OUT token is received.\nSetting this bit to 1 does not generate a STALL handshake in response to a SETUP token"]
    #[inline(always)]
    pub fn usb_stall(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbEpc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbEpc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Isochronous\nWhen this bit is set to 1, the endpoint is isochronous. This implies that no NAK is sent if the endpoint is not ready but enabled; i.e. If an IN token is received and no data is available in the FIFO to transmit, or if an OUT token is received and the FIFO is full since there is no USB handshake for isochronous transfers."]
    #[inline(always)]
    pub fn usb_iso(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbEpc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbEpc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint Enable\nWhen this bit is set to 1, the EP\\[3:0\\] field is used in address comparison, together with the AD\\[6:0\\] field in the FAR register. When cleared to 0, the endpoint does not respond to any token on the USB bus."]
    #[inline(always)]
    pub fn usb_ep_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbEpc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbEpc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint Address\nThis 4-bit field holds the endpoint address."]
    #[inline(always)]
    pub fn usb_ep(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, UsbEpc1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,UsbEpc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbEpc1Reg {
    #[inline(always)]
    fn default() -> UsbEpc1Reg {
        <crate::RegValueT<UsbEpc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbEpc2Reg_SPEC;
impl crate::sealed::RegSpec for UsbEpc2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Endpoint Control Register 2"]
pub type UsbEpc2Reg = crate::RegValueT<UsbEpc2Reg_SPEC>;

impl UsbEpc2Reg {
    #[doc = "Stall\nSetting this bit to 1 causes the chip to generate STALL handshakes under the following conditions:\nThe transmit FIFO is enabled and an IN token is received.\nThe receive FIFO is enabled and an OUT token is received.\nSetting this bit to 1 does not generate a STALL handshake in response to a SETUP token"]
    #[inline(always)]
    pub fn usb_stall(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbEpc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbEpc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Isochronous\nWhen this bit is set to 1, the endpoint is isochronous. This implies that no NAK is sent if the endpoint is not ready but enabled; i.e. If an IN token is received and no data is available in the FIFO to transmit, or if an OUT token is received and the FIFO is full since there is no USB handshake for isochronous transfers."]
    #[inline(always)]
    pub fn usb_iso(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbEpc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbEpc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint Enable\nWhen this bit is set to 1, the EP\\[3:0\\] field is used in address comparison, together with the AD\\[6:0\\] field in the FAR register. When cleared to 0, the endpoint does not respond to any token on the USB bus."]
    #[inline(always)]
    pub fn usb_ep_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbEpc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbEpc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint Address\nThis 4-bit field holds the endpoint address."]
    #[inline(always)]
    pub fn usb_ep(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, UsbEpc2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,UsbEpc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbEpc2Reg {
    #[inline(always)]
    fn default() -> UsbEpc2Reg {
        <crate::RegValueT<UsbEpc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbEpc3Reg_SPEC;
impl crate::sealed::RegSpec for UsbEpc3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Endpoint Control Register 3"]
pub type UsbEpc3Reg = crate::RegValueT<UsbEpc3Reg_SPEC>;

impl UsbEpc3Reg {
    #[doc = "Stall\nSetting this bit to 1 causes the chip to generate STALL handshakes under the following conditions:\nThe transmit FIFO is enabled and an IN token is received.\nThe receive FIFO is enabled and an OUT token is received.\nSetting this bit to 1 does not generate a STALL handshake in response to a SETUP token"]
    #[inline(always)]
    pub fn usb_stall(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbEpc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbEpc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Isochronous\nWhen this bit is set to 1, the endpoint is isochronous. This implies that no NAK is sent if the endpoint is not ready but enabled; i.e. If an IN token is received and no data is available in the FIFO to transmit, or if an OUT token is received and the FIFO is full since there is no USB handshake for isochronous transfers."]
    #[inline(always)]
    pub fn usb_iso(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbEpc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbEpc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint Enable\nWhen this bit is set to 1, the EP\\[3:0\\] field is used in address comparison, together with the AD\\[6:0\\] field in the FAR register. When cleared to 0, the endpoint does not respond to any token on the USB bus."]
    #[inline(always)]
    pub fn usb_ep_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbEpc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbEpc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint Address\nThis 4-bit field holds the endpoint address."]
    #[inline(always)]
    pub fn usb_ep(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, UsbEpc3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,UsbEpc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbEpc3Reg {
    #[inline(always)]
    fn default() -> UsbEpc3Reg {
        <crate::RegValueT<UsbEpc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbEpc4Reg_SPEC;
impl crate::sealed::RegSpec for UsbEpc4Reg_SPEC {
    type DataType = u32;
}

#[doc = "Endpoint Control Register 4"]
pub type UsbEpc4Reg = crate::RegValueT<UsbEpc4Reg_SPEC>;

impl UsbEpc4Reg {
    #[doc = "Stall\nSetting this bit to 1 causes the chip to generate STALL handshakes under the following conditions:\nThe transmit FIFO is enabled and an IN token is received.\nThe receive FIFO is enabled and an OUT token is received.\nSetting this bit to 1 does not generate a STALL handshake in response to a SETUP token"]
    #[inline(always)]
    pub fn usb_stall(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbEpc4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbEpc4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Isochronous\nWhen this bit is set to 1, the endpoint is isochronous. This implies that no NAK is sent if the endpoint is not ready but enabled; i.e. If an IN token is received and no data is available in the FIFO to transmit, or if an OUT token is received and the FIFO is full since there is no USB handshake for isochronous transfers."]
    #[inline(always)]
    pub fn usb_iso(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbEpc4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbEpc4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint Enable\nWhen this bit is set to 1, the EP\\[3:0\\] field is used in address comparison, together with the AD\\[6:0\\] field in the FAR register. When cleared to 0, the endpoint does not respond to any token on the USB bus."]
    #[inline(always)]
    pub fn usb_ep_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbEpc4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbEpc4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint Address\nThis 4-bit field holds the endpoint address."]
    #[inline(always)]
    pub fn usb_ep(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, UsbEpc4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,UsbEpc4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbEpc4Reg {
    #[inline(always)]
    fn default() -> UsbEpc4Reg {
        <crate::RegValueT<UsbEpc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbEpc5Reg_SPEC;
impl crate::sealed::RegSpec for UsbEpc5Reg_SPEC {
    type DataType = u32;
}

#[doc = "Endpoint Control Register 5"]
pub type UsbEpc5Reg = crate::RegValueT<UsbEpc5Reg_SPEC>;

impl UsbEpc5Reg {
    #[doc = "Stall\nSetting this bit to 1 causes the chip to generate STALL handshakes under the following conditions:\nThe transmit FIFO is enabled and an IN token is received.\nThe receive FIFO is enabled and an OUT token is received.\nSetting this bit to 1 does not generate a STALL handshake in response to a SETUP token"]
    #[inline(always)]
    pub fn usb_stall(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbEpc5Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbEpc5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Isochronous\nWhen this bit is set to 1, the endpoint is isochronous. This implies that no NAK is sent if the endpoint is not ready but enabled; i.e. If an IN token is received and no data is available in the FIFO to transmit, or if an OUT token is received and the FIFO is full since there is no USB handshake for isochronous transfers."]
    #[inline(always)]
    pub fn usb_iso(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbEpc5Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbEpc5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint Enable\nWhen this bit is set to 1, the EP\\[3:0\\] field is used in address comparison, together with the AD\\[6:0\\] field in the FAR register. When cleared to 0, the endpoint does not respond to any token on the USB bus."]
    #[inline(always)]
    pub fn usb_ep_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbEpc5Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbEpc5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint Address\nThis 4-bit field holds the endpoint address."]
    #[inline(always)]
    pub fn usb_ep(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, UsbEpc5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,UsbEpc5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbEpc5Reg {
    #[inline(always)]
    fn default() -> UsbEpc5Reg {
        <crate::RegValueT<UsbEpc5Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbEpc6Reg_SPEC;
impl crate::sealed::RegSpec for UsbEpc6Reg_SPEC {
    type DataType = u32;
}

#[doc = "Endpoint Control Register 6"]
pub type UsbEpc6Reg = crate::RegValueT<UsbEpc6Reg_SPEC>;

impl UsbEpc6Reg {
    #[doc = "Stall\nSetting this bit to 1 causes the chip to generate STALL handshakes under the following conditions:\nThe transmit FIFO is enabled and an IN token is received.\nThe receive FIFO is enabled and an OUT token is received.\nSetting this bit to 1 does not generate a STALL handshake in response to a SETUP token"]
    #[inline(always)]
    pub fn usb_stall(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbEpc6Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbEpc6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Isochronous\nWhen this bit is set to 1, the endpoint is isochronous. This implies that no NAK is sent if the endpoint is not ready but enabled; i.e. If an IN token is received and no data is available in the FIFO to transmit, or if an OUT token is received and the FIFO is full since there is no USB handshake for isochronous transfers."]
    #[inline(always)]
    pub fn usb_iso(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbEpc6Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbEpc6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint Enable\nWhen this bit is set to 1, the EP\\[3:0\\] field is used in address comparison, together with the AD\\[6:0\\] field in the FAR register. When cleared to 0, the endpoint does not respond to any token on the USB bus."]
    #[inline(always)]
    pub fn usb_ep_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbEpc6Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbEpc6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint Address\nThis 4-bit field holds the endpoint address."]
    #[inline(always)]
    pub fn usb_ep(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, UsbEpc6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,UsbEpc6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbEpc6Reg {
    #[inline(always)]
    fn default() -> UsbEpc6Reg {
        <crate::RegValueT<UsbEpc6Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbFarReg_SPEC;
impl crate::sealed::RegSpec for UsbFarReg_SPEC {
    type DataType = u32;
}

#[doc = "Function Address Register"]
pub type UsbFarReg = crate::RegValueT<UsbFarReg_SPEC>;

impl UsbFarReg {
    #[doc = "Address Enable\nWhen set to 1, USB address field bits 6-0 are used in address comparison\nWhen cleared to 0, the device does not respond to any token on the USB bus.\nNote: If the DEF bit in the Endpoint Control 0 register is set, Endpoint 0 responds to the default address."]
    #[inline(always)]
    pub fn usb_ad_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbFarReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbFarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Address\nThis field holds the 7-bit function address used to transmit and receive all tokens addressed to this device."]
    #[inline(always)]
    pub fn usb_ad(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, UsbFarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,UsbFarReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbFarReg {
    #[inline(always)]
    fn default() -> UsbFarReg {
        <crate::RegValueT<UsbFarReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbFnhReg_SPEC;
impl crate::sealed::RegSpec for UsbFnhReg_SPEC {
    type DataType = u32;
}

#[doc = "Frame Number High Byte Register"]
pub type UsbFnhReg = crate::RegValueT<UsbFnhReg_SPEC>;

impl UsbFnhReg {
    #[doc = "Missed SOF Flag\nThis flag is set to 1, when the frame number in a valid received SOF does not match the expected next value, or when an SOF is not received within 12060 bit times. This bit is set by the hardware and is cleared by reading the FNH register."]
    #[inline(always)]
    pub fn usb_mf(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbFnhReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, UsbFnhReg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Unlock Flag\nThis bit indicates that at least two frames were received without an expected frame number, or that no valid SOF was received within 12060 bit times. If this bit is set, the frame number from the next valid SOF packet is loaded in FN. This bit is set by the hardware and is cleared by reading the FNH register."]
    #[inline(always)]
    pub fn usb_ul(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbFnhReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, UsbFnhReg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Reset Frame Count\nWriting a 1 to this bit resets the frame number to 00016, after which this bit clears itself to 0 again. This bit always reads 0."]
    #[inline(always)]
    pub fn usb_rfc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbFnhReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, UsbFnhReg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Number\nThis 3-bit field contains the three most significant bits (MSB) of the current frame number, received in the last SOF packet. If a valid frame number is not received within 12060 bit times (Frame Length Maximum, FLMAX, with tolerance) of the previous change, the frame number is incremented artificially. If two successive frames are missed or are incorrect, the current FN is frozen and loaded with the next frame number from a valid SOF packet.\nIf the frame number low byte was read by firmware before reading the FNH register, the user actually reads the contents of a buffer register which holds the value of the three frame number bits of this register when the low byte was read. Therefore, the correct sequence to read the frame number is: FNL, FNH. Read operations to the FNH register, without first reading the Frame Number Low Byte (FNL) register directly, read the actual value of the three MSBs of the frame number."]
    #[inline(always)]
    pub fn usb_fn_10_8(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, UsbFnhReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,UsbFnhReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbFnhReg {
    #[inline(always)]
    fn default() -> UsbFnhReg {
        <crate::RegValueT<UsbFnhReg_SPEC> as RegisterValue<_>>::new(192)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbFnlReg_SPEC;
impl crate::sealed::RegSpec for UsbFnlReg_SPEC {
    type DataType = u32;
}

#[doc = "Frame Number Low Byte Register"]
pub type UsbFnlReg = crate::RegValueT<UsbFnlReg_SPEC>;

impl UsbFnlReg {
    #[doc = "The Frame Number Low Byte Register holds the low byte of the frame number. To ensure consistency, reading this low byte causes the three frame number bits in the FNH register to be locked until this register is read. The correct sequence to read the frame number is: FNL, FNH."]
    #[inline(always)]
    pub fn usb_fn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UsbFnlReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,UsbFnlReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbFnlReg {
    #[inline(always)]
    fn default() -> UsbFnlReg {
        <crate::RegValueT<UsbFnlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbFwevReg_SPEC;
impl crate::sealed::RegSpec for UsbFwevReg_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Warning Event Register"]
pub type UsbFwevReg = crate::RegValueT<UsbFwevReg_SPEC>;

impl UsbFwevReg {
    #[doc = "Receive Warning n: 3:1\nThe bit n is set to 1 when the respective receive endpoint FIFO reaches the warning limit, as specified by the RFWL bits of the respective EPCx register. This bit is cleared when the warning condition is cleared by either reading data from the FIFO or when the FIFO is flushed."]
    #[inline(always)]
    pub fn usb_rxwarn31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbFwevReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbFwevReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmit Warning n: 3:1\nThe bit n is set to 1 when the respective transmit endpoint FIFO reaches the warning limit, as specified by the TFWL bits of the respective TXCn register, and transmission from the respective endpoint is enabled. This bit is cleared when the warning condition is cleared by either writing new data to the FIFO when the FIFO is flushed, or when transmission is done, as indicated by the TX_DONE bit in the TXSn register."]
    #[inline(always)]
    pub fn usb_txwarn31(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, UsbFwevReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,UsbFwevReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbFwevReg {
    #[inline(always)]
    fn default() -> UsbFwevReg {
        <crate::RegValueT<UsbFwevReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbFwmskReg_SPEC;
impl crate::sealed::RegSpec for UsbFwmskReg_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Warning Mask Register"]
pub type UsbFwmskReg = crate::RegValueT<UsbFwmskReg_SPEC>;

impl UsbFwmskReg {
    #[doc = "The FIFO Warning Mask Register selects, which FWEV bits are reported in the MAEV register. A bit set to 1 and the corresponding bit in the FWEV register is set 1, causes the WARN bit in the MAEV register to be set to 1. When cleared to 0, the corresponding bit in the FWEV register does not cause WARN to be set to 1. Same Bit Definition as FWEV Register"]
    #[inline(always)]
    pub fn usb_m_rxwarn31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbFwmskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbFwmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The FIFO Warning Mask Register selects, which FWEV bits are reported in the MAEV register. A bit set to 1 and the corresponding bit in the FWEV register is set 1, causes the WARN bit in the MAEV register to be set to 1. When cleared to 0, the corresponding bit in the FWEV register does not cause WARN to be set to 1. Same Bit Definition as FWEV Register"]
    #[inline(always)]
    pub fn usb_m_txwarn31(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, UsbFwmskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,UsbFwmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbFwmskReg {
    #[inline(always)]
    fn default() -> UsbFwmskReg {
        <crate::RegValueT<UsbFwmskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbMaevReg_SPEC;
impl crate::sealed::RegSpec for UsbMaevReg_SPEC {
    type DataType = u32;
}

#[doc = "Main Event Register"]
pub type UsbMaevReg = crate::RegValueT<UsbMaevReg_SPEC>;

impl UsbMaevReg {
    #[doc = "USB Charger event\nThis bit is set if one of the bits in USB_CHARGER_STAT_REG\\[2-0\\] change. This bit is cleared to 0 when if USB_CHARGER_STAT_REG is read."]
    #[inline(always)]
    pub fn usb_ch_ev(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint 0 NAK Event\nThis bit is an OR of EP0_NAK_REG\\[EP0_OUTNAK\\] and EP0_NAK_REG\\[EP0_INNAK\\] bits. USB_EP0_NAK is cleared to 0 when EP0_NAK_REG is read."]
    #[inline(always)]
    pub fn usb_ep0_nak(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint 0 Receive Event\nThis bit is a copy of the RXS0\\[RX_LAST\\] and is cleared to 0 when this RXS0 register is read.\nNote: Since Endpoint 0 implements a store and forward principle, an overrun condition for FIFO0 cannot occur"]
    #[inline(always)]
    pub fn usb_ep0_rx(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint 0 Transmit Event\nThis bit is a copy of the TXS0\\[TX_DONE\\] bit and is cleared to 0 when the TXS0 register is read.\nNote: Since Endpoint 0 implements a store and forward principle, an underrun condition for FIFO0 cannot occur."]
    #[inline(always)]
    pub fn usb_ep0_tx(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Interrupt Enable\nThis bit is hardwired to 0 in the Main Event (MAEV) register; bit 7 in the Main Mask (MAMSK) register is the Master Interrupt Enable."]
    #[inline(always)]
    pub fn usb_intr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive Event\nThis bit is set to 1 if any of the unmasked bits in the Receive Event (RXEV) register is set to 1. It indicates that a SETUP or OUT transaction has been completed. This bit is cleared to 0 when all of the RX_LAST bits in each Receive Status (RXSn) register and all RXOVRRN bits in the RXEV register are cleared to 0."]
    #[inline(always)]
    pub fn usb_rx_ev(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Unlocked/Locked Detected\nThis bit is set to 1, when the frame timer has either entered unlocked condition from a locked condition, or has re-entered a locked condition from an unlocked condition as determined by the UL bit in the Frame Number (FNH or FNL) register. This bit is cleared to 0 when the register is read."]
    #[inline(always)]
    pub fn usb_uld(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Negative Acknowledge Event\nThis bit indicates that one of the unmasked NAK Event (NAKEV) register bits has been set to 1. This bit is cleared to 0 when the NAKEV register is read."]
    #[inline(always)]
    pub fn usb_nak(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Frame Event\nThis bit is set to 1, if the frame counter is updated with a new value. This can be due to the receipt of a valid SOF packet on the USB or to an artificial update if the frame counter was unlocked or a frame was missed. This bit is cleared to 0 when the register is read."]
    #[inline(always)]
    pub fn usb_frame(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit Event\nThis bit is set to 1, if any of the unmasked bits in the Transmit Event (TXEV) register (TXFIFOn or TXUNDRNn) is set to 1. Therefore, it indicates that an IN transaction has been completed. This bit is cleared to 0 when all the TX_DONE bits and the TXUNDRN bits in each Transmit Status (TXSn) register are cleared to 0."]
    #[inline(always)]
    pub fn usb_tx_ev(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alternate Event\nThis bit indicates that one of the unmasked ALTEV register bits has been set to 1. This bit is cleared to 0 by reading the ALTEV register."]
    #[inline(always)]
    pub fn usb_alt(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Warning Event\nThis bit indicates that one of the unmasked bits in the FIFO Warning Event (FWEV) register has been set to 1. This bit is cleared to 0 by reading the FWEV register."]
    #[inline(always)]
    pub fn usb_warn(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbMaevReg {
    #[inline(always)]
    fn default() -> UsbMaevReg {
        <crate::RegValueT<UsbMaevReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbMamskReg_SPEC;
impl crate::sealed::RegSpec for UsbMamskReg_SPEC {
    type DataType = u32;
}

#[doc = "Main Mask Register"]
pub type UsbMamskReg = crate::RegValueT<UsbMamskReg_SPEC>;

impl UsbMamskReg {
    #[doc = "The Main Mask Register masks out events reported in the MAEV registers. A bit set to 1, enables the interrupts for the respective event in the MAEV register. If the corresponding bit is cleared to 0, interrupt generation for this event is disabled. Same Bit Definition as MAEV Register"]
    #[inline(always)]
    pub fn usb_m_ch_ev(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as MAEV Register"]
    #[inline(always)]
    pub fn usb_m_ep0_nak(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as MAEV Register"]
    #[inline(always)]
    pub fn usb_m_ep0_rx(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as MAEV Register"]
    #[inline(always)]
    pub fn usb_m_ep0_tx(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as MAEV Register"]
    #[inline(always)]
    pub fn usb_m_intr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as MAEV Register"]
    #[inline(always)]
    pub fn usb_m_rx_ev(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as MAEV Register"]
    #[inline(always)]
    pub fn usb_m_uld(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as MAEV Register"]
    #[inline(always)]
    pub fn usb_m_nak(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as MAEV Register"]
    #[inline(always)]
    pub fn usb_m_frame(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as MAEV Register"]
    #[inline(always)]
    pub fn usb_m_tx_ev(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as MAEV Register"]
    #[inline(always)]
    pub fn usb_m_alt(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as MAEV Register"]
    #[inline(always)]
    pub fn usb_m_warn(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbMamskReg {
    #[inline(always)]
    fn default() -> UsbMamskReg {
        <crate::RegValueT<UsbMamskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbMctrlReg_SPEC;
impl crate::sealed::RegSpec for UsbMctrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Main Control Register)"]
pub type UsbMctrlReg = crate::RegValueT<UsbMctrlReg_SPEC>;

impl UsbMctrlReg {
    #[doc = "Low Speed Mode\nThis bit enables USB 1.5 Mbit/s low speed and swaps D+ and D- pull-up resistors. Changing speed may only be done if USBEN is set to 0.\nAlso D+ and D- rise and fall times are adjusted according to the USB specification."]
    #[inline(always)]
    pub fn lsmode(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbMctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbMctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Node Attached\nThis bit indicates that this node is ready to be detected as attached to USB. When cleared to 0 the transceiver forces SE0 on the USB port to prevent the hub (to which this node is connected to) from detecting an attach event. After reset or when the USB node is disabled, this bit is cleared to 0 to give the device time before it must respond to commands. After this bit has been set to 1, the device no longer drives the USB and should be ready to receive Reset signalling from the hub.\nNote: This bit can only be set is USBEN is \'1\'"]
    #[inline(always)]
    pub fn usb_nat(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbMctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbMctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Debug Mode.\nWhen this bit is set, the following registers are writable: Main Event (MAEV), Alternate Event (ALTEV), NAK Event (NAKEV), Transmit Status and Receive Status. Setting the DBG bit forces the node into a locked state. The node states can be read out of the transceiver diagnostic register (XCVDIAG) at location 0xFF6802 by setting the DIAG bit in the Test Control register (UTR).\nNote: The operation of CoR bits is not effected by entering Debug mode) Note: This bit can only be set is USBEN is \'1\'"]
    #[inline(always)]
    pub fn usb_dbg(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbMctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbMctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "USB EnableSetting this bit to 1 enables the Full/Low Speed USB node. If the USBEN bit is cleared to 0, the USB is disabled and the 48 MHz clock within the USB node is stopped. In addition, all USB registers are set to their reset state.\nNote that the transceiver forces SE0 on the bus to prevent the hub to detected the USB node, when it is disabled (not attached).\nThe USBEN bit is cleared to 0 after reset"]
    #[inline(always)]
    pub fn usben(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbMctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbMctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbMctrlReg {
    #[inline(always)]
    fn default() -> UsbMctrlReg {
        <crate::RegValueT<UsbMctrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbNakevReg_SPEC;
impl crate::sealed::RegSpec for UsbNakevReg_SPEC {
    type DataType = u32;
}

#[doc = "NAK Event Register"]
pub type UsbNakevReg = crate::RegValueT<UsbNakevReg_SPEC>;

impl UsbNakevReg {
    #[doc = "OUT n: 3:1\nThe bit n is set to 1 when a NAK handshake is generated for an enabled address/endpoint combination (AD_EN in the FAR register is set to 1 and EP_EN in the EPCx register is set to 1) in response to an OUT token. This bit is not set if NAK is generated as result of an overrun condition. It is cleared when the register is read."]
    #[inline(always)]
    pub fn usb_out31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbNakevReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbNakevReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IN n: 3:1\nThe bit n is set to 1 when a NAK handshake is generated for an enabled address/endpoint combination (AD_EN in the Function Address, FAR, register is set to 1 and EP_EN in the Endpoint Control, EPCx, register is set to 1) in response to an IN token. This bit is cleared when the register is read."]
    #[inline(always)]
    pub fn usb_in31(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, UsbNakevReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,UsbNakevReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbNakevReg {
    #[inline(always)]
    fn default() -> UsbNakevReg {
        <crate::RegValueT<UsbNakevReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbNakmskReg_SPEC;
impl crate::sealed::RegSpec for UsbNakmskReg_SPEC {
    type DataType = u32;
}

#[doc = "NAK Mask Register"]
pub type UsbNakmskReg = crate::RegValueT<UsbNakmskReg_SPEC>;

impl UsbNakmskReg {
    #[doc = "When set and the corresponding bit in the NAKEV register is set, the NAK bit in the MAEV register is set. When cleared, the corresponding bit in the NAKEV register does not cause NAK to be set. Same Bit Definition as NAKEV Register"]
    #[inline(always)]
    pub fn usb_m_out31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbNakmskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbNakmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as NAKEV Register"]
    #[inline(always)]
    pub fn usb_m_in31(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, UsbNakmskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,UsbNakmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbNakmskReg {
    #[inline(always)]
    fn default() -> UsbNakmskReg {
        <crate::RegValueT<UsbNakmskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbNfsrReg_SPEC;
impl crate::sealed::RegSpec for UsbNfsrReg_SPEC {
    type DataType = u32;
}

#[doc = "Node Functional State Register"]
pub type UsbNfsrReg = crate::RegValueT<UsbNfsrReg_SPEC>;

impl UsbNfsrReg {
    #[doc = "The Node Functional State Register reports and controls the current functional state of the USB node.\n00: NodeReset.\nThis is the USB Reset state. This is entered upon a module reset or by software upon detection of a USB Reset. Upon entry, all endpoint pipes are disabled. DEF in the Endpoint Control 0 (EPC0) register and AD_EN in the Function Address (FAR) register should be cleared by software on entry to this state. On exit, DEF should be reset so the device responds to the default address.\n01: NodeResume\nIn this state, resume signalling is generated. This state should be entered by firmware to initiate a remote wake-up sequence by the device. The node must remain in this state for at least 1 ms and no more than 15 ms.\n10: NodeOperational\nThis is the normal operational state. In this state the node is configured for operation on the USB bus.\n11: NodeSuspend\nSuspend state should be entered by firmware on detection of a Suspend event while in Operational state. While in Suspend state, the transceivers operate in their low-power suspend mode. All endpoint controllers and the bits TX_EN, LAST and RX_EN are reset, while all other internal states are frozen. On detection of bus activity, the RESUME bit in the ALTEV register is set. In response, software can cause entry to NodeOperational state."]
    #[inline(always)]
    pub fn usb_nfs(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, UsbNfsrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,UsbNfsrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbNfsrReg {
    #[inline(always)]
    fn default() -> UsbNfsrReg {
        <crate::RegValueT<UsbNfsrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbRxc0Reg_SPEC;
impl crate::sealed::RegSpec for UsbRxc0Reg_SPEC {
    type DataType = u32;
}

#[doc = "Receive Command 0 Register"]
pub type UsbRxc0Reg = crate::RegValueT<UsbRxc0Reg_SPEC>;

impl UsbRxc0Reg {
    #[doc = "Flush\nWriting a 1 to this bit flushes all data from the control endpoint FIFOs, resets the endpoint to Idle state, clears the FIFO read and write pointer, and then clears itself. If the endpoint is currently using FIFO0 to transfer data on USB, flushing is delayed until after the transfer is done. This bit is cleared to 0 on reset. This bit is equivalent to FLUSH in the TXC0 register."]
    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbRxc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbRxc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ignore SETUP Tokens\nWhen this bit is set to 1, the endpoint ignores any SETUP tokens directed to its configured address."]
    #[inline(always)]
    pub fn usb_ign_setup(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbRxc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbRxc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ignore OUT Tokens\nWhen this bit is set to 1, the endpoint ignores any OUT tokens directed to its configured address."]
    #[inline(always)]
    pub fn usb_ign_out(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbRxc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbRxc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive Enable\nOUT packet reception is disabled after every data packet is received, or when a STALL handshake is returned in response to an OUT token. A 1 must be written to this bit to re-enable data reception. Reception of SETUP packets is always enabled. In the case of back-to-back SETUP packets (for a given endpoint) where a valid SETUP packet is received with no other intervening non-SETUP tokens, the Endpoint Controller discards the new SETUP packet and returns an ACK handshake. If any other reasons prevent the Endpoint Controller from accepting the SETUP packet, it must not generate a handshake. This allows recovery from a condition where the ACK of the first SETUP token was lost by the host."]
    #[inline(always)]
    pub fn usb_rx_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbRxc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbRxc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbRxc0Reg {
    #[inline(always)]
    fn default() -> UsbRxc0Reg {
        <crate::RegValueT<UsbRxc0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbRxc1Reg_SPEC;
impl crate::sealed::RegSpec for UsbRxc1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Receive Command Register 1"]
pub type UsbRxc1Reg = crate::RegValueT<UsbRxc1Reg_SPEC>;

impl UsbRxc1Reg {
    #[doc = "Receive FIFO Warning Limit\nThese bits specify how many more bytes can be received to the respective FIFO before an overrun condition occurs. If the number of empty bytes remaining in the FIFO is equal to or less than the selected warning limit, the RXWARN bit in the FWEV register is set to 1.RFWL\\[1:0\\] :\n00: RFWL disabled\n01: Less than 5 bytes remaining in FIFO\n10: Less than 9 bytes remaining in FIFO\n11: Less than 17 bytes remaining in FIFO"]
    #[inline(always)]
    pub fn usb_rfwl(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, UsbRxc1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,UsbRxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Flush FIFO\nWriting a 1 to this bit flushes all data from the corresponding receive FIFO, resets the endpoint to Idle state, and resets both the FIFO read and write pointers. If the MAC is currently using the FIFO to receive data, flushing is delayed until after receiving is completed."]
    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbRxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbRxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ignore SETUP Tokens\nWhen this bit is set to 1, the endpoint ignores any SETUP tokens directed to its configured address."]
    #[inline(always)]
    pub fn usb_ign_setup(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbRxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbRxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive Enable\nOUT packet cannot be received after every data packet is received, or when a STALL handshake is returned in response to an OUT token. This bit must be written with a 1 to re-enable data reception. SETUP packets can always be received. In the case of back-to-back SETUP packets (for a given endpoint) where a valid SETUP packet has been received with no other intervening non-SETUP tokens, the receive state machine discards the new SETUP packet and returns an ACK handshake. If, for any other reason, the receive state machine cannot accept the SETUP packet, no HANDSHAKE should be generated."]
    #[inline(always)]
    pub fn usb_rx_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbRxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbRxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbRxc1Reg {
    #[inline(always)]
    fn default() -> UsbRxc1Reg {
        <crate::RegValueT<UsbRxc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbRxc2Reg_SPEC;
impl crate::sealed::RegSpec for UsbRxc2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Receive Command Register 2"]
pub type UsbRxc2Reg = crate::RegValueT<UsbRxc2Reg_SPEC>;

impl UsbRxc2Reg {
    #[doc = "Receive FIFO Warning Limit\nThese bits specify how many more bytes can be received to the respective FIFO before an overrun condition occurs. If the number of empty bytes remaining in the FIFO is equal to or less than the selected warning limit, the RXWARN bit in the FWEV register is set to 1.RFWL\\[1:0\\] :\n00: RFWL disabled\n01: Less than 5 bytes remaining in FIFO\n10: Less than 9 bytes remaining in FIFO\n11: Less than 17 bytes remaining in FIFO"]
    #[inline(always)]
    pub fn usb_rfwl(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, UsbRxc2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,UsbRxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Flush FIFO\nWriting a 1 to this bit flushes all data from the corresponding receive FIFO, resets the endpoint to Idle state, and resets both the FIFO read and write pointers. If the MAC is currently using the FIFO to receive data, flushing is delayed until after receiving is completed."]
    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbRxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbRxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ignore SETUP Tokens\nWhen this bit is set to 1, the endpoint ignores any SETUP tokens directed to its configured address."]
    #[inline(always)]
    pub fn usb_ign_setup(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbRxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbRxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive Enable\nOUT packet cannot be received after every data packet is received, or when a STALL handshake is returned in response to an OUT token. This bit must be written with a 1 to re-enable data reception. SETUP packets can always be received. In the case of back-to-back SETUP packets (for a given endpoint) where a valid SETUP packet has been received with no other intervening non-SETUP tokens, the receive state machine discards the new SETUP packet and returns an ACK handshake. If, for any other reason, the receive state machine cannot accept the SETUP packet, no HANDSHAKE should be generated."]
    #[inline(always)]
    pub fn usb_rx_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbRxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbRxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbRxc2Reg {
    #[inline(always)]
    fn default() -> UsbRxc2Reg {
        <crate::RegValueT<UsbRxc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbRxc3Reg_SPEC;
impl crate::sealed::RegSpec for UsbRxc3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Receive Command Register 3"]
pub type UsbRxc3Reg = crate::RegValueT<UsbRxc3Reg_SPEC>;

impl UsbRxc3Reg {
    #[doc = "Receive FIFO Warning Limit\nThese bits specify how many more bytes can be received to the respective FIFO before an overrun condition occurs. If the number of empty bytes remaining in the FIFO is equal to or less than the selected warning limit, the RXWARN bit in the FWEV register is set to 1.RFWL\\[1:0\\] :\n00: RFWL disabled\n01: Less than 5 bytes remaining in FIFO\n10: Less than 9 bytes remaining in FIFO\n11: Less than 17 bytes remaining in FIFO"]
    #[inline(always)]
    pub fn usb_rfwl(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, UsbRxc3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,UsbRxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Flush FIFO\nWriting a 1 to this bit flushes all data from the corresponding receive FIFO, resets the endpoint to Idle state, and resets both the FIFO read and write pointers. If the MAC is currently using the FIFO to receive data, flushing is delayed until after receiving is completed."]
    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbRxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbRxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ignore SETUP Tokens\nWhen this bit is set to 1, the endpoint ignores any SETUP tokens directed to its configured address."]
    #[inline(always)]
    pub fn usb_ign_setup(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbRxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbRxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive Enable\nOUT packet cannot be received after every data packet is received, or when a STALL handshake is returned in response to an OUT token. This bit must be written with a 1 to re-enable data reception. SETUP packets can always be received. In the case of back-to-back SETUP packets (for a given endpoint) where a valid SETUP packet has been received with no other intervening non-SETUP tokens, the receive state machine discards the new SETUP packet and returns an ACK handshake. If, for any other reason, the receive state machine cannot accept the SETUP packet, no HANDSHAKE should be generated."]
    #[inline(always)]
    pub fn usb_rx_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbRxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbRxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbRxc3Reg {
    #[inline(always)]
    fn default() -> UsbRxc3Reg {
        <crate::RegValueT<UsbRxc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbRxd0Reg_SPEC;
impl crate::sealed::RegSpec for UsbRxd0Reg_SPEC {
    type DataType = u32;
}

#[doc = "Receive Data 0 Register"]
pub type UsbRxd0Reg = crate::RegValueT<UsbRxd0Reg_SPEC>;

impl UsbRxd0Reg {
    #[doc = "Receive FIFO Data Byte\nThe firmware should expect to read only the packet payload data. The PID and CRC16 are removed from the incoming data stream automatically.\nIn TEST mode this register allow read/write access."]
    #[inline(always)]
    pub fn usb_rxfd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UsbRxd0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,UsbRxd0Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbRxd0Reg {
    #[inline(always)]
    fn default() -> UsbRxd0Reg {
        <crate::RegValueT<UsbRxd0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbRxd1Reg_SPEC;
impl crate::sealed::RegSpec for UsbRxd1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Receive Data Register,1"]
pub type UsbRxd1Reg = crate::RegValueT<UsbRxd1Reg_SPEC>;

impl UsbRxd1Reg {
    #[doc = "Receive FIFO Data Byte\nThe firmware should expect to read only the packet payload data. The PID and CRC16 are terminated by the receive state machine.\nIn TEST mode this register allow read/write access via the core bus."]
    #[inline(always)]
    pub fn usb_rxfd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UsbRxd1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,UsbRxd1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbRxd1Reg {
    #[inline(always)]
    fn default() -> UsbRxd1Reg {
        <crate::RegValueT<UsbRxd1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbRxd2Reg_SPEC;
impl crate::sealed::RegSpec for UsbRxd2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Receive Data Register 2"]
pub type UsbRxd2Reg = crate::RegValueT<UsbRxd2Reg_SPEC>;

impl UsbRxd2Reg {
    #[doc = "Receive FIFO Data Byte\nThe firmware should expect to read only the packet payload data. The PID and CRC16 are terminated by the receive state machine.\nIn TEST mode this register allow read/write access via the core bus."]
    #[inline(always)]
    pub fn usb_rxfd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UsbRxd2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,UsbRxd2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbRxd2Reg {
    #[inline(always)]
    fn default() -> UsbRxd2Reg {
        <crate::RegValueT<UsbRxd2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbRxd3Reg_SPEC;
impl crate::sealed::RegSpec for UsbRxd3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Receive Data Register 3"]
pub type UsbRxd3Reg = crate::RegValueT<UsbRxd3Reg_SPEC>;

impl UsbRxd3Reg {
    #[doc = "Receive FIFO Data Byte\nThe firmware should expect to read only the packet payload data. The PID and CRC16 are terminated by the receive state machine.\nIn TEST mode this register allow read/write access via the core bus."]
    #[inline(always)]
    pub fn usb_rxfd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UsbRxd3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,UsbRxd3Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbRxd3Reg {
    #[inline(always)]
    fn default() -> UsbRxd3Reg {
        <crate::RegValueT<UsbRxd3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbRxevReg_SPEC;
impl crate::sealed::RegSpec for UsbRxevReg_SPEC {
    type DataType = u32;
}

#[doc = "Receive Event Register"]
pub type UsbRxevReg = crate::RegValueT<UsbRxevReg_SPEC>;

impl UsbRxevReg {
    #[doc = "Receive Overrun n: 3:1\nThe bit n is set to 1 in the event of an overrun condition in the corresponding receive FIFO n. They are cleared to 0 when the register is read. The firmware must check the respective RX_ERR bits that packets received for the other receive endpoints (EP2, EP4 and EP6, ) are not corrupted by errors, as these endpoints support data streaming (packets which are longer than the actual FIFO depth)."]
    #[inline(always)]
    pub fn usb_rxovrrn31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbRxevReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbRxevReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive FIFO n: 3:1\nThe bit n is set to 1 whenever either RX_ERR or RX_LAST in the respective Receive Status register (RXSn) is set to 1. Reading the corresponding RXSn register automatically clears these bits.The CoR function is disabled, when the Freeze signal is asserted.The USB node discards all packets for Endpoint 0 received with errors. This is necessary in case of retransmission due to media errors, ensuring that a good copy of a SETUP packet is captured. Otherwise, the FIFO may potentially be tied up, holding corrupted data and unable to receive a retransmission of the same packet.\nIf data streaming is used for the receive endpoints (EP2, EP4 and EP6, EP8) the firmware must check the respective RX_ERR bits to ensure the packets received are not corrupted by errors."]
    #[inline(always)]
    pub fn usb_rxfifo31(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, UsbRxevReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,UsbRxevReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbRxevReg {
    #[inline(always)]
    fn default() -> UsbRxevReg {
        <crate::RegValueT<UsbRxevReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbRxmskReg_SPEC;
impl crate::sealed::RegSpec for UsbRxmskReg_SPEC {
    type DataType = u32;
}

#[doc = "Receive Mask Register"]
pub type UsbRxmskReg = crate::RegValueT<UsbRxmskReg_SPEC>;

impl UsbRxmskReg {
    #[doc = "The Receive Mask Register is used to select the bits of the RXEV registers, which causes the RX_EV bit in the MAEV register to be set to 1. When set to 1 and the corresponding bit in the RXEV register is set to 1, RX_EV bit in the MAEV register is set to1. When cleared to 0, the corresponding bit in the RXEV register does not cause RX_EV to be set to1. Same Bit Definition as RXEV Register"]
    #[inline(always)]
    pub fn usb_m_rxovrrn31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbRxmskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbRxmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as RXEV Register"]
    #[inline(always)]
    pub fn usb_m_rxfifo31(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, UsbRxmskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,UsbRxmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbRxmskReg {
    #[inline(always)]
    fn default() -> UsbRxmskReg {
        <crate::RegValueT<UsbRxmskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbRxs0Reg_SPEC;
impl crate::sealed::RegSpec for UsbRxs0Reg_SPEC {
    type DataType = u32;
}

#[doc = "Receive Status 0 Register"]
pub type UsbRxs0Reg = crate::RegValueT<UsbRxs0Reg_SPEC>;

impl UsbRxs0Reg {
    #[doc = "Setup\nThis bit indicates that the setup packet has been received. This bit is unchanged for zero length packets. It is cleared to 0 when this register is read."]
    #[inline(always)]
    pub fn usb_setup(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbRxs0Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbRxs0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Toggle\nThis bit specified the PID used when receiving the packet. A value of 0 indicates that the last successfully received packet had a DATA0 PID, while a value of 1 indicates that this packet had a DATA1 PID. This bit is unchanged for zero length packets. It is cleared to 0 when this register is read."]
    #[inline(always)]
    pub fn usb_toggle_rx0(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbRxs0Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbRxs0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Last Bytes\nThis bit indicates that an ACK was sent upon completion of a successful receive operation. This bit is unchanged for zero length packets. It is cleared to 0 when this register is read."]
    #[inline(always)]
    pub fn usb_rx_last(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbRxs0Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,UsbRxs0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Count\nThis 4-bit field contains the number of bytes presently in the RX FIFO. This number is never larger than 8 for Endpoint 0."]
    #[inline(always)]
    pub fn usb_rcount(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, UsbRxs0Reg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,UsbRxs0Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbRxs0Reg {
    #[inline(always)]
    fn default() -> UsbRxs0Reg {
        <crate::RegValueT<UsbRxs0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbRxs1Reg_SPEC;
impl crate::sealed::RegSpec for UsbRxs1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Receive Status Register 1"]
pub type UsbRxs1Reg = crate::RegValueT<UsbRxs1Reg_SPEC>;

impl UsbRxs1Reg {
    #[doc = "it contains the number of bytes presently in the endpoint receive FIFO (range 0..64)"]
    #[inline(always)]
    pub fn usb_rxcount(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, UsbRxs1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,UsbRxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Error\nWhen set to 1, this bit indicates a media error, such as bit-stuffing or CRC. If this bit is set to 1, the firmware must flush the respective FIFO."]
    #[inline(always)]
    pub fn usb_rx_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbRxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbRxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Setup\nThis bit indicates that the setup packet has been received. It is cleared when this register is read."]
    #[inline(always)]
    pub fn usb_setup(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbRxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbRxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Toggle\nThe function of this bit differs depending on whether ISO (ISO in the EPCn register is set) or non-ISO operation (ISO is reset) is used.\nFor non-ISO operation, a value of 0 indicates that the last successfully received packet had a DATA0 PID, while a value of 1 indicates that this packet had a DATA1 PID.\nFor ISO operation, this bit reflects the LSB of the frame number (FNL0) after a packet was successfully received for this endpoint.\nThis bit is reset to 0 by reading the RXSn register."]
    #[inline(always)]
    pub fn usb_toggle_rx(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbRxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbRxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Last\nThis bit indicates that an ACK was sent upon completion of a successful receive operation. This bit is cleared to 0 when this register is read."]
    #[inline(always)]
    pub fn usb_rx_last(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbRxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,UsbRxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Counter\nThis 4-bit field contains the number of bytes presently in the endpoint receive FIFO. If this number is greater than 15, a value of 15 is actually reported."]
    #[inline(always)]
    pub fn usb_rcount(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, UsbRxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,UsbRxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbRxs1Reg {
    #[inline(always)]
    fn default() -> UsbRxs1Reg {
        <crate::RegValueT<UsbRxs1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbRxs2Reg_SPEC;
impl crate::sealed::RegSpec for UsbRxs2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Receive Status Register 2"]
pub type UsbRxs2Reg = crate::RegValueT<UsbRxs2Reg_SPEC>;

impl UsbRxs2Reg {
    #[doc = "it contains the number of bytes presently in the endpoint receive FIFO (range 0..64)"]
    #[inline(always)]
    pub fn usb_rxcount(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, UsbRxs2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,UsbRxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Error\nWhen set to 1, this bit indicates a media error, such as bit-stuffing or CRC. If this bit is set to 1, the firmware must flush the respective FIFO."]
    #[inline(always)]
    pub fn usb_rx_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbRxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbRxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Setup\nThis bit indicates that the setup packet has been received. It is cleared when this register is read."]
    #[inline(always)]
    pub fn usb_setup(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbRxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbRxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Toggle\nThe function of this bit differs depending on whether ISO (ISO in the EPCn register is set) or non-ISO operation (ISO is reset) is used.\nFor non-ISO operation, a value of 0 indicates that the last successfully received packet had a DATA0 PID, while a value of 1 indicates that this packet had a DATA1 PID.\nFor ISO operation, this bit reflects the LSB of the frame number (FNL0) after a packet was successfully received for this endpoint.\nThis bit is reset to 0 by reading the RXSn register."]
    #[inline(always)]
    pub fn usb_toggle_rx(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbRxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbRxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Last\nThis bit indicates that an ACK was sent upon completion of a successful receive operation. This bit is cleared to 0 when this register is read."]
    #[inline(always)]
    pub fn usb_rx_last(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbRxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,UsbRxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Counter\nThis 4-bit field contains the number of bytes presently in the endpoint receive FIFO. If this number is greater than 15, a value of 15 is actually reported."]
    #[inline(always)]
    pub fn usb_rcount(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, UsbRxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,UsbRxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbRxs2Reg {
    #[inline(always)]
    fn default() -> UsbRxs2Reg {
        <crate::RegValueT<UsbRxs2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbRxs3Reg_SPEC;
impl crate::sealed::RegSpec for UsbRxs3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Receive Status Register 3"]
pub type UsbRxs3Reg = crate::RegValueT<UsbRxs3Reg_SPEC>;

impl UsbRxs3Reg {
    #[doc = "it contains the number of bytes presently in the endpoint receive FIFO (range 0..64)"]
    #[inline(always)]
    pub fn usb_rxcount(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, UsbRxs3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,UsbRxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Error\nWhen set to 1, this bit indicates a media error, such as bit-stuffing or CRC. If this bit is set to 1, the firmware must flush the respective FIFO."]
    #[inline(always)]
    pub fn usb_rx_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbRxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbRxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Setup\nThis bit indicates that the setup packet has been received. It is cleared when this register is read."]
    #[inline(always)]
    pub fn usb_setup(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbRxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbRxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Toggle\nThe function of this bit differs depending on whether ISO (ISO in the EPCn register is set) or non-ISO operation (ISO is reset) is used.\nFor non-ISO operation, a value of 0 indicates that the last successfully received packet had a DATA0 PID, while a value of 1 indicates that this packet had a DATA1 PID.\nFor ISO operation, this bit reflects the LSB of the frame number (FNL0) after a packet was successfully received for this endpoint.\nThis bit is reset to 0 by reading the RXSn register."]
    #[inline(always)]
    pub fn usb_toggle_rx(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbRxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbRxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Last\nThis bit indicates that an ACK was sent upon completion of a successful receive operation. This bit is cleared to 0 when this register is read."]
    #[inline(always)]
    pub fn usb_rx_last(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbRxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,UsbRxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Counter\nThis 4-bit field contains the number of bytes presently in the endpoint receive FIFO. If this number is greater than 15, a value of 15 is actually reported."]
    #[inline(always)]
    pub fn usb_rcount(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, UsbRxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,UsbRxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbRxs3Reg {
    #[inline(always)]
    fn default() -> UsbRxs3Reg {
        <crate::RegValueT<UsbRxs3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTcrReg_SPEC;
impl crate::sealed::RegSpec for UsbTcrReg_SPEC {
    type DataType = u32;
}

#[doc = "Transceiver configuration Register"]
pub type UsbTcrReg = crate::RegValueT<UsbTcrReg_SPEC>;

impl UsbTcrReg {
    #[doc = "Reference Voltage/ Threshold voltage AdjustControls the single-ended receiver threshold.\nShall not be modified unless instructed by Dialog Semiconductor\nOnly enabled if USB_UTR_REG\\[7\\] = 1"]
    #[inline(always)]
    pub fn usb_vadj(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, UsbTcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,UsbTcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmitter Current Adjust\nControls the driver edge rate control current.\nShall not be modified unless instructed by Dialog Semiconductor\nOnly enabled if USB_UTR_REG\\[7\\] = 1"]
    #[inline(always)]
    pub fn usb_cadj(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, UsbTcrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,UsbTcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTcrReg {
    #[inline(always)]
    fn default() -> UsbTcrReg {
        <crate::RegValueT<UsbTcrReg_SPEC> as RegisterValue<_>>::new(144)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTxc0Reg_SPEC;
impl crate::sealed::RegSpec for UsbTxc0Reg_SPEC {
    type DataType = u32;
}

#[doc = "Transmit command 0 Register"]
pub type UsbTxc0Reg = crate::RegValueT<UsbTxc0Reg_SPEC>;

impl UsbTxc0Reg {
    #[doc = "Ignore IN Tokens\nWhen this bit is set to 1, the endpoint will ignore any IN tokens directed to its configured address."]
    #[inline(always)]
    pub fn usb_ign_in(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbTxc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbTxc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Flush FIFO\nWriting a 1 to this bit flushes all data from the control endpoint FIFOs, resets the endpoint to Idle state, clears the FIFO read and write pointer, and then clears itself. If the endpoint is currently using the FIFO0 to transfer data on USB, flushing is delayed until after the transfer is done. It is equivalent to the FLUSH bit in the RXC0 register."]
    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbTxc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbTxc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Toggle\nThis bit specifies the PID used when transmitting the packet. A value of 0 causes a DATA0 PID to be generated, while a value of 1 causes a DATA1 PID to be generated. This bit is not altered by the hardware."]
    #[inline(always)]
    pub fn usb_toggle_tx0(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbTxc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbTxc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmission Enable\nThis bit enables data transmission from the FIFO. It is cleared to 0 by hardware after transmitting a single packet, or a STALL handshake, in response to an IN token. It must be set to 1 by firmware to start packet transmission. The RX_EN bit in the Receive Command 0 (RXC0) register takes precedence over this bit; i.e. if RX_EN is set, TX_EN bit is ignored until RX_EN is reset.\nZero length packets are indicated by setting this bit without writing any data to the FIFO."]
    #[inline(always)]
    pub fn usb_tx_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbTxc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbTxc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTxc0Reg {
    #[inline(always)]
    fn default() -> UsbTxc0Reg {
        <crate::RegValueT<UsbTxc0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTxc1Reg_SPEC;
impl crate::sealed::RegSpec for UsbTxc1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Command Register 1"]
pub type UsbTxc1Reg = crate::RegValueT<UsbTxc1Reg_SPEC>;

impl UsbTxc1Reg {
    #[doc = "Ignore ISO Mask\nThis bit has an effect only if the endpoint is set to be isochronous. If set to 1, this bit disables locking of specific frame numbers with the alternate function of the TOGGLE bit. Thus data is transmitted upon reception of the next IN token. If cleared to 0, data is only transmitted when FNL0 matches TOGGLE. This bit is cleared to 0 after reset."]
    #[inline(always)]
    pub fn usb_ign_isomsk(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbTxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbTxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit FIFO Warning Limit\nThese bits specify how many more bytes can be transmitted from the respective FIFO before an underrun condition occurs. If the number of bytes remaining in the FIFO is equal to or less than the selected warning limit, the TXWARN bit in the FWEV register is set. To avoid interrupts caused by setting this bit while the FIFO is being filled before a transmission begins, TXWARN is only set when transmission from the endpoint is enabled (TX_ENn in the TXCn register is set).\nTFWL\\[1:0\\] :\n00: TFWL disabled\n01: Less than 5 bytes remaining in FIFO\n10: Less than 9 bytes remaining in FIFO\n11: Less than 17 bytes remaining in FIFO"]
    #[inline(always)]
    pub fn usb_tfwl(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, UsbTxc1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,UsbTxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refill FIFO\nSetting the LAST bit to 1 automatically saves the Transmit Read Pointer (TXRP) to a buffer. When the RFF bit is set to 1, the buffered TXRP is reloaded into the TXRP. This allows the user to repeat the last transaction if no ACK was received from the host. If the MAC is currently using the FIFO to transmit, TXRP is reloaded only after the transmission is complete. After reload, this bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn usb_rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbTxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbTxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Flush FIFO\nWriting a 1 to this bit flushes all data from the corresponding transmit FIFO, resets the endpoint to Idle state, and clears both the FIFO read and write pointers. If the MAC is currently using the FIFO to transmit, data is flushed after the transmission is complete. After data flushing, this bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbTxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbTxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Toggle\nThe function of this bit differs depending on whether ISO (ISO bit in the EPCn register is set to 1) or non-ISO operation (ISO bit is cleared to 0) is used.\nFor non-ISO operation, it specifies the PID used when transmitting the packet. A value of 0 causes a DATA0 PID to be generated, while a value of 1 causes a DATA1 PID to be generated.\nFor ISO operation, this bit and the LSB of the frame counter (FNL0) act as a mask for the TX_EN bit to allow pre-queuing of packets to specific frame numbers; I.e. transmission is enabled only if bit 0 in the FNL register is set to TOGGLE. If an IN token is not received while this condition is true, the contents of the FIFO are flushed with the next SOF. If the endpoint is set to ISO, data is always transferred with a DATA0 PID."]
    #[inline(always)]
    pub fn usb_toggle_tx(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbTxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbTxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Last Byte\nSetting this bit to 1 indicates that the entire packet has been written into the FIFO. This is used especially for streaming data to the FIFO while the actual transmission occurs. If the LAST bit is not set to 1 and the transmit FIFO becomes empty during a transmission, a stuff error followed by an EOP is forced on the bus. Zero length packets are indicated by setting this bit without writing any data to the FIFO.\nThe transmit state machine transmits the payload data, CRC16 and the EOP signal before clearing this bit."]
    #[inline(always)]
    pub fn usb_last(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbTxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbTxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmission Enable\nThis bit enables data transmission from the FIFO. It is cleared to 0 by hardware after transmitting a single packet or after a STALL handshake in response to an IN token. It must be set to 1 by firmware to start packet transmission."]
    #[inline(always)]
    pub fn usb_tx_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbTxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbTxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTxc1Reg {
    #[inline(always)]
    fn default() -> UsbTxc1Reg {
        <crate::RegValueT<UsbTxc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTxc2Reg_SPEC;
impl crate::sealed::RegSpec for UsbTxc2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Command Register 2"]
pub type UsbTxc2Reg = crate::RegValueT<UsbTxc2Reg_SPEC>;

impl UsbTxc2Reg {
    #[doc = "Ignore ISO Mask\nThis bit has an effect only if the endpoint is set to be isochronous. If set to 1, this bit disables locking of specific frame numbers with the alternate function of the TOGGLE bit. Thus data is transmitted upon reception of the next IN token. If cleared to 0, data is only transmitted when FNL0 matches TOGGLE. This bit is cleared to 0 after reset."]
    #[inline(always)]
    pub fn usb_ign_isomsk(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbTxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbTxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit FIFO Warning Limit\nThese bits specify how many more bytes can be transmitted from the respective FIFO before an underrun condition occurs. If the number of bytes remaining in the FIFO is equal to or less than the selected warning limit, the TXWARN bit in the FWEV register is set. To avoid interrupts caused by setting this bit while the FIFO is being filled before a transmission begins, TXWARN is only set when transmission from the endpoint is enabled (TX_ENn in the TXCn register is set).\nTFWL\\[1:0\\] :\n00: TFWL disabled\n01: Less than 5 bytes remaining in FIFO\n10: Less than 9 bytes remaining in FIFO\n11: Less than 17 bytes remaining in FIFO"]
    #[inline(always)]
    pub fn usb_tfwl(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, UsbTxc2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,UsbTxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refill FIFO\nSetting the LAST bit to 1 automatically saves the Transmit Read Pointer (TXRP) to a buffer. When the RFF bit is set to 1, the buffered TXRP is reloaded into the TXRP. This allows the user to repeat the last transaction if no ACK was received from the host. If the MAC is currently using the FIFO to transmit, TXRP is reloaded only after the transmission is complete. After reload, this bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn usb_rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbTxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbTxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Flush FIFO\nWriting a 1 to this bit flushes all data from the corresponding transmit FIFO, resets the endpoint to Idle state, and clears both the FIFO read and write pointers. If the MAC is currently using the FIFO to transmit, data is flushed after the transmission is complete. After data flushing, this bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbTxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbTxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Toggle\nThe function of this bit differs depending on whether ISO (ISO bit in the EPCn register is set to 1) or non-ISO operation (ISO bit is cleared to 0) is used.\nFor non-ISO operation, it specifies the PID used when transmitting the packet. A value of 0 causes a DATA0 PID to be generated, while a value of 1 causes a DATA1 PID to be generated.\nFor ISO operation, this bit and the LSB of the frame counter (FNL0) act as a mask for the TX_EN bit to allow pre-queuing of packets to specific frame numbers; I.e. transmission is enabled only if bit 0 in the FNL register is set to TOGGLE. If an IN token is not received while this condition is true, the contents of the FIFO are flushed with the next SOF. If the endpoint is set to ISO, data is always transferred with a DATA0 PID."]
    #[inline(always)]
    pub fn usb_toggle_tx(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbTxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbTxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Last Byte\nSetting this bit to 1 indicates that the entire packet has been written into the FIFO. This is used especially for streaming data to the FIFO while the actual transmission occurs. If the LAST bit is not set to 1 and the transmit FIFO becomes empty during a transmission, a stuff error followed by an EOP is forced on the bus. Zero length packets are indicated by setting this bit without writing any data to the FIFO.\nThe transmit state machine transmits the payload data, CRC16 and the EOP signal before clearing this bit."]
    #[inline(always)]
    pub fn usb_last(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbTxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbTxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmission Enable\nThis bit enables data transmission from the FIFO. It is cleared to 0 by hardware after transmitting a single packet or after a STALL handshake in response to an IN token. It must be set to 1 by firmware to start packet transmission."]
    #[inline(always)]
    pub fn usb_tx_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbTxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbTxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTxc2Reg {
    #[inline(always)]
    fn default() -> UsbTxc2Reg {
        <crate::RegValueT<UsbTxc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTxc3Reg_SPEC;
impl crate::sealed::RegSpec for UsbTxc3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Command Register 3"]
pub type UsbTxc3Reg = crate::RegValueT<UsbTxc3Reg_SPEC>;

impl UsbTxc3Reg {
    #[doc = "Ignore ISO Mask\nThis bit has an effect only if the endpoint is set to be isochronous. If set to 1, this bit disables locking of specific frame numbers with the alternate function of the TOGGLE bit. Thus data is transmitted upon reception of the next IN token. If cleared to 0, data is only transmitted when FNL0 matches TOGGLE. This bit is cleared to 0 after reset."]
    #[inline(always)]
    pub fn usb_ign_isomsk(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbTxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbTxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit FIFO Warning Limit\nThese bits specify how many more bytes can be transmitted from the respective FIFO before an underrun condition occurs. If the number of bytes remaining in the FIFO is equal to or less than the selected warning limit, the TXWARN bit in the FWEV register is set. To avoid interrupts caused by setting this bit while the FIFO is being filled before a transmission begins, TXWARN is only set when transmission from the endpoint is enabled (TX_ENn in the TXCn register is set).\nTFWL\\[1:0\\] :\n00: TFWL disabled\n01: Less than 5 bytes remaining in FIFO\n10: Less than 9 bytes remaining in FIFO\n11: Less than 17 bytes remaining in FIFO"]
    #[inline(always)]
    pub fn usb_tfwl(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, UsbTxc3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,UsbTxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refill FIFO\nSetting the LAST bit to 1 automatically saves the Transmit Read Pointer (TXRP) to a buffer. When the RFF bit is set to 1, the buffered TXRP is reloaded into the TXRP. This allows the user to repeat the last transaction if no ACK was received from the host. If the MAC is currently using the FIFO to transmit, TXRP is reloaded only after the transmission is complete. After reload, this bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn usb_rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbTxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbTxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Flush FIFO\nWriting a 1 to this bit flushes all data from the corresponding transmit FIFO, resets the endpoint to Idle state, and clears both the FIFO read and write pointers. If the MAC is currently using the FIFO to transmit, data is flushed after the transmission is complete. After data flushing, this bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbTxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbTxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Toggle\nThe function of this bit differs depending on whether ISO (ISO bit in the EPCn register is set to 1) or non-ISO operation (ISO bit is cleared to 0) is used.\nFor non-ISO operation, it specifies the PID used when transmitting the packet. A value of 0 causes a DATA0 PID to be generated, while a value of 1 causes a DATA1 PID to be generated.\nFor ISO operation, this bit and the LSB of the frame counter (FNL0) act as a mask for the TX_EN bit to allow pre-queuing of packets to specific frame numbers; I.e. transmission is enabled only if bit 0 in the FNL register is set to TOGGLE. If an IN token is not received while this condition is true, the contents of the FIFO are flushed with the next SOF. If the endpoint is set to ISO, data is always transferred with a DATA0 PID."]
    #[inline(always)]
    pub fn usb_toggle_tx(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbTxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbTxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Last Byte\nSetting this bit to 1 indicates that the entire packet has been written into the FIFO. This is used especially for streaming data to the FIFO while the actual transmission occurs. If the LAST bit is not set to 1 and the transmit FIFO becomes empty during a transmission, a stuff error followed by an EOP is forced on the bus. Zero length packets are indicated by setting this bit without writing any data to the FIFO.\nThe transmit state machine transmits the payload data, CRC16 and the EOP signal before clearing this bit."]
    #[inline(always)]
    pub fn usb_last(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbTxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbTxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmission Enable\nThis bit enables data transmission from the FIFO. It is cleared to 0 by hardware after transmitting a single packet or after a STALL handshake in response to an IN token. It must be set to 1 by firmware to start packet transmission."]
    #[inline(always)]
    pub fn usb_tx_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbTxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbTxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTxc3Reg {
    #[inline(always)]
    fn default() -> UsbTxc3Reg {
        <crate::RegValueT<UsbTxc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTxd0Reg_SPEC;
impl crate::sealed::RegSpec for UsbTxd0Reg_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Data 0 Register"]
pub type UsbTxd0Reg = crate::RegValueT<UsbTxd0Reg_SPEC>;

impl UsbTxd0Reg {
    #[doc = "Transmit FIFO Data Byte\nThe firmware is expected to write only the packet payload data. The PID and CRC16 are created automatically."]
    #[inline(always)]
    pub fn usb_txfd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UsbTxd0Reg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,UsbTxd0Reg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTxd0Reg {
    #[inline(always)]
    fn default() -> UsbTxd0Reg {
        <crate::RegValueT<UsbTxd0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTxd1Reg_SPEC;
impl crate::sealed::RegSpec for UsbTxd1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Data Register 1"]
pub type UsbTxd1Reg = crate::RegValueT<UsbTxd1Reg_SPEC>;

impl UsbTxd1Reg {
    #[doc = "Transmit FIFO Data Byte\nThe firmware is expected to write only the packet payload data. PID and CRC16 are inserted automatically in the transmit data stream.\nIn TEST mode this register allow read/write access via the core bus."]
    #[inline(always)]
    pub fn usb_txfd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UsbTxd1Reg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,UsbTxd1Reg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTxd1Reg {
    #[inline(always)]
    fn default() -> UsbTxd1Reg {
        <crate::RegValueT<UsbTxd1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTxd2Reg_SPEC;
impl crate::sealed::RegSpec for UsbTxd2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Data Register 2"]
pub type UsbTxd2Reg = crate::RegValueT<UsbTxd2Reg_SPEC>;

impl UsbTxd2Reg {
    #[doc = "Transmit FIFO Data Byte\nThe firmware is expected to write only the packet payload data. PID and CRC16 are inserted automatically in the transmit data stream.\nIn TEST mode this register allow read/write access via the core bus."]
    #[inline(always)]
    pub fn usb_txfd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UsbTxd2Reg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,UsbTxd2Reg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTxd2Reg {
    #[inline(always)]
    fn default() -> UsbTxd2Reg {
        <crate::RegValueT<UsbTxd2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTxd3Reg_SPEC;
impl crate::sealed::RegSpec for UsbTxd3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Data Register 3"]
pub type UsbTxd3Reg = crate::RegValueT<UsbTxd3Reg_SPEC>;

impl UsbTxd3Reg {
    #[doc = "Transmit FIFO Data Byte\nThe firmware is expected to write only the packet payload data. PID and CRC16 are inserted automatically in the transmit data stream.\nIn TEST mode this register allow read/write access via the core bus."]
    #[inline(always)]
    pub fn usb_txfd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UsbTxd3Reg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,UsbTxd3Reg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTxd3Reg {
    #[inline(always)]
    fn default() -> UsbTxd3Reg {
        <crate::RegValueT<UsbTxd3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTxevReg_SPEC;
impl crate::sealed::RegSpec for UsbTxevReg_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Event Register"]
pub type UsbTxevReg = crate::RegValueT<UsbTxevReg_SPEC>;

impl UsbTxevReg {
    #[doc = "Transmit Underrun n: 3:1\nThe bit n is a copy of the respective TX_URUN bit from the corresponding Transmit Status register (TXSn). Whenever any of the Transmit FIFOs underflows, the respective TXUDRRN bit is set to 1. These bits are cleared to 0 when the corresponding Transmit Status register is read"]
    #[inline(always)]
    pub fn usb_txudrrn31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbTxevReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbTxevReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmit FIFO n: 3:1\nThe bit n is a copy of the TX_DONE bit from the corresponding Transmit Status register (TXSn). A bit is set to 1 when the IN transaction for the corresponding transmit endpoint n has been completed. These bits are cleared to 0 when the corresponding TXSn register is read."]
    #[inline(always)]
    pub fn usb_txfifo31(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, UsbTxevReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,UsbTxevReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTxevReg {
    #[inline(always)]
    fn default() -> UsbTxevReg {
        <crate::RegValueT<UsbTxevReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTxmskReg_SPEC;
impl crate::sealed::RegSpec for UsbTxmskReg_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Mask Register"]
pub type UsbTxmskReg = crate::RegValueT<UsbTxmskReg_SPEC>;

impl UsbTxmskReg {
    #[doc = "The Transmit Mask Register is used to select the bits of the TXEV registers, which causes the TX_EV bit in the MAEV register to be set to 1. When a bit is set to 1 and the corresponding bit in the TXEV register is set to 1, the TX_EV bit in the MAEV register is set to1. When cleared to 0, the corresponding bit in the TXEV register does not cause TX_EV to be set to 1. Same Bit Definition as TXEV Register"]
    #[inline(always)]
    pub fn usb_m_txudrrn31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbTxmskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbTxmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Same Bit Definition as TXEV Register"]
    #[inline(always)]
    pub fn usb_m_txfifo31(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, UsbTxmskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,UsbTxmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTxmskReg {
    #[inline(always)]
    fn default() -> UsbTxmskReg {
        <crate::RegValueT<UsbTxmskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTxs0Reg_SPEC;
impl crate::sealed::RegSpec for UsbTxs0Reg_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Status 0 Register"]
pub type UsbTxs0Reg = crate::RegValueT<UsbTxs0Reg_SPEC>;

impl UsbTxs0Reg {
    #[doc = "Acknowledge Status\nThis bit indicates the status, as received from the host, of the ACK for the packet previously sent. This bit is to be interpreted when TX_DONE is set to 1. It is set to 1, when an ACK is received; otherwise, it remains cleared. This bit is also cleared to 0, when this register is read."]
    #[inline(always)]
    pub fn usb_ack_stat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbTxs0Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbTxs0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmission Done\nWhen set to 1, this bit indicates that a packet has completed transmission. It is cleared to 0, when this register is read."]
    #[inline(always)]
    pub fn usb_tx_done(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbTxs0Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbTxs0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmission Count\nThis 5-bit field indicates the number of empty bytes available in the FIFO. This field is never larger than 8 for Endpoint 0."]
    #[inline(always)]
    pub fn usb_tcount(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, UsbTxs0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,UsbTxs0Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTxs0Reg {
    #[inline(always)]
    fn default() -> UsbTxs0Reg {
        <crate::RegValueT<UsbTxs0Reg_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTxs1Reg_SPEC;
impl crate::sealed::RegSpec for UsbTxs1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Status Register 1"]
pub type UsbTxs1Reg = crate::RegValueT<UsbTxs1Reg_SPEC>;

impl UsbTxs1Reg {
    #[doc = "Transmit FIFO Underrun\nThis bit is set to 1, if the transmit FIFO becomes empty during a transmission, and no new data is written to the FIFO. If so, the Media Access Controller (MAC) forces a bit stuff error followed by an EOP. This bit is cleared to 0, when this register is read."]
    #[inline(always)]
    pub fn usb_tx_urun(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbTxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbTxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Acknowledge Status\nThis bit is interpreted when TX_DONE is set. It\'s function differs depending on whether ISO (ISO in the EPCx register is set) or non-ISO operation (ISO is reset) is used.\nFor non-ISO operation, this bit indicates the acknowledge status (from the host) about the ACK for the previously sent packet. This bit itself is set to 1, when an ACK is received; otherwise, it is cleared to 0.\nFor ISO operation, this bit is set if a frame number LSB match (see IGN_ISOMSK bit in the USB_TXCx_REG) occurs, and data was sent in response to an IN token. Otherwise, this bit is cleared to 0, the FIFO is flushed and TX_DONE is set.\nThis bit is also cleared to 0, when this register is read."]
    #[inline(always)]
    pub fn usb_ack_stat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbTxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbTxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmission Done\nWhen set to 1, this bit indicates that the endpoint responded to a USB packet. Three conditions can cause this bit to be set:\nA data packet completed transmission in response to an IN token with non-ISO operation.\nThe endpoint sent a STALL handshake in response to an IN token\nA scheduled ISO frame was transmitted or discarded.\nThis bit is cleared to 0 when this register is read."]
    #[inline(always)]
    pub fn usb_tx_done(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbTxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbTxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmission Count\nThis 5-bit field holds the number of empty bytes available in the FIFO. If this number is greater than 31, a value of 31 is actually reported."]
    #[inline(always)]
    pub fn usb_tcount(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, UsbTxs1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,UsbTxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTxs1Reg {
    #[inline(always)]
    fn default() -> UsbTxs1Reg {
        <crate::RegValueT<UsbTxs1Reg_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTxs2Reg_SPEC;
impl crate::sealed::RegSpec for UsbTxs2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Status Register 2"]
pub type UsbTxs2Reg = crate::RegValueT<UsbTxs2Reg_SPEC>;

impl UsbTxs2Reg {
    #[doc = "Transmit FIFO Underrun\nThis bit is set to 1, if the transmit FIFO becomes empty during a transmission, and no new data is written to the FIFO. If so, the Media Access Controller (MAC) forces a bit stuff error followed by an EOP. This bit is cleared to 0, when this register is read."]
    #[inline(always)]
    pub fn usb_tx_urun(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbTxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbTxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Acknowledge Status\nThis bit is interpreted when TX_DONE is set. It\'s function differs depending on whether ISO (ISO in the EPCx register is set) or non-ISO operation (ISO is reset) is used.\nFor non-ISO operation, this bit indicates the acknowledge status (from the host) about the ACK for the previously sent packet. This bit itself is set to 1, when an ACK is received; otherwise, it is cleared to 0.\nFor ISO operation, this bit is set if a frame number LSB match (see IGN_ISOMSK bit in the USB_TXCx_REG) occurs, and data was sent in response to an IN token. Otherwise, this bit is cleared to 0, the FIFO is flushed and TX_DONE is set.\nThis bit is also cleared to 0, when this register is read."]
    #[inline(always)]
    pub fn usb_ack_stat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbTxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbTxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmission Done\nWhen set to 1, this bit indicates that the endpoint responded to a USB packet. Three conditions can cause this bit to be set:\nA data packet completed transmission in response to an IN token with non-ISO operation.\nThe endpoint sent a STALL handshake in response to an IN token\nA scheduled ISO frame was transmitted or discarded.\nThis bit is cleared to 0 when this register is read."]
    #[inline(always)]
    pub fn usb_tx_done(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbTxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbTxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmission Count\nThis 5-bit field holds the number of empty bytes available in the FIFO. If this number is greater than 31, a value of 31 is actually reported."]
    #[inline(always)]
    pub fn usb_tcount(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, UsbTxs2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,UsbTxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTxs2Reg {
    #[inline(always)]
    fn default() -> UsbTxs2Reg {
        <crate::RegValueT<UsbTxs2Reg_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbTxs3Reg_SPEC;
impl crate::sealed::RegSpec for UsbTxs3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Status Register 3"]
pub type UsbTxs3Reg = crate::RegValueT<UsbTxs3Reg_SPEC>;

impl UsbTxs3Reg {
    #[doc = "Transmit FIFO Underrun\nThis bit is set to 1, if the transmit FIFO becomes empty during a transmission, and no new data is written to the FIFO. If so, the Media Access Controller (MAC) forces a bit stuff error followed by an EOP. This bit is cleared to 0, when this register is read."]
    #[inline(always)]
    pub fn usb_tx_urun(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbTxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbTxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Acknowledge Status\nThis bit is interpreted when TX_DONE is set. It\'s function differs depending on whether ISO (ISO in the EPCx register is set) or non-ISO operation (ISO is reset) is used.\nFor non-ISO operation, this bit indicates the acknowledge status (from the host) about the ACK for the previously sent packet. This bit itself is set to 1, when an ACK is received; otherwise, it is cleared to 0.\nFor ISO operation, this bit is set if a frame number LSB match (see IGN_ISOMSK bit in the USB_TXCx_REG) occurs, and data was sent in response to an IN token. Otherwise, this bit is cleared to 0, the FIFO is flushed and TX_DONE is set.\nThis bit is also cleared to 0, when this register is read."]
    #[inline(always)]
    pub fn usb_ack_stat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbTxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbTxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmission Done\nWhen set to 1, this bit indicates that the endpoint responded to a USB packet. Three conditions can cause this bit to be set:\nA data packet completed transmission in response to an IN token with non-ISO operation.\nThe endpoint sent a STALL handshake in response to an IN token\nA scheduled ISO frame was transmitted or discarded.\nThis bit is cleared to 0 when this register is read."]
    #[inline(always)]
    pub fn usb_tx_done(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbTxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbTxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmission Count\nThis 5-bit field holds the number of empty bytes available in the FIFO. If this number is greater than 31, a value of 31 is actually reported."]
    #[inline(always)]
    pub fn usb_tcount(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, UsbTxs3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,UsbTxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbTxs3Reg {
    #[inline(always)]
    fn default() -> UsbTxs3Reg {
        <crate::RegValueT<UsbTxs3Reg_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbUtrReg_SPEC;
impl crate::sealed::RegSpec for UsbUtrReg_SPEC {
    type DataType = u32;
}

#[doc = "USB test Register (for test purpose only)"]
pub type UsbUtrReg = crate::RegValueT<UsbUtrReg_SPEC>;

impl UsbUtrReg {
    #[doc = "Diagnostic enable\n\'0\': Normal operational.\n\'1\': Access to the USB_XCVDIAG_REG and USB_TCR_REG enabled. For diagnostic purposes only"]
    #[inline(always)]
    pub fn usb_diag(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbUtrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbUtrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "No CRC16\nWhen this bit is set to 1, all packets transmitted by the Full/Low Speed USB node are sent without a trailing CRC16. Receive operations are unaffected. This mode is used to check that CRC errors can be detected by other nodes. For diagnostic purposes only"]
    #[inline(always)]
    pub fn usb_ncrc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbUtrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbUtrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Short Frame\nEnables the Frame timer to lock and track, short, non-compliant USB frame sizes. The Short Frame bit should not be set during normal operation. For test purposes only"]
    #[inline(always)]
    pub fn usb_sf(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbUtrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbUtrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Reserved. Must be kept to \'0\'"]
    #[inline(always)]
    pub fn usb_utr_res(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, UsbUtrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,UsbUtrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbUtrReg {
    #[inline(always)]
    fn default() -> UsbUtrReg {
        <crate::RegValueT<UsbUtrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbUx20CdrReg_SPEC;
impl crate::sealed::RegSpec for UsbUx20CdrReg_SPEC {
    type DataType = u32;
}

#[doc = "Transceiver 2.0 Configuration and Diagnostics Register(for test purpose only)"]
pub type UsbUx20CdrReg = crate::RegValueT<UsbUx20CdrReg_SPEC>;

impl UsbUx20CdrReg {
    #[doc = "Test bit"]
    #[inline(always)]
    pub fn rpu_test7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbUx20CdrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbUx20CdrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: Closes SW2 switch to reduced pull-up resistor connected to the USB_Dp and USB_Dm.\n1: Opens SW2 switch resistor connected to the USB_Dp and USB_Dm (independent of the VBus state)."]
    #[inline(always)]
    pub fn rpu_test_sw2(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbUx20CdrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbUx20CdrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Enable the pull-up resistor on USB_Dp (SW1 closed)\n1: Disable the pull-up resistor on USB_Dp (SW1 open) (Independent of the VBus state)."]
    #[inline(always)]
    pub fn rpu_test_sw1(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbUx20CdrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbUx20CdrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pull-Up Resistor Test Enable\n0: Normal operation\n1: Enables the test features controlled by RPU_TEST_SW1, RPU_TEST_SW1DM and RPU_TEST_SW2"]
    #[inline(always)]
    pub fn rpu_test_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbUx20CdrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbUx20CdrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Enable the pull-up resistor on USB_Dm (SW1DM closed)\n1: Disable the pull-up resistor on USB_Dm (SW1DM open) (Independent of the VBus state)."]
    #[inline(always)]
    pub fn rpu_test_sw1dm(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbUx20CdrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbUx20CdrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Test bit, must be kept 0"]
    #[inline(always)]
    pub fn rpu_rcdelay(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbUx20CdrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbUx20CdrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Test bit, must be kept 0"]
    #[inline(always)]
    pub fn rpu_ssproten(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbUx20CdrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbUx20CdrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbUx20CdrReg {
    #[inline(always)]
    fn default() -> UsbUx20CdrReg {
        <crate::RegValueT<UsbUx20CdrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbXcvdiagReg_SPEC;
impl crate::sealed::RegSpec for UsbXcvdiagReg_SPEC {
    type DataType = u32;
}

#[doc = "Transceiver diagnostic Register (for test purpose only)"]
pub type UsbXcvdiagReg = crate::RegValueT<UsbXcvdiagReg_SPEC>;

impl UsbXcvdiagReg {
    #[doc = "With Bit0 = 1 this bit shows the level of the USB_Dp receive data from transceiver; i.e. D+ <= VSE."]
    #[inline(always)]
    pub fn usb_vpin(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbXcvdiagReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbXcvdiagReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "With Bit0 = 1 this bit shows the level USB_Dm receive data from transceiver; i.e. D- <= VSE."]
    #[inline(always)]
    pub fn usb_vmin(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbXcvdiagReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbXcvdiagReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "With Bit0 = 1 this bit shows the differential level of the receive comparator."]
    #[inline(always)]
    pub fn usb_rcv(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbXcvdiagReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbXcvdiagReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "With Bit0 = 1, this bit enables test Bits 2,1. Must be kept to \'0\' for normal operation"]
    #[inline(always)]
    pub fn usb_xcv_txen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbXcvdiagReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbXcvdiagReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "With Bit3,0 = 1, this bit sets USB_Dm to a high level, independent of LSMODE selection"]
    #[inline(always)]
    pub fn usb_xcv_txn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbXcvdiagReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbXcvdiagReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "With Bit3,0 = 1, this bit sets USB_Dp to a high level, independent of LSMODE selection"]
    #[inline(always)]
    pub fn usb_xcv_txp(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbXcvdiagReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbXcvdiagReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable USB_XCVDIAG_REG\n0: Normal operation, test bits disabled\n1: Enable test bits 7,6,5,3,2,1"]
    #[inline(always)]
    pub fn usb_xcv_test(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbXcvdiagReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbXcvdiagReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbXcvdiagReg {
    #[inline(always)]
    fn default() -> UsbXcvdiagReg {
        <crate::RegValueT<UsbXcvdiagReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
