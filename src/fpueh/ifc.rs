#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFCrs>;
#[doc = "Field `FPIOC` writer - Clear FPIOC Interrupt Flag"]
pub type FPIOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPDZC` writer - Clear FPDZC Interrupt Flag"]
pub type FPDZC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPUFC` writer - Clear FPUFC Interrupt Flag"]
pub type FPUFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPOFC` writer - Clear FPOFC Interrupt Flag"]
pub type FPOFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIDC` writer - Clear FPIDC Interrupt Flag"]
pub type FPIDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIXC` writer - Clear FPIXC Interrupt Flag"]
pub type FPIXC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear FPIOC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpioc(&mut self) -> FPIOC_W<IFCrs> {
        FPIOC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear FPDZC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpdzc(&mut self) -> FPDZC_W<IFCrs> {
        FPDZC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear FPUFC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpufc(&mut self) -> FPUFC_W<IFCrs> {
        FPUFC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear FPOFC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpofc(&mut self) -> FPOFC_W<IFCrs> {
        FPOFC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear FPIDC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpidc(&mut self) -> FPIDC_W<IFCrs> {
        FPIDC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear FPIXC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpixc(&mut self) -> FPIXC_W<IFCrs> {
        FPIXC_W::new(self, 5)
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
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCrs;
impl crate::RegisterSpec for IFCrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFCrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFCrs {
    const RESET_VALUE: u32 = 0;
}
