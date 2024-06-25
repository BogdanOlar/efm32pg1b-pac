#[doc = "Register `DATA0` reader"]
pub type R = crate::R<DATA0rs>;
#[doc = "Register `DATA0` writer"]
pub type W = crate::W<DATA0rs>;
#[doc = "Field `DATA0` reader - Data 0 Access"]
pub type Data0R = crate::FieldReader<u32>;
#[doc = "Field `DATA0` writer - Data 0 Access"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data 0 Access"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new(self.bits)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DATA0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Data 0 Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> Data0W<DATA0rs> {
        Data0W::new(self, 0)
    }
}
#[doc = "DATA0 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct DATA0rs;
impl crate::RegisterSpec for DATA0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0::R`](R) reader structure"]
impl crate::Readable for DATA0rs {}
#[doc = "`write(|w| ..)` method takes [`data0::W`](W) writer structure"]
impl crate::Writable for DATA0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA0 to value 0"]
impl crate::Resettable for DATA0rs {
    const RESET_VALUE: u32 = 0;
}
