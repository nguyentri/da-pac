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
#[doc = r"OQSPIF registers"]
unsafe impl ::core::marker::Send for super::Oqspif {}
unsafe impl ::core::marker::Sync for super::Oqspif {}
impl super::Oqspif {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn oqspif_burstbrk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifBurstbrkReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifBurstbrkReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_burstcmda_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifBurstcmdaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifBurstcmdaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_burstcmdb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifBurstcmdbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifBurstcmdbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_chckerase_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifChckeraseReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifChckeraseReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctrlbus_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrlbusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrlbusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctrlmode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrlmodeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctr_eaddr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrEaddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrEaddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctr_key_0_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey03Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey03Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(276usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctr_key_12_15_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey1215Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey1215Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctr_key_16_19_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey1619Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey1619Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctr_key_20_23_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey2023Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey2023Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(296usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctr_key_24_27_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey2427Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey2427Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(300usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctr_key_28_31_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey2831Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey2831Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctr_key_4_7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey47Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey47Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(280usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctr_key_8_11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey811Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey811Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(284usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctr_nonce_0_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrNonce03Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrNonce03Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctr_nonce_4_7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrNonce47Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrNonce47Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_ctr_saddr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrSaddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrSaddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_dummydata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifDummydataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifDummydataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_erasecmda_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifErasecmdaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifErasecmdaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_erasecmdb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifErasecmdbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifErasecmdbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_erasecmdc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifErasecmdcReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifErasecmdcReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_erasectrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifErasectrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifErasectrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_gp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifGpReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifGpReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_readdata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifReaddataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifReaddataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_recvdata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifRecvdataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifRecvdataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_statuscmd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifStatuscmdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifStatuscmdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn oqspif_writedata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifWritedataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifWritedataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifBurstbrkReg_SPEC;
impl crate::sealed::RegSpec for OqspifBurstbrkReg_SPEC {
    type DataType = u32;
}

pub type OqspifBurstbrkReg = crate::RegValueT<OqspifBurstbrkReg_SPEC>;

impl OqspifBurstbrkReg {
    #[inline(always)]
    pub fn ospic_brk_en(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, OqspifBurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,OqspifBurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_sec_hf_ds(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, OqspifBurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,OqspifBurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_brk_tx_md(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifBurstbrkReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifBurstbrkReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_brk_sz(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        u8,
        u8,
        OqspifBurstbrkReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            u8,
            u8,
            OqspifBurstbrkReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_brk_wrd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        OqspifBurstbrkReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            OqspifBurstbrkReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifBurstbrkReg {
    #[inline(always)]
    fn default() -> OqspifBurstbrkReg {
        <crate::RegValueT<OqspifBurstbrkReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifBurstcmdaReg_SPEC;
impl crate::sealed::RegSpec for OqspifBurstcmdaReg_SPEC {
    type DataType = u32;
}

pub type OqspifBurstcmdaReg = crate::RegValueT<OqspifBurstcmdaReg_SPEC>;

impl OqspifBurstcmdaReg {
    #[inline(always)]
    pub fn ospic_dmy_tx_md(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifBurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifBurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_ext_tx_md(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifBurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifBurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_adr_tx_md(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifBurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifBurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_inst_tx_md(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifBurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifBurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_ext_byte(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        OqspifBurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            OqspifBurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_inst_wb(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        OqspifBurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            OqspifBurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_inst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        OqspifBurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            OqspifBurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifBurstcmdaReg {
    #[inline(always)]
    fn default() -> OqspifBurstcmdaReg {
        <crate::RegValueT<OqspifBurstcmdaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifBurstcmdbReg_SPEC;
impl crate::sealed::RegSpec for OqspifBurstcmdbReg_SPEC {
    type DataType = u32;
}

pub type OqspifBurstcmdbReg = crate::RegValueT<OqspifBurstcmdbReg_SPEC>;

impl OqspifBurstcmdbReg {
    #[inline(always)]
    pub fn ospic_cs_high_min(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        u8,
        u8,
        OqspifBurstcmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            u8,
            u8,
            OqspifBurstcmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_wrap_size(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifBurstcmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifBurstcmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_wrap_len(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifBurstcmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifBurstcmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_wrap_md(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, OqspifBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_inst_md(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, OqspifBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_dmy_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, OqspifBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_dmy_num(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1f,
        1,
        0,
        u8,
        u8,
        OqspifBurstcmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1f,
            1,
            0,
            u8,
            u8,
            OqspifBurstcmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_ext_hf_ds(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, OqspifBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_ext_byte_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, OqspifBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_dat_rx_md(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifBurstcmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifBurstcmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifBurstcmdbReg {
    #[inline(always)]
    fn default() -> OqspifBurstcmdbReg {
        <crate::RegValueT<OqspifBurstcmdbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifChckeraseReg_SPEC;
impl crate::sealed::RegSpec for OqspifChckeraseReg_SPEC {
    type DataType = u32;
}

pub type OqspifChckeraseReg = crate::RegValueT<OqspifChckeraseReg_SPEC>;

impl OqspifChckeraseReg {
    #[inline(always)]
    pub fn ospic_chckerase(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifChckeraseReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifChckeraseReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifChckeraseReg {
    #[inline(always)]
    fn default() -> OqspifChckeraseReg {
        <crate::RegValueT<OqspifChckeraseReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrlbusReg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrlbusReg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrlbusReg = crate::RegValueT<OqspifCtrlbusReg_SPEC>;

impl OqspifCtrlbusReg {
    #[inline(always)]
    pub fn ospic_dis_cs(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, OqspifCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,OqspifCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_en_cs(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, OqspifCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,OqspifCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_set_octal(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, OqspifCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,OqspifCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_set_quad(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, OqspifCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,OqspifCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_set_dual(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, OqspifCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,OqspifCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_set_single(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, OqspifCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,OqspifCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifCtrlbusReg {
    #[inline(always)]
    fn default() -> OqspifCtrlbusReg {
        <crate::RegValueT<OqspifCtrlbusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrlmodeReg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrlmodeReg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrlmodeReg = crate::RegValueT<OqspifCtrlmodeReg_SPEC>;

impl OqspifCtrlmodeReg {
    #[inline(always)]
    pub fn ospic_io_uh_dat(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        u8,
        u8,
        OqspifCtrlmodeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            u8,
            u8,
            OqspifCtrlmodeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_io_uh_oen(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_inc_lim_en(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_rd_err_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_man_dirchg_md(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_dmy_md(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_cmd_x2_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_use_32ba(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_buf_lim_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_pclk_md(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, OqspifCtrlmodeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            u8,
            u8,
            OqspifCtrlmodeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_rpipe_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_rxd_neg(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_hrdy_md(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_io3_dat(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_io2_dat(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_io3_oen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_io2_oen(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_clk_md(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_auto_md(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifCtrlmodeReg {
    #[inline(always)]
    fn default() -> OqspifCtrlmodeReg {
        <crate::RegValueT<OqspifCtrlmodeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrCtrlReg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrCtrlReg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrCtrlReg = crate::RegValueT<OqspifCtrCtrlReg_SPEC>;

impl OqspifCtrCtrlReg {
    #[inline(always)]
    pub fn ospic_ctr_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, OqspifCtrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,OqspifCtrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifCtrCtrlReg {
    #[inline(always)]
    fn default() -> OqspifCtrCtrlReg {
        <crate::RegValueT<OqspifCtrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrEaddrReg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrEaddrReg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrEaddrReg = crate::RegValueT<OqspifCtrEaddrReg_SPEC>;

impl OqspifCtrEaddrReg {
    #[inline(always)]
    pub fn ospic_ctr_eaddr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3fffff,
        1,
        0,
        u32,
        u32,
        OqspifCtrEaddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3fffff,
            1,
            0,
            u32,
            u32,
            OqspifCtrEaddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrEaddrReg {
    #[inline(always)]
    fn default() -> OqspifCtrEaddrReg {
        <crate::RegValueT<OqspifCtrEaddrReg_SPEC> as RegisterValue<_>>::new(1023)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey03Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey03Reg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrKey03Reg = crate::RegValueT<OqspifCtrKey03Reg_SPEC>;

impl OqspifCtrKey03Reg {
    #[inline(always)]
    pub fn ospic_ctr_key_0_3(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifCtrKey03Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifCtrKey03Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey03Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey03Reg {
        <crate::RegValueT<OqspifCtrKey03Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey1215Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey1215Reg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrKey1215Reg = crate::RegValueT<OqspifCtrKey1215Reg_SPEC>;

impl OqspifCtrKey1215Reg {
    #[inline(always)]
    pub fn ospic_ctr_key_12_15(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifCtrKey1215Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifCtrKey1215Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey1215Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey1215Reg {
        <crate::RegValueT<OqspifCtrKey1215Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey1619Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey1619Reg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrKey1619Reg = crate::RegValueT<OqspifCtrKey1619Reg_SPEC>;

impl OqspifCtrKey1619Reg {
    #[inline(always)]
    pub fn ospic_ctr_key_16_19(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifCtrKey1619Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifCtrKey1619Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey1619Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey1619Reg {
        <crate::RegValueT<OqspifCtrKey1619Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey2023Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey2023Reg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrKey2023Reg = crate::RegValueT<OqspifCtrKey2023Reg_SPEC>;

impl OqspifCtrKey2023Reg {
    #[inline(always)]
    pub fn ospic_ctr_key_20_23(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifCtrKey2023Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifCtrKey2023Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey2023Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey2023Reg {
        <crate::RegValueT<OqspifCtrKey2023Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey2427Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey2427Reg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrKey2427Reg = crate::RegValueT<OqspifCtrKey2427Reg_SPEC>;

impl OqspifCtrKey2427Reg {
    #[inline(always)]
    pub fn ospic_ctr_key_24_27(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifCtrKey2427Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifCtrKey2427Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey2427Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey2427Reg {
        <crate::RegValueT<OqspifCtrKey2427Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey2831Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey2831Reg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrKey2831Reg = crate::RegValueT<OqspifCtrKey2831Reg_SPEC>;

impl OqspifCtrKey2831Reg {
    #[inline(always)]
    pub fn ospic_ctr_key_28_31(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifCtrKey2831Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifCtrKey2831Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey2831Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey2831Reg {
        <crate::RegValueT<OqspifCtrKey2831Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey47Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey47Reg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrKey47Reg = crate::RegValueT<OqspifCtrKey47Reg_SPEC>;

impl OqspifCtrKey47Reg {
    #[inline(always)]
    pub fn ospic_ctr_key_4_7(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifCtrKey47Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifCtrKey47Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey47Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey47Reg {
        <crate::RegValueT<OqspifCtrKey47Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey811Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey811Reg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrKey811Reg = crate::RegValueT<OqspifCtrKey811Reg_SPEC>;

impl OqspifCtrKey811Reg {
    #[inline(always)]
    pub fn ospic_ctr_key_8_11(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifCtrKey811Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifCtrKey811Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey811Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey811Reg {
        <crate::RegValueT<OqspifCtrKey811Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrNonce03Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrNonce03Reg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrNonce03Reg = crate::RegValueT<OqspifCtrNonce03Reg_SPEC>;

impl OqspifCtrNonce03Reg {
    #[inline(always)]
    pub fn ospic_ctr_nonce_0_3(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifCtrNonce03Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifCtrNonce03Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrNonce03Reg {
    #[inline(always)]
    fn default() -> OqspifCtrNonce03Reg {
        <crate::RegValueT<OqspifCtrNonce03Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrNonce47Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrNonce47Reg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrNonce47Reg = crate::RegValueT<OqspifCtrNonce47Reg_SPEC>;

impl OqspifCtrNonce47Reg {
    #[inline(always)]
    pub fn ospic_ctr_nonce_4_7(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifCtrNonce47Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifCtrNonce47Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrNonce47Reg {
    #[inline(always)]
    fn default() -> OqspifCtrNonce47Reg {
        <crate::RegValueT<OqspifCtrNonce47Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrSaddrReg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrSaddrReg_SPEC {
    type DataType = u32;
}

pub type OqspifCtrSaddrReg = crate::RegValueT<OqspifCtrSaddrReg_SPEC>;

impl OqspifCtrSaddrReg {
    #[inline(always)]
    pub fn ospic_ctr_saddr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3fffff,
        1,
        0,
        u32,
        u32,
        OqspifCtrSaddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3fffff,
            1,
            0,
            u32,
            u32,
            OqspifCtrSaddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrSaddrReg {
    #[inline(always)]
    fn default() -> OqspifCtrSaddrReg {
        <crate::RegValueT<OqspifCtrSaddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifDummydataReg_SPEC;
impl crate::sealed::RegSpec for OqspifDummydataReg_SPEC {
    type DataType = u32;
}

pub type OqspifDummydataReg = crate::RegValueT<OqspifDummydataReg_SPEC>;

impl OqspifDummydataReg {
    #[inline(always)]
    pub fn ospic_dummydata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifDummydataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifDummydataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifDummydataReg {
    #[inline(always)]
    fn default() -> OqspifDummydataReg {
        <crate::RegValueT<OqspifDummydataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifErasecmdaReg_SPEC;
impl crate::sealed::RegSpec for OqspifErasecmdaReg_SPEC {
    type DataType = u32;
}

pub type OqspifErasecmdaReg = crate::RegValueT<OqspifErasecmdaReg_SPEC>;

impl OqspifErasecmdaReg {
    #[inline(always)]
    pub fn ospic_res_inst(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        u8,
        u8,
        OqspifErasecmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            u8,
            u8,
            OqspifErasecmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_sus_inst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        OqspifErasecmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            OqspifErasecmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_wen_inst(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        OqspifErasecmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            OqspifErasecmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_ers_inst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        OqspifErasecmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            OqspifErasecmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifErasecmdaReg {
    #[inline(always)]
    fn default() -> OqspifErasecmdaReg {
        <crate::RegValueT<OqspifErasecmdaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifErasecmdbReg_SPEC;
impl crate::sealed::RegSpec for OqspifErasecmdbReg_SPEC {
    type DataType = u32;
}

pub type OqspifErasecmdbReg = crate::RegValueT<OqspifErasecmdbReg_SPEC>;

impl OqspifErasecmdbReg {
    #[inline(always)]
    pub fn ospic_ressus_dly(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        u8,
        u8,
        OqspifErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            u8,
            u8,
            OqspifErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_ersres_hld(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        u8,
        u8,
        OqspifErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            u8,
            u8,
            OqspifErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_ers_cs_hi(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1f,
        1,
        0,
        u8,
        u8,
        OqspifErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1f,
            1,
            0,
            u8,
            u8,
            OqspifErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_ead_tx_md(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_res_tx_md(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_sus_tx_md(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_wen_tx_md(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_ers_tx_md(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifErasecmdbReg {
    #[inline(always)]
    fn default() -> OqspifErasecmdbReg {
        <crate::RegValueT<OqspifErasecmdbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifErasecmdcReg_SPEC;
impl crate::sealed::RegSpec for OqspifErasecmdcReg_SPEC {
    type DataType = u32;
}

pub type OqspifErasecmdcReg = crate::RegValueT<OqspifErasecmdcReg_SPEC>;

impl OqspifErasecmdcReg {
    #[inline(always)]
    pub fn ospic_sussts_dly(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        OqspifErasecmdcReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            OqspifErasecmdcReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifErasecmdcReg {
    #[inline(always)]
    fn default() -> OqspifErasecmdcReg {
        <crate::RegValueT<OqspifErasecmdcReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifErasectrlReg_SPEC;
impl crate::sealed::RegSpec for OqspifErasectrlReg_SPEC {
    type DataType = u32;
}

pub type OqspifErasectrlReg = crate::RegValueT<OqspifErasectrlReg_SPEC>;

impl OqspifErasectrlReg {
    #[inline(always)]
    pub fn ospic_ers_res_dis(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, OqspifErasectrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<28,1,0,OqspifErasectrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_ers_state(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x7,
        1,
        0,
        u8,
        u8,
        OqspifErasectrlReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x7,
            1,
            0,
            u8,
            u8,
            OqspifErasectrlReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_erase_en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, OqspifErasectrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<24,1,0,OqspifErasectrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_ers_addr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xfffff,
        1,
        0,
        u32,
        u32,
        OqspifErasectrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xfffff,
            1,
            0,
            u32,
            u32,
            OqspifErasectrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifErasectrlReg {
    #[inline(always)]
    fn default() -> OqspifErasectrlReg {
        <crate::RegValueT<OqspifErasectrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifGpReg_SPEC;
impl crate::sealed::RegSpec for OqspifGpReg_SPEC {
    type DataType = u32;
}

pub type OqspifGpReg = crate::RegValueT<OqspifGpReg_SPEC>;

impl OqspifGpReg {
    #[inline(always)]
    pub fn ospic_pads_slew(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, OqspifGpReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,OqspifGpReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_pads_drv(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, OqspifGpReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,OqspifGpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifGpReg {
    #[inline(always)]
    fn default() -> OqspifGpReg {
        <crate::RegValueT<OqspifGpReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifReaddataReg_SPEC;
impl crate::sealed::RegSpec for OqspifReaddataReg_SPEC {
    type DataType = u32;
}

pub type OqspifReaddataReg = crate::RegValueT<OqspifReaddataReg_SPEC>;

impl OqspifReaddataReg {
    #[inline(always)]
    pub fn ospic_readdata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifReaddataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifReaddataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifReaddataReg {
    #[inline(always)]
    fn default() -> OqspifReaddataReg {
        <crate::RegValueT<OqspifReaddataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifRecvdataReg_SPEC;
impl crate::sealed::RegSpec for OqspifRecvdataReg_SPEC {
    type DataType = u32;
}

pub type OqspifRecvdataReg = crate::RegValueT<OqspifRecvdataReg_SPEC>;

impl OqspifRecvdataReg {
    #[inline(always)]
    pub fn ospic_recvdata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifRecvdataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifRecvdataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifRecvdataReg {
    #[inline(always)]
    fn default() -> OqspifRecvdataReg {
        <crate::RegValueT<OqspifRecvdataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifStatuscmdReg_SPEC;
impl crate::sealed::RegSpec for OqspifStatuscmdReg_SPEC {
    type DataType = u32;
}

pub type OqspifStatuscmdReg = crate::RegValueT<OqspifStatuscmdReg_SPEC>;

impl OqspifStatuscmdReg {
    #[inline(always)]
    pub fn ospic_rstat_dmy_zero(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<30,1,0,OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_rstat_dmy_tx_md(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifStatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifStatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_rstat_dmy_num(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        u8,
        u8,
        OqspifStatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            u8,
            u8,
            OqspifStatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_rstat_dmy_en(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<23,1,0,OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_stsdly_sel(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<22,1,0,OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_ressts_dly(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3f,
        1,
        0,
        u8,
        u8,
        OqspifStatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3f,
            1,
            0,
            u8,
            u8,
            OqspifStatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_busy_val(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ospic_busy_pos(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        u8,
        u8,
        OqspifStatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            u8,
            u8,
            OqspifStatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_rstat_rx_md(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifStatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifStatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_rstat_tx_md(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        u8,
        u8,
        OqspifStatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            u8,
            u8,
            OqspifStatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ospic_rstat_inst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        OqspifStatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            OqspifStatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifStatuscmdReg {
    #[inline(always)]
    fn default() -> OqspifStatuscmdReg {
        <crate::RegValueT<OqspifStatuscmdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifStatusReg_SPEC;
impl crate::sealed::RegSpec for OqspifStatusReg_SPEC {
    type DataType = u32;
}

pub type OqspifStatusReg = crate::RegValueT<OqspifStatusReg_SPEC>;

impl OqspifStatusReg {
    #[inline(always)]
    pub fn ospic_busy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, OqspifStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,OqspifStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifStatusReg {
    #[inline(always)]
    fn default() -> OqspifStatusReg {
        <crate::RegValueT<OqspifStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifWritedataReg_SPEC;
impl crate::sealed::RegSpec for OqspifWritedataReg_SPEC {
    type DataType = u32;
}

pub type OqspifWritedataReg = crate::RegValueT<OqspifWritedataReg_SPEC>;

impl OqspifWritedataReg {
    #[inline(always)]
    pub fn ospic_writedata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OqspifWritedataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OqspifWritedataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifWritedataReg {
    #[inline(always)]
    fn default() -> OqspifWritedataReg {
        <crate::RegValueT<OqspifWritedataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
