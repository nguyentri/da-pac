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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:28 +0000

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
    #[doc = "Cache control register 2 (only Word (32-bits) access supported)."]
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

    #[doc = "Cache QSPI Flash program size and base address register (only Word (32-bits) access supported)."]
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

    #[doc = "Cache MRM (Miss Rate Monitor) CONTROL register (only Word (32-bits) access supported)."]
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

    #[doc = "Cache MRM (Miss Rate Monitor) HITS with 1 Wait State register (only Word (32-bits) access supported)."]
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

    #[doc = "Cache MRM (Miss Rate Monitor) HITS register (only Word (32-bits) access supported)."]
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

    #[doc = "Cache MRM (Miss Rate Monitor) HITS THRESHOLD register (only Word (32-bits) access supported)."]
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

    #[doc = "Cache MRM (Miss Rate Monitor) MISSES register (only Word (32-bits) access supported)."]
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

    #[doc = "Cache MRM (Miss Rate Monitor) THRESHOLD register (only Word (32-bits) access supported)."]
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

    #[doc = "Cache MRM (Miss Rate Monitor) TIME INTERVAL register (only Word (32-bits) access supported)."]
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

    #[doc = "SWD HW reset control register (only Word (32-bits) access supported)."]
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
#[doc = "Cache control register 2 (only Word (32-bits) access supported)."]
pub type CacheCtrl2Reg = crate::RegValueT<CacheCtrl2Reg_SPEC>;

