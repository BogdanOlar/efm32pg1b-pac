#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `ADDRFAULTEN` reader - Invalid Address Bus Fault Response Enable"]
pub type ADDRFAULTEN_R = crate::BitReader;
#[doc = "Field `ADDRFAULTEN` writer - Invalid Address Bus Fault Response Enable"]
pub type ADDRFAULTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKDISFAULTEN` reader - Clock-disabled Bus Fault Response Enable"]
pub type CLKDISFAULTEN_R = crate::BitReader;
#[doc = "Field `CLKDISFAULTEN` writer - Clock-disabled Bus Fault Response Enable"]
pub type CLKDISFAULTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWRUPONDEMAND` reader - Power Up on Demand During Wake Up"]
pub type PWRUPONDEMAND_R = crate::BitReader;
#[doc = "Field `PWRUPONDEMAND` writer - Power Up on Demand During Wake Up"]
pub type PWRUPONDEMAND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IFCREADCLEAR` reader - IFC Read Clears IF"]
pub type IFCREADCLEAR_R = crate::BitReader;
#[doc = "Field `IFCREADCLEAR` writer - IFC Read Clears IF"]
pub type IFCREADCLEAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enable"]
    #[inline(always)]
    pub fn addrfaulten(&self) -> ADDRFAULTEN_R {
        ADDRFAULTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock-disabled Bus Fault Response Enable"]
    #[inline(always)]
    pub fn clkdisfaulten(&self) -> CLKDISFAULTEN_R {
        CLKDISFAULTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Up on Demand During Wake Up"]
    #[inline(always)]
    pub fn pwrupondemand(&self) -> PWRUPONDEMAND_R {
        PWRUPONDEMAND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IFC Read Clears IF"]
    #[inline(always)]
    pub fn ifcreadclear(&self) -> IFCREADCLEAR_R {
        IFCREADCLEAR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("addrfaulten", &format_args!("{}", self.addrfaulten().bit()))
            .field(
                "clkdisfaulten",
                &format_args!("{}", self.clkdisfaulten().bit()),
            )
            .field(
                "pwrupondemand",
                &format_args!("{}", self.pwrupondemand().bit()),
            )
            .field(
                "ifcreadclear",
                &format_args!("{}", self.ifcreadclear().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enable"]
    #[inline(always)]
    #[must_use]
    pub fn addrfaulten(&mut self) -> ADDRFAULTEN_W<CTRL_SPEC, 0> {
        ADDRFAULTEN_W::new(self)
    }
    #[doc = "Bit 1 - Clock-disabled Bus Fault Response Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkdisfaulten(&mut self) -> CLKDISFAULTEN_W<CTRL_SPEC, 1> {
        CLKDISFAULTEN_W::new(self)
    }
    #[doc = "Bit 2 - Power Up on Demand During Wake Up"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupondemand(&mut self) -> PWRUPONDEMAND_W<CTRL_SPEC, 2> {
        PWRUPONDEMAND_W::new(self)
    }
    #[doc = "Bit 3 - IFC Read Clears IF"]
    #[inline(always)]
    #[must_use]
    pub fn ifcreadclear(&mut self) -> IFCREADCLEAR_W<CTRL_SPEC, 3> {
        IFCREADCLEAR_W::new(self)
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
#[doc = "Memory System Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x01"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
