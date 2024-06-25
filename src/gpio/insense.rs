#[doc = "Register `INSENSE` reader"]
pub type R = crate::R<INSENSErs>;
#[doc = "Register `INSENSE` writer"]
pub type W = crate::W<INSENSErs>;
#[doc = "Field `INT` reader - Interrupt Sense Enable"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - Interrupt Sense Enable"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU` reader - EM4WU Interrupt Sense Enable"]
pub type Em4wuR = crate::BitReader;
#[doc = "Field `EM4WU` writer - EM4WU Interrupt Sense Enable"]
pub type Em4wuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Sense Enable"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EM4WU Interrupt Sense Enable"]
    #[inline(always)]
    pub fn em4wu(&self) -> Em4wuR {
        Em4wuR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INSENSE")
            .field("int", &self.int())
            .field("em4wu", &self.em4wu())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Sense Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<INSENSErs> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - EM4WU Interrupt Sense Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu(&mut self) -> Em4wuW<INSENSErs> {
        Em4wuW::new(self, 1)
    }
}
#[doc = "Input Sense Register\n\nYou can [`read`](crate::Reg::read) this register and get [`insense::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`insense::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INSENSErs;
impl crate::RegisterSpec for INSENSErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`insense::R`](R) reader structure"]
impl crate::Readable for INSENSErs {}
#[doc = "`write(|w| ..)` method takes [`insense::W`](W) writer structure"]
impl crate::Writable for INSENSErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INSENSE to value 0x03"]
impl crate::Resettable for INSENSErs {
    const RESET_VALUE: u32 = 0x03;
}
