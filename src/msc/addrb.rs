#[doc = "Register `ADDRB` reader"]
pub type R = crate::R<ADDRBrs>;
#[doc = "Register `ADDRB` writer"]
pub type W = crate::W<ADDRBrs>;
#[doc = "Field `ADDRB` reader - Page Erase or Write Address Buffer"]
pub type ADDRB_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRB` writer - Page Erase or Write Address Buffer"]
pub type ADDRB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Page Erase or Write Address Buffer"]
    #[inline(always)]
    pub fn addrb(&self) -> ADDRB_R {
        ADDRB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Page Erase or Write Address Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn addrb(&mut self) -> ADDRB_W<ADDRBrs> {
        ADDRB_W::new(self, 0)
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
#[doc = "Page Erase/Write Address Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDRBrs;
impl crate::RegisterSpec for ADDRBrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addrb::R`](R) reader structure"]
impl crate::Readable for ADDRBrs {}
#[doc = "`write(|w| ..)` method takes [`addrb::W`](W) writer structure"]
impl crate::Writable for ADDRBrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDRB to value 0"]
impl crate::Resettable for ADDRBrs {
    const RESET_VALUE: u32 = 0;
}
