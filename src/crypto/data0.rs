#[doc = "Register `DATA0` reader"]
pub type R = crate::R<DATA0rs>;
#[doc = "Register `DATA0` writer"]
pub type W = crate::W<DATA0rs>;
#[doc = "Field `DATA0` reader - Data 0 Access"]
pub type DATA0_R = crate::FieldReader<u32>;
#[doc = "Field `DATA0` writer - Data 0 Access"]
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data 0 Access"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data 0 Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<DATA0rs> {
        DATA0_W::new(self, 0)
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
#[doc = "DATA0 Register Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA0rs;
impl crate::RegisterSpec for DATA0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0::R`](R) reader structure"]
impl crate::Readable for DATA0rs {}
#[doc = "`write(|w| ..)` method takes [`data0::W`](W) writer structure"]
impl crate::Writable for DATA0rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA0 to value 0"]
impl crate::Resettable for DATA0rs {
    const RESET_VALUE: u32 = 0;
}