impl CacheCtrl2Reg {
    #[doc = "Cache Controller RO status bit.\n0: Default.\n1: Set to \'1\' when CACHE_CTRL is enabled, initialized and immediately ready for a cacheable access to service."]
    #[inline(always)]
    pub fn cache_ready(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, CacheCtrl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,CacheCtrl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Cache Controller RO status bit.\n0: Default.\n1: Set to \'1\' when SRAM is being initialized (i.e. being flushed).\nNote: The flushing of the cache memory takes 256 HCLK cycles."]
    #[inline(always)]
    pub fn cache_ram_init(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, CacheCtrl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,CacheCtrl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0: Cache is not flushed yet.\n1: Cache is flushed.\n\nNote 1: Setting and clearing of this (status) bit field is automatically done by the hardware. It is set on the falling edge and cleared on the rising edge of the (Cache Controller) CACHE_RAM_INIT signal.\nNote 2: When the Cache is flushed by disabling and enabling the Cache Controller with a SYS_CTRL_REG\\[CACHERAM_MUX\\] sequence of 1 -> 0 -> 1, the CACHE_FLUSHED bit can also be cleared first by the software (if needed) with writing a CACHE_CTRL2_REG(CACHE_FLUSHED) sequence of 1 -> 0. This is needed when the software uses the CACHE_FLUSHED bit as a status bit to wait for (e.g. in a while-loop)."]
    #[inline(always)]
    pub fn cache_flushed(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Default.\n1: Flushing of the Cache memory is disabled when SYS_CTRL_REG\\[CACHERAM_MUX\\] is switched from \'1\' to \'0\'.\n\nNote: Setting this bit to \'1\' is only allowed for debugging purposes."]
    #[inline(always)]
    pub fn cache_flush_disable(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "00: CACHERAM (mirrored) read/write and NO use of the full 184 bits databus (for executing program code or extension of the SysRAM with the Cache RAM).\nIn this mode 8 bits, 16 bits and 32 bits write access is supported.\n\n01: CACHERAM (mirrored) read and use of the full 184 bits databus of \'SRAM_1_0\' (for testing and debugging purposes).\nIn this mode only 32 bits write access is supported.\n\n10: CACHERAM (mirrored) read and use of the full 184 bits databus of \'SRAM_3_2\' (for testing and debugging purposes).\nIn this mode only 32 bits write access is supported.\n\n11: Reserved.\n\nNote 1: SYS_CTRL_REG\\[CACHERAM_MUX\\] must be set to \'0\' before accessing the memory mapped (mirrored) Cache Data and TAG memory.\nNote 2: For all 3 settings, max. 8 KByte is available from the memory map."]
    #[inline(always)]
    pub fn cache_use_full_db_range(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, CacheCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8, CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Default.\n1: The \"m_HCLK_EN\" input is ignored and the controller avoids inserting m_HTRANS=BUSY because of wait states.\n\nNote:\nThis bit is only relevant for executing from QSPI Flash (when set to \'1\' it will improve performance). This bit should be kept on \'0\' for executing from eFlash."]
    #[inline(always)]
    pub fn cache_mhclken_disable(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Default.\n1: The cache line refill is performed with INCR type burst and \"Critical Word First\" is disabled.\n\nNote:\nThis bit is only relevant for executing from QSPI Flash (when set to \'1\' it will improve performance).This bit should be kept on \'0\' for executing from eFlash."]
    #[inline(always)]
    pub fn cache_cwf_disable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Cache controller clock gating is not enabled.\n1: Cache controller clock gating is enabled (enabling power saving)."]
    #[inline(always)]
    pub fn cache_cgen(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Cache Data and TAG memory read only.\n1: Cache Data and TAG memory read/write.\nThe Data and TAG memory are only updated by the cache controller.\nThere is no HW protection to prevent unauthorized access by the ARM.\nNote 1: When accessing the memory mapped Cache Data and TAG memory (which is only allowed for debugging purposes) only 32 bits access is supported.\nNote 2: SYS_CTRL_REG\\[CACHERAM_MUX\\] must be set to \'0\' before accessing the memory mapped Cache Data and TAG memory.\nSee also the CACHE_CTRL2_REG\\[CACHE_USE_FULL_DB_RANGE\\] description."]
    #[inline(always)]
    pub fn cache_wen(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Length of OQSPI FLASH cacheable memory.\nN*64 KBytes. N = 0 to 2048, incl. 2048 (max. of 128 Mbytes).\nSetting CACHE_LEN=0 disables the caching.\n\nNote 1: The max. relevant CACHE_LEN setting depends on the chosen Flash region (program) size.\nNote 2: The first block (CACHE_LEN=1) includes the memory space specified by CACHE_FLASH_REG\\[FLASH_REGION_OFFSET\\]."]
    #[inline(always)]
    pub fn cache_len(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, CacheCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16, CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Cache QSPI Flash program size and base address register (only Word (32-bits) access supported)."]
pub type CacheFlashReg = crate::RegValueT<CacheFlashReg_SPEC>;

impl CacheFlashReg {
    #[doc = "These bits corresponds with the Flash region base address bits \\[31:16\\].\nDefault value is \'0x1800\'.\nThe Flash region base address bits \\[31:27\\] are fixed to \'0b00011\', supporting the range of 0x1800-0x1FFF\'.\nThese register bits are retained.\nNote 1: The updated value takes effect only after a software reset.\nNote 2 The Flash region base address setting depends on the chosen Flash region size."]
    #[inline(always)]
    pub fn flash_region_base(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, CacheFlashReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, CacheFlashReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Flash region offset address (in words).\nThis value is added to the Flash (CPU) address bits \\[13:2\\].\nThese register bits are retained.\nNote 1: The updated value takes effect only after a software reset."]
    #[inline(always)]
    pub fn flash_region_offset(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, CacheFlashReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfff,1,0,u16, CacheFlashReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Flash region size.\nDefault value is \'1\' (0.5 MBytes).\n0 = 0.25 MBytes,\n1 = 0.5 MBytes,\n2 = 1 MBytes,\n3 = 2 MBytes,\n4 = 4 MBytes,\n5 = 8 MBytes,\n6 = 16 MBytes,\n7 = 32 MBytes,\n8 = 64 MBytes,\n9 = 128 MBytes.\nThese register bits are retained.\nNote 1: The updated value takes effect only after a software reset.\nNote 2: See for the max. region (program) size the memory map."]
    #[inline(always)]
    pub fn flash_region_size(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, CacheFlashReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, CacheFlashReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Cache MRM (Miss Rate Monitor) CONTROL register (only Word (32-bits) access supported)."]
pub type CacheMrmCtrlReg = crate::RegValueT<CacheMrmCtrlReg_SPEC>;

impl CacheMrmCtrlReg {
    #[doc = "0: No interrupt is generated.\n1: Interrupt (pulse-sensitive) is generated because the number of cache hits reached the programmed threshold (threshold != 0)."]
    #[inline(always)]
    pub fn mrm_irq_hits_thres_status(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: No interrupt is generated.\n1: Interrupt (pulse-sensitive) is generated because the number of cache misses reached the programmed threshold (threshold != 0)."]
    #[inline(always)]
    pub fn mrm_irq_misses_thres_status(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,CacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: No interrupt is generated.\n1: Interrupt (pulse-sensitive) is generated because the time interval counter reached the end (time interval != 0)."]
    #[inline(always)]
    pub fn mrm_irq_tint_status(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,CacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Disables interrupt generation.\n1: Enables interrupt generation.\nNote: The Cache MRM generates a pulse-sensitive interrupt towards the ARM processor,"]
    #[inline(always)]
    pub fn mrm_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CacheMrmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CacheMrmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Freeze the \"misses/hits\" counters and reset the time interval counter to the programmed value in CACHE_MRM_TINT_REG.\n1: Enables the counters.\nNote: In case CACHE_MRM_CTRL_REG\\[MRM_START\\] is set to \'1\' and CACHE_MRM_TINT_REG (!=0) is used for the MRM interrupt generation, the time interval counter counts down (on a fixed reference clock of 32 MHz) until it\'s \'0\'. At that time CACHE_MRM_CTRL_REG\\[MRM_START\\] will be reset automatically to \'0\' by the MRM hardware and the MRM interrupt will be generated."]
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
#[doc = "Cache MRM (Miss Rate Monitor) HITS with 1 Wait State register (only Word (32-bits) access supported)."]
pub type CacheMrmHits1WsReg = crate::RegValueT<CacheMrmHits1WsReg_SPEC>;

impl CacheMrmHits1WsReg {
    #[doc = "Contains the amount of cache hits."]
    #[inline(always)]
    pub fn mrm_hits1ws(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Cache MRM (Miss Rate Monitor) HITS register (only Word (32-bits) access supported)."]
pub type CacheMrmHitsReg = crate::RegValueT<CacheMrmHitsReg_SPEC>;

impl CacheMrmHitsReg {
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
        CacheMrmHitsReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
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
#[doc = "Cache MRM (Miss Rate Monitor) HITS THRESHOLD register (only Word (32-bits) access supported)."]
pub type CacheMrmHitsThresReg = crate::RegValueT<CacheMrmHitsThresReg_SPEC>;

impl CacheMrmHitsThresReg {
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
        CacheMrmHitsThresReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
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
#[doc = "Cache MRM (Miss Rate Monitor) MISSES register (only Word (32-bits) access supported)."]
pub type CacheMrmMissesReg = crate::RegValueT<CacheMrmMissesReg_SPEC>;

impl CacheMrmMissesReg {
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
        CacheMrmMissesReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
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
#[doc = "Cache MRM (Miss Rate Monitor) THRESHOLD register (only Word (32-bits) access supported)."]
pub type CacheMrmMissesThresReg = crate::RegValueT<CacheMrmMissesThresReg_SPEC>;

impl CacheMrmMissesThresReg {
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
        CacheMrmMissesThresReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
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
#[doc = "Cache MRM (Miss Rate Monitor) TIME INTERVAL register (only Word (32-bits) access supported)."]
pub type CacheMrmTintReg = crate::RegValueT<CacheMrmTintReg_SPEC>;

impl CacheMrmTintReg {
    #[doc = "Defines the time interval for the monitoring in 32 MHz clock cycles. See also the description of CACHE_MRM_CTRL_REG\\[MRM_IRQ_TINT_STATUS\\].\nNote: When MRM_TINT=0 (unrealistic value), no interrupt will be generated."]
    #[inline(always)]
    pub fn mrm_tint(
        self,
    ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, CacheMrmTintReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ffff,1,0,u32, CacheMrmTintReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "SWD HW reset control register (only Word (32-bits) access supported)."]
pub type SwdResetReg = crate::RegValueT<SwdResetReg_SPEC>;

impl SwdResetReg {
    #[doc = "0: default.\n1: HW reset request (from the debugger tool). The register is automatically reset with a HW_RESET.\nThis bit can only be accessed by the debugger software and not by the application."]
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
