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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:52 +0000

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

    #[doc = "Dcache base address for cacheable region"]
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

    #[doc = "Dcache Control register"]
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

    #[doc = "Dcache MRM (Miss Rate Monitor) CONTROL register"]
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

    #[doc = "Dcache MRM (Miss Rate Monitor) EVICTS register"]
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

    #[doc = "Dcache MRM (Miss Rate Monitor) EVICTS THRESHOLD register"]
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

    #[doc = "Dcache MRM (Miss Rate Monitor) HITS register"]
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

    #[doc = "Dcache MRM (Miss Rate Monitor) HITS THRESHOLD register"]
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

    #[doc = "Dcache MRM (Miss Rate Monitor) MISSES register"]
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

    #[doc = "Dcache MRM (Miss Rate Monitor) THRESHOLD register"]
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

    #[doc = "Dcache MRM (Miss Rate Monitor) TIME INTERVAL register"]
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

#[doc = "Dcache base address for cacheable region"]
pub type DcacheBaseAddrReg = crate::RegValueT<DcacheBaseAddrReg_SPEC>;

impl DcacheBaseAddrReg {
    #[doc = "Base of PSRAM cacheable memory\nN*1kByte. N = 0 to 131072 (max. of 128 MByte)."]
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

#[doc = "Dcache Control register"]
pub type DcacheCtrlReg = crate::RegValueT<DcacheCtrlReg_SPEC>;

impl DcacheCtrlReg {
    #[doc = "Disable the clockgating for the DCACHE\n0: Enable clockgating (default)\n1: Disable clockgating"]
    #[inline(always)]
    pub fn dcache_disable_clkgate(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, DcacheCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,DcacheCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write buffer flush\n0: Write buffer isn\'t flushed (default)\n1: Write buffer is flushed"]
    #[inline(always)]
    pub fn dcache_wbuffer_flush(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, DcacheCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,DcacheCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Status of the write buffer\n0: Write buffer isn\'t empty\n1: Write buffer is empty"]
    #[inline(always)]
    pub fn dcache_wbuffer_empty(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, DcacheCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,DcacheCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: DCACHE is not write flushed yet.\n1: DCACHE is write flushed.\n\nNote 1: Setting and clearing of this (status) bit field is automatically done by the hardware.\nNote 2: The CACHE_WFLUSHED bit can also be cleared first by the software by writing a \'0\'"]
    #[inline(always)]
    pub fn dcache_wflushed(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, DcacheCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,DcacheCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: DCACHE isn\'t initialized yet\n1: DCACHE initialization has been completed"]
    #[inline(always)]
    pub fn dcache_ready(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, DcacheCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,DcacheCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Write a \'1\' to this field will trigger a write flush of the \'dirty\' lines. All modified data in \'dirty\' line will be written back to the PSRAM. The corresponding \'dirty\' bits will be cleared. Reading this bit will return \'0\'"]
    #[inline(always)]
    pub fn dcache_wflush(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, DcacheCtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20,1,0,DcacheCtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Write a \'1\' to this field will trigger an initialization of the cache (\'0\'s are written in the TAG area).. Reading from this field will always return \'0\'"]
    #[inline(always)]
    pub fn dcache_init(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, DcacheCtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19,1,0,DcacheCtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Enable the dcache controller HW block:\n0: Disabled, all AHB accesses towards the QSPI are bypassing the HW block straight into the PSRAM\n1: Enabled, all AHB access towards the QSPI within the cacheable region are cached."]
    #[inline(always)]
    pub fn dcache_enable(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, DcacheCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,DcacheCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Length of PSRAM cacheable memory\nN*1kByte. N = 0 to 131072 (max. of 128 MByte).\nSetting DCACHE_LEN=0 disables the caching."]
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

#[doc = "Dcache MRM (Miss Rate Monitor) CONTROL register"]
pub type DcacheMrmCtrlReg = crate::RegValueT<DcacheMrmCtrlReg_SPEC>;

impl DcacheMrmCtrlReg {
    #[doc = "0: No interrupt is generated.\n1: Interrupt (pulse-sensitive) is generated because the number of cache evicts reached the programmed threshold (threshold != 0)."]
    #[inline(always)]
    pub fn mrm_irq_evicts_thres_status(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DcacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,DcacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: No interrupt is generated.\n1: Interrupt (pulse-sensitive) is generated because the number of cache hits reached the programmed threshold (threshold != 0)."]
    #[inline(always)]
    pub fn mrm_irq_hits_thres_status(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DcacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,DcacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: No interrupt is generated.\n1: Interrupt (pulse-sensitive) is generated because the number of cache misses reached the programmed threshold (threshold != 0)."]
    #[inline(always)]
    pub fn mrm_irq_misses_thres_status(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DcacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,DcacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: No interrupt is generated.\n1: Interrupt (pulse-sensitive) is generated because the time interval counter reached the end (time interval != 0)."]
    #[inline(always)]
    pub fn mrm_irq_tint_status(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DcacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DcacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Disables interrupt generation.\n1: Enables interrupt generation.\nNote: The Cache MRM generates a pulse-sensitive interrupt towards the ARM processor,"]
    #[inline(always)]
    pub fn mrm_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DcacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Freeze the \"misses/hits\" counters and reset the time interval counter to the programmed value in CACHE_MRM_TINT_REG.\n1: Enables the counters.\nNote: In case CACHE_MRM_CTRL_REG\\[MRM_START\\] is set to \'1\' and CACHE_MRM_TINT_REG (!=0) is used for the MRM interrupt generation, the time interval counter counts down (on a fixed reference clock of 16 MHz) until it\'s \'0\'. At that time CACHE_MRM_CTRL_REG\\[MRM_START\\] will be reset automatically to \'0\' by the MRM hardware and the MRM interrupt will be generated."]
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

#[doc = "Dcache MRM (Miss Rate Monitor) EVICTS register"]
pub type DcacheMrmEvictsReg = crate::RegValueT<DcacheMrmEvictsReg_SPEC>;

impl DcacheMrmEvictsReg {
    #[doc = "Contains the amount of cache evicts"]
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

#[doc = "Dcache MRM (Miss Rate Monitor) EVICTS THRESHOLD register"]
pub type DcacheMrmEvictsThresReg = crate::RegValueT<DcacheMrmEvictsThresReg_SPEC>;

impl DcacheMrmEvictsThresReg {
    #[doc = "Defines the hits threshold to trigger the interrupt generation. See also the description of CACHE_MRM_CTRL_REG\\[MRM_IRQ_EVICTS_THRES_STATUS\\].\nNote: When MRM_EVICTS_THRES=0 (unrealistic value), no interrupt will be generated."]
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

#[doc = "Dcache MRM (Miss Rate Monitor) HITS register"]
pub type DcacheMrmHitsReg = crate::RegValueT<DcacheMrmHitsReg_SPEC>;

impl DcacheMrmHitsReg {
    #[doc = "Contains the amount of cache hits."]
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

#[doc = "Dcache MRM (Miss Rate Monitor) HITS THRESHOLD register"]
pub type DcacheMrmHitsThresReg = crate::RegValueT<DcacheMrmHitsThresReg_SPEC>;

impl DcacheMrmHitsThresReg {
    #[doc = "Defines the hits threshold to trigger the interrupt generation. See also the description of CACHE_MRM_CTRL_REG\\[MRM_IRQ_HITS_THRES_STATUS\\].\nNote: When MRM_HITS_THRES=0 (unrealistic value), no interrupt will be generated."]
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

#[doc = "Dcache MRM (Miss Rate Monitor) MISSES register"]
pub type DcacheMrmMissesReg = crate::RegValueT<DcacheMrmMissesReg_SPEC>;

impl DcacheMrmMissesReg {
    #[doc = "Contains the amount of cache misses."]
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

#[doc = "Dcache MRM (Miss Rate Monitor) THRESHOLD register"]
pub type DcacheMrmMissesThresReg = crate::RegValueT<DcacheMrmMissesThresReg_SPEC>;

impl DcacheMrmMissesThresReg {
    #[doc = "Defines the misses threshold to trigger the interrupt generation. See also the description of CACHE_MRM_CTRL_REG\\[MRM_IRQ_MISSES_THRES_STATUS\\].\nNote: When MRM_MISSES_THRES=0 (unrealistic value), no interrupt will be generated."]
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

#[doc = "Dcache MRM (Miss Rate Monitor) TIME INTERVAL register"]
pub type DcacheMrmTintReg = crate::RegValueT<DcacheMrmTintReg_SPEC>;

impl DcacheMrmTintReg {
    #[doc = "Defines the time interval for the monitoring in 32 MHz clock cycles. See also the description of CACHE_MRM_CTRL_REG\\[MRM_IRQ_TINT_STATUS\\].\nNote: When MRM_TINT=0 (unrealistic value), no interrupt will be generated."]
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
