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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:57 +0000

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

    #[doc = "Cache associativity configuration register"]
    #[inline(always)]
    pub const fn cache_assoccfg_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CacheAssoccfgReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CacheAssoccfgReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Cache control register 1"]
    #[inline(always)]
    pub const fn cache_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CacheCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CacheCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Cache control register 2"]
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

    #[doc = "Cache line size configuration register"]
    #[inline(always)]
    pub const fn cache_lnsizecfg_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CacheLnsizecfgReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CacheLnsizecfgReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Cache MRM (Miss Rate Monitor) CONTROL register"]
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

    #[doc = "Cache MRM (Miss Rate Monitor) HITS register"]
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

    #[doc = "Cache MRM (Miss Rate Monitor) MISSES register"]
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

    #[doc = "Cache MRM (Miss Rate Monitor) THRESHOLD register"]
    #[inline(always)]
    pub const fn cache_mrm_thres_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CacheMrmThresReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CacheMrmThresReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Cache MRM (Miss Rate Monitor) TIME INTERVAL register"]
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

    #[doc = "SWD HW reset control register"]
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
pub struct CacheAssoccfgReg_SPEC;
impl crate::sealed::RegSpec for CacheAssoccfgReg_SPEC {
    type DataType = u32;
}

#[doc = "Cache associativity configuration register"]
pub type CacheAssoccfgReg = crate::RegValueT<CacheAssoccfgReg_SPEC>;

