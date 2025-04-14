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
#[doc = r"QSPIC2 registers"]
unsafe impl ::core::marker::Send for super::Qspic2 {}
unsafe impl ::core::marker::Sync for super::Qspic2 {}
impl super::Qspic2 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn qspic2_awritecmd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2AwritecmdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2AwritecmdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_burstbrk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2BurstbrkReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2BurstbrkReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_burstcmda_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2BurstcmdaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2BurstcmdaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_burstcmdb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2BurstcmdbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2BurstcmdbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_chckerase_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2ChckeraseReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2ChckeraseReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_ctrlbus_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2CtrlbusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2CtrlbusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_ctrlmode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2CtrlmodeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_dummydata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2DummydataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2DummydataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_erasecmda_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2ErasecmdaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2ErasecmdaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_erasecmdb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2ErasecmdbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2ErasecmdbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_erasectrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2ErasectrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2ErasectrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_gp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2GpReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2GpReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_memblen_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2MemblenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2MemblenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_readdata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2ReaddataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2ReaddataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_recvdata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2RecvdataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2RecvdataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_statuscmd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2StatuscmdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2StatuscmdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2StatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2StatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn qspic2_writedata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2WritedataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2WritedataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2AwritecmdReg_SPEC;
impl crate::sealed::RegSpec for Qspic2AwritecmdReg_SPEC {
    type DataType = u32;
}

pub type Qspic2AwritecmdReg = crate::RegValueT<Qspic2AwritecmdReg_SPEC>;

