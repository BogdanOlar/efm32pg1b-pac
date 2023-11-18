#[doc = "Register `READCTRL` reader"]
pub type R = crate::R<READCTRL_SPEC>;
#[doc = "Register `READCTRL` writer"]
pub type W = crate::W<READCTRL_SPEC>;
#[doc = "Field `IFCDIS` reader - Internal Flash Cache Disable"]
pub type IFCDIS_R = crate::BitReader;
#[doc = "Field `IFCDIS` writer - Internal Flash Cache Disable"]
pub type IFCDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AIDIS` reader - Automatic Invalidate Disable"]
pub type AIDIS_R = crate::BitReader;
#[doc = "Field `AIDIS` writer - Automatic Invalidate Disable"]
pub type AIDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICCDIS` reader - Interrupt Context Cache Disable"]
pub type ICCDIS_R = crate::BitReader;
#[doc = "Field `ICCDIS` writer - Interrupt Context Cache Disable"]
pub type ICCDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PREFETCH` reader - Prefetch Mode"]
pub type PREFETCH_R = crate::BitReader;
#[doc = "Field `PREFETCH` writer - Prefetch Mode"]
pub type PREFETCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USEHPROT` reader - AHB_HPROT Mode"]
pub type USEHPROT_R = crate::BitReader;
#[doc = "Field `USEHPROT` writer - AHB_HPROT Mode"]
pub type USEHPROT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODE` reader - Read Mode"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Read Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Zero wait-states inserted in fetch or read transfers"]
    WS0 = 0,
    #[doc = "1: One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    WS1 = 1,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::WS0),
            1 => Some(MODE_A::WS1),
            _ => None,
        }
    }
    #[doc = "Zero wait-states inserted in fetch or read transfers"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == MODE_A::WS0
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == MODE_A::WS1
    }
}
#[doc = "Field `MODE` writer - Read Mode"]
pub type MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, MODE_A>;
impl<'a, REG, const O: u8> MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zero wait-states inserted in fetch or read transfers"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::WS0)
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::WS1)
    }
}
#[doc = "Field `SCBTP` reader - Suppress Conditional Branch Target Perfetch"]
pub type SCBTP_R = crate::BitReader;
#[doc = "Field `SCBTP` writer - Suppress Conditional Branch Target Perfetch"]
pub type SCBTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    pub fn ifcdis(&self) -> IFCDIS_R {
        IFCDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    pub fn aidis(&self) -> AIDIS_R {
        AIDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Context Cache Disable"]
    #[inline(always)]
    pub fn iccdis(&self) -> ICCDIS_R {
        ICCDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Prefetch Mode"]
    #[inline(always)]
    pub fn prefetch(&self) -> PREFETCH_R {
        PREFETCH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AHB_HPROT Mode"]
    #[inline(always)]
    pub fn usehprot(&self) -> USEHPROT_R {
        USEHPROT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Read Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Suppress Conditional Branch Target Perfetch"]
    #[inline(always)]
    pub fn scbtp(&self) -> SCBTP_R {
        SCBTP_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("READCTRL")
            .field("ifcdis", &format_args!("{}", self.ifcdis().bit()))
            .field("aidis", &format_args!("{}", self.aidis().bit()))
            .field("iccdis", &format_args!("{}", self.iccdis().bit()))
            .field("prefetch", &format_args!("{}", self.prefetch().bit()))
            .field("usehprot", &format_args!("{}", self.usehprot().bit()))
            .field("mode", &format_args!("{}", self.mode().bits()))
            .field("scbtp", &format_args!("{}", self.scbtp().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<READCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ifcdis(&mut self) -> IFCDIS_W<READCTRL_SPEC, 3> {
        IFCDIS_W::new(self)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aidis(&mut self) -> AIDIS_W<READCTRL_SPEC, 4> {
        AIDIS_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Context Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn iccdis(&mut self) -> ICCDIS_W<READCTRL_SPEC, 5> {
        ICCDIS_W::new(self)
    }
    #[doc = "Bit 8 - Prefetch Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prefetch(&mut self) -> PREFETCH_W<READCTRL_SPEC, 8> {
        PREFETCH_W::new(self)
    }
    #[doc = "Bit 9 - AHB_HPROT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usehprot(&mut self) -> USEHPROT_W<READCTRL_SPEC, 9> {
        USEHPROT_W::new(self)
    }
    #[doc = "Bits 24:25 - Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<READCTRL_SPEC, 24> {
        MODE_W::new(self)
    }
    #[doc = "Bit 28 - Suppress Conditional Branch Target Perfetch"]
    #[inline(always)]
    #[must_use]
    pub fn scbtp(&mut self) -> SCBTP_W<READCTRL_SPEC, 28> {
        SCBTP_W::new(self)
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
#[doc = "Read Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`readctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READCTRL_SPEC;
impl crate::RegisterSpec for READCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readctrl::R`](R) reader structure"]
impl crate::Readable for READCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`readctrl::W`](W) writer structure"]
impl crate::Writable for READCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets READCTRL to value 0x0100_0100"]
impl crate::Resettable for READCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0100;
}