impl CacheAssoccfgReg {
    #[doc = "Cache associativity:\n0: 1-way (direct mapped)\n1: 2-way\n2: 4-way\n3: reserved.\nNote: Flush the cache controller before dynamically decreasing the associativity."]
    #[inline(always)]
    pub fn cache_assoc(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, CacheAssoccfgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,CacheAssoccfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CacheAssoccfgReg {
    #[inline(always)]
    fn default() -> CacheAssoccfgReg {
        <crate::RegValueT<CacheAssoccfgReg_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for CacheCtrl1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Cache control register 1"]
pub type CacheCtrl1Reg = crate::RegValueT<CacheCtrl1Reg_SPEC>;

impl CacheCtrl1Reg {
    #[doc = "Reserved. Always keep 0."]
    #[inline(always)]
    pub fn cache_res1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CacheCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CacheCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' into this bit, flushes the contents of the tag memories which invalidates the content of the cache memory.\nThe read of this bit is always \'0\'.\nNote: The flushing of the cache TAG memory takes 0x100 or 0x200 HCLK cycles for a Cache Data RAM size of 8 KB resp. 16 KB."]
    #[inline(always)]
    pub fn cache_flush(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CacheCtrl1Reg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,CacheCtrl1Reg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for CacheCtrl1Reg {
    #[inline(always)]
    fn default() -> CacheCtrl1Reg {
        <crate::RegValueT<CacheCtrl1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for CacheCtrl2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Cache control register 2"]
pub type CacheCtrl2Reg = crate::RegValueT<CacheCtrl2Reg_SPEC>;

impl CacheCtrl2Reg {
    #[doc = "Enable also the QSPI FLASH cacheability when remapped to OTP (cached).\nSee also the notes at \"CACHE_LEN\"."]
    #[inline(always)]
    pub fn enable_also_qspiflash_cached(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable also the OTP cacheability when remapped to QSPI FLASH (cached).\nSee also the notes at \"CACHE_LEN\"."]
    #[inline(always)]
    pub fn enable_also_otp_cached(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Cache controller clock gating is not enabled.\n1: Cache controller clock gating is enabled (enabling power saving).\nNote: This bit must be set to \'0\' (default) when setting the CACHE_FLUSH bit while executing from other than QSPI FLASH cached or OTP cached, e.g. from Booter or SYSRAM."]
    #[inline(always)]
    pub fn cache_cgen(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Cache Data and TAG memory read only.\n1: Cache Data and TAG memory read/write.\nThe TAG and Data memory are only updated by the cache controller.\nThere is no HW protection to prevent unauthorized access by the ARM.\nNote: When accessing the memory mapped Cache Data and TAG memory (for debugging purposes) only 32 bits access is allowed to the Cache Data memory and only 16 bits access is allowed to the Cache TAG memory."]
    #[inline(always)]
    pub fn cache_wen(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CacheCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,CacheCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Length of QSPI FLASH cacheable memory.\n(N+1)*64kbyte. N=1 to 512 (Max of 32 Mbyte).\nSetting CACHE_LEN=0 disables the cache.\nNote 1: The OTP memory is completely cacheable (when enabled).\nNote 2: The max. size/length of QSPI FLASH cacheable memory is 16 Mbyte when also OTP is cached."]
    #[inline(always)]
    pub fn cache_len(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, CacheCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1ff,
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
pub struct CacheLnsizecfgReg_SPEC;
impl crate::sealed::RegSpec for CacheLnsizecfgReg_SPEC {
    type DataType = u32;
}

#[doc = "Cache line size configuration register"]
pub type CacheLnsizecfgReg = crate::RegValueT<CacheLnsizecfgReg_SPEC>;

impl CacheLnsizecfgReg {
    #[doc = "Cache line size:\n0: 8 bytes,\n1: 16 bytes,\n2: 32 bytes,\n3: reserved.\nNote: Flush the cache just after the dynamic (run-time) reconfiguration of the cache with an 8 bytes cache line size: write the value \"01\" into the cache control register CACHE_CTRL1_REG just after the write of the value \"00\" into the cache line size configuration register CACHE_LNSIZECFG_REG."]
    #[inline(always)]
    pub fn cache_line(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, CacheLnsizecfgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            CacheLnsizecfgReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CacheLnsizecfgReg {
    #[inline(always)]
    fn default() -> CacheLnsizecfgReg {
        <crate::RegValueT<CacheLnsizecfgReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheMrmCtrlReg_SPEC;
impl crate::sealed::RegSpec for CacheMrmCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Cache MRM (Miss Rate Monitor) CONTROL register"]
pub type CacheMrmCtrlReg = crate::RegValueT<CacheMrmCtrlReg_SPEC>;

impl CacheMrmCtrlReg {
    #[doc = "0: No interrupt is generated.\n1: Interrupt (pulse-sensitive) is generated because the number of cache misses reached the programmed threshold (threshold != 0)."]
    #[inline(always)]
    pub fn mrm_irq_thres_status(
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

    #[doc = "0: Freeze the \"misses/hits\" counters and reset the time interval counter to the programmed value in CACHE_MRM_TINT_REG.\n1: Enables the counters.\nNote: In case CACHE_MRM_CTRL_REG\\[MRM_START\\] is set to \'1\' and CACHE_MRM_TINT_REG (!=0) is used for the MRM interrupt generation, the time interval counter counts down (on a fixed reference clock of 16 MHz) until it\'s \'0\'. At that time CACHE_MRM_CTRL_REG\\[MRM_START\\] will be reset automatically to \'0\' by the MRM hardware and the MRM interrupt will be generated."]
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
pub struct CacheMrmHitsReg_SPEC;
impl crate::sealed::RegSpec for CacheMrmHitsReg_SPEC {
    type DataType = u32;
}

#[doc = "Cache MRM (Miss Rate Monitor) HITS register"]
pub type CacheMrmHitsReg = crate::RegValueT<CacheMrmHitsReg_SPEC>;

impl CacheMrmHitsReg {
    #[doc = "Contains the amount of cache hits."]
    #[inline(always)]
    pub fn mrm_hits(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ffff,
        1,
        0,
        u32,
        u32,
        CacheMrmHitsReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ffff,
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
pub struct CacheMrmMissesReg_SPEC;
impl crate::sealed::RegSpec for CacheMrmMissesReg_SPEC {
    type DataType = u32;
}

#[doc = "Cache MRM (Miss Rate Monitor) MISSES register"]
pub type CacheMrmMissesReg = crate::RegValueT<CacheMrmMissesReg_SPEC>;

impl CacheMrmMissesReg {
    #[doc = "Contains the amount of cache misses."]
    #[inline(always)]
    pub fn mrm_misses(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ffff,
        1,
        0,
        u32,
        u32,
        CacheMrmMissesReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ffff,
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
pub struct CacheMrmThresReg_SPEC;
impl crate::sealed::RegSpec for CacheMrmThresReg_SPEC {
    type DataType = u32;
}

#[doc = "Cache MRM (Miss Rate Monitor) THRESHOLD register"]
pub type CacheMrmThresReg = crate::RegValueT<CacheMrmThresReg_SPEC>;

impl CacheMrmThresReg {
    #[doc = "Defines the threshold to trigger the interrupt generation. See also the description of CACHE_MRM_CTRL_REG\\[MRM_IRQ_THRES_STATUS\\].\nNote: When MRM_THRES=0 (unrealistic value), no interrupt will be generated."]
    #[inline(always)]
    pub fn mrm_thres(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ffff,
        1,
        0,
        u32,
        u32,
        CacheMrmThresReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ffff,
            1,
            0,
            u32,
            u32,
            CacheMrmThresReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CacheMrmThresReg {
    #[inline(always)]
    fn default() -> CacheMrmThresReg {
        <crate::RegValueT<CacheMrmThresReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheMrmTintReg_SPEC;
impl crate::sealed::RegSpec for CacheMrmTintReg_SPEC {
    type DataType = u32;
}

#[doc = "Cache MRM (Miss Rate Monitor) TIME INTERVAL register"]
pub type CacheMrmTintReg = crate::RegValueT<CacheMrmTintReg_SPEC>;

impl CacheMrmTintReg {
    #[doc = "Defines the time interval for the monitoring in 16 MHz clock cycles. See also the description of CACHE_MRM_CTRL_REG\\[MRM_IRQ_TINT_STATUS\\].\nNote: When MRM_TINT=0 (unrealistic value), no interrupt will be generated."]
    #[inline(always)]
    pub fn mrm_tint(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ffff,
        1,
        0,
        u32,
        u32,
        CacheMrmTintReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ffff,
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

#[doc = "SWD HW reset control register"]
pub type SwdResetReg = crate::RegValueT<SwdResetReg_SPEC>;

impl SwdResetReg {
    #[doc = "0: default.\n1: HW reset request without resetting the SWD and DAP controller. The register is automatically reset with a HW_RESET.\nThis bit can only be accessed by the debugger software and not by the application."]
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
