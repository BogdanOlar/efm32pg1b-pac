#[doc = "Register `POLY` reader"]
pub type R = crate::R<POLYrs>;
#[doc = "Register `POLY` writer"]
pub type W = crate::W<POLYrs>;
#[doc = "Field `POLY` reader - CRC Polynomial Value"]
pub type POLY_R = crate::FieldReader<u16>;
#[doc = "Field `POLY` writer - CRC Polynomial Value"]
pub type POLY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC Polynomial Value"]
    #[inline(always)]
    pub fn poly(&self) -> POLY_R {
        POLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC Polynomial Value"]
    #[inline(always)]
    #[must_use]
    pub fn poly(&mut self) -> POLY_W<POLYrs> {
        POLY_W::new(self, 0)
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
#[doc = "CRC Polynomial Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`poly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`poly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POLYrs;
impl crate::RegisterSpec for POLYrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`poly::R`](R) reader structure"]
impl crate::Readable for POLYrs {}
#[doc = "`write(|w| ..)` method takes [`poly::W`](W) writer structure"]
impl crate::Writable for POLYrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POLY to value 0"]
impl crate::Resettable for POLYrs {
    const RESET_VALUE: u32 = 0;
}
