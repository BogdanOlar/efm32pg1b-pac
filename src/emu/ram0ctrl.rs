#[doc = "Register `RAM0CTRL` reader"]
pub type R = crate::R<RAM0CTRL_SPEC>;
#[doc = "Register `RAM0CTRL` writer"]
pub type W = crate::W<RAM0CTRL_SPEC>;
#[doc = "Field `RAMPOWERDOWN` reader - RAM0 Blockset Power-down"]
pub type RAMPOWERDOWN_R = crate::FieldReader<RAMPOWERDOWN_A>;
#[doc = "RAM0 Blockset Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMPOWERDOWN_A {
    #[doc = "0: None of the RAM blocks powered down"]
    NONE = 0,
    #[doc = "8: Power down RAM blocks 4 and above"]
    BLK4 = 8,
    #[doc = "12: Power down RAM blocks 3 and above"]
    BLK3TO4 = 12,
    #[doc = "14: Power down RAM blocks 2 and above"]
    BLK2TO4 = 14,
    #[doc = "15: Power down RAM blocks 1 and above"]
    BLK1TO4 = 15,
}
impl From<RAMPOWERDOWN_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPOWERDOWN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RAMPOWERDOWN_A {
    type Ux = u8;
}
impl RAMPOWERDOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RAMPOWERDOWN_A> {
        match self.bits {
            0 => Some(RAMPOWERDOWN_A::NONE),
            8 => Some(RAMPOWERDOWN_A::BLK4),
            12 => Some(RAMPOWERDOWN_A::BLK3TO4),
            14 => Some(RAMPOWERDOWN_A::BLK2TO4),
            15 => Some(RAMPOWERDOWN_A::BLK1TO4),
            _ => None,
        }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RAMPOWERDOWN_A::NONE
    }
    #[doc = "Power down RAM blocks 4 and above"]
    #[inline(always)]
    pub fn is_blk4(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK4
    }
    #[doc = "Power down RAM blocks 3 and above"]
    #[inline(always)]
    pub fn is_blk3to4(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK3TO4
    }
    #[doc = "Power down RAM blocks 2 and above"]
    #[inline(always)]
    pub fn is_blk2to4(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK2TO4
    }
    #[doc = "Power down RAM blocks 1 and above"]
    #[inline(always)]
    pub fn is_blk1to4(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK1TO4
    }
}
#[doc = "Field `RAMPOWERDOWN` writer - RAM0 Blockset Power-down"]
pub type RAMPOWERDOWN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, RAMPOWERDOWN_A>;
impl<'a, REG, const O: u8> RAMPOWERDOWN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::NONE)
    }
    #[doc = "Power down RAM blocks 4 and above"]
    #[inline(always)]
    pub fn blk4(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK4)
    }
    #[doc = "Power down RAM blocks 3 and above"]
    #[inline(always)]
    pub fn blk3to4(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK3TO4)
    }
    #[doc = "Power down RAM blocks 2 and above"]
    #[inline(always)]
    pub fn blk2to4(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK2TO4)
    }
    #[doc = "Power down RAM blocks 1 and above"]
    #[inline(always)]
    pub fn blk1to4(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK1TO4)
    }
}
impl R {
    #[doc = "Bits 0:3 - RAM0 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RAMPOWERDOWN_R {
        RAMPOWERDOWN_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM0CTRL")
            .field(
                "rampowerdown",
                &format_args!("{}", self.rampowerdown().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RAM0CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - RAM0 Blockset Power-down"]
    #[inline(always)]
    #[must_use]
    pub fn rampowerdown(&mut self) -> RAMPOWERDOWN_W<RAM0CTRL_SPEC, 0> {
        RAMPOWERDOWN_W::new(self)
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
#[doc = "Memory Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram0ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram0ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM0CTRL_SPEC;
impl crate::RegisterSpec for RAM0CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram0ctrl::R`](R) reader structure"]
impl crate::Readable for RAM0CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ram0ctrl::W`](W) writer structure"]
impl crate::Writable for RAM0CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAM0CTRL to value 0"]
impl crate::Resettable for RAM0CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
