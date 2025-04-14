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
#[doc = r"AES_HASH registers"]
unsafe impl ::core::marker::Send for super::AesHash {}
unsafe impl ::core::marker::Sync for super::AesHash {}
impl super::AesHash {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn crypto_clrirq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoClrirqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoClrirqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crypto_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crypto_dest_addr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoDestAddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoDestAddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crypto_fetch_addr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoFetchAddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoFetchAddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crypto_keys_start(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoKeysStart_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoKeysStart_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crypto_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoLenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoLenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crypto_mreg0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoMreg0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoMreg0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crypto_mreg1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoMreg1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoMreg1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crypto_mreg2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoMreg2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoMreg2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crypto_mreg3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoMreg3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoMreg3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crypto_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crypto_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoClrirqReg_SPEC;
impl crate::sealed::RegSpec for CryptoClrirqReg_SPEC {
    type DataType = u32;
}

pub type CryptoClrirqReg = crate::RegValueT<CryptoClrirqReg_SPEC>;

impl CryptoClrirqReg {
    #[inline(always)]
    pub fn crypto_clrirq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CryptoClrirqReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,CryptoClrirqReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for CryptoClrirqReg {
    #[inline(always)]
    fn default() -> CryptoClrirqReg {
        <crate::RegValueT<CryptoClrirqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoCtrlReg_SPEC;
impl crate::sealed::RegSpec for CryptoCtrlReg_SPEC {
    type DataType = u32;
}

pub type CryptoCtrlReg = crate::RegValueT<CryptoCtrlReg_SPEC>;

impl CryptoCtrlReg {
    #[inline(always)]
    pub fn crypto_aes_kexp(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, CryptoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crypto_more_in(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, CryptoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crypto_hash_out_len(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, CryptoCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crypto_hash_sel(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CryptoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crypto_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CryptoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crypto_encdec(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CryptoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crypto_aes_key_sz(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, CryptoCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crypto_out_md(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CryptoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crypto_alg_md(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, CryptoCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crypto_alg(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, CryptoCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CryptoCtrlReg {
    #[inline(always)]
    fn default() -> CryptoCtrlReg {
        <crate::RegValueT<CryptoCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoDestAddrReg_SPEC;
impl crate::sealed::RegSpec for CryptoDestAddrReg_SPEC {
    type DataType = u32;
}

pub type CryptoDestAddrReg = crate::RegValueT<CryptoDestAddrReg_SPEC>;

impl CryptoDestAddrReg {
    #[inline(always)]
    pub fn crypto_dest_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CryptoDestAddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CryptoDestAddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoDestAddrReg {
    #[inline(always)]
    fn default() -> CryptoDestAddrReg {
        <crate::RegValueT<CryptoDestAddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoFetchAddrReg_SPEC;
impl crate::sealed::RegSpec for CryptoFetchAddrReg_SPEC {
    type DataType = u32;
}

pub type CryptoFetchAddrReg = crate::RegValueT<CryptoFetchAddrReg_SPEC>;

impl CryptoFetchAddrReg {
    #[inline(always)]
    pub fn crypto_fetch_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CryptoFetchAddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CryptoFetchAddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoFetchAddrReg {
    #[inline(always)]
    fn default() -> CryptoFetchAddrReg {
        <crate::RegValueT<CryptoFetchAddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoKeysStart_SPEC;
impl crate::sealed::RegSpec for CryptoKeysStart_SPEC {
    type DataType = u32;
}

pub type CryptoKeysStart = crate::RegValueT<CryptoKeysStart_SPEC>;

impl CryptoKeysStart {
    #[inline(always)]
    pub fn crypto_key_x(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CryptoKeysStart_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CryptoKeysStart_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoKeysStart {
    #[inline(always)]
    fn default() -> CryptoKeysStart {
        <crate::RegValueT<CryptoKeysStart_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoLenReg_SPEC;
impl crate::sealed::RegSpec for CryptoLenReg_SPEC {
    type DataType = u32;
}

pub type CryptoLenReg = crate::RegValueT<CryptoLenReg_SPEC>;

impl CryptoLenReg {
    #[inline(always)]
    pub fn crypto_len(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        CryptoLenReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            CryptoLenReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoLenReg {
    #[inline(always)]
    fn default() -> CryptoLenReg {
        <crate::RegValueT<CryptoLenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoMreg0Reg_SPEC;
impl crate::sealed::RegSpec for CryptoMreg0Reg_SPEC {
    type DataType = u32;
}

pub type CryptoMreg0Reg = crate::RegValueT<CryptoMreg0Reg_SPEC>;

impl CryptoMreg0Reg {
    #[inline(always)]
    pub fn crypto_mreg0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CryptoMreg0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CryptoMreg0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoMreg0Reg {
    #[inline(always)]
    fn default() -> CryptoMreg0Reg {
        <crate::RegValueT<CryptoMreg0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoMreg1Reg_SPEC;
impl crate::sealed::RegSpec for CryptoMreg1Reg_SPEC {
    type DataType = u32;
}

pub type CryptoMreg1Reg = crate::RegValueT<CryptoMreg1Reg_SPEC>;

impl CryptoMreg1Reg {
    #[inline(always)]
    pub fn crypto_mreg1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CryptoMreg1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CryptoMreg1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoMreg1Reg {
    #[inline(always)]
    fn default() -> CryptoMreg1Reg {
        <crate::RegValueT<CryptoMreg1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoMreg2Reg_SPEC;
impl crate::sealed::RegSpec for CryptoMreg2Reg_SPEC {
    type DataType = u32;
}

pub type CryptoMreg2Reg = crate::RegValueT<CryptoMreg2Reg_SPEC>;

impl CryptoMreg2Reg {
    #[inline(always)]
    pub fn crypto_mreg2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CryptoMreg2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CryptoMreg2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoMreg2Reg {
    #[inline(always)]
    fn default() -> CryptoMreg2Reg {
        <crate::RegValueT<CryptoMreg2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoMreg3Reg_SPEC;
impl crate::sealed::RegSpec for CryptoMreg3Reg_SPEC {
    type DataType = u32;
}

pub type CryptoMreg3Reg = crate::RegValueT<CryptoMreg3Reg_SPEC>;

impl CryptoMreg3Reg {
    #[inline(always)]
    pub fn crypto_mreg3(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CryptoMreg3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CryptoMreg3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoMreg3Reg {
    #[inline(always)]
    fn default() -> CryptoMreg3Reg {
        <crate::RegValueT<CryptoMreg3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoStartReg_SPEC;
impl crate::sealed::RegSpec for CryptoStartReg_SPEC {
    type DataType = u32;
}

pub type CryptoStartReg = crate::RegValueT<CryptoStartReg_SPEC>;

impl CryptoStartReg {
    #[inline(always)]
    pub fn crypto_start(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CryptoStartReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,CryptoStartReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for CryptoStartReg {
    #[inline(always)]
    fn default() -> CryptoStartReg {
        <crate::RegValueT<CryptoStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoStatusReg_SPEC;
impl crate::sealed::RegSpec for CryptoStatusReg_SPEC {
    type DataType = u32;
}

pub type CryptoStatusReg = crate::RegValueT<CryptoStatusReg_SPEC>;

impl CryptoStatusReg {
    #[inline(always)]
    pub fn crypto_irq_st(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CryptoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,CryptoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crypto_wait_for_in(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CryptoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,CryptoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crypto_inactive(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CryptoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,CryptoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CryptoStatusReg {
    #[inline(always)]
    fn default() -> CryptoStatusReg {
        <crate::RegValueT<CryptoStatusReg_SPEC> as RegisterValue<_>>::new(1)
    }
}
