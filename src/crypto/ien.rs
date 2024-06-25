#[doc = "Register `IEN` reader"]
pub type R = crate::R<IENrs>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IENrs>;
#[doc = "Field `INSTRDONE` reader - INSTRDONE Interrupt Enable"]
pub type InstrdoneR = crate::BitReader;
#[doc = "Field `INSTRDONE` writer - INSTRDONE Interrupt Enable"]
pub type InstrdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQDONE` reader - SEQDONE Interrupt Enable"]
pub type SeqdoneR = crate::BitReader;
#[doc = "Field `SEQDONE` writer - SEQDONE Interrupt Enable"]
pub type SeqdoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - INSTRDONE Interrupt Enable"]
    #[inline(always)]
    pub fn instrdone(&self) -> InstrdoneR {
        InstrdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SEQDONE Interrupt Enable"]
    #[inline(always)]
    pub fn seqdone(&self) -> SeqdoneR {
        SeqdoneR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("instrdone", &self.instrdone())
            .field("seqdone", &self.seqdone())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - INSTRDONE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn instrdone(&mut self) -> InstrdoneW<IENrs> {
        InstrdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - SEQDONE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn seqdone(&mut self) -> SeqdoneW<IENrs> {
        SeqdoneW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IENrs {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
