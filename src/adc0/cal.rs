#[doc = "Register `CAL` reader"]
pub type R = crate::R<CALrs>;
#[doc = "Register `CAL` writer"]
pub type W = crate::W<CALrs>;
#[doc = "Field `SINGLEOFFSET` reader - Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
pub type SingleoffsetR = crate::FieldReader;
#[doc = "Field `SINGLEOFFSET` writer - Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
pub type SingleoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SINGLEOFFSETINV` reader - Single Mode Offset Calibration Value for Negative Single-ended Mode"]
pub type SingleoffsetinvR = crate::FieldReader;
#[doc = "Field `SINGLEOFFSETINV` writer - Single Mode Offset Calibration Value for Negative Single-ended Mode"]
pub type SingleoffsetinvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SINGLEGAIN` reader - Single Mode Gain Calibration Value"]
pub type SinglegainR = crate::FieldReader;
#[doc = "Field `SINGLEGAIN` writer - Single Mode Gain Calibration Value"]
pub type SinglegainW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OFFSETINVMODE` reader - Negative Single-ended Offset Calibration is Enabled"]
pub type OffsetinvmodeR = crate::BitReader;
#[doc = "Field `OFFSETINVMODE` writer - Negative Single-ended Offset Calibration is Enabled"]
pub type OffsetinvmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANOFFSET` reader - Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
pub type ScanoffsetR = crate::FieldReader;
#[doc = "Field `SCANOFFSET` writer - Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
pub type ScanoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCANOFFSETINV` reader - Scan Mode Offset Calibration Value for Negative Single-ended Mode"]
pub type ScanoffsetinvR = crate::FieldReader;
#[doc = "Field `SCANOFFSETINV` writer - Scan Mode Offset Calibration Value for Negative Single-ended Mode"]
pub type ScanoffsetinvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCANGAIN` reader - Scan Mode Gain Calibration Value"]
pub type ScangainR = crate::FieldReader;
#[doc = "Field `SCANGAIN` writer - Scan Mode Gain Calibration Value"]
pub type ScangainW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CALEN` reader - Calibration Mode is Enabled"]
pub type CalenR = crate::BitReader;
#[doc = "Field `CALEN` writer - Calibration Mode is Enabled"]
pub type CalenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    pub fn singleoffset(&self) -> SingleoffsetR {
        SingleoffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Single Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    pub fn singleoffsetinv(&self) -> SingleoffsetinvR {
        SingleoffsetinvR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - Single Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn singlegain(&self) -> SinglegainR {
        SinglegainR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Negative Single-ended Offset Calibration is Enabled"]
    #[inline(always)]
    pub fn offsetinvmode(&self) -> OffsetinvmodeR {
        OffsetinvmodeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    pub fn scanoffset(&self) -> ScanoffsetR {
        ScanoffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Scan Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    pub fn scanoffsetinv(&self) -> ScanoffsetinvR {
        ScanoffsetinvR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30 - Scan Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn scangain(&self) -> ScangainR {
        ScangainR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Calibration Mode is Enabled"]
    #[inline(always)]
    pub fn calen(&self) -> CalenR {
        CalenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    #[must_use]
    pub fn singleoffset(&mut self) -> SingleoffsetW<CALrs> {
        SingleoffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Single Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    #[must_use]
    pub fn singleoffsetinv(&mut self) -> SingleoffsetinvW<CALrs> {
        SingleoffsetinvW::new(self, 4)
    }
    #[doc = "Bits 8:14 - Single Mode Gain Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn singlegain(&mut self) -> SinglegainW<CALrs> {
        SinglegainW::new(self, 8)
    }
    #[doc = "Bit 15 - Negative Single-ended Offset Calibration is Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn offsetinvmode(&mut self) -> OffsetinvmodeW<CALrs> {
        OffsetinvmodeW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    #[must_use]
    pub fn scanoffset(&mut self) -> ScanoffsetW<CALrs> {
        ScanoffsetW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Scan Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    #[must_use]
    pub fn scanoffsetinv(&mut self) -> ScanoffsetinvW<CALrs> {
        ScanoffsetinvW::new(self, 20)
    }
    #[doc = "Bits 24:30 - Scan Mode Gain Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn scangain(&mut self) -> ScangainW<CALrs> {
        ScangainW::new(self, 24)
    }
    #[doc = "Bit 31 - Calibration Mode is Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn calen(&mut self) -> CalenW<CALrs> {
        CalenW::new(self, 31)
    }
}
#[doc = "Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALrs;
impl crate::RegisterSpec for CALrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CALrs {}
#[doc = "`write(|w| ..)` method takes [`cal::W`](W) writer structure"]
impl crate::Writable for CALrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL to value 0x4078_4078"]
impl crate::Resettable for CALrs {
    const RESET_VALUE: u32 = 0x4078_4078;
}
