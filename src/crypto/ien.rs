#[doc = "Register `IEN` reader"]
pub type R = crate::R<IENrs>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IENrs>;
#[doc = "Field `INSTRDONE` reader - INSTRDONE Interrupt Enable"]
pub type INSTRDONE_R = crate::BitReader;
#[doc = "Field `INSTRDONE` writer - INSTRDONE Interrupt Enable"]
pub type INSTRDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQDONE` reader - SEQDONE Interrupt Enable"]
pub type SEQDONE_R = crate::BitReader;
#[doc = "Field `SEQDONE` writer - SEQDONE Interrupt Enable"]
pub type SEQDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - INSTRDONE Interrupt Enable"]
    #[inline(always)]
    pub fn instrdone(&self) -> INSTRDONE_R {
        INSTRDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SEQDONE Interrupt Enable"]
    #[inline(always)]
    pub fn seqdone(&self) -> SEQDONE_R {
        SEQDONE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INSTRDONE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn instrdone(&mut self) -> INSTRDONE_W<IENrs> {
        INSTRDONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - SEQDONE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn seqdone(&mut self) -> SEQDONE_W<IENrs> {
        SEQDONE_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IENrs {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IENrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
