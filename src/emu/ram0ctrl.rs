#[doc = "Register `RAM0CTRL` reader"]
pub type R = crate::R<RAM0CTRLrs>;
#[doc = "Register `RAM0CTRL` writer"]
pub type W = crate::W<RAM0CTRLrs>;
#[doc = "RAM0 Blockset Power-down\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMPOWERDOWN {
    #[doc = "0: None of the RAM blocks powered down"]
    None = 0,
    #[doc = "8: Power down RAM blocks 4 and above"]
    Blk4 = 8,
    #[doc = "12: Power down RAM blocks 3 and above"]
    Blk3to4 = 12,
    #[doc = "14: Power down RAM blocks 2 and above"]
    Blk2to4 = 14,
    #[doc = "15: Power down RAM blocks 1 and above"]
    Blk1to4 = 15,
}
impl From<RAMPOWERDOWN> for u8 {
    #[inline(always)]
    fn from(variant: RAMPOWERDOWN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RAMPOWERDOWN {
    type Ux = u8;
}
impl crate::IsEnum for RAMPOWERDOWN {}
#[doc = "Field `RAMPOWERDOWN` reader - RAM0 Blockset Power-down"]
pub type RampowerdownR = crate::FieldReader<RAMPOWERDOWN>;
impl RampowerdownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RAMPOWERDOWN> {
        match self.bits {
            0 => Some(RAMPOWERDOWN::None),
            8 => Some(RAMPOWERDOWN::Blk4),
            12 => Some(RAMPOWERDOWN::Blk3to4),
            14 => Some(RAMPOWERDOWN::Blk2to4),
            15 => Some(RAMPOWERDOWN::Blk1to4),
            _ => None,
        }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RAMPOWERDOWN::None
    }
    #[doc = "Power down RAM blocks 4 and above"]
    #[inline(always)]
    pub fn is_blk4(&self) -> bool {
        *self == RAMPOWERDOWN::Blk4
    }
    #[doc = "Power down RAM blocks 3 and above"]
    #[inline(always)]
    pub fn is_blk3to4(&self) -> bool {
        *self == RAMPOWERDOWN::Blk3to4
    }
    #[doc = "Power down RAM blocks 2 and above"]
    #[inline(always)]
    pub fn is_blk2to4(&self) -> bool {
        *self == RAMPOWERDOWN::Blk2to4
    }
    #[doc = "Power down RAM blocks 1 and above"]
    #[inline(always)]
    pub fn is_blk1to4(&self) -> bool {
        *self == RAMPOWERDOWN::Blk1to4
    }
}
#[doc = "Field `RAMPOWERDOWN` writer - RAM0 Blockset Power-down"]
pub type RampowerdownW<'a, REG> = crate::FieldWriter<'a, REG, 4, RAMPOWERDOWN>;
impl<'a, REG> RampowerdownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN::None)
    }
    #[doc = "Power down RAM blocks 4 and above"]
    #[inline(always)]
    pub fn blk4(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN::Blk4)
    }
    #[doc = "Power down RAM blocks 3 and above"]
    #[inline(always)]
    pub fn blk3to4(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN::Blk3to4)
    }
    #[doc = "Power down RAM blocks 2 and above"]
    #[inline(always)]
    pub fn blk2to4(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN::Blk2to4)
    }
    #[doc = "Power down RAM blocks 1 and above"]
    #[inline(always)]
    pub fn blk1to4(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN::Blk1to4)
    }
}
impl R {
    #[doc = "Bits 0:3 - RAM0 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RampowerdownR {
        RampowerdownR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM0CTRL")
            .field("rampowerdown", &self.rampowerdown())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - RAM0 Blockset Power-down"]
    #[inline(always)]
    #[must_use]
    pub fn rampowerdown(&mut self) -> RampowerdownW<RAM0CTRLrs> {
        RampowerdownW::new(self, 0)
    }
}
#[doc = "Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram0ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram0ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM0CTRLrs;
impl crate::RegisterSpec for RAM0CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram0ctrl::R`](R) reader structure"]
impl crate::Readable for RAM0CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ram0ctrl::W`](W) writer structure"]
impl crate::Writable for RAM0CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAM0CTRL to value 0"]
impl crate::Resettable for RAM0CTRLrs {
    const RESET_VALUE: u32 = 0;
}
