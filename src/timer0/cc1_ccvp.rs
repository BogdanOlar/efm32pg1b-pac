#[doc = "Register `CC1_CCVP` reader"]
pub type R = crate::R<CC1_CCVPrs>;
#[doc = "Field `CCVP` reader - CC Channel Value Peek"]
pub type CCVP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value Peek"]
    #[inline(always)]
    pub fn ccvp(&self) -> CCVP_R {
        CCVP_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CC Channel Value Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc1_ccvp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC1_CCVPrs;
impl crate::RegisterSpec for CC1_CCVPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc1_ccvp::R`](R) reader structure"]
impl crate::Readable for CC1_CCVPrs {}
#[doc = "`reset()` method sets CC1_CCVP to value 0"]
impl crate::Resettable for CC1_CCVPrs {
    const RESET_VALUE: u32 = 0;
}
