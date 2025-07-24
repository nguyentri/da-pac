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
#[doc = r"I3C registers"]
unsafe impl ::core::marker::Send for super::I3C {}
unsafe impl ::core::marker::Sync for super::I3C {}
impl super::I3C {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Bus Free Timing Register"]
    #[inline(always)]
    pub const fn i3c_bus_free_avail_timing_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CBusFreeAvailTimingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CBusFreeAvailTimingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[doc = "COMMAND_QUEUE_PORT"]
    #[inline(always)]
    pub const fn i3c_command_queue_port_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CCommandQueuePortReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CCommandQueuePortReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Data Buffer Status Level Register"]
    #[inline(always)]
    pub const fn i3c_data_buffer_stat_level_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDataBufferStatLevelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDataBufferStatLevelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Data Buffer Threshold Control Register"]
    #[inline(always)]
    pub const fn i3c_data_buffer_thld_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDataBufferThldCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDataBufferThldCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Device Address Register"]
    #[inline(always)]
    pub const fn i3c_device_addr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDeviceAddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDeviceAddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Pointer for Device Address Table Registers"]
    #[inline(always)]
    pub const fn i3c_device_addr_table_ptr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDeviceAddrTablePtrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDeviceAddrTablePtrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "Device Control Extended Register"]
    #[inline(always)]
    pub const fn i3c_device_ctrl_extended_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDeviceCtrlExtendedReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDeviceCtrlExtendedReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[doc = "Device Control Register"]
    #[inline(always)]
    pub const fn i3c_device_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDeviceCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Device Address Table of Device1"]
    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(640usize),
            )
        }
    }

    #[doc = "Device Address Table of Device2"]
    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(644usize),
            )
        }
    }

    #[doc = "Device Address Table of Device3"]
    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(648usize),
            )
        }
    }

    #[doc = "Device Address Table of Device4"]
    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(652usize),
            )
        }
    }

    #[doc = "Device Address Table of Device5"]
    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(656usize),
            )
        }
    }

    #[doc = "Device Address Table of Device6"]
    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(660usize),
            )
        }
    }

    #[doc = "Device Address Table of Device7"]
    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(664usize),
            )
        }
    }

    #[doc = "Device Address Table of Device8"]
    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(668usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-1 of Device1"]
    #[inline(always)]
    pub const fn i3c_dev_char_table1_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable1Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable1Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-2 of Device1"]
    #[inline(always)]
    pub const fn i3c_dev_char_table1_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable1Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable1Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-3 of Device1"]
    #[inline(always)]
    pub const fn i3c_dev_char_table1_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable1Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable1Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(520usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-4 of Device1"]
    #[inline(always)]
    pub const fn i3c_dev_char_table1_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable1Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable1Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(524usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-1 of Device2"]
    #[inline(always)]
    pub const fn i3c_dev_char_table2_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable2Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable2Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(528usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-2 of Device2"]
    #[inline(always)]
    pub const fn i3c_dev_char_table2_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable2Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable2Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(532usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-3 of Device2"]
    #[inline(always)]
    pub const fn i3c_dev_char_table2_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable2Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable2Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(536usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-4 of Device2"]
    #[inline(always)]
    pub const fn i3c_dev_char_table2_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable2Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable2Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(540usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-1 of Device3"]
    #[inline(always)]
    pub const fn i3c_dev_char_table3_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable3Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable3Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(544usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-2 of Device3"]
    #[inline(always)]
    pub const fn i3c_dev_char_table3_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable3Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable3Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(548usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-3 of Device3"]
    #[inline(always)]
    pub const fn i3c_dev_char_table3_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable3Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable3Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(552usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-4 of Device3"]
    #[inline(always)]
    pub const fn i3c_dev_char_table3_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable3Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable3Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(556usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-1 of Device4"]
    #[inline(always)]
    pub const fn i3c_dev_char_table4_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable4Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable4Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(560usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-2 of Device4"]
    #[inline(always)]
    pub const fn i3c_dev_char_table4_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable4Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable4Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(564usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-3 of Device4"]
    #[inline(always)]
    pub const fn i3c_dev_char_table4_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable4Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable4Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(568usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-4 of Device4"]
    #[inline(always)]
    pub const fn i3c_dev_char_table4_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable4Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable4Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(572usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-1 of Device5"]
    #[inline(always)]
    pub const fn i3c_dev_char_table5_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable5Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable5Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(576usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-2 of Device5"]
    #[inline(always)]
    pub const fn i3c_dev_char_table5_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable5Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable5Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(580usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-3 of Device5"]
    #[inline(always)]
    pub const fn i3c_dev_char_table5_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable5Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable5Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(584usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-4 of Device5"]
    #[inline(always)]
    pub const fn i3c_dev_char_table5_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable5Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable5Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(588usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-1 of Device6"]
    #[inline(always)]
    pub const fn i3c_dev_char_table6_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable6Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable6Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(592usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-2 of Device6"]
    #[inline(always)]
    pub const fn i3c_dev_char_table6_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable6Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable6Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(596usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-3 of Device6"]
    #[inline(always)]
    pub const fn i3c_dev_char_table6_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable6Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable6Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(600usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-4 of Device6"]
    #[inline(always)]
    pub const fn i3c_dev_char_table6_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable6Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable6Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(604usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-1 of Device7"]
    #[inline(always)]
    pub const fn i3c_dev_char_table7_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable7Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable7Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(608usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-2 of Device7"]
    #[inline(always)]
    pub const fn i3c_dev_char_table7_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable7Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable7Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(612usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-3 of Device7"]
    #[inline(always)]
    pub const fn i3c_dev_char_table7_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable7Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable7Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(616usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-4 of Device7"]
    #[inline(always)]
    pub const fn i3c_dev_char_table7_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable7Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable7Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(620usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-1 of Device8"]
    #[inline(always)]
    pub const fn i3c_dev_char_table8_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable8Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable8Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(624usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-2 of Device8"]
    #[inline(always)]
    pub const fn i3c_dev_char_table8_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable8Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable8Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(628usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-3 of Device8"]
    #[inline(always)]
    pub const fn i3c_dev_char_table8_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable8Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable8Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(632usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Location-4 of Device8"]
    #[inline(always)]
    pub const fn i3c_dev_char_table8_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable8Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable8Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(636usize),
            )
        }
    }

    #[doc = "Pointer for Device Characteristics Table"]
    #[inline(always)]
    pub const fn i3c_dev_char_table_pointer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTablePointerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTablePointerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Hardware Capability register"]
    #[inline(always)]
    pub const fn i3c_hw_capability_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CHwCapabilityReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CHwCapabilityReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "IBI Queue Control Register"]
    #[inline(always)]
    pub const fn i3c_ibi_queue_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CIbiQueueCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CIbiQueueCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "In-Band Interrupt Queue Status and Data Register"]
    #[inline(always)]
    pub const fn i3c_ibi_queue_status_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CIbiQueueStatusDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CIbiQueueStatusDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "IBI SIR Request Rejection Control Register"]
    #[inline(always)]
    pub const fn i3c_ibi_sir_req_reject_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CIbiSirReqRejectReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CIbiSirReqRejectReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Interrupt Force Enable Register"]
    #[inline(always)]
    pub const fn i3c_intr_force_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CIntrForceReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CIntrForceReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn i3c_intr_signal_en_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CIntrSignalEnReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn i3c_intr_status_en_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CIntrStatusEnReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Interrupt Status Register"]
    #[inline(always)]
    pub const fn i3c_intr_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CIntrStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CIntrStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Present State Register"]
    #[inline(always)]
    pub const fn i3c_present_state_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CPresentStateReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CPresentStateReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "DWC_mipi_i3c Queue Size Capability Register"]
    #[inline(always)]
    pub const fn i3c_queue_size_capability_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CQueueSizeCapabilityReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CQueueSizeCapabilityReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[doc = "Queue Status Level Register"]
    #[inline(always)]
    pub const fn i3c_queue_status_level_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CQueueStatusLevelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CQueueStatusLevelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "Queue Threshold Control Register"]
    #[inline(always)]
    pub const fn i3c_queue_thld_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CQueueThldCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CQueueThldCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Reset Control Register"]
    #[inline(always)]
    pub const fn i3c_reset_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CResetCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CResetCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "RESPONSE_QUEUE_PORT"]
    #[inline(always)]
    pub const fn i3c_response_queue_port_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CResponseQueuePortReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CResponseQueuePortReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Receive and Transmit Data Port Register"]
    #[inline(always)]
    pub const fn i3c_rx_tx_data_port_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CRxTxDataPortReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CRxTxDataPortReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "SCL Extended Low Count Timing Register"]
    #[inline(always)]
    pub const fn i3c_scl_ext_lcnt_timing_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSclExtLcntTimingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSclExtLcntTimingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[doc = "SCL Termination Bit Low count Timing Register"]
    #[inline(always)]
    pub const fn i3c_scl_ext_termn_lcnt_time_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSclExtTermnLcntTimeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSclExtTermnLcntTimeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[doc = "SCL I2C Fast Mode Plus Timing Register"]
    #[inline(always)]
    pub const fn i3c_scl_i2c_fmp_timing_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSclI2CFmpTimingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSclI2CFmpTimingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "SCL I2C Fast Mode Timing Register"]
    #[inline(always)]
    pub const fn i3c_scl_i2c_fm_timing_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSclI2CFmTimingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSclI2CFmTimingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(188usize),
            )
        }
    }

    #[doc = "SCL I3C Open Drain Timing Register"]
    #[inline(always)]
    pub const fn i3c_scl_i3c_od_timing_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSclI3COdTimingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSclI3COdTimingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(180usize),
            )
        }
    }

    #[doc = "SCL I3C Push Pull Timing Register"]
    #[inline(always)]
    pub const fn i3c_scl_i3c_pp_timing_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSclI3CPpTimingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSclI3CPpTimingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[doc = "SDA Hold Delay Timing Register"]
    #[inline(always)]
    pub const fn i3c_sda_hold_dly_timing_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSdaHoldDlyTimingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSdaHoldDlyTimingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "Slave Event Status Register"]
    #[inline(always)]
    pub const fn i3c_slv_event_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSlvEventStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSlvEventStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Pointer for Vendor specific Registers"]
    #[inline(always)]
    pub const fn i3c_vendor_specific_reg_ptr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CVendorSpecificRegPtrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CVendorSpecificRegPtrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "DWC_mipi_i3c Version ID Register"]
    #[inline(always)]
    pub const fn i3c_ver_id_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CVerIdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CVerIdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[doc = "DWC_mipi_i3c Version Type Register"]
    #[inline(always)]
    pub const fn i3c_ver_type_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CVerTypeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CVerTypeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CBusFreeAvailTimingReg_SPEC;
impl crate::sealed::RegSpec for I3CBusFreeAvailTimingReg_SPEC {
    type DataType = u32;
}

#[doc = "Bus Free Timing Register"]
pub type I3CBusFreeAvailTimingReg = crate::RegValueT<I3CBusFreeAvailTimingReg_SPEC>;

impl I3CBusFreeAvailTimingReg {
    #[doc = "This register field is used only in Master mode of operation\n\nI3C Bus Free Count Value.\n\nIn Pure Bus System, this field represents tCAS parameter. In Mixed Bus system, this field is expected to be programmed to tLOW of I2C Timing."]
    #[inline(always)]
    pub fn bus_free_time(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CBusFreeAvailTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CBusFreeAvailTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CBusFreeAvailTimingReg {
    #[inline(always)]
    fn default() -> I3CBusFreeAvailTimingReg {
        <crate::RegValueT<I3CBusFreeAvailTimingReg_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CCommandQueuePortReg_SPEC;
impl crate::sealed::RegSpec for I3CCommandQueuePortReg_SPEC {
    type DataType = u32;
}

#[doc = "COMMAND_QUEUE_PORT"]
pub type I3CCommandQueuePortReg = crate::RegValueT<I3CCommandQueuePortReg_SPEC>;

impl I3CCommandQueuePortReg {
    #[doc = "32 bit command"]
    #[inline(always)]
    pub fn command(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CCommandQueuePortReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CCommandQueuePortReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CCommandQueuePortReg {
    #[inline(always)]
    fn default() -> I3CCommandQueuePortReg {
        <crate::RegValueT<I3CCommandQueuePortReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDataBufferStatLevelReg_SPEC;
impl crate::sealed::RegSpec for I3CDataBufferStatLevelReg_SPEC {
    type DataType = u32;
}

#[doc = "Data Buffer Status Level Register"]
pub type I3CDataBufferStatLevelReg = crate::RegValueT<I3CDataBufferStatLevelReg_SPEC>;

impl I3CDataBufferStatLevelReg {
    #[doc = "Receive Buffer Level Value.\n\nContains the number of valid data entries in the receive Buffer."]
    #[inline(always)]
    pub fn rx_buf_blr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDataBufferStatLevelReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDataBufferStatLevelReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Buffer Empty Level Value.\n\nContains the number of empty locations in the transmit Buffer."]
    #[inline(always)]
    pub fn tx_buf_empty_loc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDataBufferStatLevelReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDataBufferStatLevelReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDataBufferStatLevelReg {
    #[inline(always)]
    fn default() -> I3CDataBufferStatLevelReg {
        <crate::RegValueT<I3CDataBufferStatLevelReg_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDataBufferThldCtrlReg_SPEC;
impl crate::sealed::RegSpec for I3CDataBufferThldCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Data Buffer Threshold Control Register"]
pub type I3CDataBufferThldCtrlReg = crate::RegValueT<I3CDataBufferThldCtrlReg_SPEC>;

impl I3CDataBufferThldCtrlReg {
    #[doc = "Receive Start Threshold Value.\n\nIn master mode of operation when the controller is set up to initiate a read transfer, it waits until either one of the conditions are met before it initiates the read transfer on the I3C Interface.\n\nData length (as specified in the command) number of locations are empty in the Receive FIFO.\n\nThreshold number of locations (or more) are empty in the Receive FIFO.\n\nThe supported values for RX_START_THLD are:\n\n000 - 1\n001 - 4\n010 - 8\n011 - 16\n100 - 32\n101 - 64"]
    #[inline(always)]
    pub fn rx_start_thld(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        u8,
        u8,
        I3CDataBufferThldCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            u8,
            u8,
            I3CDataBufferThldCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transfer Start Threshold Value.\n\nIn master mode of operation when the controller is set up to initiate a write transfer, it waits until either one of the following conditions are met before it initiates the write transfer on the I3C Interface.\n\nData length (as specified in the command) number of locations are filled in the Transmit FIFO\n\nThreshold number of entries (or more) are available in the Transmit FIFO\n\nThe supported values for TX_START_THLD are:\n\n000: 1\n001: 4\n010: 8\n011: 16\n100: 32\n101: 64"]
    #[inline(always)]
    pub fn tx_start_thld(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        u8,
        u8,
        I3CDataBufferThldCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            u8,
            u8,
            I3CDataBufferThldCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Buffer Threshold Value.\n\nThis field controls the number of entries (or above) in the Receive FIFO that trigger the RX_THLD_STAT interrupt.\n\nIf the programmed value is greater than the buffer depth, then threshold will be set to 32. The supported values for RX_BUF_THLD are\n\n000: 1\n001: 4\n010: 8\n011: 16\n100: 32\n101: 64"]
    #[inline(always)]
    pub fn rx_buf_thld(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        u8,
        u8,
        I3CDataBufferThldCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            u8,
            u8,
            I3CDataBufferThldCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Buffer Threshold Value.\n\nThis field controls the number of empty locations (or above) in the Transmit FIFO that trigger the TX_THLD_STAT interrupt.\n\nIf the programmed value is greater than the buffer depth, then threshold will be set to 32. The supported values for TX_BUF_THLD are\n\n000: 1\n001: 4\n010: 8\n011: 16\n100: 32\n101: 64"]
    #[inline(always)]
    pub fn tx_empty_buf_thld(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        u8,
        u8,
        I3CDataBufferThldCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            u8,
            u8,
            I3CDataBufferThldCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDataBufferThldCtrlReg {
    #[inline(always)]
    fn default() -> I3CDataBufferThldCtrlReg {
        <crate::RegValueT<I3CDataBufferThldCtrlReg_SPEC> as RegisterValue<_>>::new(50529027)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDeviceAddrReg_SPEC;
impl crate::sealed::RegSpec for I3CDeviceAddrReg_SPEC {
    type DataType = u32;
}

#[doc = "Device Address Register"]
pub type I3CDeviceAddrReg = crate::RegValueT<I3CDeviceAddrReg_SPEC>;

impl I3CDeviceAddrReg {
    #[doc = "Dynamic Address Valid\n\nThis bit is used to control whether the DYNAMIC_ADDR is valid or not.\n\nIn I3C Main Master mode, the user sets this bit to 1 as it self-assigns its dynamic address.\nIn all other operation modes, the Controller sets this bit to 1 when Main Master assigns the Dynamic address during ENTDAA or SETDASA mechanism.\n\nValues:\n\n0x0 (INVALID): Dynamic Address is invalid\n0x1 (VALID): Dynamic Address is valid"]
    #[inline(always)]
    pub fn dynamic_addr_valid(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDeviceAddrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,I3CDeviceAddrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device Dynamic Address.\n\nThis field is used to program the Device Dynamic Address. The Controller uses this address for I3C transfers.\n\nIn Main Master mode, the user/application has to program the Dynamic Address through the Slave interface as it self-assigns its Dynamic Address.\nIn all other modes, the Main Master assigns this address during ENTDAA or SETDASA mechanism."]
    #[inline(always)]
    pub fn dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDeviceAddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDeviceAddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDeviceAddrReg {
    #[inline(always)]
    fn default() -> I3CDeviceAddrReg {
        <crate::RegValueT<I3CDeviceAddrReg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDeviceAddrTablePtrReg_SPEC;
impl crate::sealed::RegSpec for I3CDeviceAddrTablePtrReg_SPEC {
    type DataType = u32;
}

#[doc = "Pointer for Device Address Table Registers"]
pub type I3CDeviceAddrTablePtrReg = crate::RegValueT<I3CDeviceAddrTablePtrReg_SPEC>;

impl I3CDeviceAddrTablePtrReg {
    #[doc = "Depth of Device Address Table"]
    #[inline(always)]
    pub fn dev_addr_table_depth(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDeviceAddrTablePtrReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDeviceAddrTablePtrReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Start Address of Device Address Table."]
    #[inline(always)]
    pub fn p_dev_addr_table_start_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDeviceAddrTablePtrReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDeviceAddrTablePtrReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDeviceAddrTablePtrReg {
    #[inline(always)]
    fn default() -> I3CDeviceAddrTablePtrReg {
        <crate::RegValueT<I3CDeviceAddrTablePtrReg_SPEC> as RegisterValue<_>>::new(524928)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDeviceCtrlExtendedReg_SPEC;
impl crate::sealed::RegSpec for I3CDeviceCtrlExtendedReg_SPEC {
    type DataType = u32;
}

#[doc = "Device Control Extended Register"]
pub type I3CDeviceCtrlExtendedReg = crate::RegValueT<I3CDeviceCtrlExtendedReg_SPEC>;

impl I3CDeviceCtrlExtendedReg {
    #[doc = "This bit is used to select the Device Operation Mode before the controller is enabled.\n\nThis field shall be written only when the DWC_mipi_i3c is disabled.\n\n0: Master\n1: Slave\n2: Reserved\n3: Reserved\n\nThis field will be automatically updated by the controller once the role change happens in secondary master mode.."]
    #[inline(always)]
    pub fn dev_operation_mode(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDeviceCtrlExtendedReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDeviceCtrlExtendedReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDeviceCtrlExtendedReg {
    #[inline(always)]
    fn default() -> I3CDeviceCtrlExtendedReg {
        <crate::RegValueT<I3CDeviceCtrlExtendedReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDeviceCtrlReg_SPEC;
impl crate::sealed::RegSpec for I3CDeviceCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Device Control Register"]
pub type I3CDeviceCtrlReg = crate::RegValueT<I3CDeviceCtrlReg_SPEC>;

impl I3CDeviceCtrlReg {
    #[doc = "Controls whether or not DWC_mipi_i3c is enabled.\n\n1: Enables the DWC_mipi_i3c controller.\n0: Disables the DWC_mipi_i3c controller.\n\nIn Master mode of operation, software can Disable DWC_mipi_i3c while it is active. However, the controller may not get Disabled immediately and will be \'Disabled\' after commands in the Command queue (if any) are executed leading to a STOP condition on the bus and Master FSM is in IDLE state (as indicated by PRESENT_STATE Register)."]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,I3CDeviceCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DWC_mipi_i3c Resume.\n\nThis bit is used to resume the Controller after it goes to Halt state.\n\nIn the master mode of operation the controller goes to the halt state (as indicated in PRESENT_STATE Register) due to any type of error in the transfer (the type of error is indicated by ERR_STATUS field in the RESPONSE_QUEUE_PORT register).\n\nAfter the controller has gone to halt state, the application has to write 1\'b1 to this bit to resume the controller. This bit is auto-cleared once the controller resumes the transfers by initiating the next command."]
    #[inline(always)]
    pub fn resume(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,I3CDeviceCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DWC_mipi_i3c Abort.\n\nThis bit is used in master mode of operation.\n\nThis bit allows the controller to relinquish the I3C Bus before completing the issued transfer.\n\nIn response to an ABORT request, the controller issues the STOP condition after the complete data byte is transferred or received.\n\nThis bit is auto-cleared once the transfer is aborted and controller issues a \'Transfer Abort\' interrupt."]
    #[inline(always)]
    pub fn abort(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,I3CDeviceCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DMA Handshake Interface Enable.\n\nThis bit is used to enable or disable the DMA Handshaking interface and is applicable to both Master and Slave mode of operation.\n\n1: Enables the DMA handshake control to interact with external DMA.\n\n0: The DMA handshake control has no significance."]
    #[inline(always)]
    pub fn dma_enable_i3c(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,I3CDeviceCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Hot-Join Ack/Nack Control\n\nThis bit is used in master mode of operation.\n\nThis bit acts as global control to ACK/NACK the Hot-Join Request from the devices. The DWC_mipi_i3c Master will ACK/NACK the Hot-Join request from other devices connected on the I3C Bus, based on programming of this bit.\n\n0: ACK the Hot-join request.\n1: NACK and send broadcast CCC to disable Hot-join.\n\nValues:\n\n0x0 (DISABLED): Ack Hot-Join requests\n0x1 (ENABLED): Nack and auto-disable Hot-Join request"]
    #[inline(always)]
    pub fn hot_join_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,I3CDeviceCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "I2C Slave Present\n\nThis bit is used in master mode of operation.\n\nThis Bit indicates whether any Legacy I2C Devices are present in the system.\n\nIn HDR mode, this field is used to select TSL over TSP in mixed bus configuration.\n\nValues:\n\n0x0 (DISABLED): I2C Slave not present\n0x1 (ENABLED): I2C Slave present"]
    #[inline(always)]
    pub fn i2c_slave_present(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,I3CDeviceCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "I3C Broadcast Address include.\n\nThis bit is used in master mode of operation.\n\nThis bit is used to include I3C broadcast address (0x7E) for private transfer.\n\nNote: If I3C broadcast address is not included for the private transfers, In-band Interrupts (IBI) driven from Slaves may not win address arbitration. Hence, the IBIs will get delayed.\n\nValues:\n\n0x0 (NOT_INCLUDED): I3C Broadcast Address is not included for Private Transfers\n0x1 (INCLUDED): I3C Broadcast Address is included for Private Transfers"]
    #[inline(always)]
    pub fn iba_include(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I3CDeviceCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CDeviceCtrlReg {
    #[inline(always)]
    fn default() -> I3CDeviceCtrlReg {
        <crate::RegValueT<I3CDeviceCtrlReg_SPEC> as RegisterValue<_>>::new(385)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Address Table of Device1"]
pub type I3CDevAddrTableLoc1Reg = crate::RegValueT<I3CDevAddrTableLoc1Reg_SPEC>;

impl I3CDevAddrTableLoc1Reg {
    #[doc = "Legacy I2C device or not.\n\nThis bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This field is used to set the Device NACK Retry count for the particular device.\n\nIf the Device NACK\'s for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then Controller generates an error response and move to the Halt state.\n\nThis feature is used for Retry Model for the following features mentioned in the I3C Specification:\n\nRetry Model for Direct GET CCC Commands.\nThe incoming SIR-IBI matches with the slave address initated by the Master."]
    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Dynamic Address with parity. The MSB, bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Static Address."]
    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc1Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc1Reg {
        <crate::RegValueT<I3CDevAddrTableLoc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Address Table of Device2"]
pub type I3CDevAddrTableLoc2Reg = crate::RegValueT<I3CDevAddrTableLoc2Reg_SPEC>;

impl I3CDevAddrTableLoc2Reg {
    #[doc = "Legacy I2C device or not.\n\nThis bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This field is used to set the Device NACK Retry count for the particular device.\n\nIf the Device NACK\'s for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then Controller generates an error response and move to the Halt state.\n\nThis feature is used for Retry Model for the following features mentioned in the I3C Specification:\n\nRetry Model for Direct GET CCC Commands.\nThe incoming SIR-IBI matches with the slave address initated by the Master."]
    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Dynamic Address with parity. The MSB, bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Static Address."]
    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc2Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc2Reg {
        <crate::RegValueT<I3CDevAddrTableLoc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Address Table of Device3"]
pub type I3CDevAddrTableLoc3Reg = crate::RegValueT<I3CDevAddrTableLoc3Reg_SPEC>;

impl I3CDevAddrTableLoc3Reg {
    #[doc = "Legacy I2C device or not.\n\nThis bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This field is used to set the Device NACK Retry count for the particular device.\n\nIf the Device NACK\'s for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then Controller generates an error response and move to the Halt state.\n\nThis feature is used for Retry Model for the following features mentioned in the I3C Specification:\n\nRetry Model for Direct GET CCC Commands.\nThe incoming SIR-IBI matches with the slave address initated by the Master."]
    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Dynamic Address with parity. The MSB, bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Static Address."]
    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc3Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc3Reg {
        <crate::RegValueT<I3CDevAddrTableLoc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc4Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Address Table of Device4"]
pub type I3CDevAddrTableLoc4Reg = crate::RegValueT<I3CDevAddrTableLoc4Reg_SPEC>;

impl I3CDevAddrTableLoc4Reg {
    #[doc = "Legacy I2C device or not.\n\nThis bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This field is used to set the Device NACK Retry count for the particular device.\n\nIf the Device NACK\'s for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then Controller generates an error response and move to the Halt state.\n\nThis feature is used for Retry Model for the following features mentioned in the I3C Specification:\n\nRetry Model for Direct GET CCC Commands.\nThe incoming SIR-IBI matches with the slave address initated by the Master."]
    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Dynamic Address with parity. The MSB, bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Static Address."]
    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc4Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc4Reg {
        <crate::RegValueT<I3CDevAddrTableLoc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc5Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc5Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Address Table of Device5"]
pub type I3CDevAddrTableLoc5Reg = crate::RegValueT<I3CDevAddrTableLoc5Reg_SPEC>;

impl I3CDevAddrTableLoc5Reg {
    #[doc = "Legacy I2C device or not.\n\nThis bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This field is used to set the Device NACK Retry count for the particular device.\n\nIf the Device NACK\'s for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then Controller generates an error response and move to the Halt state.\n\nThis feature is used for Retry Model for the following features mentioned in the I3C Specification:\n\nRetry Model for Direct GET CCC Commands.\nThe incoming SIR-IBI matches with the slave address initated by the Master."]
    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Dynamic Address with parity. The MSB, bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Static Address."]
    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc5Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc5Reg {
        <crate::RegValueT<I3CDevAddrTableLoc5Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc6Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc6Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Address Table of Device6"]
pub type I3CDevAddrTableLoc6Reg = crate::RegValueT<I3CDevAddrTableLoc6Reg_SPEC>;

impl I3CDevAddrTableLoc6Reg {
    #[doc = "Legacy I2C device or not.\n\nThis bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This field is used to set the Device NACK Retry count for the particular device.\n\nIf the Device NACK\'s for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then Controller generates an error response and move to the Halt state.\n\nThis feature is used for Retry Model for the following features mentioned in the I3C Specification:\n\nRetry Model for Direct GET CCC Commands.\nThe incoming SIR-IBI matches with the slave address initated by the Master."]
    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Dynamic Address with parity. The MSB, bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Static Address."]
    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc6Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc6Reg {
        <crate::RegValueT<I3CDevAddrTableLoc6Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc7Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc7Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Address Table of Device7"]
pub type I3CDevAddrTableLoc7Reg = crate::RegValueT<I3CDevAddrTableLoc7Reg_SPEC>;

impl I3CDevAddrTableLoc7Reg {
    #[doc = "Legacy I2C device or not.\n\nThis bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc7Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This field is used to set the Device NACK Retry count for the particular device.\n\nIf the Device NACK\'s for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then Controller generates an error response and move to the Halt state.\n\nThis feature is used for Retry Model for the following features mentioned in the I3C Specification:\n\nRetry Model for Direct GET CCC Commands.\nThe incoming SIR-IBI matches with the slave address initated by the Master."]
    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Dynamic Address with parity. The MSB, bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Static Address."]
    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc7Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc7Reg {
        <crate::RegValueT<I3CDevAddrTableLoc7Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc8Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc8Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Address Table of Device8"]
pub type I3CDevAddrTableLoc8Reg = crate::RegValueT<I3CDevAddrTableLoc8Reg_SPEC>;

impl I3CDevAddrTableLoc8Reg {
    #[doc = "Legacy I2C device or not.\n\nThis bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc8Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This field is used to set the Device NACK Retry count for the particular device.\n\nIf the Device NACK\'s for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then Controller generates an error response and move to the Halt state.\n\nThis feature is used for Retry Model for the following features mentioned in the I3C Specification:\n\nRetry Model for Direct GET CCC Commands.\nThe incoming SIR-IBI matches with the slave address initated by the Master."]
    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc8Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Dynamic Address with parity. The MSB, bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc8Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device Static Address."]
    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc8Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc8Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc8Reg {
        <crate::RegValueT<I3CDevAddrTableLoc8Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable1Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable1Loc1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-1 of Device1"]
pub type I3CDevCharTable1Loc1Reg = crate::RegValueT<I3CDevCharTable1Loc1Reg_SPEC>;

impl I3CDevCharTable1Loc1Reg {
    #[doc = "The LSB 32-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable1Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable1Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable1Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable1Loc1Reg {
        <crate::RegValueT<I3CDevCharTable1Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable1Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable1Loc2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-2 of Device1"]
pub type I3CDevCharTable1Loc2Reg = crate::RegValueT<I3CDevCharTable1Loc2Reg_SPEC>;

impl I3CDevCharTable1Loc2Reg {
    #[doc = "The MSB 16-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable1Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable1Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable1Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable1Loc2Reg {
        <crate::RegValueT<I3CDevCharTable1Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable1Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable1Loc3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-3 of Device1"]
pub type I3CDevCharTable1Loc3Reg = crate::RegValueT<I3CDevCharTable1Loc3Reg_SPEC>;

impl I3CDevCharTable1Loc3Reg {
    #[doc = "Bus Characteristic Value"]
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable1Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable1Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Device Characteristic Value"]
    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable1Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable1Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable1Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable1Loc3Reg {
        <crate::RegValueT<I3CDevCharTable1Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable1Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable1Loc4Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-4 of Device1"]
pub type I3CDevCharTable1Loc4Reg = crate::RegValueT<I3CDevCharTable1Loc4Reg_SPEC>;

impl I3CDevCharTable1Loc4Reg {
    #[doc = "Device Dynamic Address assigned."]
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable1Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable1Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable1Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable1Loc4Reg {
        <crate::RegValueT<I3CDevCharTable1Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable2Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable2Loc1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-1 of Device2"]
pub type I3CDevCharTable2Loc1Reg = crate::RegValueT<I3CDevCharTable2Loc1Reg_SPEC>;

impl I3CDevCharTable2Loc1Reg {
    #[doc = "The LSB 32-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable2Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable2Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable2Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable2Loc1Reg {
        <crate::RegValueT<I3CDevCharTable2Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable2Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable2Loc2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-2 of Device2"]
pub type I3CDevCharTable2Loc2Reg = crate::RegValueT<I3CDevCharTable2Loc2Reg_SPEC>;

impl I3CDevCharTable2Loc2Reg {
    #[doc = "The MSB 16-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable2Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable2Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable2Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable2Loc2Reg {
        <crate::RegValueT<I3CDevCharTable2Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable2Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable2Loc3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-3 of Device2"]
pub type I3CDevCharTable2Loc3Reg = crate::RegValueT<I3CDevCharTable2Loc3Reg_SPEC>;

impl I3CDevCharTable2Loc3Reg {
    #[doc = "Bus Characteristic Value"]
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable2Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable2Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Device Characteristic Value"]
    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable2Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable2Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable2Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable2Loc3Reg {
        <crate::RegValueT<I3CDevCharTable2Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable2Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable2Loc4Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-4 of Device2"]
pub type I3CDevCharTable2Loc4Reg = crate::RegValueT<I3CDevCharTable2Loc4Reg_SPEC>;

impl I3CDevCharTable2Loc4Reg {
    #[doc = "Device Dynamic Address assigned."]
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable2Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable2Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable2Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable2Loc4Reg {
        <crate::RegValueT<I3CDevCharTable2Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable3Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable3Loc1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-1 of Device3"]
pub type I3CDevCharTable3Loc1Reg = crate::RegValueT<I3CDevCharTable3Loc1Reg_SPEC>;

impl I3CDevCharTable3Loc1Reg {
    #[doc = "The LSB 32-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable3Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable3Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable3Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable3Loc1Reg {
        <crate::RegValueT<I3CDevCharTable3Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable3Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable3Loc2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-2 of Device3"]
pub type I3CDevCharTable3Loc2Reg = crate::RegValueT<I3CDevCharTable3Loc2Reg_SPEC>;

impl I3CDevCharTable3Loc2Reg {
    #[doc = "The MSB 16-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable3Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable3Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable3Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable3Loc2Reg {
        <crate::RegValueT<I3CDevCharTable3Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable3Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable3Loc3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-3 of Device3"]
pub type I3CDevCharTable3Loc3Reg = crate::RegValueT<I3CDevCharTable3Loc3Reg_SPEC>;

impl I3CDevCharTable3Loc3Reg {
    #[doc = "Bus Characteristic Value"]
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable3Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable3Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Device Characteristic Value"]
    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable3Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable3Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable3Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable3Loc3Reg {
        <crate::RegValueT<I3CDevCharTable3Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable3Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable3Loc4Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-4 of Device3"]
pub type I3CDevCharTable3Loc4Reg = crate::RegValueT<I3CDevCharTable3Loc4Reg_SPEC>;

impl I3CDevCharTable3Loc4Reg {
    #[doc = "Device Dynamic Address assigned."]
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable3Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable3Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable3Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable3Loc4Reg {
        <crate::RegValueT<I3CDevCharTable3Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable4Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable4Loc1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-1 of Device4"]
pub type I3CDevCharTable4Loc1Reg = crate::RegValueT<I3CDevCharTable4Loc1Reg_SPEC>;

impl I3CDevCharTable4Loc1Reg {
    #[doc = "The LSB 32-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable4Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable4Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable4Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable4Loc1Reg {
        <crate::RegValueT<I3CDevCharTable4Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable4Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable4Loc2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-2 of Device4"]
pub type I3CDevCharTable4Loc2Reg = crate::RegValueT<I3CDevCharTable4Loc2Reg_SPEC>;

impl I3CDevCharTable4Loc2Reg {
    #[doc = "The MSB 16-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable4Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable4Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable4Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable4Loc2Reg {
        <crate::RegValueT<I3CDevCharTable4Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable4Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable4Loc3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-3 of Device4"]
pub type I3CDevCharTable4Loc3Reg = crate::RegValueT<I3CDevCharTable4Loc3Reg_SPEC>;

impl I3CDevCharTable4Loc3Reg {
    #[doc = "Bus Characteristic Value"]
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable4Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable4Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Device Characteristic Value"]
    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable4Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable4Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable4Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable4Loc3Reg {
        <crate::RegValueT<I3CDevCharTable4Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable4Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable4Loc4Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-4 of Device4"]
pub type I3CDevCharTable4Loc4Reg = crate::RegValueT<I3CDevCharTable4Loc4Reg_SPEC>;

impl I3CDevCharTable4Loc4Reg {
    #[doc = "Device Dynamic Address assigned."]
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable4Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable4Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable4Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable4Loc4Reg {
        <crate::RegValueT<I3CDevCharTable4Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable5Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable5Loc1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-1 of Device5"]
pub type I3CDevCharTable5Loc1Reg = crate::RegValueT<I3CDevCharTable5Loc1Reg_SPEC>;

impl I3CDevCharTable5Loc1Reg {
    #[doc = "The LSB 32-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable5Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable5Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable5Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable5Loc1Reg {
        <crate::RegValueT<I3CDevCharTable5Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable5Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable5Loc2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-2 of Device5"]
pub type I3CDevCharTable5Loc2Reg = crate::RegValueT<I3CDevCharTable5Loc2Reg_SPEC>;

impl I3CDevCharTable5Loc2Reg {
    #[doc = "The MSB 16-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable5Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable5Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable5Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable5Loc2Reg {
        <crate::RegValueT<I3CDevCharTable5Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable5Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable5Loc3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-3 of Device5"]
pub type I3CDevCharTable5Loc3Reg = crate::RegValueT<I3CDevCharTable5Loc3Reg_SPEC>;

impl I3CDevCharTable5Loc3Reg {
    #[doc = "Bus Characteristic Value"]
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable5Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable5Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Device Characteristic Value"]
    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable5Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable5Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable5Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable5Loc3Reg {
        <crate::RegValueT<I3CDevCharTable5Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable5Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable5Loc4Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-4 of Device5"]
pub type I3CDevCharTable5Loc4Reg = crate::RegValueT<I3CDevCharTable5Loc4Reg_SPEC>;

impl I3CDevCharTable5Loc4Reg {
    #[doc = "Device Dynamic Address assigned."]
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable5Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable5Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable5Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable5Loc4Reg {
        <crate::RegValueT<I3CDevCharTable5Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable6Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable6Loc1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-1 of Device6"]
pub type I3CDevCharTable6Loc1Reg = crate::RegValueT<I3CDevCharTable6Loc1Reg_SPEC>;

impl I3CDevCharTable6Loc1Reg {
    #[doc = "The LSB 32-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable6Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable6Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable6Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable6Loc1Reg {
        <crate::RegValueT<I3CDevCharTable6Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable6Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable6Loc2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-2 of Device6"]
pub type I3CDevCharTable6Loc2Reg = crate::RegValueT<I3CDevCharTable6Loc2Reg_SPEC>;

impl I3CDevCharTable6Loc2Reg {
    #[doc = "The MSB 16-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable6Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable6Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable6Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable6Loc2Reg {
        <crate::RegValueT<I3CDevCharTable6Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable6Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable6Loc3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-3 of Device6"]
pub type I3CDevCharTable6Loc3Reg = crate::RegValueT<I3CDevCharTable6Loc3Reg_SPEC>;

impl I3CDevCharTable6Loc3Reg {
    #[doc = "Bus Characteristic Value"]
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable6Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable6Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Device Characteristic Value"]
    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable6Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable6Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable6Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable6Loc3Reg {
        <crate::RegValueT<I3CDevCharTable6Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable6Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable6Loc4Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-4 of Device6"]
pub type I3CDevCharTable6Loc4Reg = crate::RegValueT<I3CDevCharTable6Loc4Reg_SPEC>;

impl I3CDevCharTable6Loc4Reg {
    #[doc = "Device Dynamic Address assigned."]
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable6Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable6Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable6Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable6Loc4Reg {
        <crate::RegValueT<I3CDevCharTable6Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable7Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable7Loc1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-1 of Device7"]
pub type I3CDevCharTable7Loc1Reg = crate::RegValueT<I3CDevCharTable7Loc1Reg_SPEC>;

impl I3CDevCharTable7Loc1Reg {
    #[doc = "The LSB 32-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable7Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable7Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable7Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable7Loc1Reg {
        <crate::RegValueT<I3CDevCharTable7Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable7Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable7Loc2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-2 of Device7"]
pub type I3CDevCharTable7Loc2Reg = crate::RegValueT<I3CDevCharTable7Loc2Reg_SPEC>;

impl I3CDevCharTable7Loc2Reg {
    #[doc = "The MSB 16-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable7Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable7Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable7Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable7Loc2Reg {
        <crate::RegValueT<I3CDevCharTable7Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable7Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable7Loc3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-3 of Device7"]
pub type I3CDevCharTable7Loc3Reg = crate::RegValueT<I3CDevCharTable7Loc3Reg_SPEC>;

impl I3CDevCharTable7Loc3Reg {
    #[doc = "Bus Characteristic Value"]
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable7Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable7Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Device Characteristic Value"]
    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable7Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable7Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable7Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable7Loc3Reg {
        <crate::RegValueT<I3CDevCharTable7Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable7Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable7Loc4Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-4 of Device7"]
pub type I3CDevCharTable7Loc4Reg = crate::RegValueT<I3CDevCharTable7Loc4Reg_SPEC>;

impl I3CDevCharTable7Loc4Reg {
    #[doc = "Device Dynamic Address assigned."]
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable7Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable7Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable7Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable7Loc4Reg {
        <crate::RegValueT<I3CDevCharTable7Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable8Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable8Loc1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-1 of Device8"]
pub type I3CDevCharTable8Loc1Reg = crate::RegValueT<I3CDevCharTable8Loc1Reg_SPEC>;

impl I3CDevCharTable8Loc1Reg {
    #[doc = "The LSB 32-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable8Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable8Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable8Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable8Loc1Reg {
        <crate::RegValueT<I3CDevCharTable8Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable8Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable8Loc2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-2 of Device8"]
pub type I3CDevCharTable8Loc2Reg = crate::RegValueT<I3CDevCharTable8Loc2Reg_SPEC>;

impl I3CDevCharTable8Loc2Reg {
    #[doc = "The MSB 16-bit value of Provisional-ID"]
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable8Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable8Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable8Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable8Loc2Reg {
        <crate::RegValueT<I3CDevCharTable8Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable8Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable8Loc3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-3 of Device8"]
pub type I3CDevCharTable8Loc3Reg = crate::RegValueT<I3CDevCharTable8Loc3Reg_SPEC>;

impl I3CDevCharTable8Loc3Reg {
    #[doc = "Bus Characteristic Value"]
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable8Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable8Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Device Characteristic Value"]
    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable8Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable8Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable8Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable8Loc3Reg {
        <crate::RegValueT<I3CDevCharTable8Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable8Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable8Loc4Reg_SPEC {
    type DataType = u32;
}

#[doc = "Device Characteristic Table Location-4 of Device8"]
pub type I3CDevCharTable8Loc4Reg = crate::RegValueT<I3CDevCharTable8Loc4Reg_SPEC>;

impl I3CDevCharTable8Loc4Reg {
    #[doc = "Device Dynamic Address assigned."]
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable8Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable8Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable8Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable8Loc4Reg {
        <crate::RegValueT<I3CDevCharTable8Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTablePointerReg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTablePointerReg_SPEC {
    type DataType = u32;
}

#[doc = "Pointer for Device Characteristics Table"]
pub type I3CDevCharTablePointerReg = crate::RegValueT<I3CDevCharTablePointerReg_SPEC>;

impl I3CDevCharTablePointerReg {
    #[doc = "Current index of Device Characteristics Table.\n\nThis field returns the current location of Device Characteristics Table index. Initially, this index points to 0.\n\nOnce the complete characteristics information of a Slave device is written into Device Characteristics Table during ENTDAA, this index increments by 1. The first winning device information is stored in Device Characteristics Table index 0, the second winning device information in index 1, and so on.\n\nIf required, this index can be used to override the location, where characteristic information of Slave devices on the I3C bus are written during ENTDAA. Hence, this field is useful only if the device is Current Master. During DEFSLV CCC, the index always starts from 0.\n\nIn Non-current Master, this field is always read-only."]
    #[inline(always)]
    pub fn present_dev_char_table_indx(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x7,
        1,
        0,
        u8,
        u8,
        I3CDevCharTablePointerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x7,
            1,
            0,
            u8,
            u8,
            I3CDevCharTablePointerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Depth of Device Characteristics Table"]
    #[inline(always)]
    pub fn dev_char_table_depth(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevCharTablePointerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevCharTablePointerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Start Address of Device Characteristics Table."]
    #[inline(always)]
    pub fn p_dev_char_table_start_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTablePointerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTablePointerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTablePointerReg {
    #[inline(always)]
    fn default() -> I3CDevCharTablePointerReg {
        <crate::RegValueT<I3CDevCharTablePointerReg_SPEC> as RegisterValue<_>>::new(131584)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CHwCapabilityReg_SPEC;
impl crate::sealed::RegSpec for I3CHwCapabilityReg_SPEC {
    type DataType = u32;
}

#[doc = "Hardware Capability register"]
pub type I3CHwCapabilityReg = crate::RegValueT<I3CHwCapabilityReg_SPEC>;

impl I3CHwCapabilityReg {
    #[doc = "Reflects the IC_SLV_IBI Configurable Parameter.\n\nSpecifies slave\'s capability to initiate slave interrupt requests."]
    #[inline(always)]
    pub fn slv_ibi_cap(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, I3CHwCapabilityReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,I3CHwCapabilityReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Reflects the IC_SLV_HJ Configurable Parameter.\n\nSpecifies slave\'s capability to initiate Hot-join request."]
    #[inline(always)]
    pub fn slv_hj_cap(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, I3CHwCapabilityReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,I3CHwCapabilityReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Reflects the IC_HAS_DMA Configurable Parameter.\n\nSpecifies whether controller is configured to have DMA handshaking interface."]
    #[inline(always)]
    pub fn dma_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, I3CHwCapabilityReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,I3CHwCapabilityReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Reflects the IC_HDR_TX_CLK_PERIOD Configurable Parameter."]
    #[inline(always)]
    pub fn hdr_tx_clock_period(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x3f,
        1,
        0,
        u8,
        u8,
        I3CHwCapabilityReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x3f,
            1,
            0,
            u8,
            u8,
            I3CHwCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Reflects the IC_CLK_PERIOD Configurable Parameter"]
    #[inline(always)]
    pub fn clock_period(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x3f,
        1,
        0,
        u8,
        u8,
        I3CHwCapabilityReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x3f,
            1,
            0,
            u8,
            u8,
            I3CHwCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Reflects the IC_SPEED_HDR_TS Configurable Parameter.\n\nSpecifies the Controllers capability to perform HDR-TS transfers.\n\n0 : HDR-TS not supported\n1 : HDR-TS supported"]
    #[inline(always)]
    pub fn hdr_ts_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I3CHwCapabilityReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I3CHwCapabilityReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Reflects the IC_SPEED_HDR_DDR Configurable Parameter.\n\nSpecifies the Controllers capability to perform HDR-DDR transfers.\n\n0 : HDR-DDR not supported\n1 : HDR-DDR supported"]
    #[inline(always)]
    pub fn hdr_ddr_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I3CHwCapabilityReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I3CHwCapabilityReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Reflects the IC_DEVICE_ROLE Configurable Parameter.\n\nSpecifies the configured role of DWC_mipi_i3c controller\n\n1 : Master Only\n2 : Programmable Master-Slave\n3 : Secondary Master\n4 : Slave Only"]
    #[inline(always)]
    pub fn device_role_config(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, I3CHwCapabilityReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            u8,
            u8,
            I3CHwCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CHwCapabilityReg {
    #[inline(always)]
    fn default() -> I3CHwCapabilityReg {
        <crate::RegValueT<I3CHwCapabilityReg_SPEC> as RegisterValue<_>>::new(213249)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CIbiQueueCtrlReg_SPEC;
impl crate::sealed::RegSpec for I3CIbiQueueCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "IBI Queue Control Register"]
pub type I3CIbiQueueCtrlReg = crate::RegValueT<I3CIbiQueueCtrlReg_SPEC>;

impl I3CIbiQueueCtrlReg {
    #[doc = "Notify Rejected Slave Interrupt Request Control.\n\nThis bit is used to suppress reporting to the application about SIR request rejected.\n\n0: Suppress passing the IBI Status to the IBI FIFO (hence not notifying the application) when a Slave Interrupt Request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT Register.\n1: Writes IBI Status to the IBI FIFO (hence notifying the application) when a Slave Interrupt Request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT Register.\n\nValues:\n\n0x0 (DISABLED): Notify SIR Rejected Disable\n0x1 (ENABLED): Notify SIR Rejected Enable"]
    #[inline(always)]
    pub fn notify_sir_rejected(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I3CIbiQueueCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I3CIbiQueueCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Notify Rejected Hot-Join Control.\n\nThis bit is used to suppress reporting to the application about Hot-Join request rejected (NACK and Auto Disable).\n\n0: Suppress passing the IBI Status to the IBI FIFO (hence not notifying the application) when a HJ Request is NACKed and auto-disabled based on the DEVICE_CTRL.HOT_JOIN_CTRL.\n1: Writes IBI Status to the IBI FIFO (hence notifying the application) when a HJ Request is NACKed and auto-disabled based on the DEVICE_CTRL.HOT_JOIN_CTRL.\n\nValues:\n\n0x0 (DISABLED): Notify Hot-Join Rejected Disable\n0x1 (ENABLED): Notify Hot-Join Rejected Enable"]
    #[inline(always)]
    pub fn notify_hj_rejected(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CIbiQueueCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I3CIbiQueueCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CIbiQueueCtrlReg {
    #[inline(always)]
    fn default() -> I3CIbiQueueCtrlReg {
        <crate::RegValueT<I3CIbiQueueCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CIbiQueueStatusDataReg_SPEC;
impl crate::sealed::RegSpec for I3CIbiQueueStatusDataReg_SPEC {
    type DataType = u32;
}

#[doc = "In-Band Interrupt Queue Status and Data Register"]
pub type I3CIbiQueueStatusDataReg = crate::RegValueT<I3CIbiQueueStatusDataReg_SPEC>;

impl I3CIbiQueueStatusDataReg {
    #[doc = "IBI Received Status.\n\nDefines the master response for IBI received.\n\n4\'b0xxx: Responded with ACK\n4\'b1xxx: Responded with NACK\nOthers : RESERVED"]
    #[inline(always)]
    pub fn ibi_sts(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CIbiQueueStatusDataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CIbiQueueStatusDataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "IBI Identifier.\n\nThe byte received after START which includes the address and the R/W bit.\n\nDevice address and R/W bit in case of Slave Interrupt or Master Request.\nHot-Join ID and R/W bit in case of Hot-Join IBI."]
    #[inline(always)]
    pub fn ibi_id(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CIbiQueueStatusDataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CIbiQueueStatusDataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "In-Band Interrupt data length.\n\nThis field represents the length of data received along with the IBI, in bytes."]
    #[inline(always)]
    pub fn data_length(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CIbiQueueStatusDataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CIbiQueueStatusDataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CIbiQueueStatusDataReg {
    #[inline(always)]
    fn default() -> I3CIbiQueueStatusDataReg {
        <crate::RegValueT<I3CIbiQueueStatusDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CIbiSirReqRejectReg_SPEC;
impl crate::sealed::RegSpec for I3CIbiSirReqRejectReg_SPEC {
    type DataType = u32;
}

#[doc = "IBI SIR Request Rejection Control Register"]
pub type I3CIbiSirReqRejectReg = crate::RegValueT<I3CIbiSirReqRejectReg_SPEC>;

impl I3CIbiSirReqRejectReg {
    #[doc = "In-band Slave Interrupt Request Reject.\n\nThe application of the DWC_mipi_i3c can decide whether to send ACK or NACK for a Slave request received from any I3C device.\n\nA device specific response control bit is provided to select the response option. Master will ACK/NACK the Master Request based on programming of control bit, corresponding to the interrupting device.\n\n0 - ACK the SIR Request\n1 - NACK and send directed auto disable CCC"]
    #[inline(always)]
    pub fn sir_req_reject(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CIbiSirReqRejectReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CIbiSirReqRejectReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CIbiSirReqRejectReg {
    #[inline(always)]
    fn default() -> I3CIbiSirReqRejectReg {
        <crate::RegValueT<I3CIbiSirReqRejectReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CIntrForceReg_SPEC;
impl crate::sealed::RegSpec for I3CIntrForceReg_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Force Enable Register"]
pub type I3CIntrForceReg = crate::RegValueT<I3CIntrForceReg_SPEC>;

impl I3CIntrForceReg {
    #[doc = "Transfer Error Force Enable"]
    #[inline(always)]
    pub fn transfer_err_force_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I3CIntrForceReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9,1,0,I3CIntrForceReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Transfer Abort Force Enable.\n\nThis field is used only in master mode of operation."]
    #[inline(always)]
    pub fn transfer_abort_force_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I3CIntrForceReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,I3CIntrForceReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Response Queue Ready Force Enable"]
    #[inline(always)]
    pub fn resp_ready_force_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I3CIntrForceReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,I3CIntrForceReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Command Queue Ready Force Enable"]
    #[inline(always)]
    pub fn cmd_queue_ready_force_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I3CIntrForceReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,I3CIntrForceReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "IBI Buffer Threshold Force Enable.\n\nThis field is used only in master mode of operation."]
    #[inline(always)]
    pub fn ibi_thld_force_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I3CIntrForceReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,I3CIntrForceReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Receive Buffer Threshold Force Enable"]
    #[inline(always)]
    pub fn rx_thld_force_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I3CIntrForceReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,I3CIntrForceReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Transmit Buffer Threshold Force Enable."]
    #[inline(always)]
    pub fn tx_thld_force_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CIntrForceReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,I3CIntrForceReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CIntrForceReg {
    #[inline(always)]
    fn default() -> I3CIntrForceReg {
        <crate::RegValueT<I3CIntrForceReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CIntrSignalEnReg_SPEC;
impl crate::sealed::RegSpec for I3CIntrSignalEnReg_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Signal Enable Register"]
pub type I3CIntrSignalEnReg = crate::RegValueT<I3CIntrSignalEnReg_SPEC>;

impl I3CIntrSignalEnReg {
    #[doc = "Transfer Error Signal Enable"]
    #[inline(always)]
    pub fn transfer_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,I3CIntrSignalEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transfer Abort Signal Enable.\n\nThis field is used only in master mode of operation."]
    #[inline(always)]
    pub fn transfer_abort_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I3CIntrSignalEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Response Queue Ready Signal Enable"]
    #[inline(always)]
    pub fn resp_ready_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,I3CIntrSignalEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command Queue Ready Signal Enable"]
    #[inline(always)]
    pub fn cmd_queue_ready_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I3CIntrSignalEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IBI Buffer Threshold Signal Enable.\n\nThis field is used only in master mode of operation."]
    #[inline(always)]
    pub fn ibi_thld_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,I3CIntrSignalEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive Buffer Threshold Signal Enable"]
    #[inline(always)]
    pub fn rx_thld_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I3CIntrSignalEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit Buffer Threshold Signal Enable.\n\n*Note: For the deassertion of the interrupt, first the *SIGNAL_EN bitfield should be cleared and then the *STATUS_EN one, otherwise is not working. This comment applies for the rest bitfields of this register"]
    #[inline(always)]
    pub fn tx_thld_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I3CIntrSignalEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CIntrSignalEnReg {
    #[inline(always)]
    fn default() -> I3CIntrSignalEnReg {
        <crate::RegValueT<I3CIntrSignalEnReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CIntrStatusEnReg_SPEC;
impl crate::sealed::RegSpec for I3CIntrStatusEnReg_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Status Enable Register"]
pub type I3CIntrStatusEnReg = crate::RegValueT<I3CIntrStatusEnReg_SPEC>;

impl I3CIntrStatusEnReg {
    #[doc = "Transfer Error Status Enable"]
    #[inline(always)]
    pub fn transfer_err_sts_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,I3CIntrStatusEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transfer Abort Status Enable.\n\nThis field is used only in master mode of operation."]
    #[inline(always)]
    pub fn transfer_abort_sts_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I3CIntrStatusEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Response Queue Ready Status Enable"]
    #[inline(always)]
    pub fn resp_ready_sts_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,I3CIntrStatusEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command Queue Ready Status Enable"]
    #[inline(always)]
    pub fn cmd_queue_ready_sts_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I3CIntrStatusEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IBI Buffer Threshold Status Enable.\n\nThis field is used only in master mode of operation."]
    #[inline(always)]
    pub fn ibi_thld_sts_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,I3CIntrStatusEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive Buffer Threshold Status Enable"]
    #[inline(always)]
    pub fn rx_thld_sts_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I3CIntrStatusEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit Buffer Threshold Status Enable"]
    #[inline(always)]
    pub fn tx_thld_sts_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I3CIntrStatusEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CIntrStatusEnReg {
    #[inline(always)]
    fn default() -> I3CIntrStatusEnReg {
        <crate::RegValueT<I3CIntrStatusEnReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CIntrStatusReg_SPEC;
impl crate::sealed::RegSpec for I3CIntrStatusReg_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Status Register"]
pub type I3CIntrStatusReg = crate::RegValueT<I3CIntrStatusReg_SPEC>;

impl I3CIntrStatusReg {
    #[doc = "Transfer Error Status.\n\nThis interrupt is generated if any error occurs during transfer. The error type will be specified in the response packet associated with the command (in ERR_STATUS field of RESPONSE_QUEUE_PORT register). This bit can be cleared by writing 1\'b1."]
    #[inline(always)]
    pub fn transfer_err_sts(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I3CIntrStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,I3CIntrStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transfer Abort Status.\n\nThis field is used only in master mode of operation.\n\nThis interrupt is generated if transfer is aborted. This interrupt can be cleared by writing 1\'b1."]
    #[inline(always)]
    pub fn transfer_abort_sts(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I3CIntrStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I3CIntrStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Response Queue Ready Status.\n\nThis interrupt is generated when number of entries in response queue is greater than or equal to threshold value specified by RESP_BUF_THLD field in QUEUE_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in response buffer is less than threshold value specified."]
    #[inline(always)]
    pub fn resp_ready_sts(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I3CIntrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I3CIntrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Command Queue Ready.\n\nThis interrupt is generated when number of empty locations in command queue is greater than or equal to threshold value specified by CMD_EMPTY_BUF_THLD field in QUEUE_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in command buffer is less than threshold value specified."]
    #[inline(always)]
    pub fn cmd_queue_ready_sts(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I3CIntrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I3CIntrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IBI Buffer Threshold Status.\n\nThis field is only used in master mode of operation This interrupt is generated when number of entries in IBI buffer is greater than or equal to threshold value specified by IBI_BUF_THLD field in QUEUE_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in IBI buffer is less than threshold value specified."]
    #[inline(always)]
    pub fn ibi_thld_sts(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I3CIntrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I3CIntrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Buffer Threshold Status.\n\nThis interrupt is generated when number of entries in receive buffer is greater than or equal to threshold value specified by RX_BUF_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in receive buffer is less than threshold value specified."]
    #[inline(always)]
    pub fn rx_thld_sts(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I3CIntrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I3CIntrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmit Buffer Threshold Status.\n\nThis interrupt is generated when number of empty locations in transmit buffer is greater than or equal to threshold value specified by TX_EMPTY_BUF_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in transmit buffer is less than threshold value specified."]
    #[inline(always)]
    pub fn tx_thld_sts(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CIntrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I3CIntrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CIntrStatusReg {
    #[inline(always)]
    fn default() -> I3CIntrStatusReg {
        <crate::RegValueT<I3CIntrStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CPresentStateReg_SPEC;
impl crate::sealed::RegSpec for I3CPresentStateReg_SPEC {
    type DataType = u32;
}

#[doc = "Present State Register"]
pub type I3CPresentStateReg = crate::RegValueT<I3CPresentStateReg_SPEC>;

impl I3CPresentStateReg {
    #[doc = "This field reflects whether the Master Controller is in Idle state or not. This bit will set when all the Queues(Command , Response, IBI) and Buffers(Transmit and Receive) are empty along with the Master State machine is in Idle state.\n\nValues:\n\n0x0 (MST_NOT_IDLE): Master Controller is not in IDLE State\n0x1 (MST_IDLE): Master Controller is in IDLE State."]
    #[inline(always)]
    pub fn master_idle(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, I3CPresentStateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,I3CPresentStateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This field reflects the Transaction-ID of the current executing command."]
    #[inline(always)]
    pub fn cmd_tid(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CPresentStateReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CPresentStateReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Current Master Transfer State Status.\n\nIndicates the state of current transfer currently executing by the DWC_mipi_i3c controller. This is valid in Master mode only.\n\n6\'h0: IDLE (Controller is Idle state, waiting for commands from application or Slave initated In-band Interrupt)\n6\'h1: START Generation State.\n6\'h2: RESTART Generation State.\n6\'h3: STOP Generation State.\n6\'h4: START Hold Generation for the Slave Initiated START State.\n6\'h5: Broadcast Write Address Header(7\'h7E,W) Generation State.\n6\'h6: Broadcast Read Address Header(7\'h7E,R) Generation State.\n6\'h7: Dynamic Address Assignment State.\n6\'h8: Slave Address Generation State.\n6\'hB: CCC Byte Generation State.\n6\'hC: HDR Command Generation State.\n6\'hD: Write Data Transfer State.\n6\'hE: Read Data Transfer State.\n6\'hF: In-Band Interrupt(SIR) Read Data State.\n6\'h10: In-Band Interrupt Auto-Disable State\n6\'h11: HDR-DDR CRC Data Generation/Receive State.\n6\'h12: Clock Extension State.\n6\'h13: Halt State."]
    #[inline(always)]
    pub fn cm_tfr_st_sts(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3f,
        1,
        0,
        u8,
        u8,
        I3CPresentStateReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3f,
            1,
            0,
            u8,
            u8,
            I3CPresentStateReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Transfer Type Status.\n\nIndicates the type of transfer currently executing by the DWC_mipi_i3c controller.\n\nIn Master mode of operation :\n\n6\'h0: IDLE (Controller is in Idle state, waiting for commands from application or Slave initated In-band Interrupt)\n6\'h1: Broadcast CCC Write Transfer.\n6\'h2: Directed CCC Write Transfer.\n6\'h3: Directed CCC Read Transfer.\n6\'h4: ENTDAA Address Assignment Transfer.\n6\'h5: SETDASA Address Assignment Transfer.\n6\'h6: Private I3C SDR Write Transfer.\n6\'h7: Private I3C SDR Read Transfer.\n6\'h8: Private I2C SDR Write Transfer.\n6\'h9: Private I2C SDR Read Transfer.\n6\'hA: Private HDR Ternary Symbol(TS) Write Transfer.\n6\'hB: Private HDR Ternary Symbol(TS) Read Transfer.\n6\'hC: Private HDR Double-Data Rate(DDR) Write Transfer.\n6\'hD: Private HDR Double-Data Rate(DDR) Read Transfer.\n6\'hE: Servicing In-Band Interrupt Transfer.\n6\'hF: Halt state (Controller is in Halt State, waiting for the application to resume through DEVICE_CTRL Register)"]
    #[inline(always)]
    pub fn cm_tfr_sts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3f,
        1,
        0,
        u8,
        u8,
        I3CPresentStateReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            u8,
            u8,
            I3CPresentStateReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "This Bit is used to check whether the Master is Current Master or not. The Current Master is the Master that owns the SCL line.\n\nIf this bit is set to 0, the Master is not Current Master and requires to request and the ownership before initiating any transfer on the line.\n\nIf this bit is set to 1, the Master is the Current Master and can initate the transfers on the line.\n\n0: Master is not Current Master\n1: Master is Current Master"]
    #[inline(always)]
    pub fn current_master(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I3CPresentStateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I3CPresentStateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit is used to check the SDA line level to recover from errors and for debugging. This bit reflects the value of synchronized sda_in_a signal. This is valid in Master mode only."]
    #[inline(always)]
    pub fn sda_line_signal_level(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I3CPresentStateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I3CPresentStateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit is used to check the SCL line level to recover from errors and for debugging. This bit reflects the value of synchronized scl_in_a signal. This is valid in Master mode only"]
    #[inline(always)]
    pub fn scl_line_signal_level(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CPresentStateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I3CPresentStateReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CPresentStateReg {
    #[inline(always)]
    fn default() -> I3CPresentStateReg {
        <crate::RegValueT<I3CPresentStateReg_SPEC> as RegisterValue<_>>::new(268435459)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CQueueSizeCapabilityReg_SPEC;
impl crate::sealed::RegSpec for I3CQueueSizeCapabilityReg_SPEC {
    type DataType = u32;
}

#[doc = "DWC_mipi_i3c Queue Size Capability Register"]
pub type I3CQueueSizeCapabilityReg = crate::RegValueT<I3CQueueSizeCapabilityReg_SPEC>;

impl I3CQueueSizeCapabilityReg {
    #[doc = "IBI Queue Size\n\nThis field reflects the configured IBI Queue size (in DWORDS) in Encoded Values.\n\nValues:\n\n- 0x0 : 2 DWORDS\n\n- 0x1 : 4 DWORDS\n\n- 0x2 : 8 DWORDS\n\n- 0x3 : 16 DWORDS"]
    #[inline(always)]
    pub fn ibi_buf_size(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CQueueSizeCapabilityReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CQueueSizeCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Response Queue Size\n\nThis field reflects the configured Response Queue size (in DWORDS) in Encoded Values.\n\nValues:\n\n- 0x0 : 2 DWORDS\n\n- 0x1 : 4 DWORDS\n\n- 0x2 : 8 DWORDS\n\n- 0x3 : 16 DWORDS"]
    #[inline(always)]
    pub fn resp_buf_size(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CQueueSizeCapabilityReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CQueueSizeCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Command Queue Size\n\nThis field reflects the configured Command Queue size (in DWORDS) in Encoded Values.\n\nValues:\n\n- 0x0 : 2 DWORDS\n\n- 0x1 : 4 DWORDS\n\n- 0x2 : 8 DWORDS\n\n- 0x3 : 16 DWORDS"]
    #[inline(always)]
    pub fn cmd_buf_size(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CQueueSizeCapabilityReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CQueueSizeCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Data Buffer Size\n\nThis field reflects the configured Receive Buffer size (in DWORDS) in Encoded Values.\n\nValues:\n\n- 0x0 : 2 DWORDS\n\n- 0x1 : 4 DWORDS\n\n- 0x2 : 8 DWORDS\n\n- 0x3 : 16 DWORDS\n\n- 0x4 : 32 DWORDS\n\n- 0x5 : 64 DWORDS"]
    #[inline(always)]
    pub fn rx_buf_size(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CQueueSizeCapabilityReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CQueueSizeCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Data Buffer Size\n\nThis field reflects the configured Transmit Buffer size (in DWORDS) in Encoded Values.\n\nValues:\n\n- 0x0 : 2 DWORDS\n\n- 0x1 : 4 DWORDS\n\n- 0x2 : 8 DWORDS\n\n- 0x3 : 16 DWORDS\n\n- 0x4 : 32 DWORDS\n\n- 0x5 : 64 DWORDS"]
    #[inline(always)]
    pub fn tx_buf_size(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CQueueSizeCapabilityReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CQueueSizeCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CQueueSizeCapabilityReg {
    #[inline(always)]
    fn default() -> I3CQueueSizeCapabilityReg {
        <crate::RegValueT<I3CQueueSizeCapabilityReg_SPEC> as RegisterValue<_>>::new(135748)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CQueueStatusLevelReg_SPEC;
impl crate::sealed::RegSpec for I3CQueueStatusLevelReg_SPEC {
    type DataType = u32;
}

#[doc = "Queue Status Level Register"]
pub type I3CQueueStatusLevelReg = crate::RegValueT<I3CQueueStatusLevelReg_SPEC>;

impl I3CQueueStatusLevelReg {
    #[doc = "IBI Buffer Status Count.\n\nContains the number of IBI status entries in the IBI Buffer.\n\nThis field is used in master mode of operation."]
    #[inline(always)]
    pub fn ibi_sts_cnt(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1f,
        1,
        0,
        u8,
        u8,
        I3CQueueStatusLevelReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1f,
            1,
            0,
            u8,
            u8,
            I3CQueueStatusLevelReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "IBI Buffer Level Value.\n\nContains the number of valid entries in the IBI Buffer.\n\nThis field is used in master mode of operation."]
    #[inline(always)]
    pub fn ibi_buf_blr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CQueueStatusLevelReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CQueueStatusLevelReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Response Buffer Level Value.\n\nContains the number of valid data entries in the response Buffer."]
    #[inline(always)]
    pub fn resp_buf_blr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CQueueStatusLevelReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CQueueStatusLevelReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Command Queue Empty Locations.\n\nContains the number of empty locations in the command Buffer."]
    #[inline(always)]
    pub fn cmd_queue_empty_loc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CQueueStatusLevelReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CQueueStatusLevelReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CQueueStatusLevelReg {
    #[inline(always)]
    fn default() -> I3CQueueStatusLevelReg {
        <crate::RegValueT<I3CQueueStatusLevelReg_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CQueueThldCtrlReg_SPEC;
impl crate::sealed::RegSpec for I3CQueueThldCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Queue Threshold Control Register"]
pub type I3CQueueThldCtrlReg = crate::RegValueT<I3CQueueThldCtrlReg_SPEC>;

impl I3CQueueThldCtrlReg {
    #[doc = "In-Band Interrupt Status Threshold Value.\n\nEvery In Band Interrupt received (with or without data) by I3C controller generates an IBI status. This field controls the number of IBI status entries (or greater) in the IBI queue that trigger the IBI_THLD_STAT interrupt.\n\nThe valid range is 0 to 7. The software shall program only valid values. A value of 0 sets the threshold for 1 entry, and a value of N sets the threshold for N+1 entries."]
    #[inline(always)]
    pub fn ibi_status_thld(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CQueueThldCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CQueueThldCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Response Buffer Threshold Value.\n\nControls the number of entries (or greater) in the Response Queue that trigger the RESP_READY_STAT_INTR interrupt.\n\nThe valid range is 0 to 3. The software shall program only valid values. A value of 0 sets the threshold for 1 entry, and a value of N sets the threshold for N+1 entries."]
    #[inline(always)]
    pub fn resp_buf_thld(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CQueueThldCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CQueueThldCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Command Buffer Empty Threshold Value. Controls the number of empty locations (or greater) in the Command Queue that trigger CMD_QUEUE_READY_STAT interrupt.\n\nThe valid range is 0 to 7. The software shall program only valid values. Value of N ranging from 1 to 7 sets the threshold to N empty locations and a value of 0 sets the threshold to indicate that the queue is completely empty."]
    #[inline(always)]
    pub fn cmd_empty_buf_thld(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CQueueThldCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CQueueThldCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CQueueThldCtrlReg {
    #[inline(always)]
    fn default() -> I3CQueueThldCtrlReg {
        <crate::RegValueT<I3CQueueThldCtrlReg_SPEC> as RegisterValue<_>>::new(16777473)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CResetCtrlReg_SPEC;
impl crate::sealed::RegSpec for I3CResetCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Reset Control Register"]
pub type I3CResetCtrlReg = crate::RegValueT<I3CResetCtrlReg_SPEC>;

impl I3CResetCtrlReg {
    #[doc = "IBI Queue Software Reset.\n\nThis bit is only used in master mode of operation.\n\nWrite 1\'b1 to this bit to exercise IBI Queue reset This bit will be cleared automatically once the IBI Queue reset is completed."]
    #[inline(always)]
    pub fn ibi_queue_rst(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I3CResetCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I3CResetCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive Buffer Software Reset.\n\nWrite 1\'b1 to this bit to exercise Receive Buffer reset. This bit will be cleared automatically once the Receive buffer reset is completed."]
    #[inline(always)]
    pub fn rx_fifo_rst(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I3CResetCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,I3CResetCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit Buffer Software Reset.\n\nWrite 1\'b1 to this bit to exercise Transmit Buffer reset. This bit will be cleared automatically once the Transmit Buffer reset is completed."]
    #[inline(always)]
    pub fn tx_fifo_rst(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I3CResetCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I3CResetCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Response Queue Software Reset.\n\nWrite 1\'b1 to this bit to exercise Response Queue reset. This bit will be cleared automatically once the Response Queue reset is completed."]
    #[inline(always)]
    pub fn resp_queue_rst(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I3CResetCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,I3CResetCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command Queue Software Reset.\n\nWrite 1\'b1 to this bit to exercise Command Queue reset. This bit will the cleared automatically once the Command Queue reset is completed."]
    #[inline(always)]
    pub fn cmd_queue_rst(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I3CResetCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I3CResetCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Core Software Reset.\n\nWrite 1\'b1 to this bit to exercise software reset. This will reset all Buffers - Receive, Transmit, Command and Response This bit will be cleared automatically once the core reset is completed."]
    #[inline(always)]
    pub fn soft_rst(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CResetCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I3CResetCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CResetCtrlReg {
    #[inline(always)]
    fn default() -> I3CResetCtrlReg {
        <crate::RegValueT<I3CResetCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CResponseQueuePortReg_SPEC;
impl crate::sealed::RegSpec for I3CResponseQueuePortReg_SPEC {
    type DataType = u32;
}

#[doc = "RESPONSE_QUEUE_PORT"]
pub type I3CResponseQueuePortReg = crate::RegValueT<I3CResponseQueuePortReg_SPEC>;

impl I3CResponseQueuePortReg {
    #[doc = "32 bit Response"]
    #[inline(always)]
    pub fn response(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CResponseQueuePortReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CResponseQueuePortReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CResponseQueuePortReg {
    #[inline(always)]
    fn default() -> I3CResponseQueuePortReg {
        <crate::RegValueT<I3CResponseQueuePortReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CRxTxDataPortReg_SPEC;
impl crate::sealed::RegSpec for I3CRxTxDataPortReg_SPEC {
    type DataType = u32;
}

#[doc = "Receive and Transmit Data Port Register"]
pub type I3CRxTxDataPortReg = crate::RegValueT<I3CRxTxDataPortReg_SPEC>;

impl I3CRxTxDataPortReg {
    #[doc = "Receive and Transmit Data Port. (Merged in Doxbox)\n\nThe Receive data port is mapped to the Rx-Data Buffer.\n\nThe Receive data is always packed in 4-byte aligned data words and stored in the Rx-Data Buffer. If the command length is not aligned to the 4-bytes, then the additional data bytes have to be ignored."]
    #[inline(always)]
    pub fn rx_tx_data_port(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CRxTxDataPortReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CRxTxDataPortReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CRxTxDataPortReg {
    #[inline(always)]
    fn default() -> I3CRxTxDataPortReg {
        <crate::RegValueT<I3CRxTxDataPortReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSclExtLcntTimingReg_SPEC;
impl crate::sealed::RegSpec for I3CSclExtLcntTimingReg_SPEC {
    type DataType = u32;
}

#[doc = "SCL Extended Low Count Timing Register"]
pub type I3CSclExtLcntTimingReg = crate::RegValueT<I3CSclExtLcntTimingReg_SPEC>;

impl I3CSclExtLcntTimingReg {
    #[doc = "I3C Extended Low Count Register 4\n\nSDR4 uses this register field for data transfer."]
    #[inline(always)]
    pub fn i3c_ext_lcnt_4(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclExtLcntTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclExtLcntTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I3C Extended Low Count Register 3\n\nSDR3 uses this register field for data transfer."]
    #[inline(always)]
    pub fn i3c_ext_lcnt_3(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclExtLcntTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclExtLcntTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I3C Extended Low Count Register 2\n\nSDR2 uses this register field for data transfer."]
    #[inline(always)]
    pub fn i3c_ext_lcnt_2(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclExtLcntTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclExtLcntTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I3C Extended Low Count Register 1\n\nSDR1 uses this register field for data transfer."]
    #[inline(always)]
    pub fn i3c_ext_lcnt_1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclExtLcntTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclExtLcntTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSclExtLcntTimingReg {
    #[inline(always)]
    fn default() -> I3CSclExtLcntTimingReg {
        <crate::RegValueT<I3CSclExtLcntTimingReg_SPEC> as RegisterValue<_>>::new(538976288)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSclExtTermnLcntTimeReg_SPEC;
impl crate::sealed::RegSpec for I3CSclExtTermnLcntTimeReg_SPEC {
    type DataType = u32;
}

#[doc = "SCL Termination Bit Low count Timing Register"]
pub type I3CSclExtTermnLcntTimeReg = crate::RegValueT<I3CSclExtTermnLcntTimeReg_SPEC>;

impl I3CSclExtTermnLcntTimeReg {
    #[doc = "I3C Read Termination Bit Low count.\n\nExtended I3C Read Termination Bit low count for I3C Read transfers. Effective Termination-Bit Low Period is derived based on the SDR speed as shown below\n\nSDR0 speed: I3C_PP_LCNT + I3C_EXT_TERMN_LCNT\nSDR1 speed: I3C_EXT_LCNT_1 + I3C_EXT_TERMN_LCNT\nSDR2 speed: I3C_EXT_LCNT_2 + I3C_EXT_TERMN_LCNT\nSDR3 speed: I3C_EXT_LCNT_3 + I3C_EXT_TERMN_LCNT\nSDR4 speed: I3C_EXT_LCNT_4 + I3C_EXT_TERMN_LCNT"]
    #[inline(always)]
    pub fn i3c_ext_termn_lcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CSclExtTermnLcntTimeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CSclExtTermnLcntTimeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSclExtTermnLcntTimeReg {
    #[inline(always)]
    fn default() -> I3CSclExtTermnLcntTimeReg {
        <crate::RegValueT<I3CSclExtTermnLcntTimeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSclI2CFmpTimingReg_SPEC;
impl crate::sealed::RegSpec for I3CSclI2CFmpTimingReg_SPEC {
    type DataType = u32;
}

#[doc = "SCL I2C Fast Mode Plus Timing Register"]
pub type I3CSclI2CFmpTimingReg = crate::RegValueT<I3CSclI2CFmpTimingReg_SPEC>;

impl I3CSclI2CFmpTimingReg {
    #[doc = "I2C Fast Mode Plus High Count\n\nThe SCL open-drain high count timing for I2C fast mode transfers."]
    #[inline(always)]
    pub fn i2c_fmp_hcnt(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclI2CFmpTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclI2CFmpTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I2C Fast Mode Plus Low Count\n\nThe SCL open-drain low count timing for I2C fast mode transfers."]
    #[inline(always)]
    pub fn i2c_fmp_lcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CSclI2CFmpTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CSclI2CFmpTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSclI2CFmpTimingReg {
    #[inline(always)]
    fn default() -> I3CSclI2CFmpTimingReg {
        <crate::RegValueT<I3CSclI2CFmpTimingReg_SPEC> as RegisterValue<_>>::new(1048592)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSclI2CFmTimingReg_SPEC;
impl crate::sealed::RegSpec for I3CSclI2CFmTimingReg_SPEC {
    type DataType = u32;
}

#[doc = "SCL I2C Fast Mode Timing Register"]
pub type I3CSclI2CFmTimingReg = crate::RegValueT<I3CSclI2CFmTimingReg_SPEC>;

impl I3CSclI2CFmTimingReg {
    #[doc = "I2C Fast Mode High Count\n\nThe SCL open-drain high count timing for I2C fast mode transfers."]
    #[inline(always)]
    pub fn i2c_fm_hcnt(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CSclI2CFmTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CSclI2CFmTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I2C Fast Mode Low Count\n\nThe SCL open-drain low count timing for I2C fast mode transfers."]
    #[inline(always)]
    pub fn i2c_fm_lcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CSclI2CFmTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CSclI2CFmTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSclI2CFmTimingReg {
    #[inline(always)]
    fn default() -> I3CSclI2CFmTimingReg {
        <crate::RegValueT<I3CSclI2CFmTimingReg_SPEC> as RegisterValue<_>>::new(1048592)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSclI3COdTimingReg_SPEC;
impl crate::sealed::RegSpec for I3CSclI3COdTimingReg_SPEC {
    type DataType = u32;
}

#[doc = "SCL I3C Open Drain Timing Register"]
pub type I3CSclI3COdTimingReg = crate::RegValueT<I3CSclI3COdTimingReg_SPEC>;

impl I3CSclI3COdTimingReg {
    #[doc = "I3C Open Drain High Count.\n\nSCL open-drain High count (I3C) for I3C transfers targeted to I3C devices."]
    #[inline(always)]
    pub fn i3c_od_hcnt(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclI3COdTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclI3COdTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I3C Open Drain Low Count.\n\nSCL Open-drain low count for I3C transfers targeted to I3C devices."]
    #[inline(always)]
    pub fn i3c_od_lcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclI3COdTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclI3COdTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSclI3COdTimingReg {
    #[inline(always)]
    fn default() -> I3CSclI3COdTimingReg {
        <crate::RegValueT<I3CSclI3COdTimingReg_SPEC> as RegisterValue<_>>::new(655376)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSclI3CPpTimingReg_SPEC;
impl crate::sealed::RegSpec for I3CSclI3CPpTimingReg_SPEC {
    type DataType = u32;
}

#[doc = "SCL I3C Push Pull Timing Register"]
pub type I3CSclI3CPpTimingReg = crate::RegValueT<I3CSclI3CPpTimingReg_SPEC>;

impl I3CSclI3CPpTimingReg {
    #[doc = "I3C Push Pull High Count.\n\nSCL push-pull High count for I3C transfers targeted to I3C devices."]
    #[inline(always)]
    pub fn i3c_pp_hcnt(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclI3CPpTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclI3CPpTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I3C Push Pull Low Count.\n\nSCL Push-pull low count for I3C transfers targeted to I3C devices."]
    #[inline(always)]
    pub fn i3c_pp_lcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclI3CPpTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclI3CPpTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSclI3CPpTimingReg {
    #[inline(always)]
    fn default() -> I3CSclI3CPpTimingReg {
        <crate::RegValueT<I3CSclI3CPpTimingReg_SPEC> as RegisterValue<_>>::new(655370)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSdaHoldDlyTimingReg_SPEC;
impl crate::sealed::RegSpec for I3CSdaHoldDlyTimingReg_SPEC {
    type DataType = u32;
}

#[doc = "SDA Hold Delay Timing Register"]
pub type I3CSdaHoldDlyTimingReg = crate::RegValueT<I3CSdaHoldDlyTimingReg_SPEC>;

impl I3CSdaHoldDlyTimingReg {
    #[doc = "This field controls the hold time (in term of the core clock period) of the transmit data (SDA) with respect to the SCL edge in FM FM+ SDR and DDR speed mode of operations. This field is not applicable for the ternary speed modes. The valid values are 1 to 7. Others are Reserved."]
    #[inline(always)]
    pub fn sda_tx_hold(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        u8,
        u8,
        I3CSdaHoldDlyTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            u8,
            u8,
            I3CSdaHoldDlyTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSdaHoldDlyTimingReg {
    #[inline(always)]
    fn default() -> I3CSdaHoldDlyTimingReg {
        <crate::RegValueT<I3CSdaHoldDlyTimingReg_SPEC> as RegisterValue<_>>::new(65536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSlvEventStatusReg_SPEC;
impl crate::sealed::RegSpec for I3CSlvEventStatusReg_SPEC {
    type DataType = u32;
}

#[doc = "Slave Event Status Register"]
pub type I3CSlvEventStatusReg = crate::RegValueT<I3CSlvEventStatusReg_SPEC>;

impl I3CSlvEventStatusReg {
    #[doc = "MWL Updated Status.\n\nThis bit indicates a SETMWL CCC is received by the slave. The updated MWL value can be read from SLV_MAX_LEN register. This status can be cleared by writing 1\'b1 to this field after reading the updated MWL."]
    #[inline(always)]
    pub fn mwl_updated(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I3CSlvEventStatusReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,I3CSlvEventStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MRL Updated Status.\n\nThis bit indicates a SETMRL CCC is received by the slave. The updated MRL value can be read from SLV_MAX_LEN register. This status can be cleared by writing 1\'b1 to this field after reading the updated MRL."]
    #[inline(always)]
    pub fn mrl_updated(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I3CSlvEventStatusReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,I3CSlvEventStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Activity State Status.\n\nENTAS0 - 00\n\nENTAS1 - 01\n\nENTAS2 - 10\n\nENTAS3 - 11\n\nThis bit reflects the Activity State of slave set by the Master."]
    #[inline(always)]
    pub fn activity_state(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CSlvEventStatusReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CSlvEventStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSlvEventStatusReg {
    #[inline(always)]
    fn default() -> I3CSlvEventStatusReg {
        <crate::RegValueT<I3CSlvEventStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CVendorSpecificRegPtrReg_SPEC;
impl crate::sealed::RegSpec for I3CVendorSpecificRegPtrReg_SPEC {
    type DataType = u32;
}

#[doc = "Pointer for Vendor specific Registers"]
pub type I3CVendorSpecificRegPtrReg = crate::RegValueT<I3CVendorSpecificRegPtrReg_SPEC>;

impl I3CVendorSpecificRegPtrReg {
    #[doc = "Start Address of Vendor specific registers."]
    #[inline(always)]
    pub fn p_vendor_reg_start_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CVendorSpecificRegPtrReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CVendorSpecificRegPtrReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CVendorSpecificRegPtrReg {
    #[inline(always)]
    fn default() -> I3CVendorSpecificRegPtrReg {
        <crate::RegValueT<I3CVendorSpecificRegPtrReg_SPEC> as RegisterValue<_>>::new(176)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CVerIdReg_SPEC;
impl crate::sealed::RegSpec for I3CVerIdReg_SPEC {
    type DataType = u32;
}

#[doc = "DWC_mipi_i3c Version ID Register"]
pub type I3CVerIdReg = crate::RegValueT<I3CVerIdReg_SPEC>;

impl I3CVerIdReg {
    #[doc = "Current release type\n\nThis field indicates the Synopsys DesignWare Cores DWC_mipi_i3c current release type that is read by an application.\n\nFor example, release type \"ga\" is represented in ASCII as 0x6761 and \"ea\" is represented as 0x6561. Lower 16 bits read from this register can be ignored by the application if release type is \"ga\". If release type is \"ea\" the lower 16 bits represents the \"ea\" release version.\n\nAn application reading this register along with the I3C_VER_ID register, gathers details of the current release."]
    #[inline(always)]
    pub fn i3c_ver_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CVerIdReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CVerIdReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CVerIdReg {
    #[inline(always)]
    fn default() -> I3CVerIdReg {
        <crate::RegValueT<I3CVerIdReg_SPEC> as RegisterValue<_>>::new(825241642)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CVerTypeReg_SPEC;
impl crate::sealed::RegSpec for I3CVerTypeReg_SPEC {
    type DataType = u32;
}

#[doc = "DWC_mipi_i3c Version Type Register"]
pub type I3CVerTypeReg = crate::RegValueT<I3CVerTypeReg_SPEC>;

impl I3CVerTypeReg {
    #[doc = "Current release type\n\nThis field indicates the Synopsys DesignWare Cores DWC_mipi_i3c current release type that is read by an application.\n\nFor example, release type \"ga\" is represented in ASCII as 0x6761 and \"ea\" is represented as 0x6561. Lower 16 bits read from this register can be ignored by the application if release type is \"ga\". If release type is \"ea\" the lower 16 bits represents the \"ea\" release version.\n\nAn application reading this register along with the I3C_VER_ID register, gathers details of the current release."]
    #[inline(always)]
    pub fn i3c_ver_type(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CVerTypeReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CVerTypeReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CVerTypeReg {
    #[inline(always)]
    fn default() -> I3CVerTypeReg {
        <crate::RegValueT<I3CVerTypeReg_SPEC> as RegisterValue<_>>::new(1818439731)
    }
}
