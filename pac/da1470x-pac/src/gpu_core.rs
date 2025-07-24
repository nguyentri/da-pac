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
#[doc = r"GPU_CORE registers"]
unsafe impl ::core::marker::Send for super::GpuCore {}
unsafe impl ::core::marker::Sync for super::GpuCore {}
impl super::GpuCore {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "#49: cache control register\n\n\nInternal caches can be enabled/disabled and flushed using this register.\n\nNote that caches will b"]
    #[inline(always)]
    pub const fn d2_cachectl(
        &self,
    ) -> &'static crate::common::Reg<self::D2Cachectl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Cachectl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[doc = "#58: Color key value\n\n\nThe R, G and B components of the internal color representation of a texel is compared with the color key"]
    #[inline(always)]
    pub const fn d2_colkey(
        &self,
    ) -> &'static crate::common::Reg<self::D2Colkey_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Colkey_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[doc = "#25: Base color register\n\n\nAll color registers are *write only*, reading will return undefined results.\n\nWhen using textures th"]
    #[inline(always)]
    pub const fn d2_color1(
        &self,
    ) -> &'static crate::common::Reg<self::D2Color1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Color1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "#26: Secondary color register\n\n\nSecondary color is relevant only when rendering patterns, textures or using\n\na D2C_BC2 blendmod"]
    #[inline(always)]
    pub const fn d2_color2(
        &self,
    ) -> &'static crate::common::Reg<self::D2Color2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Color2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "#0: geometry control register \n\n\nThis register controls the pixel enumeration and selection units, deciding \n\nwhich pixels are"]
    #[inline(always)]
    pub const fn d2_control(
        &self,
    ) -> &'static crate::common::Reg<self::D2Control_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Control_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "#1: surface control register\n\n\nThis register controls the colorization, texturing and blending units, deciding \n\nwhat color a p"]
    #[inline(always)]
    pub const fn d2_control2(
        &self,
    ) -> &'static crate::common::Reg<self::D2Control2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Control2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "#2: burst length limit control register\n\n\nThis register controls the burst length limit for the master bus interfaces\n\n\nLog2 of"]
    #[inline(always)]
    pub const fn d2_control3(
        &self,
    ) -> &'static crate::common::Reg<self::D2Control3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Control3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "#50: Displaylist start address\n\n\nSetting a new displaylist base address (writing to D2_DLISTSTART) \n\n*triggers* execution of th"]
    #[inline(always)]
    pub const fn d2_dliststart(
        &self,
    ) -> &'static crate::common::Reg<self::D2Dliststart_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Dliststart_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[doc = "#1: hardware version and feature set ID\n\n\nRead this (constant) register to identify the present hardware revision and\n\nfeature"]
    #[inline(always)]
    pub const fn d2_hwrevision(
        &self,
    ) -> &'static crate::common::Reg<self::D2Hwrevision_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Hwrevision_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[doc = "#48: interrupt control register\n\n\nDAVE2 features three sources for interrupts. They can be enabled and cleared individually."]
    #[inline(always)]
    pub const fn d2_irqctl(
        &self,
    ) -> &'static crate::common::Reg<self::D2Irqctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Irqctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "#22: Limiter1 band width parameter\n\n\nPostfilter\n\nFirst two limiter outputs can be routed through an additional unit before\n\ncla"]
    #[inline(always)]
    pub const fn d2_l1band(
        &self,
    ) -> &'static crate::common::Reg<self::D2L1Band_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L1Band_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "#4: Limiter1 start value\n\n\nAll limiter registers are *write only*, reading will return undefined results.\n\nLimiters must be ena"]
    #[inline(always)]
    pub const fn d2_l1start(
        &self,
    ) -> &'static crate::common::Reg<self::D2L1Start_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L1Start_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "#10: Limiter1 X-Axis increment\n\n\nThe xadd value is the 16:16 fixedpoint difference between two samples with a distance of 1 pix"]
    #[inline(always)]
    pub const fn d2_l1xadd(
        &self,
    ) -> &'static crate::common::Reg<self::D2L1Xadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L1Xadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "#16: Limiter1 Y-Axis increment\n\n\nThe yadd value is the 16:16 fixedpoint difference between two samples with a distance of 1 pix"]
    #[inline(always)]
    pub const fn d2_l1yadd(
        &self,
    ) -> &'static crate::common::Reg<self::D2L1Yadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L1Yadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "#23: Limiter2 band width parameter\n\n\nsee &lt;D2_L1BAND&gt;"]
    #[inline(always)]
    pub const fn d2_l2band(
        &self,
    ) -> &'static crate::common::Reg<self::D2L2Band_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L2Band_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "#5: Limiter2 start value\n\n\nsee &lt;D2_L1START&gt;"]
    #[inline(always)]
    pub const fn d2_l2start(
        &self,
    ) -> &'static crate::common::Reg<self::D2L2Start_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L2Start_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "#11: Limiter2 X-Axis increment\n\n\nsee &lt;D2_L1XADD&gt;"]
    #[inline(always)]
    pub const fn d2_l2xadd(
        &self,
    ) -> &'static crate::common::Reg<self::D2L2Xadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L2Xadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "#17: Limiter2 Y-Axis increment\n\n\nsee &lt;D2_L1YADD&gt;"]
    #[inline(always)]
    pub const fn d2_l2yadd(
        &self,
    ) -> &'static crate::common::Reg<self::D2L2Yadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L2Yadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "#6: Limiter3 start value\n\n\nsee &lt;D2_L1START&gt;"]
    #[inline(always)]
    pub const fn d2_l3start(
        &self,
    ) -> &'static crate::common::Reg<self::D2L3Start_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L3Start_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "#12: Limiter3 X-Axis increment"]
    #[inline(always)]
    pub const fn d2_l3xadd(
        &self,
    ) -> &'static crate::common::Reg<self::D2L3Xadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L3Xadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "#18: Limiter3 Y-Axis increment\n\n\nsee &lt;D2_L1YADD&gt;"]
    #[inline(always)]
    pub const fn d2_l3yadd(
        &self,
    ) -> &'static crate::common::Reg<self::D2L3Yadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L3Yadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "#7: Limiter4 start value\n\n\nsee &lt;D2_L1START&gt;"]
    #[inline(always)]
    pub const fn d2_l4start(
        &self,
    ) -> &'static crate::common::Reg<self::D2L4Start_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L4Start_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "#13: Limiter4 X-Axis increment\n\n\nsee &lt;D2_L1XADD&gt;"]
    #[inline(always)]
    pub const fn d2_l4xadd(
        &self,
    ) -> &'static crate::common::Reg<self::D2L4Xadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L4Xadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "#19: Limiter4 Y-Axis increment\n\n\nsee &lt;D2_L1YADD&gt;"]
    #[inline(always)]
    pub const fn d2_l4yadd(
        &self,
    ) -> &'static crate::common::Reg<self::D2L4Yadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L4Yadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "#8: Limiter5 start value\n\n\nsee &lt;D2_L1START&gt;"]
    #[inline(always)]
    pub const fn d2_l5start(
        &self,
    ) -> &'static crate::common::Reg<self::D2L5Start_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L5Start_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "#14: Limiter5 X-Axis increment\n\n\nsee &lt;D2_L1XADD&gt;"]
    #[inline(always)]
    pub const fn d2_l5xadd(
        &self,
    ) -> &'static crate::common::Reg<self::D2L5Xadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L5Xadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "#20: Limiter5 Y-Axis increment\n\n\nsee &lt;D2_L1YADD&gt;"]
    #[inline(always)]
    pub const fn d2_l5yadd(
        &self,
    ) -> &'static crate::common::Reg<self::D2L5Yadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L5Yadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "#9: Limiter6 start value\n\n\nsee &lt;D2_L1START&gt;"]
    #[inline(always)]
    pub const fn d2_l6start(
        &self,
    ) -> &'static crate::common::Reg<self::D2L6Start_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L6Start_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "#15: Limiter6 X-Axis increment\n\n\nsee &lt;D2_L1XADD&gt;"]
    #[inline(always)]
    pub const fn d2_l6xadd(
        &self,
    ) -> &'static crate::common::Reg<self::D2L6Xadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L6Xadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "#21: Limiter6 Y-Axis increment\n\n\nsee &lt;D2_L1YADD&gt;"]
    #[inline(always)]
    pub const fn d2_l6yadd(
        &self,
    ) -> &'static crate::common::Reg<self::D2L6Yadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2L6Yadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "#36: U Limiter start value\n\n\nThe start value is a 16:16 fixedpoint number valid at the first pixel of the\n\nbounding box."]
    #[inline(always)]
    pub const fn d2_lustart(
        &self,
    ) -> &'static crate::common::Reg<self::D2Lustart_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Lustart_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "#37: U Limiter X-Axis increment\n\n\nThe add value for U is the 16:16 fixedpoint difference between two samples with\n\na distance o"]
    #[inline(always)]
    pub const fn d2_luxadd(
        &self,
    ) -> &'static crate::common::Reg<self::D2Luxadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Luxadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "#38: U Limiter Y-Axis increment\n\n\nThe add value for U is the 16:16 fixedpoint difference between two samples with\n\na distance o"]
    #[inline(always)]
    pub const fn d2_luyadd(
        &self,
    ) -> &'static crate::common::Reg<self::D2Luyadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Luyadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "#40: V Limiter start value fractional part\n\n\nThe start value is a 32:16 fixedpoint number valid at the first pixel of the\n\nboun"]
    #[inline(always)]
    pub const fn d2_lvstartf(
        &self,
    ) -> &'static crate::common::Reg<self::D2Lvstartf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Lvstartf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "#39: V Limiter start value integer part"]
    #[inline(always)]
    pub const fn d2_lvstarti(
        &self,
    ) -> &'static crate::common::Reg<self::D2Lvstarti_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Lvstarti_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[doc = "#41: V Limiter X-Axis increment integer part\n\n\nThe add value for V is the 32:16 fixedpoint difference between two samples with"]
    #[inline(always)]
    pub const fn d2_lvxaddi(
        &self,
    ) -> &'static crate::common::Reg<self::D2Lvxaddi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Lvxaddi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "#42: V Limiter Y-Axis increment integer part\n\n\nThe add value for V is the 32:16 fixedpoint difference between two samples with"]
    #[inline(always)]
    pub const fn d2_lvyaddi(
        &self,
    ) -> &'static crate::common::Reg<self::D2Lvyaddi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Lvyaddi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[doc = "#43: V Limiter X and Y increment fractional parts"]
    #[inline(always)]
    pub const fn d2_lvyxaddf(
        &self,
    ) -> &'static crate::common::Reg<self::D2Lvyxaddf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Lvyxaddf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[doc = "#32: address of the first pixel in framebuffer\n\n\nWriting to &lt;D2_ORIGIN&gt; will *trigger* DAVE2 to start rendering."]
    #[inline(always)]
    pub const fn d2_origin(
        &self,
    ) -> &'static crate::common::Reg<self::D2Origin_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Origin_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "#29: Pattern register\n\n\nEach bit in the pattern register is interpreted as a reference to one of the\n\ntwo color registers ( 0bi"]
    #[inline(always)]
    pub const fn d2_pattern(
        &self,
    ) -> &'static crate::common::Reg<self::D2Pattern_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Pattern_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "#51: Performance counter\n\n\nWriting to the D2_PERFCOUNT1 register resets the first internal performance counter\n\nto the specifie"]
    #[inline(always)]
    pub const fn d2_perfcount1(
        &self,
    ) -> &'static crate::common::Reg<self::D2Perfcount1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Perfcount1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[doc = "#52: Performance counter\n\n\nWriting to the D2_PERFCOUNT2 register resets the second internal performance counter\n\nto the specifi"]
    #[inline(always)]
    pub const fn d2_perfcount2(
        &self,
    ) -> &'static crate::common::Reg<self::D2Perfcount2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Perfcount2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "#53: Performance counters control register\n\n\nSelect the internal event that will increment &lt;D2_PERFCOUNT1&gt; respectively &"]
    #[inline(always)]
    pub const fn d2_perftrigger(
        &self,
    ) -> &'static crate::common::Reg<self::D2Perftrigger_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Perftrigger_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[doc = "#31: framebuffer pitch and spanstore delay"]
    #[inline(always)]
    pub const fn d2_pitch(
        &self,
    ) -> &'static crate::common::Reg<self::D2Pitch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Pitch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = "#30: bounding box dimension"]
    #[inline(always)]
    pub const fn d2_size(
        &self,
    ) -> &'static crate::common::Reg<self::D2Size_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Size_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "#0: status control register\n\n\nThe current dave status can be polled by reading this register. It contains a \n\ncombination of th"]
    #[inline(always)]
    pub const fn d2_status(
        &self,
    ) -> &'static crate::common::Reg<self::D2Status_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Status_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[doc = "#54: Color Lookup Table for the indexed texture format 16x24bit\n\n\nTriggers a write into the CLUT if CLUT size is 16 x 24bit."]
    #[inline(always)]
    pub const fn d2_texclut(
        &self,
    ) -> &'static crate::common::Reg<self::D2Texclut_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Texclut_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }

    #[doc = "#55: Color Lookup Table write address for the indexed texture format 256x32bit\n\n\nStart address for consecutive writes to &lt;D2"]
    #[inline(always)]
    pub const fn d2_texclut_addr(
        &self,
    ) -> &'static crate::common::Reg<self::D2TexclutAddr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2TexclutAddr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(220usize),
            )
        }
    }

    #[doc = "#56: Color Lookup Table write data for the indexed texture format 256x32bit\n\n\nWrites one 32 bit color value into the CLUT if CL"]
    #[inline(always)]
    pub const fn d2_texclut_data(
        &self,
    ) -> &'static crate::common::Reg<self::D2TexclutData_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2TexclutData_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[doc = "#57: Offset to the texture index for the indexed texture formats i8, i4, i2 and i1\n\n\nThe index offset is combined with the text"]
    #[inline(always)]
    pub const fn d2_texclut_offset(
        &self,
    ) -> &'static crate::common::Reg<self::D2TexclutOffset_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2TexclutOffset_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[doc = "#46: Texture size or texture address mask\n\n\nDepending on the clamping mode this register encodes the clamp limit or wrap\n\nmask."]
    #[inline(always)]
    pub const fn d2_texmask(
        &self,
    ) -> &'static crate::common::Reg<self::D2Texmask_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Texmask_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[doc = "#47: Texture base address\n\n\nAll texture registers are *write only*, reading will return undefined results.\n\nPatterns and textur"]
    #[inline(always)]
    pub const fn d2_texorigin(
        &self,
    ) -> &'static crate::common::Reg<self::D2Texorigin_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Texorigin_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(188usize),
            )
        }
    }

    #[doc = "#45: Texels per texture line\n\n\nPitch is equal or bigger than texture width."]
    #[inline(always)]
    pub const fn d2_texpitch(
        &self,
    ) -> &'static crate::common::Reg<self::D2Texpitch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D2Texpitch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(180usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Cachectl_SPEC;
