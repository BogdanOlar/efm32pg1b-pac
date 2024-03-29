#[doc = "Register `COMP0` reader"]
pub type R = crate::R<COMP0rs>;
#[doc = "Register `COMP0` writer"]
pub type W = crate::W<COMP0rs>;
#[doc = "Field `COMP0` reader - Compare Value 0"]
pub type COMP0_R = crate::FieldReader<u16>;
#[doc = "Field `COMP0` writer - Compare Value 0"]
pub type COMP0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare Value 0"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Value 0"]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> COMP0_W<COMP0rs> {
        COMP0_W::new(self, 0)
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
#[doc = "Compare Value Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP0rs;
impl crate::RegisterSpec for COMP0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0::R`](R) reader structure"]
impl crate::Readable for COMP0rs {}
#[doc = "`write(|w| ..)` method takes [`comp0::W`](W) writer structure"]
impl crate::Writable for COMP0rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP0 to value 0"]
impl crate::Resettable for COMP0rs {
    const RESET_VALUE: u32 = 0;
}
