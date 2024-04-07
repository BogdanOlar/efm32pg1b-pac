#[doc = "Register `PC_DOUTTGL` writer"]
pub type W = crate::W<PC_DOUTTGLrs>;
#[doc = "Field `DOUTTGL` writer - Data Out Toggle"]
pub type DouttglW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl(&mut self) -> DouttglW<PC_DOUTTGLrs> {
        DouttglW::new(self, 0)
    }
}
#[doc = "Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_douttgl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC_DOUTTGLrs;
impl crate::RegisterSpec for PC_DOUTTGLrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pc_douttgl::W`](W) writer structure"]
impl crate::Writable for PC_DOUTTGLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC_DOUTTGL to value 0"]
impl crate::Resettable for PC_DOUTTGLrs {
    const RESET_VALUE: u32 = 0;
}
