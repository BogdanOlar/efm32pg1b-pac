#[doc = "Register `SADDRMASK` reader"]
pub type R = crate::R<SADDRMASKrs>;
#[doc = "Register `SADDRMASK` writer"]
pub type W = crate::W<SADDRMASKrs>;
#[doc = "Field `MASK` reader - Slave Address Mask"]
pub type MASK_R = crate::FieldReader;
#[doc = "Field `MASK` writer - Slave Address Mask"]
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Slave Address Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<SADDRMASKrs> {
        MASK_W::new(self, 1)
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
#[doc = "Slave Address Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddrmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddrmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SADDRMASKrs;
impl crate::RegisterSpec for SADDRMASKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddrmask::R`](R) reader structure"]
impl crate::Readable for SADDRMASKrs {}
#[doc = "`write(|w| ..)` method takes [`saddrmask::W`](W) writer structure"]
impl crate::Writable for SADDRMASKrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SADDRMASK to value 0"]
impl crate::Resettable for SADDRMASKrs {
    const RESET_VALUE: u32 = 0;
}
