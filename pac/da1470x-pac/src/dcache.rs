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
#[doc = r"DCACHE registers"]
unsafe impl ::core::marker::Send for super::Dcache {}
unsafe impl ::core::marker::Sync for super::Dcache {}
impl super::Dcache {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn dcache_base_addr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcacheBaseAddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcacheBaseAddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcache_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcacheCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcacheCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcache_mrm_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcacheMrmCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcacheMrmCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcache_mrm_evicts_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcacheMrmEvictsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcacheMrmEvictsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcache_mrm_evicts_thres_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcacheMrmEvictsThresReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcacheMrmEvictsThresReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcache_mrm_hits_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcacheMrmHitsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcacheMrmHitsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcache_mrm_hits_thres_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcacheMrmHitsThresReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcacheMrmHitsThresReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcache_mrm_misses_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcacheMrmMissesReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcacheMrmMissesReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcache_mrm_misses_thres_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcacheMrmMissesThresReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcacheMrmMissesThresReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcache_mrm_tint_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcacheMrmTintReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcacheMrmTintReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcacheBaseAddrReg_SPEC;
impl crate::sealed::RegSpec for DcacheBaseAddrReg_SPEC {
    type DataType = u32;
}

pub type DcacheBaseAddrReg = crate::RegValueT<DcacheBaseAddrReg_SPEC>;

impl DcacheBaseAddrReg {
    #[inline(always)]
    pub fn dcache_base_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        DcacheBaseAddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            DcacheBaseAddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcacheBaseAddrReg {
    #[inline(always)]
    fn default() -> DcacheBaseAddrReg {
        <crate::RegValueT<DcacheBaseAddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcacheCtrlReg_SPEC;
impl crate::sealed::RegSpec for DcacheCtrlReg_SPEC {
    type DataType = u32;
}

pub type DcacheCtrlReg = crate::RegValueT<DcacheCtrlReg_SPEC>;

impl DcacheCtrlReg {
    #[inline(always)]
    pub fn dcache_disable_clkgate(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, DcacheCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,DcacheCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcache_wbuffer_flush(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, DcacheCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,DcacheCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcache_wbuffer_empty(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, DcacheCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,DcacheCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcache_wflushed(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, DcacheCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,DcacheCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcache_ready(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, DcacheCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,DcacheCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcache_wflush(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, DcacheCtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20,1,0,DcacheCtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcache_init(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, DcacheCtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19,1,0,DcacheCtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcache_enable(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, DcacheCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,DcacheCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcache_len(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ffff,
        1,
        0,
        u32,
        u32,
        DcacheCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ffff,
            1,
            0,
            u32,
            u32,
            DcacheCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcacheCtrlReg {
    #[inline(always)]
    fn default() -> DcacheCtrlReg {
        <crate::RegValueT<DcacheCtrlReg_SPEC> as RegisterValue<_>>::new(8388608)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcacheMrmCtrlReg_SPEC;
impl crate::sealed::RegSpec for DcacheMrmCtrlReg_SPEC {
    type DataType = u32;
}

pub type DcacheMrmCtrlReg = crate::RegValueT<DcacheMrmCtrlReg_SPEC>;

impl DcacheMrmCtrlReg {
    #[inline(always)]
    pub fn mrm_irq_evicts_thres_status(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DcacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,DcacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mrm_irq_hits_thres_status(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DcacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,DcacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mrm_irq_misses_thres_status(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DcacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,DcacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mrm_irq_tint_status(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DcacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DcacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mrm_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DcacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mrm_start(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DcacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcacheMrmCtrlReg {
    #[inline(always)]
    fn default() -> DcacheMrmCtrlReg {
        <crate::RegValueT<DcacheMrmCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcacheMrmEvictsReg_SPEC;
impl crate::sealed::RegSpec for DcacheMrmEvictsReg_SPEC {
    type DataType = u32;
}

pub type DcacheMrmEvictsReg = crate::RegValueT<DcacheMrmEvictsReg_SPEC>;

impl DcacheMrmEvictsReg {
    #[inline(always)]
    pub fn mrm_evicts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcacheMrmEvictsReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcacheMrmEvictsReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcacheMrmEvictsReg {
    #[inline(always)]
    fn default() -> DcacheMrmEvictsReg {
        <crate::RegValueT<DcacheMrmEvictsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcacheMrmEvictsThresReg_SPEC;
impl crate::sealed::RegSpec for DcacheMrmEvictsThresReg_SPEC {
    type DataType = u32;
}

pub type DcacheMrmEvictsThresReg = crate::RegValueT<DcacheMrmEvictsThresReg_SPEC>;

impl DcacheMrmEvictsThresReg {
    #[inline(always)]
    pub fn mrm_evicts_thres(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcacheMrmEvictsThresReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcacheMrmEvictsThresReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcacheMrmEvictsThresReg {
    #[inline(always)]
    fn default() -> DcacheMrmEvictsThresReg {
        <crate::RegValueT<DcacheMrmEvictsThresReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcacheMrmHitsReg_SPEC;
impl crate::sealed::RegSpec for DcacheMrmHitsReg_SPEC {
    type DataType = u32;
}

pub type DcacheMrmHitsReg = crate::RegValueT<DcacheMrmHitsReg_SPEC>;

impl DcacheMrmHitsReg {
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
        DcacheMrmHitsReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcacheMrmHitsReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcacheMrmHitsReg {
    #[inline(always)]
    fn default() -> DcacheMrmHitsReg {
        <crate::RegValueT<DcacheMrmHitsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcacheMrmHitsThresReg_SPEC;
impl crate::sealed::RegSpec for DcacheMrmHitsThresReg_SPEC {
    type DataType = u32;
}

pub type DcacheMrmHitsThresReg = crate::RegValueT<DcacheMrmHitsThresReg_SPEC>;

impl DcacheMrmHitsThresReg {
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
        DcacheMrmHitsThresReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcacheMrmHitsThresReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcacheMrmHitsThresReg {
    #[inline(always)]
    fn default() -> DcacheMrmHitsThresReg {
        <crate::RegValueT<DcacheMrmHitsThresReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcacheMrmMissesReg_SPEC;
impl crate::sealed::RegSpec for DcacheMrmMissesReg_SPEC {
    type DataType = u32;
}

pub type DcacheMrmMissesReg = crate::RegValueT<DcacheMrmMissesReg_SPEC>;

impl DcacheMrmMissesReg {
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
        DcacheMrmMissesReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcacheMrmMissesReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcacheMrmMissesReg {
    #[inline(always)]
    fn default() -> DcacheMrmMissesReg {
        <crate::RegValueT<DcacheMrmMissesReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcacheMrmMissesThresReg_SPEC;
impl crate::sealed::RegSpec for DcacheMrmMissesThresReg_SPEC {
    type DataType = u32;
}

pub type DcacheMrmMissesThresReg = crate::RegValueT<DcacheMrmMissesThresReg_SPEC>;

impl DcacheMrmMissesThresReg {
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
        DcacheMrmMissesThresReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcacheMrmMissesThresReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcacheMrmMissesThresReg {
    #[inline(always)]
    fn default() -> DcacheMrmMissesThresReg {
        <crate::RegValueT<DcacheMrmMissesThresReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcacheMrmTintReg_SPEC;
impl crate::sealed::RegSpec for DcacheMrmTintReg_SPEC {
    type DataType = u32;
}

pub type DcacheMrmTintReg = crate::RegValueT<DcacheMrmTintReg_SPEC>;

impl DcacheMrmTintReg {
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
        DcacheMrmTintReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ffff,
            1,
            0,
            u32,
            u32,
            DcacheMrmTintReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcacheMrmTintReg {
    #[inline(always)]
    fn default() -> DcacheMrmTintReg {
        <crate::RegValueT<DcacheMrmTintReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
