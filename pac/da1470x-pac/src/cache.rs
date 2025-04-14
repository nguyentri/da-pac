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
#[doc = r"CACHE registers"]
unsafe impl ::core::marker::Send for super::Cache {}
unsafe impl ::core::marker::Sync for super::Cache {}
impl super::Cache {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn cache_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CacheCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CacheCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cache_flash_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CacheFlashReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CacheFlashReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cache_mrm_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CacheMrmCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CacheMrmCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cache_mrm_hits1ws_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CacheMrmHits1WsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CacheMrmHits1WsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cache_mrm_hits_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CacheMrmHitsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CacheMrmHitsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cache_mrm_hits_thres_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CacheMrmHitsThresReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CacheMrmHitsThresReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cache_mrm_misses_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CacheMrmMissesReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CacheMrmMissesReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cache_mrm_misses_thres_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CacheMrmMissesThresReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CacheMrmMissesThresReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cache_mrm_tint_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CacheMrmTintReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CacheMrmTintReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn swd_reset_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SwdResetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SwdResetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for CacheCtrl2Reg_SPEC {
    type DataType = u32;
}

pub type CacheCtrl2Reg = crate::RegValueT<CacheCtrl2Reg_SPEC>;

impl CacheCtrl2Reg {
    #[inline(always)]
    pub fn cache_ready(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, CacheCtrl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,CacheCtrl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cache_ram_init(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, CacheCtrl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,CacheCtrl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cache_flushed(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cache_flush_disable(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cache_use_full_db_range(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, CacheCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cache_mhclken_disable(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cache_cwf_disable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cache_cgen(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cache_wen(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cache_len(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, CacheCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            CacheCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CacheCtrl2Reg {
    #[inline(always)]
    fn default() -> CacheCtrl2Reg {
        <crate::RegValueT<CacheCtrl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheFlashReg_SPEC;
impl crate::sealed::RegSpec for CacheFlashReg_SPEC {
    type DataType = u32;
}

pub type CacheFlashReg = crate::RegValueT<CacheFlashReg_SPEC>;

impl CacheFlashReg {
    #[inline(always)]
    pub fn flash_region_base(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        CacheFlashReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            CacheFlashReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn flash_region_offset(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, u16, CacheFlashReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0xfff,
            1,
            0,
            u16,
            u16,
            CacheFlashReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn flash_region_size(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, CacheFlashReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,CacheFlashReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CacheFlashReg {
    #[inline(always)]
    fn default() -> CacheFlashReg {
        <crate::RegValueT<CacheFlashReg_SPEC> as RegisterValue<_>>::new(402653185)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheMrmCtrlReg_SPEC;
impl crate::sealed::RegSpec for CacheMrmCtrlReg_SPEC {
    type DataType = u32;
}

pub type CacheMrmCtrlReg = crate::RegValueT<CacheMrmCtrlReg_SPEC>;

impl CacheMrmCtrlReg {
    #[inline(always)]
    pub fn mrm_irq_hits_thres_status(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mrm_irq_misses_thres_status(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,CacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mrm_irq_tint_status(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,CacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mrm_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mrm_start(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,CacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CacheMrmCtrlReg {
    #[inline(always)]
    fn default() -> CacheMrmCtrlReg {
        <crate::RegValueT<CacheMrmCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheMrmHits1WsReg_SPEC;
impl crate::sealed::RegSpec for CacheMrmHits1WsReg_SPEC {
    type DataType = u32;
}

pub type CacheMrmHits1WsReg = crate::RegValueT<CacheMrmHits1WsReg_SPEC>;

impl CacheMrmHits1WsReg {
    #[inline(always)]
    pub fn mrm_hits1ws(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CacheMrmHits1WsReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CacheMrmHits1WsReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CacheMrmHits1WsReg {
    #[inline(always)]
    fn default() -> CacheMrmHits1WsReg {
        <crate::RegValueT<CacheMrmHits1WsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheMrmHitsReg_SPEC;
impl crate::sealed::RegSpec for CacheMrmHitsReg_SPEC {
    type DataType = u32;
}

pub type CacheMrmHitsReg = crate::RegValueT<CacheMrmHitsReg_SPEC>;

impl CacheMrmHitsReg {
    #[inline(always)]
    pub fn mrm_hits(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CacheMrmHitsReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CacheMrmHitsReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CacheMrmHitsReg {
    #[inline(always)]
    fn default() -> CacheMrmHitsReg {
        <crate::RegValueT<CacheMrmHitsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheMrmHitsThresReg_SPEC;
impl crate::sealed::RegSpec for CacheMrmHitsThresReg_SPEC {
    type DataType = u32;
}

pub type CacheMrmHitsThresReg = crate::RegValueT<CacheMrmHitsThresReg_SPEC>;

impl CacheMrmHitsThresReg {
    #[inline(always)]
    pub fn mrm_hits_thres(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CacheMrmHitsThresReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CacheMrmHitsThresReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CacheMrmHitsThresReg {
    #[inline(always)]
    fn default() -> CacheMrmHitsThresReg {
        <crate::RegValueT<CacheMrmHitsThresReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheMrmMissesReg_SPEC;
impl crate::sealed::RegSpec for CacheMrmMissesReg_SPEC {
    type DataType = u32;
}

pub type CacheMrmMissesReg = crate::RegValueT<CacheMrmMissesReg_SPEC>;

impl CacheMrmMissesReg {
    #[inline(always)]
    pub fn mrm_misses(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CacheMrmMissesReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CacheMrmMissesReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CacheMrmMissesReg {
    #[inline(always)]
    fn default() -> CacheMrmMissesReg {
        <crate::RegValueT<CacheMrmMissesReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheMrmMissesThresReg_SPEC;
impl crate::sealed::RegSpec for CacheMrmMissesThresReg_SPEC {
    type DataType = u32;
}

pub type CacheMrmMissesThresReg = crate::RegValueT<CacheMrmMissesThresReg_SPEC>;

impl CacheMrmMissesThresReg {
    #[inline(always)]
    pub fn mrm_misses_thres(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CacheMrmMissesThresReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CacheMrmMissesThresReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CacheMrmMissesThresReg {
    #[inline(always)]
    fn default() -> CacheMrmMissesThresReg {
        <crate::RegValueT<CacheMrmMissesThresReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheMrmTintReg_SPEC;
impl crate::sealed::RegSpec for CacheMrmTintReg_SPEC {
    type DataType = u32;
}

pub type CacheMrmTintReg = crate::RegValueT<CacheMrmTintReg_SPEC>;

impl CacheMrmTintReg {
    #[inline(always)]
    pub fn mrm_tint(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ffff,
        1,
        0,
        u32,
        u32,
        CacheMrmTintReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ffff,
            1,
            0,
            u32,
            u32,
            CacheMrmTintReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CacheMrmTintReg {
    #[inline(always)]
    fn default() -> CacheMrmTintReg {
        <crate::RegValueT<CacheMrmTintReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwdResetReg_SPEC;
impl crate::sealed::RegSpec for SwdResetReg_SPEC {
    type DataType = u32;
}

pub type SwdResetReg = crate::RegValueT<SwdResetReg_SPEC>;

impl SwdResetReg {
    #[inline(always)]
    pub fn swd_hw_reset_req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SwdResetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,SwdResetReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for SwdResetReg {
    #[inline(always)]
    fn default() -> SwdResetReg {
        <crate::RegValueT<SwdResetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
