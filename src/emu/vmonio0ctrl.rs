///Register `VMONIO0CTRL` reader
pub type R = crate::R<VMONIO0CTRLrs>;
///Register `VMONIO0CTRL` writer
pub type W = crate::W<VMONIO0CTRLrs>;
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
///Field `RETDIS` reader - EM4 IO0 Retention Disable
pub type RetdisR = crate::BitReader;
///Field `RETDIS` writer - EM4 IO0 Retention Disable
pub type RetdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THRESFINE` reader - Threshold Fine Adjust
pub type ThresfineR = crate::FieldReader;
///Field `THRESFINE` writer - Threshold Fine Adjust
pub type ThresfineW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `THRESCOARSE` reader - Threshold Coarse Adjust
pub type ThrescoarseR = crate::FieldReader;
///Field `THRESCOARSE` writer - Threshold Coarse Adjust
pub type ThrescoarseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    ///Bit 4 - EM4 IO0 Retention Disable
    #[inline(always)]
    pub fn retdis(&self) -> RetdisR {
        RetdisR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - Threshold Fine Adjust
    #[inline(always)]
    pub fn thresfine(&self) -> ThresfineR {
        ThresfineR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Threshold Coarse Adjust
    #[inline(always)]
    pub fn threscoarse(&self) -> ThrescoarseR {
        ThrescoarseR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMONIO0CTRL")
            .field("en", &self.en())
            .field("risewu", &self.risewu())
            .field("fallwu", &self.fallwu())
            .field("retdis", &self.retdis())
            .field("thresfine", &self.thresfine())
            .field("threscoarse", &self.threscoarse())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<VMONIO0CTRLrs> {
        EnW::new(self, 0)
    }
    ///Bit 2 - Rise Wakeup
    #[inline(always)]
    #[must_use]
    pub fn risewu(&mut self) -> RisewuW<VMONIO0CTRLrs> {
        RisewuW::new(self, 2)
    }
    ///Bit 3 - Fall Wakeup
    #[inline(always)]
    #[must_use]
    pub fn fallwu(&mut self) -> FallwuW<VMONIO0CTRLrs> {
        FallwuW::new(self, 3)
    }
    ///Bit 4 - EM4 IO0 Retention Disable
    #[inline(always)]
    #[must_use]
    pub fn retdis(&mut self) -> RetdisW<VMONIO0CTRLrs> {
        RetdisW::new(self, 4)
    }
    ///Bits 8:11 - Threshold Fine Adjust
    #[inline(always)]
    #[must_use]
    pub fn thresfine(&mut self) -> ThresfineW<VMONIO0CTRLrs> {
        ThresfineW::new(self, 8)
    }
    ///Bits 12:15 - Threshold Coarse Adjust
    #[inline(always)]
    #[must_use]
    pub fn threscoarse(&mut self) -> ThrescoarseW<VMONIO0CTRLrs> {
        ThrescoarseW::new(self, 12)
    }
}
///VMON IOVDD0 Channel Control
///
///You can [`read`](crate::Reg::read) this register and get [`vmonio0ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmonio0ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VMONIO0CTRLrs;
impl crate::RegisterSpec for VMONIO0CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`vmonio0ctrl::R`](R) reader structure
impl crate::Readable for VMONIO0CTRLrs {}
///`write(|w| ..)` method takes [`vmonio0ctrl::W`](W) writer structure
impl crate::Writable for VMONIO0CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VMONIO0CTRL to value 0
impl crate::Resettable for VMONIO0CTRLrs {
    const RESET_VALUE: u32 = 0;
}
