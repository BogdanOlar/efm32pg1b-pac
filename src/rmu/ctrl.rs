#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "WDOG Reset Mode\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDOGRMODE {
    #[doc = "0: Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    Disabled = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    Limited = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    Extended = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    Full = 4,
}
impl From<WDOGRMODE> for u8 {
    #[inline(always)]
    fn from(variant: WDOGRMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDOGRMODE {
    type Ux = u8;
}
impl crate::IsEnum for WDOGRMODE {}
#[doc = "Field `WDOGRMODE` reader - WDOG Reset Mode"]
pub type WdogrmodeR = crate::FieldReader<WDOGRMODE>;
impl WdogrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WDOGRMODE> {
        match self.bits {
            0 => Some(WDOGRMODE::Disabled),
            1 => Some(WDOGRMODE::Limited),
            2 => Some(WDOGRMODE::Extended),
            4 => Some(WDOGRMODE::Full),
            _ => None,
        }
    }
    #[doc = "Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDOGRMODE::Disabled
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == WDOGRMODE::Limited
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == WDOGRMODE::Extended
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == WDOGRMODE::Full
    }
}
#[doc = "Field `WDOGRMODE` writer - WDOG Reset Mode"]
pub type WdogrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, WDOGRMODE>;
impl<'a, REG> WdogrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDOGRMODE::Disabled)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(WDOGRMODE::Limited)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(WDOGRMODE::Extended)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(WDOGRMODE::Full)
    }
}
#[doc = "Core LOCKUP Reset Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCKUPRMODE {
    #[doc = "0: Reset request is blocked."]
    Disabled = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    Limited = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    Extended = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    Full = 4,
}
impl From<LOCKUPRMODE> for u8 {
    #[inline(always)]
    fn from(variant: LOCKUPRMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCKUPRMODE {
    type Ux = u8;
}
impl crate::IsEnum for LOCKUPRMODE {}
#[doc = "Field `LOCKUPRMODE` reader - Core LOCKUP Reset Mode"]
pub type LockuprmodeR = crate::FieldReader<LOCKUPRMODE>;
impl LockuprmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LOCKUPRMODE> {
        match self.bits {
            0 => Some(LOCKUPRMODE::Disabled),
            1 => Some(LOCKUPRMODE::Limited),
            2 => Some(LOCKUPRMODE::Extended),
            4 => Some(LOCKUPRMODE::Full),
            _ => None,
        }
    }
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOCKUPRMODE::Disabled
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == LOCKUPRMODE::Limited
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == LOCKUPRMODE::Extended
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == LOCKUPRMODE::Full
    }
}
#[doc = "Field `LOCKUPRMODE` writer - Core LOCKUP Reset Mode"]
pub type LockuprmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, LOCKUPRMODE>;
impl<'a, REG> LockuprmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUPRMODE::Disabled)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUPRMODE::Limited)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUPRMODE::Extended)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUPRMODE::Full)
    }
}
#[doc = "Core Sysreset Reset Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSRMODE {
    #[doc = "0: Reset request is blocked."]
    Disabled = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    Limited = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    Extended = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    Full = 4,
}
impl From<SYSRMODE> for u8 {
    #[inline(always)]
    fn from(variant: SYSRMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSRMODE {
    type Ux = u8;
}
impl crate::IsEnum for SYSRMODE {}
#[doc = "Field `SYSRMODE` reader - Core Sysreset Reset Mode"]
pub type SysrmodeR = crate::FieldReader<SYSRMODE>;
impl SysrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSRMODE> {
        match self.bits {
            0 => Some(SYSRMODE::Disabled),
            1 => Some(SYSRMODE::Limited),
            2 => Some(SYSRMODE::Extended),
            4 => Some(SYSRMODE::Full),
            _ => None,
        }
    }
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSRMODE::Disabled
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == SYSRMODE::Limited
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == SYSRMODE::Extended
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == SYSRMODE::Full
    }
}
#[doc = "Field `SYSRMODE` writer - Core Sysreset Reset Mode"]
pub type SysrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, SYSRMODE>;
impl<'a, REG> SysrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRMODE::Disabled)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRMODE::Limited)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRMODE::Extended)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRMODE::Full)
    }
}
#[doc = "PIN Reset Mode\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINRMODE {
    #[doc = "0: Reset request is blocked."]
    Disabled = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    Limited = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    Extended = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    Full = 4,
}
impl From<PINRMODE> for u8 {
    #[inline(always)]
    fn from(variant: PINRMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PINRMODE {
    type Ux = u8;
}
impl crate::IsEnum for PINRMODE {}
#[doc = "Field `PINRMODE` reader - PIN Reset Mode"]
pub type PinrmodeR = crate::FieldReader<PINRMODE>;
impl PinrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PINRMODE> {
        match self.bits {
            0 => Some(PINRMODE::Disabled),
            1 => Some(PINRMODE::Limited),
            2 => Some(PINRMODE::Extended),
            4 => Some(PINRMODE::Full),
            _ => None,
        }
    }
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINRMODE::Disabled
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == PINRMODE::Limited
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == PINRMODE::Extended
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == PINRMODE::Full
    }
}
#[doc = "Field `PINRMODE` writer - PIN Reset Mode"]
pub type PinrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, PINRMODE>;
impl<'a, REG> PinrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PINRMODE::Disabled)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(PINRMODE::Limited)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(PINRMODE::Extended)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(PINRMODE::Full)
    }
}
#[doc = "Field `RESETSTATE` reader - System Software Reset State"]
pub type ResetstateR = crate::FieldReader;
#[doc = "Field `RESETSTATE` writer - System Software Reset State"]
pub type ResetstateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - WDOG Reset Mode"]
    #[inline(always)]
    pub fn wdogrmode(&self) -> WdogrmodeR {
        WdogrmodeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Core LOCKUP Reset Mode"]
    #[inline(always)]
    pub fn lockuprmode(&self) -> LockuprmodeR {
        LockuprmodeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Core Sysreset Reset Mode"]
    #[inline(always)]
    pub fn sysrmode(&self) -> SysrmodeR {
        SysrmodeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PIN Reset Mode"]
    #[inline(always)]
    pub fn pinrmode(&self) -> PinrmodeR {
        PinrmodeR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 24:25 - System Software Reset State"]
    #[inline(always)]
    pub fn resetstate(&self) -> ResetstateR {
        ResetstateR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WDOG Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wdogrmode(&mut self) -> WdogrmodeW<CTRLrs> {
        WdogrmodeW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Core LOCKUP Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lockuprmode(&mut self) -> LockuprmodeW<CTRLrs> {
        LockuprmodeW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Core Sysreset Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sysrmode(&mut self) -> SysrmodeW<CTRLrs> {
        SysrmodeW::new(self, 8)
    }
    #[doc = "Bits 12:14 - PIN Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pinrmode(&mut self) -> PinrmodeW<CTRLrs> {
        PinrmodeW::new(self, 12)
    }
    #[doc = "Bits 24:25 - System Software Reset State"]
    #[inline(always)]
    #[must_use]
    pub fn resetstate(&mut self) -> ResetstateW<CTRLrs> {
        ResetstateW::new(self, 24)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x4224"]
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0x4224;
}
