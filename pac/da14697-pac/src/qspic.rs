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
#[doc = r"QSPIC registers"]
unsafe impl ::core::marker::Send for super::Qspic {}
unsafe impl ::core::marker::Sync for super::Qspic {}
impl super::Qspic {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn qspic_burstbrk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicBurstbrkReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicBurstbrkReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_burstcmda_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicBurstcmdaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicBurstcmdaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_burstcmdb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicBurstcmdbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicBurstcmdbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_chckerase_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicChckeraseReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicChckeraseReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctrlbus_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrlbusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrlbusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctrlmode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrlmodeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrlmodeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctr_eaddr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrEaddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrEaddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctr_key_0_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrKey03Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrKey03Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctr_key_12_15_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrKey1215Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrKey1215Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctr_key_16_19_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrKey1619Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrKey1619Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctr_key_20_23_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrKey2023Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrKey2023Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctr_key_24_27_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrKey2427Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrKey2427Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctr_key_28_31_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrKey2831Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrKey2831Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctr_key_4_7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrKey47Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrKey47Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctr_key_8_11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrKey811Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrKey811Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctr_nonce_0_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrNonce03Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrNonce03Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctr_nonce_4_7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrNonce47Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrNonce47Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ctr_saddr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrSaddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrSaddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_dummydata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicDummydataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicDummydataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_erasecmda_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicErasecmdaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicErasecmdaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_erasecmdb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicErasecmdbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicErasecmdbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_erasectrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicErasectrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicErasectrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_gp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicGpReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicGpReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_readdata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicReaddataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicReaddataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_recvdata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicRecvdataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicRecvdataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_statuscmd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicStatuscmdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicStatuscmdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_ucode_start(
        &self,
    ) -> &'static crate::common::Reg<self::QspicUcodeStart_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicUcodeStart_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic_writedata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicWritedataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicWritedataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicBurstbrkReg_SPEC;
impl crate::sealed::RegSpec for QspicBurstbrkReg_SPEC {
    type DataType = u32;
}

pub type QspicBurstbrkReg = crate::RegValueT<QspicBurstbrkReg_SPEC>;

impl QspicBurstbrkReg {
    #[inline(always)]
    pub fn qspic_sec_hf_ds(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, QspicBurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,QspicBurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_brk_tx_md(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, u8, QspicBurstbrkReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            18,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicBurstbrkReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_brk_sz(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, QspicBurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,QspicBurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_brk_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, QspicBurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,QspicBurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_brk_wrd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        QspicBurstbrkReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            QspicBurstbrkReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicBurstbrkReg {
    #[inline(always)]
    fn default() -> QspicBurstbrkReg {
        <crate::RegValueT<QspicBurstbrkReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicBurstcmdaReg_SPEC;
impl crate::sealed::RegSpec for QspicBurstcmdaReg_SPEC {
    type DataType = u32;
}

pub type QspicBurstcmdaReg = crate::RegValueT<QspicBurstcmdaReg_SPEC>;

impl QspicBurstcmdaReg {
    #[inline(always)]
    pub fn qspic_dmy_tx_md(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        u8,
        u8,
        QspicBurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicBurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_ext_tx_md(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        u8,
        u8,
        QspicBurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicBurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_adr_tx_md(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x3,
        1,
        0,
        u8,
        u8,
        QspicBurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicBurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_inst_tx_md(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        u8,
        u8,
        QspicBurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicBurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_ext_byte(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        QspicBurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            QspicBurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_inst_wb(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        QspicBurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            QspicBurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_inst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        QspicBurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            QspicBurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicBurstcmdaReg {
    #[inline(always)]
    fn default() -> QspicBurstcmdaReg {
        <crate::RegValueT<QspicBurstcmdaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicBurstcmdbReg_SPEC;
impl crate::sealed::RegSpec for QspicBurstcmdbReg_SPEC {
    type DataType = u32;
}

pub type QspicBurstcmdbReg = crate::RegValueT<QspicBurstcmdbReg_SPEC>;

impl QspicBurstcmdbReg {
    #[inline(always)]
    pub fn qspic_dmy_force(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, QspicBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_cs_high_min(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        u8,
        u8,
        QspicBurstcmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            u8,
            u8,
            QspicBurstcmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_wrap_size(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        u8,
        u8,
        QspicBurstcmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicBurstcmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_wrap_len(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, QspicBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicBurstcmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_wrap_md(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, QspicBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_inst_md(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, QspicBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_dmy_num(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, QspicBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicBurstcmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_ext_hf_ds(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, QspicBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_ext_byte_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, QspicBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_dat_rx_md(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, QspicBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicBurstcmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicBurstcmdbReg {
    #[inline(always)]
    fn default() -> QspicBurstcmdbReg {
        <crate::RegValueT<QspicBurstcmdbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicChckeraseReg_SPEC;
impl crate::sealed::RegSpec for QspicChckeraseReg_SPEC {
    type DataType = u32;
}

pub type QspicChckeraseReg = crate::RegValueT<QspicChckeraseReg_SPEC>;

impl QspicChckeraseReg {
    #[inline(always)]
    pub fn qspic_chckerase(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicChckeraseReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicChckeraseReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicChckeraseReg {
    #[inline(always)]
    fn default() -> QspicChckeraseReg {
        <crate::RegValueT<QspicChckeraseReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrlbusReg_SPEC;
impl crate::sealed::RegSpec for QspicCtrlbusReg_SPEC {
    type DataType = u32;
}

pub type QspicCtrlbusReg = crate::RegValueT<QspicCtrlbusReg_SPEC>;

impl QspicCtrlbusReg {
    #[inline(always)]
    pub fn qspic_dis_cs(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, QspicCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,QspicCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_en_cs(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, QspicCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,QspicCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_set_quad(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, QspicCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,QspicCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_set_dual(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, QspicCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,QspicCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_set_single(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, QspicCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,QspicCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicCtrlbusReg {
    #[inline(always)]
    fn default() -> QspicCtrlbusReg {
        <crate::RegValueT<QspicCtrlbusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrlmodeReg_SPEC;
impl crate::sealed::RegSpec for QspicCtrlmodeReg_SPEC {
    type DataType = u32;
}

pub type QspicCtrlmodeReg = crate::RegValueT<QspicCtrlmodeReg_SPEC>;

impl QspicCtrlmodeReg {
    #[inline(always)]
    pub fn qspic_use_32ba(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_buf_lim_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_pclk_md(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, QspicCtrlmodeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_rpipe_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_rxd_neg(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_hrdy_md(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_io3_dat(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_io2_dat(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_io3_oen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_io2_oen(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_clk_md(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_auto_md(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicCtrlmodeReg {
    #[inline(always)]
    fn default() -> QspicCtrlmodeReg {
        <crate::RegValueT<QspicCtrlmodeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrCtrlReg_SPEC;
impl crate::sealed::RegSpec for QspicCtrCtrlReg_SPEC {
    type DataType = u32;
}

pub type QspicCtrCtrlReg = crate::RegValueT<QspicCtrCtrlReg_SPEC>;

impl QspicCtrCtrlReg {
    #[inline(always)]
    pub fn qspic_ctr_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, QspicCtrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,QspicCtrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicCtrCtrlReg {
    #[inline(always)]
    fn default() -> QspicCtrCtrlReg {
        <crate::RegValueT<QspicCtrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrEaddrReg_SPEC;
impl crate::sealed::RegSpec for QspicCtrEaddrReg_SPEC {
    type DataType = u32;
}

pub type QspicCtrEaddrReg = crate::RegValueT<QspicCtrEaddrReg_SPEC>;

impl QspicCtrEaddrReg {
    #[inline(always)]
    pub fn qspic_ctr_eaddr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3fffff,
        1,
        0,
        u32,
        u32,
        QspicCtrEaddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3fffff,
            1,
            0,
            u32,
            u32,
            QspicCtrEaddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicCtrEaddrReg {
    #[inline(always)]
    fn default() -> QspicCtrEaddrReg {
        <crate::RegValueT<QspicCtrEaddrReg_SPEC> as RegisterValue<_>>::new(1023)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrKey03Reg_SPEC;
impl crate::sealed::RegSpec for QspicCtrKey03Reg_SPEC {
    type DataType = u32;
}

pub type QspicCtrKey03Reg = crate::RegValueT<QspicCtrKey03Reg_SPEC>;

impl QspicCtrKey03Reg {
    #[inline(always)]
    pub fn qspic_ctr_key_0_3(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicCtrKey03Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicCtrKey03Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicCtrKey03Reg {
    #[inline(always)]
    fn default() -> QspicCtrKey03Reg {
        <crate::RegValueT<QspicCtrKey03Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrKey1215Reg_SPEC;
impl crate::sealed::RegSpec for QspicCtrKey1215Reg_SPEC {
    type DataType = u32;
}

pub type QspicCtrKey1215Reg = crate::RegValueT<QspicCtrKey1215Reg_SPEC>;

impl QspicCtrKey1215Reg {
    #[inline(always)]
    pub fn qspic_ctr_key_12_15(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicCtrKey1215Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicCtrKey1215Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicCtrKey1215Reg {
    #[inline(always)]
    fn default() -> QspicCtrKey1215Reg {
        <crate::RegValueT<QspicCtrKey1215Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrKey1619Reg_SPEC;
impl crate::sealed::RegSpec for QspicCtrKey1619Reg_SPEC {
    type DataType = u32;
}

pub type QspicCtrKey1619Reg = crate::RegValueT<QspicCtrKey1619Reg_SPEC>;

impl QspicCtrKey1619Reg {
    #[inline(always)]
    pub fn qspic_ctr_key_16_19(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicCtrKey1619Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicCtrKey1619Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicCtrKey1619Reg {
    #[inline(always)]
    fn default() -> QspicCtrKey1619Reg {
        <crate::RegValueT<QspicCtrKey1619Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrKey2023Reg_SPEC;
impl crate::sealed::RegSpec for QspicCtrKey2023Reg_SPEC {
    type DataType = u32;
}

pub type QspicCtrKey2023Reg = crate::RegValueT<QspicCtrKey2023Reg_SPEC>;

impl QspicCtrKey2023Reg {
    #[inline(always)]
    pub fn qspic_ctr_key_20_23(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicCtrKey2023Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicCtrKey2023Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicCtrKey2023Reg {
    #[inline(always)]
    fn default() -> QspicCtrKey2023Reg {
        <crate::RegValueT<QspicCtrKey2023Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrKey2427Reg_SPEC;
impl crate::sealed::RegSpec for QspicCtrKey2427Reg_SPEC {
    type DataType = u32;
}

pub type QspicCtrKey2427Reg = crate::RegValueT<QspicCtrKey2427Reg_SPEC>;

impl QspicCtrKey2427Reg {
    #[inline(always)]
    pub fn qspic_ctr_key_24_27(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicCtrKey2427Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicCtrKey2427Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicCtrKey2427Reg {
    #[inline(always)]
    fn default() -> QspicCtrKey2427Reg {
        <crate::RegValueT<QspicCtrKey2427Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrKey2831Reg_SPEC;
impl crate::sealed::RegSpec for QspicCtrKey2831Reg_SPEC {
    type DataType = u32;
}

pub type QspicCtrKey2831Reg = crate::RegValueT<QspicCtrKey2831Reg_SPEC>;

impl QspicCtrKey2831Reg {
    #[inline(always)]
    pub fn qspic_ctr_key_28_31(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicCtrKey2831Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicCtrKey2831Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicCtrKey2831Reg {
    #[inline(always)]
    fn default() -> QspicCtrKey2831Reg {
        <crate::RegValueT<QspicCtrKey2831Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrKey47Reg_SPEC;
impl crate::sealed::RegSpec for QspicCtrKey47Reg_SPEC {
    type DataType = u32;
}

pub type QspicCtrKey47Reg = crate::RegValueT<QspicCtrKey47Reg_SPEC>;

impl QspicCtrKey47Reg {
    #[inline(always)]
    pub fn qspic_ctr_key_4_7(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicCtrKey47Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicCtrKey47Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicCtrKey47Reg {
    #[inline(always)]
    fn default() -> QspicCtrKey47Reg {
        <crate::RegValueT<QspicCtrKey47Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrKey811Reg_SPEC;
impl crate::sealed::RegSpec for QspicCtrKey811Reg_SPEC {
    type DataType = u32;
}

pub type QspicCtrKey811Reg = crate::RegValueT<QspicCtrKey811Reg_SPEC>;

impl QspicCtrKey811Reg {
    #[inline(always)]
    pub fn qspic_ctr_key_8_11(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicCtrKey811Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicCtrKey811Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicCtrKey811Reg {
    #[inline(always)]
    fn default() -> QspicCtrKey811Reg {
        <crate::RegValueT<QspicCtrKey811Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrNonce03Reg_SPEC;
impl crate::sealed::RegSpec for QspicCtrNonce03Reg_SPEC {
    type DataType = u32;
}

pub type QspicCtrNonce03Reg = crate::RegValueT<QspicCtrNonce03Reg_SPEC>;

impl QspicCtrNonce03Reg {
    #[inline(always)]
    pub fn qspic_ctr_nonce_0_3(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicCtrNonce03Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicCtrNonce03Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicCtrNonce03Reg {
    #[inline(always)]
    fn default() -> QspicCtrNonce03Reg {
        <crate::RegValueT<QspicCtrNonce03Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrNonce47Reg_SPEC;
impl crate::sealed::RegSpec for QspicCtrNonce47Reg_SPEC {
    type DataType = u32;
}

pub type QspicCtrNonce47Reg = crate::RegValueT<QspicCtrNonce47Reg_SPEC>;

impl QspicCtrNonce47Reg {
    #[inline(always)]
    pub fn qspic_ctr_nonce_4_7(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicCtrNonce47Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicCtrNonce47Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicCtrNonce47Reg {
    #[inline(always)]
    fn default() -> QspicCtrNonce47Reg {
        <crate::RegValueT<QspicCtrNonce47Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrSaddrReg_SPEC;
impl crate::sealed::RegSpec for QspicCtrSaddrReg_SPEC {
    type DataType = u32;
}

pub type QspicCtrSaddrReg = crate::RegValueT<QspicCtrSaddrReg_SPEC>;

impl QspicCtrSaddrReg {
    #[inline(always)]
    pub fn qspic_ctr_saddr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3fffff,
        1,
        0,
        u32,
        u32,
        QspicCtrSaddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3fffff,
            1,
            0,
            u32,
            u32,
            QspicCtrSaddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicCtrSaddrReg {
    #[inline(always)]
    fn default() -> QspicCtrSaddrReg {
        <crate::RegValueT<QspicCtrSaddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicDummydataReg_SPEC;
impl crate::sealed::RegSpec for QspicDummydataReg_SPEC {
    type DataType = u32;
}

pub type QspicDummydataReg = crate::RegValueT<QspicDummydataReg_SPEC>;

impl QspicDummydataReg {
    #[inline(always)]
    pub fn qspic_dummydata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicDummydataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicDummydataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicDummydataReg {
    #[inline(always)]
    fn default() -> QspicDummydataReg {
        <crate::RegValueT<QspicDummydataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicErasecmdaReg_SPEC;
impl crate::sealed::RegSpec for QspicErasecmdaReg_SPEC {
    type DataType = u32;
}

pub type QspicErasecmdaReg = crate::RegValueT<QspicErasecmdaReg_SPEC>;

impl QspicErasecmdaReg {
    #[inline(always)]
    pub fn qspic_res_inst(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        u8,
        u8,
        QspicErasecmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            u8,
            u8,
            QspicErasecmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_sus_inst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        QspicErasecmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            QspicErasecmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_wen_inst(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        QspicErasecmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            QspicErasecmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_ers_inst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        QspicErasecmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            QspicErasecmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicErasecmdaReg {
    #[inline(always)]
    fn default() -> QspicErasecmdaReg {
        <crate::RegValueT<QspicErasecmdaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicErasecmdbReg_SPEC;
impl crate::sealed::RegSpec for QspicErasecmdbReg_SPEC {
    type DataType = u32;
}

pub type QspicErasecmdbReg = crate::RegValueT<QspicErasecmdbReg_SPEC>;

impl QspicErasecmdbReg {
    #[inline(always)]
    pub fn qspic_ressus_dly(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3f,
        1,
        0,
        u8,
        u8,
        QspicErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3f,
            1,
            0,
            u8,
            u8,
            QspicErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_ersres_hld(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        u8,
        u8,
        QspicErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            u8,
            u8,
            QspicErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_ers_cs_hi(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1f,
        1,
        0,
        u8,
        u8,
        QspicErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1f,
            1,
            0,
            u8,
            u8,
            QspicErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_ead_tx_md(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, QspicErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_res_tx_md(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, QspicErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_sus_tx_md(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, QspicErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_wen_tx_md(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, QspicErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_ers_tx_md(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, QspicErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicErasecmdbReg {
    #[inline(always)]
    fn default() -> QspicErasecmdbReg {
        <crate::RegValueT<QspicErasecmdbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicErasectrlReg_SPEC;
impl crate::sealed::RegSpec for QspicErasectrlReg_SPEC {
    type DataType = u32;
}

pub type QspicErasectrlReg = crate::RegValueT<QspicErasectrlReg_SPEC>;

impl QspicErasectrlReg {
    #[inline(always)]
    pub fn qspic_ers_state(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, QspicErasectrlReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            25,
            0x7,
            1,
            0,
            u8,
            u8,
            QspicErasectrlReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_erase_en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, QspicErasectrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,QspicErasectrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_ers_addr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xfffff,
        1,
        0,
        u32,
        u32,
        QspicErasectrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xfffff,
            1,
            0,
            u32,
            u32,
            QspicErasectrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicErasectrlReg {
    #[inline(always)]
    fn default() -> QspicErasectrlReg {
        <crate::RegValueT<QspicErasectrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicGpReg_SPEC;
impl crate::sealed::RegSpec for QspicGpReg_SPEC {
    type DataType = u32;
}

pub type QspicGpReg = crate::RegValueT<QspicGpReg_SPEC>;

impl QspicGpReg {
    #[inline(always)]
    pub fn qspic_pads_slew(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, QspicGpReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,QspicGpReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_pads_drv(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, QspicGpReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,QspicGpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicGpReg {
    #[inline(always)]
    fn default() -> QspicGpReg {
        <crate::RegValueT<QspicGpReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicReaddataReg_SPEC;
impl crate::sealed::RegSpec for QspicReaddataReg_SPEC {
    type DataType = u32;
}

pub type QspicReaddataReg = crate::RegValueT<QspicReaddataReg_SPEC>;

impl QspicReaddataReg {
    #[inline(always)]
    pub fn qspic_readdata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicReaddataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicReaddataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicReaddataReg {
    #[inline(always)]
    fn default() -> QspicReaddataReg {
        <crate::RegValueT<QspicReaddataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicRecvdataReg_SPEC;
impl crate::sealed::RegSpec for QspicRecvdataReg_SPEC {
    type DataType = u32;
}

pub type QspicRecvdataReg = crate::RegValueT<QspicRecvdataReg_SPEC>;

impl QspicRecvdataReg {
    #[inline(always)]
    pub fn qspic_recvdata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicRecvdataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicRecvdataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicRecvdataReg {
    #[inline(always)]
    fn default() -> QspicRecvdataReg {
        <crate::RegValueT<QspicRecvdataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicStatuscmdReg_SPEC;
impl crate::sealed::RegSpec for QspicStatuscmdReg_SPEC {
    type DataType = u32;
}

pub type QspicStatuscmdReg = crate::RegValueT<QspicStatuscmdReg_SPEC>;

impl QspicStatuscmdReg {
    #[inline(always)]
    pub fn qspic_stsdly_sel(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, QspicStatuscmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,QspicStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_ressts_dly(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3f,
        1,
        0,
        u8,
        u8,
        QspicStatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3f,
            1,
            0,
            u8,
            u8,
            QspicStatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_busy_val(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, QspicStatuscmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,QspicStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_busy_pos(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        u8,
        u8,
        QspicStatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            u8,
            u8,
            QspicStatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_rstat_rx_md(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        u8,
        u8,
        QspicStatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicStatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_rstat_tx_md(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, QspicStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            u8,
            u8,
            QspicStatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_rstat_inst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        QspicStatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            QspicStatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicStatuscmdReg {
    #[inline(always)]
    fn default() -> QspicStatuscmdReg {
        <crate::RegValueT<QspicStatuscmdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicStatusReg_SPEC;
impl crate::sealed::RegSpec for QspicStatusReg_SPEC {
    type DataType = u32;
}

pub type QspicStatusReg = crate::RegValueT<QspicStatusReg_SPEC>;

impl QspicStatusReg {
    #[inline(always)]
    pub fn qspic_busy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, QspicStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,QspicStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicStatusReg {
    #[inline(always)]
    fn default() -> QspicStatusReg {
        <crate::RegValueT<QspicStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicUcodeStart_SPEC;
impl crate::sealed::RegSpec for QspicUcodeStart_SPEC {
    type DataType = u32;
}

pub type QspicUcodeStart = crate::RegValueT<QspicUcodeStart_SPEC>;

impl QspicUcodeStart {
    #[inline(always)]
    pub fn qspic_ucode_x(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicUcodeStart_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicUcodeStart_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicUcodeStart {
    #[inline(always)]
    fn default() -> QspicUcodeStart {
        <crate::RegValueT<QspicUcodeStart_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicWritedataReg_SPEC;
impl crate::sealed::RegSpec for QspicWritedataReg_SPEC {
    type DataType = u32;
}

pub type QspicWritedataReg = crate::RegValueT<QspicWritedataReg_SPEC>;

impl QspicWritedataReg {
    #[inline(always)]
    pub fn qspic_writedata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        QspicWritedataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            QspicWritedataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicWritedataReg {
    #[inline(always)]
    fn default() -> QspicWritedataReg {
        <crate::RegValueT<QspicWritedataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
