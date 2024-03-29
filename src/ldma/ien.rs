#[doc = "Register `IEN` reader"]
pub type R = crate::R<IENrs>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IENrs>;
#[doc = "Field `DONE` reader - DONE Interrupt Enable"]
pub type DONE_R = crate::FieldReader;
#[doc = "Field `DONE` writer - DONE Interrupt Enable"]
pub type DONE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ERROR` reader - ERROR Interrupt Enable"]
pub type ERROR_R = crate::BitReader;
#[doc = "Field `ERROR` writer - ERROR Interrupt Enable"]
pub type ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - DONE Interrupt Enable"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - ERROR Interrupt Enable"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DONE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<IENrs> {
        DONE_W::new(self, 0)
    }
    #[doc = "Bit 31 - ERROR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<IENrs> {
        ERROR_W::new(self, 31)
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
