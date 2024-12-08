///Register `LFAPRESC0` reader
pub type R = crate::R<LFAPRESC0rs>;
///Register `LFAPRESC0` writer
pub type W = crate::W<LFAPRESC0rs>;
///Low Energy Timer 0 Prescaler
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LETIMER0 {
    ///0: LFACLKLETIMER0 = LFACLK
    Div1 = 0,
    ///1: LFACLKLETIMER0 = LFACLK/2
    Div2 = 1,
    ///2: LFACLKLETIMER0 = LFACLK/4
    Div4 = 2,
    ///3: LFACLKLETIMER0 = LFACLK/8
    Div8 = 3,
    ///4: LFACLKLETIMER0 = LFACLK/16
    Div16 = 4,
    ///5: LFACLKLETIMER0 = LFACLK/32
    Div32 = 5,
    ///6: LFACLKLETIMER0 = LFACLK/64
    Div64 = 6,
    ///7: LFACLKLETIMER0 = LFACLK/128
    Div128 = 7,
    ///8: LFACLKLETIMER0 = LFACLK/256
    Div256 = 8,
    ///9: LFACLKLETIMER0 = LFACLK/512
    Div512 = 9,
    ///10: LFACLKLETIMER0 = LFACLK/1024
    Div1024 = 10,
    ///11: LFACLKLETIMER0 = LFACLK/2048
    Div2048 = 11,
    ///12: LFACLKLETIMER0 = LFACLK/4096
    Div4096 = 12,
    ///13: LFACLKLETIMER0 = LFACLK/8192
    Div8192 = 13,
    ///14: LFACLKLETIMER0 = LFACLK/16384
    Div16384 = 14,
    ///15: LFACLKLETIMER0 = LFACLK/32768
    Div32768 = 15,
}
impl From<LETIMER0> for u8 {
    #[inline(always)]
    fn from(variant: LETIMER0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LETIMER0 {
    type Ux = u8;
}
impl crate::IsEnum for LETIMER0 {}
///Field `LETIMER0` reader - Low Energy Timer 0 Prescaler
pub type Letimer0R = crate::FieldReader<LETIMER0>;
impl Letimer0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LETIMER0 {
        match self.bits {
            0 => LETIMER0::Div1,
            1 => LETIMER0::Div2,
            2 => LETIMER0::Div4,
            3 => LETIMER0::Div8,
            4 => LETIMER0::Div16,
            5 => LETIMER0::Div32,
            6 => LETIMER0::Div64,
            7 => LETIMER0::Div128,
            8 => LETIMER0::Div256,
            9 => LETIMER0::Div512,
            10 => LETIMER0::Div1024,
            11 => LETIMER0::Div2048,
            12 => LETIMER0::Div4096,
            13 => LETIMER0::Div8192,
            14 => LETIMER0::Div16384,
            15 => LETIMER0::Div32768,
            _ => unreachable!(),
        }
    }
    ///LFACLKLETIMER0 = LFACLK
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LETIMER0::Div1
    }
    ///LFACLKLETIMER0 = LFACLK/2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LETIMER0::Div2
    }
    ///LFACLKLETIMER0 = LFACLK/4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LETIMER0::Div4
    }
    ///LFACLKLETIMER0 = LFACLK/8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LETIMER0::Div8
    }
    ///LFACLKLETIMER0 = LFACLK/16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == LETIMER0::Div16
    }
    ///LFACLKLETIMER0 = LFACLK/32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == LETIMER0::Div32
    }
    ///LFACLKLETIMER0 = LFACLK/64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == LETIMER0::Div64
    }
    ///LFACLKLETIMER0 = LFACLK/128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LETIMER0::Div128
    }
    ///LFACLKLETIMER0 = LFACLK/256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == LETIMER0::Div256
    }
    ///LFACLKLETIMER0 = LFACLK/512
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == LETIMER0::Div512
    }
    ///LFACLKLETIMER0 = LFACLK/1024
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == LETIMER0::Div1024
    }
    ///LFACLKLETIMER0 = LFACLK/2048
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == LETIMER0::Div2048
    }
    ///LFACLKLETIMER0 = LFACLK/4096
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == LETIMER0::Div4096
    }
    ///LFACLKLETIMER0 = LFACLK/8192
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == LETIMER0::Div8192
    }
    ///LFACLKLETIMER0 = LFACLK/16384
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == LETIMER0::Div16384
    }
    ///LFACLKLETIMER0 = LFACLK/32768
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == LETIMER0::Div32768
    }
}
///Field `LETIMER0` writer - Low Energy Timer 0 Prescaler
pub type Letimer0W<'a, REG> = crate::FieldWriter<'a, REG, 4, LETIMER0, crate::Safe>;
impl<'a, REG> Letimer0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///LFACLKLETIMER0 = LFACLK
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div1)
    }
    ///LFACLKLETIMER0 = LFACLK/2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div2)
    }
    ///LFACLKLETIMER0 = LFACLK/4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div4)
    }
    ///LFACLKLETIMER0 = LFACLK/8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div8)
    }
    ///LFACLKLETIMER0 = LFACLK/16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div16)
    }
    ///LFACLKLETIMER0 = LFACLK/32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div32)
    }
    ///LFACLKLETIMER0 = LFACLK/64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div64)
    }
    ///LFACLKLETIMER0 = LFACLK/128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div128)
    }
    ///LFACLKLETIMER0 = LFACLK/256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div256)
    }
    ///LFACLKLETIMER0 = LFACLK/512
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div512)
    }
    ///LFACLKLETIMER0 = LFACLK/1024
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div1024)
    }
    ///LFACLKLETIMER0 = LFACLK/2048
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div2048)
    }
    ///LFACLKLETIMER0 = LFACLK/4096
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div4096)
    }
    ///LFACLKLETIMER0 = LFACLK/8192
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div8192)
    }
    ///LFACLKLETIMER0 = LFACLK/16384
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div16384)
    }
    ///LFACLKLETIMER0 = LFACLK/32768
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(LETIMER0::Div32768)
    }
}
impl R {
    ///Bits 0:3 - Low Energy Timer 0 Prescaler
    #[inline(always)]
    pub fn letimer0(&self) -> Letimer0R {
        Letimer0R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFAPRESC0")
            .field("letimer0", &self.letimer0())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Low Energy Timer 0 Prescaler
    #[inline(always)]
    #[must_use]
    pub fn letimer0(&mut self) -> Letimer0W<LFAPRESC0rs> {
        Letimer0W::new(self, 0)
    }
}
///Low Frequency a Prescaler Register 0 (Async Reg)
///
///You can [`read`](crate::Reg::read) this register and get [`lfapresc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfapresc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LFAPRESC0rs;
impl crate::RegisterSpec for LFAPRESC0rs {
    type Ux = u32;
}
///`read()` method returns [`lfapresc0::R`](R) reader structure
impl crate::Readable for LFAPRESC0rs {}
///`write(|w| ..)` method takes [`lfapresc0::W`](W) writer structure
impl crate::Writable for LFAPRESC0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LFAPRESC0 to value 0
impl crate::Resettable for LFAPRESC0rs {
    const RESET_VALUE: u32 = 0;
}
