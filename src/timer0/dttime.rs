///Register `DTTIME` reader
pub type R = crate::R<DTTIMErs>;
///Register `DTTIME` writer
pub type W = crate::W<DTTIMErs>;
///DTI Prescaler Setting
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTPRESC {
    ///0: The HFPERCLK is undivided
    Div1 = 0,
    ///1: The HFPERCLK is divided by 2
    Div2 = 1,
    ///2: The HFPERCLK is divided by 4
    Div4 = 2,
    ///3: The HFPERCLK is divided by 8
    Div8 = 3,
    ///4: The HFPERCLK is divided by 16
    Div16 = 4,
    ///5: The HFPERCLK is divided by 32
    Div32 = 5,
    ///6: The HFPERCLK is divided by 64
    Div64 = 6,
    ///7: The HFPERCLK is divided by 128
    Div128 = 7,
    ///8: The HFPERCLK is divided by 256
    Div256 = 8,
    ///9: The HFPERCLK is divided by 512
    Div512 = 9,
    ///10: The HFPERCLK is divided by 1024
    Div1024 = 10,
}
impl From<DTPRESC> for u8 {
    #[inline(always)]
    fn from(variant: DTPRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTPRESC {
    type Ux = u8;
}
impl crate::IsEnum for DTPRESC {}
///Field `DTPRESC` reader - DTI Prescaler Setting
pub type DtprescR = crate::FieldReader<DTPRESC>;
impl DtprescR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DTPRESC> {
        match self.bits {
            0 => Some(DTPRESC::Div1),
            1 => Some(DTPRESC::Div2),
            2 => Some(DTPRESC::Div4),
            3 => Some(DTPRESC::Div8),
            4 => Some(DTPRESC::Div16),
            5 => Some(DTPRESC::Div32),
            6 => Some(DTPRESC::Div64),
            7 => Some(DTPRESC::Div128),
            8 => Some(DTPRESC::Div256),
            9 => Some(DTPRESC::Div512),
            10 => Some(DTPRESC::Div1024),
            _ => None,
        }
    }
    ///The HFPERCLK is undivided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DTPRESC::Div1
    }
    ///The HFPERCLK is divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DTPRESC::Div2
    }
    ///The HFPERCLK is divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DTPRESC::Div4
    }
    ///The HFPERCLK is divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DTPRESC::Div8
    }
    ///The HFPERCLK is divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DTPRESC::Div16
    }
    ///The HFPERCLK is divided by 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == DTPRESC::Div32
    }
    ///The HFPERCLK is divided by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == DTPRESC::Div64
    }
    ///The HFPERCLK is divided by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == DTPRESC::Div128
    }
    ///The HFPERCLK is divided by 256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == DTPRESC::Div256
    }
    ///The HFPERCLK is divided by 512
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == DTPRESC::Div512
    }
    ///The HFPERCLK is divided by 1024
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == DTPRESC::Div1024
    }
}
///Field `DTPRESC` writer - DTI Prescaler Setting
pub type DtprescW<'a, REG> = crate::FieldWriter<'a, REG, 4, DTPRESC>;
impl<'a, REG> DtprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The HFPERCLK is undivided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div1)
    }
    ///The HFPERCLK is divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div2)
    }
    ///The HFPERCLK is divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div4)
    }
    ///The HFPERCLK is divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div8)
    }
    ///The HFPERCLK is divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div16)
    }
    ///The HFPERCLK is divided by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div32)
    }
    ///The HFPERCLK is divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div64)
    }
    ///The HFPERCLK is divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div128)
    }
    ///The HFPERCLK is divided by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div256)
    }
    ///The HFPERCLK is divided by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div512)
    }
    ///The HFPERCLK is divided by 1024
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div1024)
    }
}
///Field `DTRISET` reader - DTI Rise-time
pub type DtrisetR = crate::FieldReader;
///Field `DTRISET` writer - DTI Rise-time
pub type DtrisetW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DTFALLT` reader - DTI Fall-time
pub type DtfalltR = crate::FieldReader;
///Field `DTFALLT` writer - DTI Fall-time
pub type DtfalltW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:3 - DTI Prescaler Setting
    #[inline(always)]
    pub fn dtpresc(&self) -> DtprescR {
        DtprescR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:13 - DTI Rise-time
    #[inline(always)]
    pub fn dtriset(&self) -> DtrisetR {
        DtrisetR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:21 - DTI Fall-time
    #[inline(always)]
    pub fn dtfallt(&self) -> DtfalltR {
        DtfalltR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTTIME")
            .field("dtpresc", &self.dtpresc())
            .field("dtriset", &self.dtriset())
            .field("dtfallt", &self.dtfallt())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - DTI Prescaler Setting
    #[inline(always)]
    #[must_use]
    pub fn dtpresc(&mut self) -> DtprescW<DTTIMErs> {
        DtprescW::new(self, 0)
    }
    ///Bits 8:13 - DTI Rise-time
    #[inline(always)]
    #[must_use]
    pub fn dtriset(&mut self) -> DtrisetW<DTTIMErs> {
        DtrisetW::new(self, 8)
    }
    ///Bits 16:21 - DTI Fall-time
    #[inline(always)]
    #[must_use]
    pub fn dtfallt(&mut self) -> DtfalltW<DTTIMErs> {
        DtfalltW::new(self, 16)
    }
}
///DTI Time Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dttime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dttime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DTTIMErs;
impl crate::RegisterSpec for DTTIMErs {
    type Ux = u32;
}
///`read()` method returns [`dttime::R`](R) reader structure
impl crate::Readable for DTTIMErs {}
///`write(|w| ..)` method takes [`dttime::W`](W) writer structure
impl crate::Writable for DTTIMErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DTTIME to value 0
impl crate::Resettable for DTTIMErs {
    const RESET_VALUE: u32 = 0;
}
