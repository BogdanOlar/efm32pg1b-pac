#[doc = "Register `VMONDVDDCTRL` reader"]
pub type R = crate::R<VMONDVDDCTRLrs>;
#[doc = "Register `VMONDVDDCTRL` writer"]
pub type W = crate::W<VMONDVDDCTRLrs>;
#[doc = "Field `EN` reader - Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RISEWU` reader - Rise Wakeup"]
pub type RisewuR = crate::BitReader;
#[doc = "Field `RISEWU` writer - Rise Wakeup"]
pub type RisewuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FALLWU` reader - Fall Wakeup"]
pub type FallwuR = crate::BitReader;
#[doc = "Field `FALLWU` writer - Fall Wakeup"]
pub type FallwuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRESFINE` reader - Threshold Fine Adjust"]
pub type ThresfineR = crate::FieldReader;
#[doc = "Field `THRESFINE` writer - Threshold Fine Adjust"]
pub type ThresfineW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `THRESCOARSE` reader - Threshold Coarse Adjust"]
pub type ThrescoarseR = crate::FieldReader;
#[doc = "Field `THRESCOARSE` writer - Threshold Coarse Adjust"]
pub type ThrescoarseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Rise Wakeup"]
    #[inline(always)]
    pub fn risewu(&self) -> RisewuR {
        RisewuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fall Wakeup"]
    #[inline(always)]
    pub fn fallwu(&self) -> FallwuR {
        FallwuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Threshold Fine Adjust"]
    #[inline(always)]
    pub fn thresfine(&self) -> ThresfineR {
        ThresfineR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn threscoarse(&self) -> ThrescoarseR {
        ThrescoarseR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<VMONDVDDCTRLrs> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 2 - Rise Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn risewu(&mut self) -> RisewuW<VMONDVDDCTRLrs> {
        RisewuW::new(self, 2)
    }
    #[doc = "Bit 3 - Fall Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn fallwu(&mut self) -> FallwuW<VMONDVDDCTRLrs> {
        FallwuW::new(self, 3)
    }
    #[doc = "Bits 8:11 - Threshold Fine Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn thresfine(&mut self) -> ThresfineW<VMONDVDDCTRLrs> {
        ThresfineW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Threshold Coarse Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn threscoarse(&mut self) -> ThrescoarseW<VMONDVDDCTRLrs> {
        ThrescoarseW::new(self, 12)
    }
}
#[doc = "VMON DVDD Channel Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmondvddctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmondvddctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMONDVDDCTRLrs;
impl crate::RegisterSpec for VMONDVDDCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmondvddctrl::R`](R) reader structure"]
impl crate::Readable for VMONDVDDCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`vmondvddctrl::W`](W) writer structure"]
impl crate::Writable for VMONDVDDCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VMONDVDDCTRL to value 0"]
impl crate::Resettable for VMONDVDDCTRLrs {
    const RESET_VALUE: u32 = 0;
}