impl Qspic2AwritecmdReg {
    #[inline(always)]
    pub fn qspic_wr_cs_high_min(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1f,
        1,
        0,
        u8,
        u8,
        Qspic2AwritecmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1f,
            1,
            0,
            u8,
            u8,
            Qspic2AwritecmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_wr_dat_tx_md(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        u8,
        u8,
        Qspic2AwritecmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2AwritecmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_wr_adr_tx_md(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        u8,
        u8,
        Qspic2AwritecmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2AwritecmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_wr_inst_tx_md(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        u8,
        u8,
        Qspic2AwritecmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2AwritecmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_wr_inst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Qspic2AwritecmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Qspic2AwritecmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2AwritecmdReg {
    #[inline(always)]
    fn default() -> Qspic2AwritecmdReg {
        <crate::RegValueT<Qspic2AwritecmdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2BurstbrkReg_SPEC;
impl crate::sealed::RegSpec for Qspic2BurstbrkReg_SPEC {
    type DataType = u32;
}

pub type Qspic2BurstbrkReg = crate::RegValueT<Qspic2BurstbrkReg_SPEC>;

impl Qspic2BurstbrkReg {
    #[inline(always)]
    pub fn qspic_sec_hf_ds(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Qspic2BurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,Qspic2BurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_brk_tx_md(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3,
        1,
        0,
        u8,
        u8,
        Qspic2BurstbrkReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2BurstbrkReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_brk_sz(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Qspic2BurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Qspic2BurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_brk_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Qspic2BurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Qspic2BurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
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
        Qspic2BurstbrkReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Qspic2BurstbrkReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2BurstbrkReg {
    #[inline(always)]
    fn default() -> Qspic2BurstbrkReg {
        <crate::RegValueT<Qspic2BurstbrkReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2BurstcmdaReg_SPEC;
impl crate::sealed::RegSpec for Qspic2BurstcmdaReg_SPEC {
    type DataType = u32;
}

pub type Qspic2BurstcmdaReg = crate::RegValueT<Qspic2BurstcmdaReg_SPEC>;

impl Qspic2BurstcmdaReg {
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
        Qspic2BurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2BurstcmdaReg_SPEC,
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
        Qspic2BurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2BurstcmdaReg_SPEC,
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
        Qspic2BurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2BurstcmdaReg_SPEC,
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
        Qspic2BurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2BurstcmdaReg_SPEC,
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
        Qspic2BurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            Qspic2BurstcmdaReg_SPEC,
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
        Qspic2BurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            Qspic2BurstcmdaReg_SPEC,
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
        Qspic2BurstcmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Qspic2BurstcmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2BurstcmdaReg {
    #[inline(always)]
    fn default() -> Qspic2BurstcmdaReg {
        <crate::RegValueT<Qspic2BurstcmdaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2BurstcmdbReg_SPEC;
impl crate::sealed::RegSpec for Qspic2BurstcmdbReg_SPEC {
    type DataType = u32;
}

pub type Qspic2BurstcmdbReg = crate::RegValueT<Qspic2BurstcmdbReg_SPEC>;

impl Qspic2BurstcmdbReg {
    #[inline(always)]
    pub fn qspic_dmy_force(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Qspic2BurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
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
        Qspic2BurstcmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            u8,
            u8,
            Qspic2BurstcmdbReg_SPEC,
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
        Qspic2BurstcmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2BurstcmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_wrap_len(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        u8,
        u8,
        Qspic2BurstcmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2BurstcmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_wrap_md(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Qspic2BurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_inst_md(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Qspic2BurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_dmy_num(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        u8,
        u8,
        Qspic2BurstcmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2BurstcmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_ext_hf_ds(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Qspic2BurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_ext_byte_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Qspic2BurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_dat_rx_md(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        u8,
        u8,
        Qspic2BurstcmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2BurstcmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2BurstcmdbReg {
    #[inline(always)]
    fn default() -> Qspic2BurstcmdbReg {
        <crate::RegValueT<Qspic2BurstcmdbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2ChckeraseReg_SPEC;
impl crate::sealed::RegSpec for Qspic2ChckeraseReg_SPEC {
    type DataType = u32;
}

pub type Qspic2ChckeraseReg = crate::RegValueT<Qspic2ChckeraseReg_SPEC>;

impl Qspic2ChckeraseReg {
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
        Qspic2ChckeraseReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Qspic2ChckeraseReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2ChckeraseReg {
    #[inline(always)]
    fn default() -> Qspic2ChckeraseReg {
        <crate::RegValueT<Qspic2ChckeraseReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2CtrlbusReg_SPEC;
impl crate::sealed::RegSpec for Qspic2CtrlbusReg_SPEC {
    type DataType = u32;
}

pub type Qspic2CtrlbusReg = crate::RegValueT<Qspic2CtrlbusReg_SPEC>;

impl Qspic2CtrlbusReg {
    #[inline(always)]
    pub fn qspic_dis_cs(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Qspic2CtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,Qspic2CtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_en_cs(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Qspic2CtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,Qspic2CtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_set_quad(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Qspic2CtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,Qspic2CtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_set_dual(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Qspic2CtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,Qspic2CtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_set_single(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Qspic2CtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Qspic2CtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2CtrlbusReg {
    #[inline(always)]
    fn default() -> Qspic2CtrlbusReg {
        <crate::RegValueT<Qspic2CtrlbusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2CtrlmodeReg_SPEC;
impl crate::sealed::RegSpec for Qspic2CtrlmodeReg_SPEC {
    type DataType = u32;
}

pub type Qspic2CtrlmodeReg = crate::RegValueT<Qspic2CtrlmodeReg_SPEC>;

impl Qspic2CtrlmodeReg {
    #[inline(always)]
    pub fn qspic_clk_free_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_cs_md(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_sram_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_use_32ba(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_forcenseq_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_pclk_md(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, Qspic2CtrlmodeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            u8,
            u8,
            Qspic2CtrlmodeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_rpipe_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_rxd_neg(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_hrdy_md(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_io3_dat(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_io2_dat(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_io3_oen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_io2_oen(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_clk_md(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_auto_md(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2CtrlmodeReg {
    #[inline(always)]
    fn default() -> Qspic2CtrlmodeReg {
        <crate::RegValueT<Qspic2CtrlmodeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2DummydataReg_SPEC;
impl crate::sealed::RegSpec for Qspic2DummydataReg_SPEC {
    type DataType = u32;
}

pub type Qspic2DummydataReg = crate::RegValueT<Qspic2DummydataReg_SPEC>;

impl Qspic2DummydataReg {
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
        Qspic2DummydataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Qspic2DummydataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2DummydataReg {
    #[inline(always)]
    fn default() -> Qspic2DummydataReg {
        <crate::RegValueT<Qspic2DummydataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2ErasecmdaReg_SPEC;
impl crate::sealed::RegSpec for Qspic2ErasecmdaReg_SPEC {
    type DataType = u32;
}

pub type Qspic2ErasecmdaReg = crate::RegValueT<Qspic2ErasecmdaReg_SPEC>;

impl Qspic2ErasecmdaReg {
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
        Qspic2ErasecmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            u8,
            u8,
            Qspic2ErasecmdaReg_SPEC,
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
        Qspic2ErasecmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            Qspic2ErasecmdaReg_SPEC,
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
        Qspic2ErasecmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            Qspic2ErasecmdaReg_SPEC,
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
        Qspic2ErasecmdaReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Qspic2ErasecmdaReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2ErasecmdaReg {
    #[inline(always)]
    fn default() -> Qspic2ErasecmdaReg {
        <crate::RegValueT<Qspic2ErasecmdaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2ErasecmdbReg_SPEC;
impl crate::sealed::RegSpec for Qspic2ErasecmdbReg_SPEC {
    type DataType = u32;
}

pub type Qspic2ErasecmdbReg = crate::RegValueT<Qspic2ErasecmdbReg_SPEC>;

impl Qspic2ErasecmdbReg {
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
        Qspic2ErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3f,
            1,
            0,
            u8,
            u8,
            Qspic2ErasecmdbReg_SPEC,
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
        Qspic2ErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            u8,
            u8,
            Qspic2ErasecmdbReg_SPEC,
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
        Qspic2ErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1f,
            1,
            0,
            u8,
            u8,
            Qspic2ErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_ead_tx_md(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        u8,
        u8,
        Qspic2ErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2ErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_res_tx_md(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        u8,
        u8,
        Qspic2ErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2ErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_sus_tx_md(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        u8,
        u8,
        Qspic2ErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2ErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_wen_tx_md(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        u8,
        u8,
        Qspic2ErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2ErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_ers_tx_md(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        u8,
        u8,
        Qspic2ErasecmdbReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2ErasecmdbReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2ErasecmdbReg {
    #[inline(always)]
    fn default() -> Qspic2ErasecmdbReg {
        <crate::RegValueT<Qspic2ErasecmdbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2ErasectrlReg_SPEC;
impl crate::sealed::RegSpec for Qspic2ErasectrlReg_SPEC {
    type DataType = u32;
}

pub type Qspic2ErasectrlReg = crate::RegValueT<Qspic2ErasectrlReg_SPEC>;

impl Qspic2ErasectrlReg {
    #[inline(always)]
    pub fn qspic_ers_state(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x7,
        1,
        0,
        u8,
        u8,
        Qspic2ErasectrlReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x7,
            1,
            0,
            u8,
            u8,
            Qspic2ErasectrlReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_erase_en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Qspic2ErasectrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<24,1,0,Qspic2ErasectrlReg_SPEC,crate::common::RW>::from_register(self,0)
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
        Qspic2ErasectrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xfffff,
            1,
            0,
            u32,
            u32,
            Qspic2ErasectrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2ErasectrlReg {
    #[inline(always)]
    fn default() -> Qspic2ErasectrlReg {
        <crate::RegValueT<Qspic2ErasectrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2GpReg_SPEC;
impl crate::sealed::RegSpec for Qspic2GpReg_SPEC {
    type DataType = u32;
}

pub type Qspic2GpReg = crate::RegValueT<Qspic2GpReg_SPEC>;

impl Qspic2GpReg {
    #[inline(always)]
    pub fn qspic_pads_slew(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, Qspic2GpReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,Qspic2GpReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_pads_drv(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Qspic2GpReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Qspic2GpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2GpReg {
    #[inline(always)]
    fn default() -> Qspic2GpReg {
        <crate::RegValueT<Qspic2GpReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2MemblenReg_SPEC;
impl crate::sealed::RegSpec for Qspic2MemblenReg_SPEC {
    type DataType = u32;
}

pub type Qspic2MemblenReg = crate::RegValueT<Qspic2MemblenReg_SPEC>;

impl Qspic2MemblenReg {
    #[inline(always)]
    pub fn qspic_t_cem_cc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3ff,
        1,
        0,
        u16,
        u16,
        Qspic2MemblenReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3ff,
            1,
            0,
            u16,
            u16,
            Qspic2MemblenReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_t_cem_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Qspic2MemblenReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Qspic2MemblenReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_memblen(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Qspic2MemblenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Qspic2MemblenReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2MemblenReg {
    #[inline(always)]
    fn default() -> Qspic2MemblenReg {
        <crate::RegValueT<Qspic2MemblenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2ReaddataReg_SPEC;
impl crate::sealed::RegSpec for Qspic2ReaddataReg_SPEC {
    type DataType = u32;
}

pub type Qspic2ReaddataReg = crate::RegValueT<Qspic2ReaddataReg_SPEC>;

impl Qspic2ReaddataReg {
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
        Qspic2ReaddataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Qspic2ReaddataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2ReaddataReg {
    #[inline(always)]
    fn default() -> Qspic2ReaddataReg {
        <crate::RegValueT<Qspic2ReaddataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2RecvdataReg_SPEC;
impl crate::sealed::RegSpec for Qspic2RecvdataReg_SPEC {
    type DataType = u32;
}

pub type Qspic2RecvdataReg = crate::RegValueT<Qspic2RecvdataReg_SPEC>;

impl Qspic2RecvdataReg {
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
        Qspic2RecvdataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Qspic2RecvdataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2RecvdataReg {
    #[inline(always)]
    fn default() -> Qspic2RecvdataReg {
        <crate::RegValueT<Qspic2RecvdataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2StatuscmdReg_SPEC;
impl crate::sealed::RegSpec for Qspic2StatuscmdReg_SPEC {
    type DataType = u32;
}

pub type Qspic2StatuscmdReg = crate::RegValueT<Qspic2StatuscmdReg_SPEC>;

impl Qspic2StatuscmdReg {
    #[inline(always)]
    pub fn qspic_stsdly_sel(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Qspic2StatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<22,1,0,Qspic2StatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
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
        Qspic2StatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3f,
            1,
            0,
            u8,
            u8,
            Qspic2StatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_busy_val(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Qspic2StatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,Qspic2StatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
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
        Qspic2StatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            u8,
            u8,
            Qspic2StatuscmdReg_SPEC,
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
        Qspic2StatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2StatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qspic_rstat_tx_md(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        u8,
        u8,
        Qspic2StatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            u8,
            u8,
            Qspic2StatuscmdReg_SPEC,
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
        Qspic2StatuscmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Qspic2StatuscmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2StatuscmdReg {
    #[inline(always)]
    fn default() -> Qspic2StatuscmdReg {
        <crate::RegValueT<Qspic2StatuscmdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2StatusReg_SPEC;
impl crate::sealed::RegSpec for Qspic2StatusReg_SPEC {
    type DataType = u32;
}

pub type Qspic2StatusReg = crate::RegValueT<Qspic2StatusReg_SPEC>;

impl Qspic2StatusReg {
    #[inline(always)]
    pub fn qspic_busy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Qspic2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Qspic2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2StatusReg {
    #[inline(always)]
    fn default() -> Qspic2StatusReg {
        <crate::RegValueT<Qspic2StatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2WritedataReg_SPEC;
impl crate::sealed::RegSpec for Qspic2WritedataReg_SPEC {
    type DataType = u32;
}

pub type Qspic2WritedataReg = crate::RegValueT<Qspic2WritedataReg_SPEC>;

impl Qspic2WritedataReg {
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
        Qspic2WritedataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Qspic2WritedataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2WritedataReg {
    #[inline(always)]
    fn default() -> Qspic2WritedataReg {
        <crate::RegValueT<Qspic2WritedataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
