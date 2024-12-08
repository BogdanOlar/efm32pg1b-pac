///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///WDOG Reset Mode
///
///Value on reset: 4
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDOGRMODE {
    ///0: Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG
    Disabled = 0,
    ///1: The CRYOTIMER, DEBUGGER, RTCC, are not reset.
    Limited = 1,
    ///2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset.
    Extended = 2,
    ///4: The entire device is reset except some EMU and RMU registers.
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
///Field `WDOGRMODE` reader - WDOG Reset Mode
pub type WdogrmodeR = crate::FieldReader<WDOGRMODE>;
impl WdogrmodeR {
    ///Get enumerated values variant
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
    ///Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDOGRMODE::Disabled
    }
    ///The CRYOTIMER, DEBUGGER, RTCC, are not reset.
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == WDOGRMODE::Limited
    }
    ///The CRYOTIMER, DEBUGGER are not reset. RTCC is reset.
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == WDOGRMODE::Extended
    }
    ///The entire device is reset except some EMU and RMU registers.
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == WDOGRMODE::Full
    }
}
///Field `WDOGRMODE` writer - WDOG Reset Mode
pub type WdogrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, WDOGRMODE>;
impl<'a, REG> WdogrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDOGRMODE::Disabled)
    }
    ///The CRYOTIMER, DEBUGGER, RTCC, are not reset.
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(WDOGRMODE::Limited)
    }
    ///The CRYOTIMER, DEBUGGER are not reset. RTCC is reset.
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(WDOGRMODE::Extended)
    }
    ///The entire device is reset except some EMU and RMU registers.
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(WDOGRMODE::Full)
    }
}
///Core LOCKUP Reset Mode
///
///Value on reset: 2
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCKUPRMODE {
    ///0: Reset request is blocked.
    Disabled = 0,
    ///1: The CRYOTIMER, DEBUGGER, RTCC, are not reset.
    Limited = 1,
    ///2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset.
    Extended = 2,
    ///4: The entire device is reset except some EMU and RMU registers.
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
///Field `LOCKUPRMODE` reader - Core LOCKUP Reset Mode
pub type LockuprmodeR = crate::FieldReader<LOCKUPRMODE>;
impl LockuprmodeR {
    ///Get enumerated values variant
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
    ///Reset request is blocked.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOCKUPRMODE::Disabled
    }
    ///The CRYOTIMER, DEBUGGER, RTCC, are not reset.
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == LOCKUPRMODE::Limited
    }
    ///The CRYOTIMER, DEBUGGER are not reset. RTCC is reset.
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == LOCKUPRMODE::Extended
    }
    ///The entire device is reset except some EMU and RMU registers.
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == LOCKUPRMODE::Full
    }
}
///Field `LOCKUPRMODE` writer - Core LOCKUP Reset Mode
pub type LockuprmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, LOCKUPRMODE>;
impl<'a, REG> LockuprmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reset request is blocked.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUPRMODE::Disabled)
    }
    ///The CRYOTIMER, DEBUGGER, RTCC, are not reset.
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUPRMODE::Limited)
    }
    ///The CRYOTIMER, DEBUGGER are not reset. RTCC is reset.
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUPRMODE::Extended)
    }
    ///The entire device is reset except some EMU and RMU registers.
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUPRMODE::Full)
    }
}
///Core Sysreset Reset Mode
///
///Value on reset: 2
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSRMODE {
    ///0: Reset request is blocked.
    Disabled = 0,
    ///1: The CRYOTIMER, DEBUGGER, RTCC, are not reset.
    Limited = 1,
    ///2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset.
    Extended = 2,
    ///4: The entire device is reset except some EMU and RMU registers.
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
///Field `SYSRMODE` reader - Core Sysreset Reset Mode
pub type SysrmodeR = crate::FieldReader<SYSRMODE>;
impl SysrmodeR {
    ///Get enumerated values variant
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
    ///Reset request is blocked.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSRMODE::Disabled
    }
    ///The CRYOTIMER, DEBUGGER, RTCC, are not reset.
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == SYSRMODE::Limited
    }
    ///The CRYOTIMER, DEBUGGER are not reset. RTCC is reset.
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == SYSRMODE::Extended
    }
    ///The entire device is reset except some EMU and RMU registers.
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == SYSRMODE::Full
    }
}
///Field `SYSRMODE` writer - Core Sysreset Reset Mode
pub type SysrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, SYSRMODE>;
impl<'a, REG> SysrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reset request is blocked.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRMODE::Disabled)
    }
    ///The CRYOTIMER, DEBUGGER, RTCC, are not reset.
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRMODE::Limited)
    }
    ///The CRYOTIMER, DEBUGGER are not reset. RTCC is reset.
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRMODE::Extended)
    }
    ///The entire device is reset except some EMU and RMU registers.
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRMODE::Full)
    }
}
///PIN Reset Mode
///
///Value on reset: 4
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINRMODE {
    ///0: Reset request is blocked.
    Disabled = 0,
    ///1: The CRYOTIMER, DEBUGGER, RTCC, are not reset.
    Limited = 1,
    ///2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset.
    Extended = 2,
    ///4: The entire device is reset except some EMU and RMU registers.
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
///Field `PINRMODE` reader - PIN Reset Mode
pub type PinrmodeR = crate::FieldReader<PINRMODE>;
impl PinrmodeR {
    ///Get enumerated values variant
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
    ///Reset request is blocked.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINRMODE::Disabled
    }
    ///The CRYOTIMER, DEBUGGER, RTCC, are not reset.
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == PINRMODE::Limited
    }
    ///The CRYOTIMER, DEBUGGER are not reset. RTCC is reset.
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == PINRMODE::Extended
    }
    ///The entire device is reset except some EMU and RMU registers.
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == PINRMODE::Full
    }
}
///Field `PINRMODE` writer - PIN Reset Mode
pub type PinrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, PINRMODE>;
impl<'a, REG> PinrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reset request is blocked.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PINRMODE::Disabled)
    }
    ///The CRYOTIMER, DEBUGGER, RTCC, are not reset.
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(PINRMODE::Limited)
    }
    ///The CRYOTIMER, DEBUGGER are not reset. RTCC is reset.
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(PINRMODE::Extended)
    }
    ///The entire device is reset except some EMU and RMU registers.
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(PINRMODE::Full)
    }
}
///Field `RESETSTATE` reader - System Software Reset State
pub type ResetstateR = crate::FieldReader;
///Field `RESETSTATE` writer - System Software Reset State
pub type ResetstateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - WDOG Reset Mode
    #[inline(always)]
    pub fn wdogrmode(&self) -> WdogrmodeR {
        WdogrmodeR::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Core LOCKUP Reset Mode
    #[inline(always)]
    pub fn lockuprmode(&self) -> LockuprmodeR {
        LockuprmodeR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Core Sysreset Reset Mode
    #[inline(always)]
    pub fn sysrmode(&self) -> SysrmodeR {
        SysrmodeR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - PIN Reset Mode
    #[inline(always)]
    pub fn pinrmode(&self) -> PinrmodeR {
        PinrmodeR::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 24:25 - System Software Reset State
    #[inline(always)]
    pub fn resetstate(&self) -> ResetstateR {
        ResetstateR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("wdogrmode", &self.wdogrmode())
            .field("lockuprmode", &self.lockuprmode())
            .field("sysrmode", &self.sysrmode())
            .field("pinrmode", &self.pinrmode())
            .field("resetstate", &self.resetstate())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - WDOG Reset Mode
    #[inline(always)]
    #[must_use]
    pub fn wdogrmode(&mut self) -> WdogrmodeW<CTRLrs> {
        WdogrmodeW::new(self, 0)
    }
    ///Bits 4:6 - Core LOCKUP Reset Mode
    #[inline(always)]
    #[must_use]
    pub fn lockuprmode(&mut self) -> LockuprmodeW<CTRLrs> {
        LockuprmodeW::new(self, 4)
    }
    ///Bits 8:10 - Core Sysreset Reset Mode
    #[inline(always)]
    #[must_use]
    pub fn sysrmode(&mut self) -> SysrmodeW<CTRLrs> {
        SysrmodeW::new(self, 8)
    }
    ///Bits 12:14 - PIN Reset Mode
    #[inline(always)]
    #[must_use]
    pub fn pinrmode(&mut self) -> PinrmodeW<CTRLrs> {
        PinrmodeW::new(self, 12)
    }
    ///Bits 24:25 - System Software Reset State
    #[inline(always)]
    #[must_use]
    pub fn resetstate(&mut self) -> ResetstateW<CTRLrs> {
        ResetstateW::new(self, 24)
    }
}
///Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CTRLrs {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL to value 0x4224
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0x4224;
}
