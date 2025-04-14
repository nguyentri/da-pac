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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:28 +0000

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

pub type UsbAltevReg = crate::RegValueT<UsbAltevReg_SPEC>;

impl UsbAltevReg {
    #[inline(always)]
    pub fn usb_resume(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbAltevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbAltevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_reset(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbAltevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbAltevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_sd5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbAltevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbAltevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_sd3(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbAltevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbAltevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbAltmskReg = crate::RegValueT<UsbAltmskReg_SPEC>;

impl UsbAltmskReg {
    #[inline(always)]
    pub fn usb_m_resume(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbAltmskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbAltmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_m_reset(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbAltmskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbAltmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_m_sd5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbAltmskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbAltmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_m_sd3(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbAltmskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbAltmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbChargerCtrlReg = crate::RegValueT<UsbChargerCtrlReg_SPEC>;

impl UsbChargerCtrlReg {
    #[inline(always)]
    pub fn idm_sink_on(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn idp_sink_on(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vdm_src_on(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vdp_src_on(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn idp_src_on(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbChargerStatReg = crate::RegValueT<UsbChargerStatReg_SPEC>;

impl UsbChargerStatReg {
    #[inline(always)]
    pub fn usb_dm_val2(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbChargerStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbChargerStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_dp_val2(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbChargerStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,UsbChargerStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_dm_val(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbChargerStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,UsbChargerStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_dp_val(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbChargerStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,UsbChargerStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_chg_det(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbChargerStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,UsbChargerStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type UsbDmaCtrlReg = crate::RegValueT<UsbDmaCtrlReg_SPEC>;

impl UsbDmaCtrlReg {
    #[inline(always)]
    pub fn usb_dma_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbDmaCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbDmaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_dma_tx(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, UsbDmaCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,UsbDmaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbEp0NakReg = crate::RegValueT<UsbEp0NakReg_SPEC>;

impl UsbEp0NakReg {
    #[inline(always)]
    pub fn usb_ep0_outnak(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbEp0NakReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,UsbEp0NakReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type UsbEpc0Reg = crate::RegValueT<UsbEpc0Reg_SPEC>;

impl UsbEpc0Reg {
    #[inline(always)]
    pub fn usb_stall(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbEpc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbEpc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_def(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbEpc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbEpc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbEpc1Reg = crate::RegValueT<UsbEpc1Reg_SPEC>;

impl UsbEpc1Reg {
    #[inline(always)]
    pub fn usb_stall(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbEpc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbEpc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_iso(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbEpc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbEpc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ep_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbEpc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbEpc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbEpc2Reg = crate::RegValueT<UsbEpc2Reg_SPEC>;

impl UsbEpc2Reg {
    #[inline(always)]
    pub fn usb_stall(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbEpc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbEpc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_iso(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbEpc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbEpc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ep_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbEpc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbEpc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbEpc3Reg = crate::RegValueT<UsbEpc3Reg_SPEC>;

impl UsbEpc3Reg {
    #[inline(always)]
    pub fn usb_stall(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbEpc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbEpc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_iso(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbEpc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbEpc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ep_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbEpc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbEpc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbEpc4Reg = crate::RegValueT<UsbEpc4Reg_SPEC>;

impl UsbEpc4Reg {
    #[inline(always)]
    pub fn usb_stall(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbEpc4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbEpc4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_iso(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbEpc4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbEpc4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ep_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbEpc4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbEpc4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbEpc5Reg = crate::RegValueT<UsbEpc5Reg_SPEC>;

impl UsbEpc5Reg {
    #[inline(always)]
    pub fn usb_stall(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbEpc5Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbEpc5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_iso(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbEpc5Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbEpc5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ep_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbEpc5Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbEpc5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbEpc6Reg = crate::RegValueT<UsbEpc6Reg_SPEC>;

impl UsbEpc6Reg {
    #[inline(always)]
    pub fn usb_stall(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbEpc6Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbEpc6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_iso(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbEpc6Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbEpc6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ep_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbEpc6Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbEpc6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbFarReg = crate::RegValueT<UsbFarReg_SPEC>;

impl UsbFarReg {
    #[inline(always)]
    pub fn usb_ad_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbFarReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbFarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbFnhReg = crate::RegValueT<UsbFnhReg_SPEC>;

impl UsbFnhReg {
    #[inline(always)]
    pub fn usb_mf(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbFnhReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, UsbFnhReg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn usb_ul(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbFnhReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, UsbFnhReg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn usb_rfc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbFnhReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, UsbFnhReg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

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

pub type UsbFnlReg = crate::RegValueT<UsbFnlReg_SPEC>;

impl UsbFnlReg {
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

pub type UsbFwevReg = crate::RegValueT<UsbFwevReg_SPEC>;

impl UsbFwevReg {
    #[inline(always)]
    pub fn usb_rxwarn31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbFwevReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbFwevReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type UsbFwmskReg = crate::RegValueT<UsbFwmskReg_SPEC>;

impl UsbFwmskReg {
    #[inline(always)]
    pub fn usb_m_rxwarn31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbFwmskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbFwmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbMaevReg = crate::RegValueT<UsbMaevReg_SPEC>;

impl UsbMaevReg {
    #[inline(always)]
    pub fn usb_ch_ev(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ep0_nak(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ep0_rx(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ep0_tx(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_intr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_rx_ev(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_uld(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_nak(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_frame(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_tx_ev(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_alt(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbMaevReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbMaevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbMamskReg = crate::RegValueT<UsbMamskReg_SPEC>;

impl UsbMamskReg {
    #[inline(always)]
    pub fn usb_m_ch_ev(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_m_ep0_nak(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_m_ep0_rx(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_m_ep0_tx(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_m_intr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_m_rx_ev(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_m_uld(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_m_nak(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_m_frame(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_m_tx_ev(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_m_alt(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbMamskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbMamskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbMctrlReg = crate::RegValueT<UsbMctrlReg_SPEC>;

impl UsbMctrlReg {
    #[inline(always)]
    pub fn lsmode(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbMctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbMctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_nat(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbMctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbMctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_dbg(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbMctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbMctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbNakevReg = crate::RegValueT<UsbNakevReg_SPEC>;

impl UsbNakevReg {
    #[inline(always)]
    pub fn usb_out31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbNakevReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbNakevReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type UsbNakmskReg = crate::RegValueT<UsbNakmskReg_SPEC>;

impl UsbNakmskReg {
    #[inline(always)]
    pub fn usb_m_out31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbNakmskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbNakmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbNfsrReg = crate::RegValueT<UsbNfsrReg_SPEC>;

impl UsbNfsrReg {
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

pub type UsbRxc0Reg = crate::RegValueT<UsbRxc0Reg_SPEC>;

impl UsbRxc0Reg {
    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbRxc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbRxc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ign_setup(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbRxc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbRxc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ign_out(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbRxc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbRxc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbRxc1Reg = crate::RegValueT<UsbRxc1Reg_SPEC>;

impl UsbRxc1Reg {
    #[inline(always)]
    pub fn usb_rfwl(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, UsbRxc1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,UsbRxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbRxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbRxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ign_setup(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbRxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbRxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbRxc2Reg = crate::RegValueT<UsbRxc2Reg_SPEC>;

impl UsbRxc2Reg {
    #[inline(always)]
    pub fn usb_rfwl(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, UsbRxc2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,UsbRxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbRxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbRxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ign_setup(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbRxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbRxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbRxc3Reg = crate::RegValueT<UsbRxc3Reg_SPEC>;

impl UsbRxc3Reg {
    #[inline(always)]
    pub fn usb_rfwl(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, UsbRxc3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,UsbRxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbRxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbRxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ign_setup(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbRxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbRxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbRxd0Reg = crate::RegValueT<UsbRxd0Reg_SPEC>;

impl UsbRxd0Reg {
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

pub type UsbRxd1Reg = crate::RegValueT<UsbRxd1Reg_SPEC>;

impl UsbRxd1Reg {
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

pub type UsbRxd2Reg = crate::RegValueT<UsbRxd2Reg_SPEC>;

impl UsbRxd2Reg {
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

pub type UsbRxd3Reg = crate::RegValueT<UsbRxd3Reg_SPEC>;

impl UsbRxd3Reg {
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

pub type UsbRxevReg = crate::RegValueT<UsbRxevReg_SPEC>;

impl UsbRxevReg {
    #[inline(always)]
    pub fn usb_rxovrrn31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbRxevReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbRxevReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type UsbRxmskReg = crate::RegValueT<UsbRxmskReg_SPEC>;

impl UsbRxmskReg {
    #[inline(always)]
    pub fn usb_m_rxovrrn31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbRxmskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbRxmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbRxs0Reg = crate::RegValueT<UsbRxs0Reg_SPEC>;

impl UsbRxs0Reg {
    #[inline(always)]
    pub fn usb_setup(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbRxs0Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbRxs0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_toggle_rx0(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbRxs0Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbRxs0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_rx_last(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbRxs0Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,UsbRxs0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type UsbRxs1Reg = crate::RegValueT<UsbRxs1Reg_SPEC>;

impl UsbRxs1Reg {
    #[inline(always)]
    pub fn usb_rxcount(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, UsbRxs1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,UsbRxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_rx_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbRxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbRxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_setup(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbRxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbRxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_toggle_rx(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbRxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbRxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_rx_last(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbRxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,UsbRxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type UsbRxs2Reg = crate::RegValueT<UsbRxs2Reg_SPEC>;

impl UsbRxs2Reg {
    #[inline(always)]
    pub fn usb_rxcount(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, UsbRxs2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,UsbRxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_rx_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbRxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbRxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_setup(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbRxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbRxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_toggle_rx(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbRxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbRxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_rx_last(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbRxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,UsbRxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type UsbRxs3Reg = crate::RegValueT<UsbRxs3Reg_SPEC>;

impl UsbRxs3Reg {
    #[inline(always)]
    pub fn usb_rxcount(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, UsbRxs3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,UsbRxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_rx_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbRxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbRxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_setup(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbRxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbRxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_toggle_rx(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbRxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbRxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_rx_last(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbRxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,UsbRxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type UsbTcrReg = crate::RegValueT<UsbTcrReg_SPEC>;

impl UsbTcrReg {
    #[inline(always)]
    pub fn usb_vadj(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, UsbTcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,UsbTcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbTxc0Reg = crate::RegValueT<UsbTxc0Reg_SPEC>;

impl UsbTxc0Reg {
    #[inline(always)]
    pub fn usb_ign_in(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbTxc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbTxc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbTxc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbTxc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_toggle_tx0(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbTxc0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbTxc0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbTxc1Reg = crate::RegValueT<UsbTxc1Reg_SPEC>;

impl UsbTxc1Reg {
    #[inline(always)]
    pub fn usb_ign_isomsk(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbTxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbTxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_tfwl(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, UsbTxc1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,UsbTxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbTxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbTxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbTxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbTxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_toggle_tx(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbTxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbTxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_last(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbTxc1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbTxc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbTxc2Reg = crate::RegValueT<UsbTxc2Reg_SPEC>;

impl UsbTxc2Reg {
    #[inline(always)]
    pub fn usb_ign_isomsk(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbTxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbTxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_tfwl(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, UsbTxc2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,UsbTxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbTxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbTxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbTxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbTxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_toggle_tx(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbTxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbTxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_last(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbTxc2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbTxc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbTxc3Reg = crate::RegValueT<UsbTxc3Reg_SPEC>;

impl UsbTxc3Reg {
    #[inline(always)]
    pub fn usb_ign_isomsk(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbTxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbTxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_tfwl(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, UsbTxc3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,UsbTxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbTxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbTxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_flush(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbTxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbTxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_toggle_tx(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbTxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbTxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_last(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbTxc3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbTxc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbTxd0Reg = crate::RegValueT<UsbTxd0Reg_SPEC>;

impl UsbTxd0Reg {
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

pub type UsbTxd1Reg = crate::RegValueT<UsbTxd1Reg_SPEC>;

impl UsbTxd1Reg {
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

pub type UsbTxd2Reg = crate::RegValueT<UsbTxd2Reg_SPEC>;

impl UsbTxd2Reg {
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

pub type UsbTxd3Reg = crate::RegValueT<UsbTxd3Reg_SPEC>;

impl UsbTxd3Reg {
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

pub type UsbTxevReg = crate::RegValueT<UsbTxevReg_SPEC>;

impl UsbTxevReg {
    #[inline(always)]
    pub fn usb_txudrrn31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbTxevReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbTxevReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type UsbTxmskReg = crate::RegValueT<UsbTxmskReg_SPEC>;

impl UsbTxmskReg {
    #[inline(always)]
    pub fn usb_m_txudrrn31(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UsbTxmskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UsbTxmskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbTxs0Reg = crate::RegValueT<UsbTxs0Reg_SPEC>;

impl UsbTxs0Reg {
    #[inline(always)]
    pub fn usb_ack_stat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbTxs0Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbTxs0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_tx_done(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbTxs0Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbTxs0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type UsbTxs1Reg = crate::RegValueT<UsbTxs1Reg_SPEC>;

impl UsbTxs1Reg {
    #[inline(always)]
    pub fn usb_tx_urun(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbTxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbTxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ack_stat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbTxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbTxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_tx_done(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbTxs1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbTxs1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type UsbTxs2Reg = crate::RegValueT<UsbTxs2Reg_SPEC>;

impl UsbTxs2Reg {
    #[inline(always)]
    pub fn usb_tx_urun(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbTxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbTxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ack_stat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbTxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbTxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_tx_done(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbTxs2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbTxs2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type UsbTxs3Reg = crate::RegValueT<UsbTxs3Reg_SPEC>;

impl UsbTxs3Reg {
    #[inline(always)]
    pub fn usb_tx_urun(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbTxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbTxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ack_stat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbTxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbTxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_tx_done(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbTxs3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbTxs3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type UsbUtrReg = crate::RegValueT<UsbUtrReg_SPEC>;

impl UsbUtrReg {
    #[inline(always)]
    pub fn usb_diag(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbUtrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UsbUtrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_ncrc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbUtrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbUtrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_sf(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbUtrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbUtrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbUx20CdrReg = crate::RegValueT<UsbUx20CdrReg_SPEC>;

impl UsbUx20CdrReg {
    #[inline(always)]
    pub fn rpu_test7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbUx20CdrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbUx20CdrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rpu_test_sw2(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbUx20CdrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UsbUx20CdrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rpu_test_sw1(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbUx20CdrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,UsbUx20CdrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rpu_test_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UsbUx20CdrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UsbUx20CdrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rpu_test_sw1dm(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbUx20CdrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbUx20CdrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rpu_rcdelay(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbUx20CdrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbUx20CdrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type UsbXcvdiagReg = crate::RegValueT<UsbXcvdiagReg_SPEC>;

impl UsbXcvdiagReg {
    #[inline(always)]
    pub fn usb_vpin(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UsbXcvdiagReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UsbXcvdiagReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_vmin(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UsbXcvdiagReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UsbXcvdiagReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_rcv(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UsbXcvdiagReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UsbXcvdiagReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_xcv_txen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UsbXcvdiagReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UsbXcvdiagReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_xcv_txn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbXcvdiagReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbXcvdiagReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_xcv_txp(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbXcvdiagReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbXcvdiagReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