impl crate::sealed::RegSpec for D2Cachectl_SPEC {
    type DataType = u32;
}

#[doc = "#49: cache control register\n\n\nInternal caches can be enabled/disabled and flushed using this register.\n\nNote that caches will b"]
pub type D2Cachectl = crate::RegValueT<D2Cachectl_SPEC>;

impl D2Cachectl {
    #[doc = "Flush texture cache"]
    #[inline(always)]
    pub fn d2c_cachectl_flush_tx(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, D2Cachectl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,D2Cachectl_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Texture cache enable"]
    #[inline(always)]
    pub fn d2c_cachectl_enable_tx(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, D2Cachectl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,D2Cachectl_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Flush framebuffer cache"]
    #[inline(always)]
    pub fn d2c_cachectl_flush_fb(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, D2Cachectl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,D2Cachectl_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Framebuffer cache enable"]
    #[inline(always)]
    pub fn d2c_cachectl_enable_fb(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, D2Cachectl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,D2Cachectl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Cachectl {
    #[inline(always)]
    fn default() -> D2Cachectl {
        <crate::RegValueT<D2Cachectl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Colkey_SPEC;
impl crate::sealed::RegSpec for D2Colkey_SPEC {
    type DataType = u32;
}

#[doc = "#58: Color key value\n\n\nThe R, G and B components of the internal color representation of a texel is compared with the color key"]
pub type D2Colkey = crate::RegValueT<D2Colkey_SPEC>;

impl D2Colkey {
    #[doc = "Color Key Value RGB888"]
    #[inline(always)]
    pub fn colkey_rgb(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, D2Colkey_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,D2Colkey_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Colkey {
    #[inline(always)]
    fn default() -> D2Colkey {
        <crate::RegValueT<D2Colkey_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Color1_SPEC;
impl crate::sealed::RegSpec for D2Color1_SPEC {
    type DataType = u32;
}

#[doc = "#25: Base color register\n\n\nAll color registers are *write only*, reading will return undefined results.\n\nWhen using textures th"]
pub type D2Color1 = crate::RegValueT<D2Color1_SPEC>;

impl D2Color1 {
    #[inline(always)]
    pub fn color1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2Color1_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2Color1_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Color1 {
    #[inline(always)]
    fn default() -> D2Color1 {
        <crate::RegValueT<D2Color1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Color2_SPEC;
impl crate::sealed::RegSpec for D2Color2_SPEC {
    type DataType = u32;
}

#[doc = "#26: Secondary color register\n\n\nSecondary color is relevant only when rendering patterns, textures or using\n\na D2C_BC2 blendmod"]
pub type D2Color2 = crate::RegValueT<D2Color2_SPEC>;

impl D2Color2 {
    #[inline(always)]
    pub fn color2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2Color2_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2Color2_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Color2 {
    #[inline(always)]
    fn default() -> D2Color2 {
        <crate::RegValueT<D2Color2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Control_SPEC;
impl crate::sealed::RegSpec for D2Control_SPEC {
    type DataType = u32;
}

#[doc = "#0: geometry control register \n\n\nThis register controls the pixel enumeration and selection units, deciding \n\nwhich pixels are"]
pub type D2Control = crate::RegValueT<D2Control_SPEC>;

impl D2Control {
    #[doc = "reserved for SoftDave internal use"]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, D2Control_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "increase precision of limiters from 16.16 to 10.22"]
    #[inline(always)]
    pub fn d2c_limiterprecision(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "nextline span start is always equal or left to current-line span start"]
    #[inline(always)]
    pub fn d2c_spanstore(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "shape is horizontally convex. only a single span per scanline"]
    #[inline(always)]
    pub fn d2c_spanabort(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "combine outputs C & D as union (output is final)"]
    #[inline(always)]
    pub fn d2c_unioncd(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "combine outputs A & B as union (output is called C)"]
    #[inline(always)]
    pub fn d2c_unionab(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "combine limter 5 & 6 as union (output is called D)"]
    #[inline(always)]
    pub fn d2c_union56(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "combine limter 3 & 4 as union (output is called B)"]
    #[inline(always)]
    pub fn d2c_union34(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "combine limter 1 & 2 as union (output is called A)"]
    #[inline(always)]
    pub fn d2c_union12(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "enable band postprocess for limiter 2 (see &lt;D2_L2BAND&gt;)"]
    #[inline(always)]
    pub fn d2c_band2enable(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "enable band postprocess for limiter 1 (see &lt;D2_L1BAND&gt;)"]
    #[inline(always)]
    pub fn d2c_band1enable(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "enable limiter 6 threshold mode"]
    #[inline(always)]
    pub fn d2c_lim6threshold(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "enable limiter 5 threshold mode"]
    #[inline(always)]
    pub fn d2c_lim5threshold(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "enable limiter 4 threshold mode"]
    #[inline(always)]
    pub fn d2c_lim4threshold(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "enable limiter 3 threshold mode"]
    #[inline(always)]
    pub fn d2c_lim3threshold(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "enable limiter 2 threshold mode"]
    #[inline(always)]
    pub fn d2c_lim2threshold(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10,1,0,D2Control_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "enable limiter 1 threshold mode"]
    #[inline(always)]
    pub fn d2c_lim1threshold(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, D2Control_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "enable quadratic coupling of limiters 5 and 6"]
    #[inline(always)]
    pub fn d2c_quad3enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, D2Control_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "enable quadratic coupling of limiters 3 and 4"]
    #[inline(always)]
    pub fn d2c_quad2enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, D2Control_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "enable quadratic coupling of limiters 1 and 2"]
    #[inline(always)]
    pub fn d2c_quad1enable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, D2Control_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "enable limiter 6"]
    #[inline(always)]
    pub fn d2c_lim6enable(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, D2Control_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "enable limiter 5"]
    #[inline(always)]
    pub fn d2c_lim5enable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, D2Control_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "enable limiter 4"]
    #[inline(always)]
    pub fn d2c_lim4enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, D2Control_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "enable limiter 3"]
    #[inline(always)]
    pub fn d2c_lim3enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, D2Control_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "enable limiter 2"]
    #[inline(always)]
    pub fn d2c_lim2enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, D2Control_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "enable limiter 1"]
    #[inline(always)]
    pub fn d2c_lim1enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, D2Control_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, D2Control_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for D2Control {
    #[inline(always)]
    fn default() -> D2Control {
        <crate::RegValueT<D2Control_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Control2_SPEC;
impl crate::sealed::RegSpec for D2Control2_SPEC {
    type DataType = u32;
}

#[doc = "#1: surface control register\n\n\nThis register controls the colorization, texturing and blending units, deciding \n\nwhat color a p"]
pub type D2Control2 = crate::RegValueT<D2Control2_SPEC>;

impl D2Control2 {
    #[doc = "bit1 of RLE texel format"]
    #[inline(always)]
    pub fn d2c_rleformat2(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "bit0 of RLE texel format"]
    #[inline(always)]
    pub fn d2c_rleformat1(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "dst factor for alpha channel will be inverted (meaning 1-a or 1-1 depending on BDFA)"]
    #[inline(always)]
    pub fn d2c_bdia(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "src factor for alpha channel will be inverted (meaning 1-a or 1-1 depending on BSFA)"]
    #[inline(always)]
    pub fn d2c_bsia(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "bit0 of the CLUT entry format"]
    #[inline(always)]
    pub fn d2c_clutformat1(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "enable color keying (see also &lt;D2_COLKEY&gt; and feature bits of <D2_HWREVISION>)"]
    #[inline(always)]
    pub fn d2c_colkey_enable(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "enable the use of the CLUT (see also feature bits of &lt;D2_HWREVISION&gt;); if disabled the texture indices are written to FB"]
    #[inline(always)]
    pub fn d2c_clut_enable(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "enable RLE decoder (see also feature bits of &lt;D2_HWREVISION&gt;)"]
    #[inline(always)]
    pub fn d2c_rle_enable(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "bit1 of the \'alpha source\' (depends on USE_ACB)"]
    #[inline(always)]
    pub fn d2c_writealpha2(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "bit0 of the \'alpha source\' (depends on USE_ACB)"]
    #[inline(always)]
    pub fn d2c_writealpha1(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "bit1 of the framebuffer format descriptor"]
    #[inline(always)]
    pub fn d2c_writeformat2(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "bit0 of the framebuffer format descriptor"]
    #[inline(always)]
    pub fn d2c_writeformat1(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "bit1 of the texture format descriptor"]
    #[inline(always)]
    pub fn d2c_readformat2(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "bit0 of the texture format descriptor"]
    #[inline(always)]
    pub fn d2c_readformat1(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "linear filtering on texture v axis"]
    #[inline(always)]
    pub fn d2c_texturefiltery(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "linear filtering on texture u axis"]
    #[inline(always)]
    pub fn d2c_texturefilterx(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "clamp instead of mask v coordinate"]
    #[inline(always)]
    pub fn d2c_textureclampy(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "clamp instead of mask u coordinate"]
    #[inline(always)]
    pub fn d2c_textureclampx(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "blending for color channels is done with &lt;D2_COLOR2&gt; instead of the real dst value"]
    #[inline(always)]
    pub fn d2c_bc2(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "dst factor for color channels will be inverted (meaning 1-a or 1-1 depending on BDF)"]
    #[inline(always)]
    pub fn d2c_bdi(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "src factor for color channels will be inverted (meaning 1-a or 1-1 depending on BSF)"]
    #[inline(always)]
    pub fn d2c_bsi(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "dst factor for color channels is alpha (factor is 1 per default)"]
    #[inline(always)]
    pub fn d2c_bdf(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "src factor for color channels is alpha (factor is 1 per default)"]
    #[inline(always)]
    pub fn d2c_bsf(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "bit2 of the framebuffer format descriptor"]
    #[inline(always)]
    pub fn d2c_writeformat3(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "dst factor for alpha channel is alpha (factor is 1 per default)"]
    #[inline(always)]
    pub fn d2c_bdfa(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "src factor for alpha channel is alpha (factor is 1 per default)"]
    #[inline(always)]
    pub fn d2c_bsfa(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "bit3 of the texture format descriptor"]
    #[inline(always)]
    pub fn d2c_readformat4(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "bit2 of the texture format descriptor"]
    #[inline(always)]
    pub fn d2c_readformat3(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "use full alpha channel blending, else use write-alpha mode"]
    #[inline(always)]
    pub fn use_acb(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Limiter 5 is used as pattern index instead of the default U-Limiter"]
    #[inline(always)]
    pub fn d2c_patternsourcel5(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Pixel source is read from texture and used as an alpha to blend between &lt;D2_COLOR1&gt; and <D2_COLOR2>"]
    #[inline(always)]
    pub fn d2c_textureenable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Pixel source is a pattern color (blend of &lt;D2_COLOR1&gt; and <D2_COLOR2> depending on <D2_PATTERN> and pattern index)"]
    #[inline(always)]
    pub fn d2c_patternenable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, D2Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,D2Control2_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Control2 {
    #[inline(always)]
    fn default() -> D2Control2 {
        <crate::RegValueT<D2Control2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Control3_SPEC;
impl crate::sealed::RegSpec for D2Control3_SPEC {
    type DataType = u32;
}

#[doc = "#2: burst length limit control register\n\n\nThis register controls the burst length limit for the master bus interfaces\n\n\nLog2 of"]
pub type D2Control3 = crate::RegValueT<D2Control3_SPEC>;

impl D2Control3 {
    #[doc = "Log2 of the burst length limit for MDL read"]
    #[inline(always)]
    pub fn burstlength_mdl(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, u8, D2Control3_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<24,0x7,1,0,u8,u8,D2Control3_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Log2 of the burst length limit for MTX read"]
    #[inline(always)]
    pub fn burstlength_mtx(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, D2Control3_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,D2Control3_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Log2 of the burst length limit for MFB write"]
    #[inline(always)]
    pub fn burstlength_mfbw(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, D2Control3_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,D2Control3_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Log2 of the burst length limit for MFB read"]
    #[inline(always)]
    pub fn burstlength_mfbr(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, D2Control3_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,D2Control3_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Control3 {
    #[inline(always)]
    fn default() -> D2Control3 {
        <crate::RegValueT<D2Control3_SPEC> as RegisterValue<_>>::new(67372036)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Dliststart_SPEC;
impl crate::sealed::RegSpec for D2Dliststart_SPEC {
    type DataType = u32;
}

#[doc = "#50: Displaylist start address\n\n\nSetting a new displaylist base address (writing to D2_DLISTSTART) \n\n*triggers* execution of th"]
pub type D2Dliststart = crate::RegValueT<D2Dliststart_SPEC>;

impl D2Dliststart {
    #[inline(always)]
    pub fn dliststart(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        D2Dliststart_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2Dliststart_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2Dliststart {
    #[inline(always)]
    fn default() -> D2Dliststart {
        <crate::RegValueT<D2Dliststart_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Hwrevision_SPEC;
impl crate::sealed::RegSpec for D2Hwrevision_SPEC {
    type DataType = u32;
}

#[doc = "#1: hardware version and feature set ID\n\n\nRead this (constant) register to identify the present hardware revision and\n\nfeature"]
pub type D2Hwrevision = crate::RegValueT<D2Hwrevision_SPEC>;

impl D2Hwrevision {
    #[doc = "bursts can be split with respect to burst length limit"]
    #[inline(always)]
    pub fn fb_burstsplitting(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, D2Hwrevision_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "full alpha channel blending available"]
    #[inline(always)]
    pub fn fb_alphachannelblending(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, D2Hwrevision_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "increasable precision of limiters available"]
    #[inline(always)]
    pub fn fb_hilimiterprecision(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, D2Hwrevision_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26,1,0,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "color keying available"]
    #[inline(always)]
    pub fn fb_colorkey(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, D2Hwrevision_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25,1,0,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "extend CLUT to 256x32bit ARGB8888"]
    #[inline(always)]
    pub fn fb_texclut256(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, D2Hwrevision_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "RLE texture decoder available"]
    #[inline(always)]
    pub fn fb_rleunit(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, D2Hwrevision_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Framebuffer prefetch available"]
    #[inline(always)]
    pub fn fb_fbprefetch(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, D2Hwrevision_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22,1,0,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Color Lookup Table 16x24bit for indexed textureformat available"]
    #[inline(always)]
    pub fn fb_texclut(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, D2Hwrevision_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Two performance counters available"]
    #[inline(always)]
    pub fn fb_perfcount(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, D2Hwrevision_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Texture Cache available"]
    #[inline(always)]
    pub fn fb_txcache(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, D2Hwrevision_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Framebuffer Cache available"]
    #[inline(always)]
    pub fn fb_fbcache(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, D2Hwrevision_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DisplayListReader available"]
    #[inline(always)]
    pub fn fb_dlr(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, D2Hwrevision_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Software D/AVE"]
    #[inline(always)]
    pub fn fb_swdave(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, D2Hwrevision_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "D/AVE Type"]
    #[inline(always)]
    pub fn hwtype(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, D2Hwrevision_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Branch number"]
    #[inline(always)]
    pub fn hwbranch(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, D2Hwrevision_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Revision number"]
    #[inline(always)]
    pub fn hwrevision(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, D2Hwrevision_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,D2Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Hwrevision {
    #[inline(always)]
    fn default() -> D2Hwrevision {
        <crate::RegValueT<D2Hwrevision_SPEC> as RegisterValue<_>>::new(532545549)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Irqctl_SPEC;
impl crate::sealed::RegSpec for D2Irqctl_SPEC {
    type DataType = u32;
}

#[doc = "#48: interrupt control register\n\n\nDAVE2 features three sources for interrupts. They can be enabled and cleared individually."]
pub type D2Irqctl = crate::RegValueT<D2Irqctl_SPEC>;

impl D2Irqctl {
    #[doc = "Clear Interrupt \'Bus error\'"]
    #[inline(always)]
    pub fn d2irqctl_clr_bus_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, D2Irqctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, D2Irqctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Interruptmask enable \'Bus error\'"]
    #[inline(always)]
    pub fn d2irqctl_enable_bus_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, D2Irqctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, D2Irqctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Clear Interrupt \'Displaylist is finished\'. Make sure to clear the IRQ before starting the DLR again! No register writes must be done by the CPU while the DLR is active!"]
    #[inline(always)]
    pub fn d2irqctl_clr_finish_dlist(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, D2Irqctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, D2Irqctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Clear Interrupt \'Enumeration is finished\'"]
    #[inline(always)]
    pub fn d2irqctl_clr_finish_enum(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, D2Irqctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, D2Irqctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Interruptmask enable \'Displaylist is finished\'"]
    #[inline(always)]
    pub fn d2irqctl_enable_finish_dlist(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, D2Irqctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, D2Irqctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Interruptmask enable \'Enumeration is finished\'"]
    #[inline(always)]
    pub fn d2irqctl_enable_finish_enum(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, D2Irqctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, D2Irqctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for D2Irqctl {
    #[inline(always)]
    fn default() -> D2Irqctl {
        <crate::RegValueT<D2Irqctl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L1Band_SPEC;
impl crate::sealed::RegSpec for D2L1Band_SPEC {
    type DataType = u32;
}

#[doc = "#22: Limiter1 band width parameter\n\n\nPostfilter\n\nFirst two limiter outputs can be routed through an additional unit before\n\ncla"]
pub type D2L1Band = crate::RegValueT<D2L1Band_SPEC>;

impl D2L1Band {
    #[inline(always)]
    pub fn l1band(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L1Band_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2L1Band_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2L1Band {
    #[inline(always)]
    fn default() -> D2L1Band {
        <crate::RegValueT<D2L1Band_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L1Start_SPEC;
impl crate::sealed::RegSpec for D2L1Start_SPEC {
    type DataType = u32;
}

#[doc = "#4: Limiter1 start value\n\n\nAll limiter registers are *write only*, reading will return undefined results.\n\nLimiters must be ena"]
pub type D2L1Start = crate::RegValueT<D2L1Start_SPEC>;

impl D2L1Start {
    #[inline(always)]
    pub fn l1start(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L1Start_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2L1Start_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2L1Start {
    #[inline(always)]
    fn default() -> D2L1Start {
        <crate::RegValueT<D2L1Start_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L1Xadd_SPEC;
impl crate::sealed::RegSpec for D2L1Xadd_SPEC {
    type DataType = u32;
}

#[doc = "#10: Limiter1 X-Axis increment\n\n\nThe xadd value is the 16:16 fixedpoint difference between two samples with a distance of 1 pix"]
pub type D2L1Xadd = crate::RegValueT<D2L1Xadd_SPEC>;

impl D2L1Xadd {
    #[inline(always)]
    pub fn l1xadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L1Xadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2L1Xadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2L1Xadd {
    #[inline(always)]
    fn default() -> D2L1Xadd {
        <crate::RegValueT<D2L1Xadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L1Yadd_SPEC;
impl crate::sealed::RegSpec for D2L1Yadd_SPEC {
    type DataType = u32;
}

#[doc = "#16: Limiter1 Y-Axis increment\n\n\nThe yadd value is the 16:16 fixedpoint difference between two samples with a distance of 1 pix"]
pub type D2L1Yadd = crate::RegValueT<D2L1Yadd_SPEC>;

impl D2L1Yadd {
    #[inline(always)]
    pub fn l1yadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L1Yadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2L1Yadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2L1Yadd {
    #[inline(always)]
    fn default() -> D2L1Yadd {
        <crate::RegValueT<D2L1Yadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L2Band_SPEC;
impl crate::sealed::RegSpec for D2L2Band_SPEC {
    type DataType = u32;
}

#[doc = "#23: Limiter2 band width parameter\n\n\nsee &lt;D2_L1BAND&gt;"]
pub type D2L2Band = crate::RegValueT<D2L2Band_SPEC>;

impl D2L2Band {
    #[inline(always)]
    pub fn l2band(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L2Band_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2L2Band_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2L2Band {
    #[inline(always)]
    fn default() -> D2L2Band {
        <crate::RegValueT<D2L2Band_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L2Start_SPEC;
impl crate::sealed::RegSpec for D2L2Start_SPEC {
    type DataType = u32;
}

#[doc = "#5: Limiter2 start value\n\n\nsee &lt;D2_L1START&gt;"]
pub type D2L2Start = crate::RegValueT<D2L2Start_SPEC>;

impl D2L2Start {
    #[inline(always)]
    pub fn l2start(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L2Start_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2L2Start_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2L2Start {
    #[inline(always)]
    fn default() -> D2L2Start {
        <crate::RegValueT<D2L2Start_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L2Xadd_SPEC;
impl crate::sealed::RegSpec for D2L2Xadd_SPEC {
    type DataType = u32;
}

#[doc = "#11: Limiter2 X-Axis increment\n\n\nsee &lt;D2_L1XADD&gt;"]
pub type D2L2Xadd = crate::RegValueT<D2L2Xadd_SPEC>;

impl D2L2Xadd {
    #[inline(always)]
    pub fn l2xadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L2Xadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2L2Xadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2L2Xadd {
    #[inline(always)]
    fn default() -> D2L2Xadd {
        <crate::RegValueT<D2L2Xadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L2Yadd_SPEC;
impl crate::sealed::RegSpec for D2L2Yadd_SPEC {
    type DataType = u32;
}

#[doc = "#17: Limiter2 Y-Axis increment\n\n\nsee &lt;D2_L1YADD&gt;"]
pub type D2L2Yadd = crate::RegValueT<D2L2Yadd_SPEC>;

impl D2L2Yadd {
    #[inline(always)]
    pub fn l2yadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L2Yadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2L2Yadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2L2Yadd {
    #[inline(always)]
    fn default() -> D2L2Yadd {
        <crate::RegValueT<D2L2Yadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L3Start_SPEC;
impl crate::sealed::RegSpec for D2L3Start_SPEC {
    type DataType = u32;
}

#[doc = "#6: Limiter3 start value\n\n\nsee &lt;D2_L1START&gt;"]
pub type D2L3Start = crate::RegValueT<D2L3Start_SPEC>;

impl D2L3Start {
    #[inline(always)]
    pub fn l3start(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L3Start_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2L3Start_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2L3Start {
    #[inline(always)]
    fn default() -> D2L3Start {
        <crate::RegValueT<D2L3Start_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L3Xadd_SPEC;
impl crate::sealed::RegSpec for D2L3Xadd_SPEC {
    type DataType = u32;
}

#[doc = "#12: Limiter3 X-Axis increment"]
pub type D2L3Xadd = crate::RegValueT<D2L3Xadd_SPEC>;

impl D2L3Xadd {
    #[inline(always)]
    pub fn l3xadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L3Xadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2L3Xadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2L3Xadd {
    #[inline(always)]
    fn default() -> D2L3Xadd {
        <crate::RegValueT<D2L3Xadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L3Yadd_SPEC;
impl crate::sealed::RegSpec for D2L3Yadd_SPEC {
    type DataType = u32;
}

#[doc = "#18: Limiter3 Y-Axis increment\n\n\nsee &lt;D2_L1YADD&gt;"]
pub type D2L3Yadd = crate::RegValueT<D2L3Yadd_SPEC>;

impl D2L3Yadd {
    #[inline(always)]
    pub fn l3yadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L3Yadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2L3Yadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2L3Yadd {
    #[inline(always)]
    fn default() -> D2L3Yadd {
        <crate::RegValueT<D2L3Yadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L4Start_SPEC;
impl crate::sealed::RegSpec for D2L4Start_SPEC {
    type DataType = u32;
}

#[doc = "#7: Limiter4 start value\n\n\nsee &lt;D2_L1START&gt;"]
pub type D2L4Start = crate::RegValueT<D2L4Start_SPEC>;

impl D2L4Start {
    #[inline(always)]
    pub fn l4start(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L4Start_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2L4Start_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2L4Start {
    #[inline(always)]
    fn default() -> D2L4Start {
        <crate::RegValueT<D2L4Start_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L4Xadd_SPEC;
impl crate::sealed::RegSpec for D2L4Xadd_SPEC {
    type DataType = u32;
}

#[doc = "#13: Limiter4 X-Axis increment\n\n\nsee &lt;D2_L1XADD&gt;"]
pub type D2L4Xadd = crate::RegValueT<D2L4Xadd_SPEC>;

impl D2L4Xadd {
    #[inline(always)]
    pub fn l4xadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L4Xadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2L4Xadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2L4Xadd {
    #[inline(always)]
    fn default() -> D2L4Xadd {
        <crate::RegValueT<D2L4Xadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L4Yadd_SPEC;
impl crate::sealed::RegSpec for D2L4Yadd_SPEC {
    type DataType = u32;
}

#[doc = "#19: Limiter4 Y-Axis increment\n\n\nsee &lt;D2_L1YADD&gt;"]
pub type D2L4Yadd = crate::RegValueT<D2L4Yadd_SPEC>;

impl D2L4Yadd {
    #[inline(always)]
    pub fn l4yadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L4Yadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2L4Yadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2L4Yadd {
    #[inline(always)]
    fn default() -> D2L4Yadd {
        <crate::RegValueT<D2L4Yadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L5Start_SPEC;
impl crate::sealed::RegSpec for D2L5Start_SPEC {
    type DataType = u32;
}

#[doc = "#8: Limiter5 start value\n\n\nsee &lt;D2_L1START&gt;"]
pub type D2L5Start = crate::RegValueT<D2L5Start_SPEC>;

impl D2L5Start {
    #[inline(always)]
    pub fn l5start(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L5Start_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2L5Start_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2L5Start {
    #[inline(always)]
    fn default() -> D2L5Start {
        <crate::RegValueT<D2L5Start_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L5Xadd_SPEC;
impl crate::sealed::RegSpec for D2L5Xadd_SPEC {
    type DataType = u32;
}

#[doc = "#14: Limiter5 X-Axis increment\n\n\nsee &lt;D2_L1XADD&gt;"]
pub type D2L5Xadd = crate::RegValueT<D2L5Xadd_SPEC>;

impl D2L5Xadd {
    #[inline(always)]
    pub fn l5xadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L5Xadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2L5Xadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2L5Xadd {
    #[inline(always)]
    fn default() -> D2L5Xadd {
        <crate::RegValueT<D2L5Xadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L5Yadd_SPEC;
impl crate::sealed::RegSpec for D2L5Yadd_SPEC {
    type DataType = u32;
}

#[doc = "#20: Limiter5 Y-Axis increment\n\n\nsee &lt;D2_L1YADD&gt;"]
pub type D2L5Yadd = crate::RegValueT<D2L5Yadd_SPEC>;

impl D2L5Yadd {
    #[inline(always)]
    pub fn l5yadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L5Yadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2L5Yadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2L5Yadd {
    #[inline(always)]
    fn default() -> D2L5Yadd {
        <crate::RegValueT<D2L5Yadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L6Start_SPEC;
impl crate::sealed::RegSpec for D2L6Start_SPEC {
    type DataType = u32;
}

#[doc = "#9: Limiter6 start value\n\n\nsee &lt;D2_L1START&gt;"]
pub type D2L6Start = crate::RegValueT<D2L6Start_SPEC>;

impl D2L6Start {
    #[inline(always)]
    pub fn l6start(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L6Start_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2L6Start_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2L6Start {
    #[inline(always)]
    fn default() -> D2L6Start {
        <crate::RegValueT<D2L6Start_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L6Xadd_SPEC;
impl crate::sealed::RegSpec for D2L6Xadd_SPEC {
    type DataType = u32;
}

#[doc = "#15: Limiter6 X-Axis increment\n\n\nsee &lt;D2_L1XADD&gt;"]
pub type D2L6Xadd = crate::RegValueT<D2L6Xadd_SPEC>;

impl D2L6Xadd {
    #[inline(always)]
    pub fn l6xadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L6Xadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2L6Xadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2L6Xadd {
    #[inline(always)]
    fn default() -> D2L6Xadd {
        <crate::RegValueT<D2L6Xadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2L6Yadd_SPEC;
impl crate::sealed::RegSpec for D2L6Yadd_SPEC {
    type DataType = u32;
}

#[doc = "#21: Limiter6 Y-Axis increment\n\n\nsee &lt;D2_L1YADD&gt;"]
pub type D2L6Yadd = crate::RegValueT<D2L6Yadd_SPEC>;

impl D2L6Yadd {
    #[inline(always)]
    pub fn l6yadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2L6Yadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2L6Yadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2L6Yadd {
    #[inline(always)]
    fn default() -> D2L6Yadd {
        <crate::RegValueT<D2L6Yadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Lustart_SPEC;
impl crate::sealed::RegSpec for D2Lustart_SPEC {
    type DataType = u32;
}

#[doc = "#36: U Limiter start value\n\n\nThe start value is a 16:16 fixedpoint number valid at the first pixel of the\n\nbounding box."]
pub type D2Lustart = crate::RegValueT<D2Lustart_SPEC>;

impl D2Lustart {
    #[inline(always)]
    pub fn lustart(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2Lustart_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2Lustart_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2Lustart {
    #[inline(always)]
    fn default() -> D2Lustart {
        <crate::RegValueT<D2Lustart_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Luxadd_SPEC;
impl crate::sealed::RegSpec for D2Luxadd_SPEC {
    type DataType = u32;
}

#[doc = "#37: U Limiter X-Axis increment\n\n\nThe add value for U is the 16:16 fixedpoint difference between two samples with\n\na distance o"]
pub type D2Luxadd = crate::RegValueT<D2Luxadd_SPEC>;

impl D2Luxadd {
    #[inline(always)]
    pub fn luxadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2Luxadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2Luxadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Luxadd {
    #[inline(always)]
    fn default() -> D2Luxadd {
        <crate::RegValueT<D2Luxadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Luyadd_SPEC;
impl crate::sealed::RegSpec for D2Luyadd_SPEC {
    type DataType = u32;
}

#[doc = "#38: U Limiter Y-Axis increment\n\n\nThe add value for U is the 16:16 fixedpoint difference between two samples with\n\na distance o"]
pub type D2Luyadd = crate::RegValueT<D2Luyadd_SPEC>;

impl D2Luyadd {
    #[inline(always)]
    pub fn luyadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2Luyadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2Luyadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Luyadd {
    #[inline(always)]
    fn default() -> D2Luyadd {
        <crate::RegValueT<D2Luyadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Lvstartf_SPEC;
impl crate::sealed::RegSpec for D2Lvstartf_SPEC {
    type DataType = u32;
}

#[doc = "#40: V Limiter start value fractional part\n\n\nThe start value is a 32:16 fixedpoint number valid at the first pixel of the\n\nboun"]
pub type D2Lvstartf = crate::RegValueT<D2Lvstartf_SPEC>;

impl D2Lvstartf {
    #[doc = "fractional part."]
    #[inline(always)]
    pub fn lvstartf(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, D2Lvstartf_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,D2Lvstartf_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Lvstartf {
    #[inline(always)]
    fn default() -> D2Lvstartf {
        <crate::RegValueT<D2Lvstartf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Lvstarti_SPEC;
impl crate::sealed::RegSpec for D2Lvstarti_SPEC {
    type DataType = u32;
}

#[doc = "#39: V Limiter start value integer part"]
pub type D2Lvstarti = crate::RegValueT<D2Lvstarti_SPEC>;

impl D2Lvstarti {
    #[inline(always)]
    pub fn lvstarti(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        D2Lvstarti_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2Lvstarti_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2Lvstarti {
    #[inline(always)]
    fn default() -> D2Lvstarti {
        <crate::RegValueT<D2Lvstarti_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Lvxaddi_SPEC;
impl crate::sealed::RegSpec for D2Lvxaddi_SPEC {
    type DataType = u32;
}

#[doc = "#41: V Limiter X-Axis increment integer part\n\n\nThe add value for V is the 32:16 fixedpoint difference between two samples with"]
pub type D2Lvxaddi = crate::RegValueT<D2Lvxaddi_SPEC>;

impl D2Lvxaddi {
    #[inline(always)]
    pub fn lvxaddi(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2Lvxaddi_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2Lvxaddi_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2Lvxaddi {
    #[inline(always)]
    fn default() -> D2Lvxaddi {
        <crate::RegValueT<D2Lvxaddi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Lvyaddi_SPEC;
impl crate::sealed::RegSpec for D2Lvyaddi_SPEC {
    type DataType = u32;
}

#[doc = "#42: V Limiter Y-Axis increment integer part\n\n\nThe add value for V is the 32:16 fixedpoint difference between two samples with"]
pub type D2Lvyaddi = crate::RegValueT<D2Lvyaddi_SPEC>;

impl D2Lvyaddi {
    #[inline(always)]
    pub fn lvyaddi(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2Lvyaddi_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2Lvyaddi_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2Lvyaddi {
    #[inline(always)]
    fn default() -> D2Lvyaddi {
        <crate::RegValueT<D2Lvyaddi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Lvyxaddf_SPEC;
impl crate::sealed::RegSpec for D2Lvyxaddf_SPEC {
    type DataType = u32;
}

#[doc = "#43: V Limiter X and Y increment fractional parts"]
pub type D2Lvyxaddf = crate::RegValueT<D2Lvyxaddf_SPEC>;

impl D2Lvyxaddf {
    #[doc = "Y increment fractional part for &lt;D2_LVYADDI&gt;."]
    #[inline(always)]
    pub fn d2_lvyaddi_frac(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, D2Lvyxaddf_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,D2Lvyxaddf_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "X increment fractional part for &lt;D2_LVXADDI&gt;."]
    #[inline(always)]
    pub fn d2_lvxaddi_frac(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, D2Lvyxaddf_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,D2Lvyxaddf_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Lvyxaddf {
    #[inline(always)]
    fn default() -> D2Lvyxaddf {
        <crate::RegValueT<D2Lvyxaddf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Origin_SPEC;
impl crate::sealed::RegSpec for D2Origin_SPEC {
    type DataType = u32;
}

#[doc = "#32: address of the first pixel in framebuffer\n\n\nWriting to &lt;D2_ORIGIN&gt; will *trigger* DAVE2 to start rendering."]
pub type D2Origin = crate::RegValueT<D2Origin_SPEC>;

impl D2Origin {
    #[inline(always)]
    pub fn origin(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2Origin_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D2Origin_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Origin {
    #[inline(always)]
    fn default() -> D2Origin {
        <crate::RegValueT<D2Origin_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Pattern_SPEC;
impl crate::sealed::RegSpec for D2Pattern_SPEC {
    type DataType = u32;
}

#[doc = "#29: Pattern register\n\n\nEach bit in the pattern register is interpreted as a reference to one of the\n\ntwo color registers ( 0bi"]
pub type D2Pattern = crate::RegValueT<D2Pattern_SPEC>;

impl D2Pattern {
    #[inline(always)]
    pub fn pattern(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D2Pattern_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2Pattern_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2Pattern {
    #[inline(always)]
    fn default() -> D2Pattern {
        <crate::RegValueT<D2Pattern_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Perfcount1_SPEC;
impl crate::sealed::RegSpec for D2Perfcount1_SPEC {
    type DataType = u32;
}

#[doc = "#51: Performance counter\n\n\nWriting to the D2_PERFCOUNT1 register resets the first internal performance counter\n\nto the specifie"]
pub type D2Perfcount1 = crate::RegValueT<D2Perfcount1_SPEC>;

impl D2Perfcount1 {
    #[inline(always)]
    pub fn perfcount1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        D2Perfcount1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2Perfcount1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2Perfcount1 {
    #[inline(always)]
    fn default() -> D2Perfcount1 {
        <crate::RegValueT<D2Perfcount1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Perfcount2_SPEC;
impl crate::sealed::RegSpec for D2Perfcount2_SPEC {
    type DataType = u32;
}

#[doc = "#52: Performance counter\n\n\nWriting to the D2_PERFCOUNT2 register resets the second internal performance counter\n\nto the specifi"]
pub type D2Perfcount2 = crate::RegValueT<D2Perfcount2_SPEC>;

impl D2Perfcount2 {
    #[inline(always)]
    pub fn perfcount2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        D2Perfcount2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2Perfcount2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2Perfcount2 {
    #[inline(always)]
    fn default() -> D2Perfcount2 {
        <crate::RegValueT<D2Perfcount2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Perftrigger_SPEC;
impl crate::sealed::RegSpec for D2Perftrigger_SPEC {
    type DataType = u32;
}

#[doc = "#53: Performance counters control register\n\n\nSelect the internal event that will increment &lt;D2_PERFCOUNT1&gt; respectively &"]
pub type D2Perftrigger = crate::RegValueT<D2Perftrigger_SPEC>;

impl D2Perftrigger {
    #[doc = "Select the internal event that will increment D2_PERFCOUNT2 register."]
    #[inline(always)]
    pub fn perftrigger2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        D2Perftrigger_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            D2Perftrigger_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Select the internal event that will increment D2_PERFCOUNT1 register."]
    #[inline(always)]
    pub fn perftrigger1(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, D2Perftrigger_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            D2Perftrigger_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2Perftrigger {
    #[inline(always)]
    fn default() -> D2Perftrigger {
        <crate::RegValueT<D2Perftrigger_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Pitch_SPEC;
impl crate::sealed::RegSpec for D2Pitch_SPEC {
    type DataType = u32;
}

#[doc = "#31: framebuffer pitch and spanstore delay"]
pub type D2Pitch = crate::RegValueT<D2Pitch_SPEC>;

impl D2Pitch {
    #[doc = "spanstore delay, the number of scanlines to delay spanstore operations."]
    #[inline(always)]
    pub fn ssd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, D2Pitch_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,D2Pitch_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "the width (in pixels) of one framebuffer scanline. A negative width can be used to render bottom-up instead of top-down."]
    #[inline(always)]
    pub fn pitch(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, D2Pitch_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,D2Pitch_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Pitch {
    #[inline(always)]
    fn default() -> D2Pitch {
        <crate::RegValueT<D2Pitch_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Size_SPEC;
impl crate::sealed::RegSpec for D2Size_SPEC {
    type DataType = u32;
}

#[doc = "#30: bounding box dimension"]
pub type D2Size = crate::RegValueT<D2Size_SPEC>;

impl D2Size {
    #[doc = "the height (in pixels) of the primitives bounding box."]
    #[inline(always)]
    pub fn sizey(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, D2Size_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,D2Size_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "the width (in pixels) of the primitives bounding box."]
    #[inline(always)]
    pub fn sizex(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, D2Size_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,D2Size_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Size {
    #[inline(always)]
    fn default() -> D2Size {
        <crate::RegValueT<D2Size_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Status_SPEC;
impl crate::sealed::RegSpec for D2Status_SPEC {
    type DataType = u32;
}

#[doc = "#0: status control register\n\n\nThe current dave status can be polled by reading this register. It contains a \n\ncombination of th"]
pub type D2Status = crate::RegValueT<D2Status_SPEC>;

impl D2Status {
    #[doc = "source interface of bus error"]
    #[inline(always)]
    pub fn d2c_irq_bus_error_src(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, D2Status_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,D2Status_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IRQ on bus error"]
    #[inline(always)]
    pub fn d2c_irq_bus_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, D2Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, D2Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "IRQ on display list finish"]
    #[inline(always)]
    pub fn d2c_irq_dlist(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, D2Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, D2Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "IRQ on enumeration finish"]
    #[inline(always)]
    pub fn d2c_irq_enum(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, D2Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, D2Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "display list active, cant direct access hwregs"]
    #[inline(always)]
    pub fn d2c_dlistactive(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, D2Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, D2Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "framebuffer cache dirty, cant flip frame"]
    #[inline(always)]
    pub fn d2c_cache_dirty(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, D2Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, D2Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "framebuffer writeback busy, cant change framebuffer type"]
    #[inline(always)]
    pub fn d2c_busy_write(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, D2Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, D2Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "enumeration unit busy, cant start new primitive"]
    #[inline(always)]
    pub fn d2c_busy_enum(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, D2Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, D2Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for D2Status {
    #[inline(always)]
    fn default() -> D2Status {
        <crate::RegValueT<D2Status_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Texclut_SPEC;
impl crate::sealed::RegSpec for D2Texclut_SPEC {
    type DataType = u32;
}

#[doc = "#54: Color Lookup Table for the indexed texture format 16x24bit\n\n\nTriggers a write into the CLUT if CLUT size is 16 x 24bit."]
pub type D2Texclut = crate::RegValueT<D2Texclut_SPEC>;

impl D2Texclut {
    #[doc = "Index of the CLUT entry, that shall be written"]
    #[inline(always)]
    pub fn texclut_index(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, D2Texclut_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,D2Texclut_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Color Value RGB888"]
    #[inline(always)]
    pub fn texclut_rgb(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, D2Texclut_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,D2Texclut_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Texclut {
    #[inline(always)]
    fn default() -> D2Texclut {
        <crate::RegValueT<D2Texclut_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2TexclutAddr_SPEC;
impl crate::sealed::RegSpec for D2TexclutAddr_SPEC {
    type DataType = u32;
}

#[doc = "#55: Color Lookup Table write address for the indexed texture format 256x32bit\n\n\nStart address for consecutive writes to &lt;D2"]
pub type D2TexclutAddr = crate::RegValueT<D2TexclutAddr_SPEC>;

impl D2TexclutAddr {
    #[doc = "write address"]
    #[inline(always)]
    pub fn texclut_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, D2TexclutAddr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,D2TexclutAddr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2TexclutAddr {
    #[inline(always)]
    fn default() -> D2TexclutAddr {
        <crate::RegValueT<D2TexclutAddr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2TexclutData_SPEC;
impl crate::sealed::RegSpec for D2TexclutData_SPEC {
    type DataType = u32;
}

#[doc = "#56: Color Lookup Table write data for the indexed texture format 256x32bit\n\n\nWrites one 32 bit color value into the CLUT if CL"]
pub type D2TexclutData = crate::RegValueT<D2TexclutData_SPEC>;

impl D2TexclutData {
    #[doc = "Color value ARGB8888 if CLUTFORMAT = argb8888"]
    #[inline(always)]
    pub fn texclut_argb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        D2TexclutData_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2TexclutData_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2TexclutData {
    #[inline(always)]
    fn default() -> D2TexclutData {
        <crate::RegValueT<D2TexclutData_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2TexclutOffset_SPEC;
impl crate::sealed::RegSpec for D2TexclutOffset_SPEC {
    type DataType = u32;
}

#[doc = "#57: Offset to the texture index for the indexed texture formats i8, i4, i2 and i1\n\n\nThe index offset is combined with the text"]
pub type D2TexclutOffset = crate::RegValueT<D2TexclutOffset_SPEC>;

impl D2TexclutOffset {
    #[doc = "index offset"]
    #[inline(always)]
    pub fn texclut_offset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, D2TexclutOffset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,D2TexclutOffset_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2TexclutOffset {
    #[inline(always)]
    fn default() -> D2TexclutOffset {
        <crate::RegValueT<D2TexclutOffset_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Texmask_SPEC;
impl crate::sealed::RegSpec for D2Texmask_SPEC {
    type DataType = u32;
}

#[doc = "#46: Texture size or texture address mask\n\n\nDepending on the clamping mode this register encodes the clamp limit or wrap\n\nmask."]
pub type D2Texmask = crate::RegValueT<D2Texmask_SPEC>;

impl D2Texmask {
    #[doc = "V mask."]
    #[inline(always)]
    pub fn texvmask(
        self,
    ) -> crate::common::RegisterField<11, 0x1fffff, 1, 0, u32, u32, D2Texmask_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<11,0x1fffff,1,0,u32,u32,D2Texmask_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "U mask."]
    #[inline(always)]
    pub fn texumask(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, D2Texmask_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,D2Texmask_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for D2Texmask {
    #[inline(always)]
    fn default() -> D2Texmask {
        <crate::RegValueT<D2Texmask_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Texorigin_SPEC;
impl crate::sealed::RegSpec for D2Texorigin_SPEC {
    type DataType = u32;
}

#[doc = "#47: Texture base address\n\n\nAll texture registers are *write only*, reading will return undefined results.\n\nPatterns and textur"]
pub type D2Texorigin = crate::RegValueT<D2Texorigin_SPEC>;

impl D2Texorigin {
    #[inline(always)]
    pub fn texorigin(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        D2Texorigin_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2Texorigin_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2Texorigin {
    #[inline(always)]
    fn default() -> D2Texorigin {
        <crate::RegValueT<D2Texorigin_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D2Texpitch_SPEC;
impl crate::sealed::RegSpec for D2Texpitch_SPEC {
    type DataType = u32;
}

#[doc = "#45: Texels per texture line\n\n\nPitch is equal or bigger than texture width."]
pub type D2Texpitch = crate::RegValueT<D2Texpitch_SPEC>;

impl D2Texpitch {
    #[inline(always)]
    pub fn texpitch(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        D2Texpitch_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            D2Texpitch_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D2Texpitch {
    #[inline(always)]
    fn default() -> D2Texpitch {
        <crate::RegValueT<D2Texpitch_SPEC> as RegisterValue<_>>::new(0)
    }
}
