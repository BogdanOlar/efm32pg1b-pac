///Register `VMONAVDDCTRL` reader
pub type R = crate::R<VMONAVDDCTRLrs>;
///Register `VMONAVDDCTRL` writer
pub type W = crate::W<VMONAVDDCTRLrs>;
///Field `EN` reader - Enable
pub type EnR = crate::BitReader;
///Field `EN` writer - Enable
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RISEWU` reader - Rise Wakeup
pub type RisewuR = crate::BitReader;
///Field `RISEWU` writer - Rise Wakeup
pub type RisewuW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FALLWU` reader - Fall Wakeup
pub type FallwuR = crate::BitReader;
///Field `FALLWU` writer - Fall Wakeup
pub type FallwuW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FALLTHRESFINE` reader - Falling Threshold Fine Adjust
pub type FallthresfineR = crate::FieldReader;
///Field `FALLTHRESFINE` writer - Falling Threshold Fine Adjust
pub type FallthresfineW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FALLTHRESCOARSE` reader - Falling Threshold Coarse Adjust
pub type FallthrescoarseR = crate::FieldReader;
///Field `FALLTHRESCOARSE` writer - Falling Threshold Coarse Adjust
pub type FallthrescoarseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RISETHRESFINE` reader - Rising Threshold Fine Adjust
pub type RisethresfineR = crate::FieldReader;
///Field `RISETHRESFINE` writer - Rising Threshold Fine Adjust
pub type RisethresfineW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RISETHRESCOARSE` reader - Rising Threshold Coarse Adjust
pub type RisethrescoarseR = crate::FieldReader;
///Field `RISETHRESCOARSE` writer - Rising Threshold Coarse Adjust
pub type RisethrescoarseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Rise Wakeup
    #[inline(always)]
    pub fn risewu(&self) -> RisewuR {
        RisewuR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Fall Wakeup
    #[inline(always)]
    pub fn fallwu(&self) -> FallwuR {
        FallwuR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:11 - Falling Threshold Fine Adjust
    #[inline(always)]
    pub fn fallthresfine(&self) -> FallthresfineR {
        FallthresfineR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Falling Threshold Coarse Adjust
    #[inline(always)]
    pub fn fallthrescoarse(&self) -> FallthrescoarseR {
        FallthrescoarseR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Rising Threshold Fine Adjust
    #[inline(always)]
    pub fn risethresfine(&self) -> RisethresfineR {
        RisethresfineR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Rising Threshold Coarse Adjust
    #[inline(always)]
    pub fn risethrescoarse(&self) -> RisethrescoarseR {
        RisethrescoarseR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMONAVDDCTRL")
            .field("en", &self.en())
            .field("risewu", &self.risewu())
            .field("fallwu", &self.fallwu())
            .field("fallthresfine", &self.fallthresfine())
            .field("fallthrescoarse", &self.fallthrescoarse())
            .field("risethresfine", &self.risethresfine())
            .field("risethrescoarse", &self.risethrescoarse())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<VMONAVDDCTRLrs> {
        EnW::new(self, 0)
    }
    ///Bit 2 - Rise Wakeup
    #[inline(always)]
    #[must_use]
    pub fn risewu(&mut self) -> RisewuW<VMONAVDDCTRLrs> {
        RisewuW::new(self, 2)
    }
    ///Bit 3 - Fall Wakeup
    #[inline(always)]
    #[must_use]
    pub fn fallwu(&mut self) -> FallwuW<VMONAVDDCTRLrs> {
        FallwuW::new(self, 3)
    }
    ///Bits 8:11 - Falling Threshold Fine Adjust
    #[inline(always)]
    #[must_use]
    pub fn fallthresfine(&mut self) -> FallthresfineW<VMONAVDDCTRLrs> {
        FallthresfineW::new(self, 8)
    }
    ///Bits 12:15 - Falling Threshold Coarse Adjust
    #[inline(always)]
    #[must_use]
    pub fn fallthrescoarse(&mut self) -> FallthrescoarseW<VMONAVDDCTRLrs> {
        FallthrescoarseW::new(self, 12)
    }
    ///Bits 16:19 - Rising Threshold Fine Adjust
    #[inline(always)]
    #[must_use]
    pub fn risethresfine(&mut self) -> RisethresfineW<VMONAVDDCTRLrs> {
        RisethresfineW::new(self, 16)
    }
    ///Bits 20:23 - Rising Threshold Coarse Adjust
    #[inline(always)]
    #[must_use]
    pub fn risethrescoarse(&mut self) -> RisethrescoarseW<VMONAVDDCTRLrs> {
        RisethrescoarseW::new(self, 20)
    }
}
///VMON AVDD Channel Control
///
///You can [`read`](crate::Reg::read) this register and get [`vmonavddctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmonavddctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VMONAVDDCTRLrs;
impl crate::RegisterSpec for VMONAVDDCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`vmonavddctrl::R`](R) reader structure
impl crate::Readable for VMONAVDDCTRLrs {}
///`write(|w| ..)` method takes [`vmonavddctrl::W`](W) writer structure
impl crate::Writable for VMONAVDDCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VMONAVDDCTRL to value 0
impl crate::Resettable for VMONAVDDCTRLrs {
    const RESET_VALUE: u32 = 0;
}
