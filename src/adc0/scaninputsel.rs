#[doc = "Register `SCANINPUTSEL` reader"]
pub type R = crate::R<SCANINPUTSELrs>;
#[doc = "Register `SCANINPUTSEL` writer"]
pub type W = crate::W<SCANINPUTSELrs>;
#[doc = "Field `INPUT0TO7SEL` reader - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
pub type INPUT0TO7SEL_R = crate::FieldReader<INPUT0TO7SEL>;
#[doc = "Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT0TO7SEL {
    #[doc = "0: `0`"]
    Aport0ch0to7 = 0,
    #[doc = "1: `1`"]
    Aport0ch8to15 = 1,
    #[doc = "4: `100`"]
    Aport1ch0to7 = 4,
    #[doc = "5: `101`"]
    Aport1ch8to15 = 5,
    #[doc = "6: `110`"]
    Aport1ch16to23 = 6,
    #[doc = "7: `111`"]
    Aport1ch24to31 = 7,
    #[doc = "8: `1000`"]
    Aport2ch0to7 = 8,
    #[doc = "9: `1001`"]
    Aport2ch8to15 = 9,
    #[doc = "10: `1010`"]
    Aport2ch16to23 = 10,
    #[doc = "11: `1011`"]
    Aport2ch24to31 = 11,
    #[doc = "12: `1100`"]
    Aport3ch0to7 = 12,
    #[doc = "13: `1101`"]
    Aport3ch8to15 = 13,
    #[doc = "14: `1110`"]
    Aport3ch16to23 = 14,
    #[doc = "15: `1111`"]
    Aport3ch24to31 = 15,
    #[doc = "16: `10000`"]
    Aport4ch0to7 = 16,
    #[doc = "17: `10001`"]
    Aport4ch8to15 = 17,
    #[doc = "18: `10010`"]
    Aport4ch16to23 = 18,
    #[doc = "19: `10011`"]
    Aport4ch24to31 = 19,
}
impl From<INPUT0TO7SEL> for u8 {
    #[inline(always)]
    fn from(variant: INPUT0TO7SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUT0TO7SEL {
    type Ux = u8;
}
impl INPUT0TO7SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INPUT0TO7SEL> {
        match self.bits {
            0 => Some(INPUT0TO7SEL::Aport0ch0to7),
            1 => Some(INPUT0TO7SEL::Aport0ch8to15),
            4 => Some(INPUT0TO7SEL::Aport1ch0to7),
            5 => Some(INPUT0TO7SEL::Aport1ch8to15),
            6 => Some(INPUT0TO7SEL::Aport1ch16to23),
            7 => Some(INPUT0TO7SEL::Aport1ch24to31),
            8 => Some(INPUT0TO7SEL::Aport2ch0to7),
            9 => Some(INPUT0TO7SEL::Aport2ch8to15),
            10 => Some(INPUT0TO7SEL::Aport2ch16to23),
            11 => Some(INPUT0TO7SEL::Aport2ch24to31),
            12 => Some(INPUT0TO7SEL::Aport3ch0to7),
            13 => Some(INPUT0TO7SEL::Aport3ch8to15),
            14 => Some(INPUT0TO7SEL::Aport3ch16to23),
            15 => Some(INPUT0TO7SEL::Aport3ch24to31),
            16 => Some(INPUT0TO7SEL::Aport4ch0to7),
            17 => Some(INPUT0TO7SEL::Aport4ch8to15),
            18 => Some(INPUT0TO7SEL::Aport4ch16to23),
            19 => Some(INPUT0TO7SEL::Aport4ch24to31),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL::Aport0ch0to7
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL::Aport0ch8to15
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL::Aport1ch0to7
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL::Aport1ch8to15
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT0TO7SEL::Aport1ch16to23
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT0TO7SEL::Aport1ch24to31
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL::Aport2ch0to7
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL::Aport2ch8to15
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT0TO7SEL::Aport2ch16to23
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT0TO7SEL::Aport2ch24to31
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL::Aport3ch0to7
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL::Aport3ch8to15
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT0TO7SEL::Aport3ch16to23
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT0TO7SEL::Aport3ch24to31
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL::Aport4ch0to7
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL::Aport4ch8to15
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT0TO7SEL::Aport4ch16to23
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT0TO7SEL::Aport4ch24to31
    }
}
#[doc = "Field `INPUT0TO7SEL` writer - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
pub type INPUT0TO7SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, INPUT0TO7SEL>;
impl<'a, REG> INPUT0TO7SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport0ch0to7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport0ch8to15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport1ch0to7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport1ch8to15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport1ch16to23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport1ch24to31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport2ch0to7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport2ch8to15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport2ch16to23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport2ch24to31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport3ch0to7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport3ch8to15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport3ch16to23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport3ch24to31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport4ch0to7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport4ch8to15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport4ch16to23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT0TO7SEL::Aport4ch24to31)
    }
}
#[doc = "Field `INPUT8TO15SEL` reader - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
pub type INPUT8TO15SEL_R = crate::FieldReader<INPUT8TO15SEL>;
#[doc = "Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT8TO15SEL {
    #[doc = "0: `0`"]
    Aport0ch0to7 = 0,
    #[doc = "1: `1`"]
    Aport0ch8to15 = 1,
    #[doc = "4: `100`"]
    Aport1ch0to7 = 4,
    #[doc = "5: `101`"]
    Aport1ch8to15 = 5,
    #[doc = "6: `110`"]
    Aport1ch16to23 = 6,
    #[doc = "7: `111`"]
    Aport1ch24to31 = 7,
    #[doc = "8: `1000`"]
    Aport2ch0to7 = 8,
    #[doc = "9: `1001`"]
    Aport2ch8to15 = 9,
    #[doc = "10: `1010`"]
    Aport2ch16to23 = 10,
    #[doc = "11: `1011`"]
    Aport2ch24to31 = 11,
    #[doc = "12: `1100`"]
    Aport3ch0to7 = 12,
    #[doc = "13: `1101`"]
    Aport3ch8to15 = 13,
    #[doc = "14: `1110`"]
    Aport3ch16to23 = 14,
    #[doc = "15: `1111`"]
    Aport3ch24to31 = 15,
    #[doc = "16: `10000`"]
    Aport4ch0to7 = 16,
    #[doc = "17: `10001`"]
    Aport4ch8to15 = 17,
    #[doc = "18: `10010`"]
    Aport4ch16to23 = 18,
    #[doc = "19: `10011`"]
    Aport4ch24to31 = 19,
}
impl From<INPUT8TO15SEL> for u8 {
    #[inline(always)]
    fn from(variant: INPUT8TO15SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUT8TO15SEL {
    type Ux = u8;
}
impl INPUT8TO15SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INPUT8TO15SEL> {
        match self.bits {
            0 => Some(INPUT8TO15SEL::Aport0ch0to7),
            1 => Some(INPUT8TO15SEL::Aport0ch8to15),
            4 => Some(INPUT8TO15SEL::Aport1ch0to7),
            5 => Some(INPUT8TO15SEL::Aport1ch8to15),
            6 => Some(INPUT8TO15SEL::Aport1ch16to23),
            7 => Some(INPUT8TO15SEL::Aport1ch24to31),
            8 => Some(INPUT8TO15SEL::Aport2ch0to7),
            9 => Some(INPUT8TO15SEL::Aport2ch8to15),
            10 => Some(INPUT8TO15SEL::Aport2ch16to23),
            11 => Some(INPUT8TO15SEL::Aport2ch24to31),
            12 => Some(INPUT8TO15SEL::Aport3ch0to7),
            13 => Some(INPUT8TO15SEL::Aport3ch8to15),
            14 => Some(INPUT8TO15SEL::Aport3ch16to23),
            15 => Some(INPUT8TO15SEL::Aport3ch24to31),
            16 => Some(INPUT8TO15SEL::Aport4ch0to7),
            17 => Some(INPUT8TO15SEL::Aport4ch8to15),
            18 => Some(INPUT8TO15SEL::Aport4ch16to23),
            19 => Some(INPUT8TO15SEL::Aport4ch24to31),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL::Aport0ch0to7
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL::Aport0ch8to15
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL::Aport1ch0to7
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL::Aport1ch8to15
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT8TO15SEL::Aport1ch16to23
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT8TO15SEL::Aport1ch24to31
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL::Aport2ch0to7
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL::Aport2ch8to15
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT8TO15SEL::Aport2ch16to23
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT8TO15SEL::Aport2ch24to31
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL::Aport3ch0to7
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL::Aport3ch8to15
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT8TO15SEL::Aport3ch16to23
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT8TO15SEL::Aport3ch24to31
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL::Aport4ch0to7
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL::Aport4ch8to15
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT8TO15SEL::Aport4ch16to23
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT8TO15SEL::Aport4ch24to31
    }
}
#[doc = "Field `INPUT8TO15SEL` writer - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
pub type INPUT8TO15SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, INPUT8TO15SEL>;
impl<'a, REG> INPUT8TO15SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport0ch0to7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport0ch8to15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport1ch0to7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport1ch8to15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport1ch16to23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport1ch24to31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport2ch0to7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport2ch8to15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport2ch16to23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport2ch24to31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport3ch0to7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport3ch8to15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport3ch16to23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport3ch24to31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport4ch0to7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport4ch8to15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport4ch16to23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT8TO15SEL::Aport4ch24to31)
    }
}
#[doc = "Field `INPUT16TO23SEL` reader - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
pub type INPUT16TO23SEL_R = crate::FieldReader<INPUT16TO23SEL>;
#[doc = "Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT16TO23SEL {
    #[doc = "0: `0`"]
    Aport0ch0to7 = 0,
    #[doc = "1: `1`"]
    Aport0ch8to15 = 1,
    #[doc = "4: `100`"]
    Aport1ch0to7 = 4,
    #[doc = "5: `101`"]
    Aport1ch8to15 = 5,
    #[doc = "6: `110`"]
    Aport1ch16to23 = 6,
    #[doc = "7: `111`"]
    Aport1ch24to31 = 7,
    #[doc = "8: `1000`"]
    Aport2ch0to7 = 8,
    #[doc = "9: `1001`"]
    Aport2ch8to15 = 9,
    #[doc = "10: `1010`"]
    Aport2ch16to23 = 10,
    #[doc = "11: `1011`"]
    Aport2ch24to31 = 11,
    #[doc = "12: `1100`"]
    Aport3ch0to7 = 12,
    #[doc = "13: `1101`"]
    Aport3ch8to15 = 13,
    #[doc = "14: `1110`"]
    Aport3ch16to23 = 14,
    #[doc = "15: `1111`"]
    Aport3ch24to31 = 15,
    #[doc = "16: `10000`"]
    Aport4ch0to7 = 16,
    #[doc = "17: `10001`"]
    Aport4ch8to15 = 17,
    #[doc = "18: `10010`"]
    Aport4ch16to23 = 18,
    #[doc = "19: `10011`"]
    Aport4ch24to31 = 19,
}
impl From<INPUT16TO23SEL> for u8 {
    #[inline(always)]
    fn from(variant: INPUT16TO23SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUT16TO23SEL {
    type Ux = u8;
}
impl INPUT16TO23SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INPUT16TO23SEL> {
        match self.bits {
            0 => Some(INPUT16TO23SEL::Aport0ch0to7),
            1 => Some(INPUT16TO23SEL::Aport0ch8to15),
            4 => Some(INPUT16TO23SEL::Aport1ch0to7),
            5 => Some(INPUT16TO23SEL::Aport1ch8to15),
            6 => Some(INPUT16TO23SEL::Aport1ch16to23),
            7 => Some(INPUT16TO23SEL::Aport1ch24to31),
            8 => Some(INPUT16TO23SEL::Aport2ch0to7),
            9 => Some(INPUT16TO23SEL::Aport2ch8to15),
            10 => Some(INPUT16TO23SEL::Aport2ch16to23),
            11 => Some(INPUT16TO23SEL::Aport2ch24to31),
            12 => Some(INPUT16TO23SEL::Aport3ch0to7),
            13 => Some(INPUT16TO23SEL::Aport3ch8to15),
            14 => Some(INPUT16TO23SEL::Aport3ch16to23),
            15 => Some(INPUT16TO23SEL::Aport3ch24to31),
            16 => Some(INPUT16TO23SEL::Aport4ch0to7),
            17 => Some(INPUT16TO23SEL::Aport4ch8to15),
            18 => Some(INPUT16TO23SEL::Aport4ch16to23),
            19 => Some(INPUT16TO23SEL::Aport4ch24to31),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL::Aport0ch0to7
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL::Aport0ch8to15
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL::Aport1ch0to7
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL::Aport1ch8to15
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT16TO23SEL::Aport1ch16to23
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT16TO23SEL::Aport1ch24to31
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL::Aport2ch0to7
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL::Aport2ch8to15
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT16TO23SEL::Aport2ch16to23
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT16TO23SEL::Aport2ch24to31
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL::Aport3ch0to7
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL::Aport3ch8to15
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT16TO23SEL::Aport3ch16to23
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT16TO23SEL::Aport3ch24to31
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL::Aport4ch0to7
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL::Aport4ch8to15
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT16TO23SEL::Aport4ch16to23
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT16TO23SEL::Aport4ch24to31
    }
}
#[doc = "Field `INPUT16TO23SEL` writer - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
pub type INPUT16TO23SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, INPUT16TO23SEL>;
impl<'a, REG> INPUT16TO23SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport0ch0to7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport0ch8to15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport1ch0to7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport1ch8to15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport1ch16to23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport1ch24to31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport2ch0to7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport2ch8to15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport2ch16to23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport2ch24to31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport3ch0to7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport3ch8to15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport3ch16to23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport3ch24to31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport4ch0to7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport4ch8to15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport4ch16to23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT16TO23SEL::Aport4ch24to31)
    }
}
#[doc = "Field `INPUT24TO31SEL` reader - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
pub type INPUT24TO31SEL_R = crate::FieldReader<INPUT24TO31SEL>;
#[doc = "Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT24TO31SEL {
    #[doc = "0: `0`"]
    Aport0ch0to7 = 0,
    #[doc = "1: `1`"]
    Aport0ch8to15 = 1,
    #[doc = "4: `100`"]
    Aport1ch0to7 = 4,
    #[doc = "5: `101`"]
    Aport1ch8to15 = 5,
    #[doc = "6: `110`"]
    Aport1ch16to23 = 6,
    #[doc = "7: `111`"]
    Aport1ch24to31 = 7,
    #[doc = "8: `1000`"]
    Aport2ch0to7 = 8,
    #[doc = "9: `1001`"]
    Aport2ch8to15 = 9,
    #[doc = "10: `1010`"]
    Aport2ch16to23 = 10,
    #[doc = "11: `1011`"]
    Aport2ch24to31 = 11,
    #[doc = "12: `1100`"]
    Aport3ch0to7 = 12,
    #[doc = "13: `1101`"]
    Aport3ch8to15 = 13,
    #[doc = "14: `1110`"]
    Aport3ch16to23 = 14,
    #[doc = "15: `1111`"]
    Aport3ch24to31 = 15,
    #[doc = "16: `10000`"]
    Aport4ch0to7 = 16,
    #[doc = "17: `10001`"]
    Aport4ch8to15 = 17,
    #[doc = "18: `10010`"]
    Aport4ch16to23 = 18,
    #[doc = "19: `10011`"]
    Aport4ch24to31 = 19,
}
impl From<INPUT24TO31SEL> for u8 {
    #[inline(always)]
    fn from(variant: INPUT24TO31SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUT24TO31SEL {
    type Ux = u8;
}
impl INPUT24TO31SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INPUT24TO31SEL> {
        match self.bits {
            0 => Some(INPUT24TO31SEL::Aport0ch0to7),
            1 => Some(INPUT24TO31SEL::Aport0ch8to15),
            4 => Some(INPUT24TO31SEL::Aport1ch0to7),
            5 => Some(INPUT24TO31SEL::Aport1ch8to15),
            6 => Some(INPUT24TO31SEL::Aport1ch16to23),
            7 => Some(INPUT24TO31SEL::Aport1ch24to31),
            8 => Some(INPUT24TO31SEL::Aport2ch0to7),
            9 => Some(INPUT24TO31SEL::Aport2ch8to15),
            10 => Some(INPUT24TO31SEL::Aport2ch16to23),
            11 => Some(INPUT24TO31SEL::Aport2ch24to31),
            12 => Some(INPUT24TO31SEL::Aport3ch0to7),
            13 => Some(INPUT24TO31SEL::Aport3ch8to15),
            14 => Some(INPUT24TO31SEL::Aport3ch16to23),
            15 => Some(INPUT24TO31SEL::Aport3ch24to31),
            16 => Some(INPUT24TO31SEL::Aport4ch0to7),
            17 => Some(INPUT24TO31SEL::Aport4ch8to15),
            18 => Some(INPUT24TO31SEL::Aport4ch16to23),
            19 => Some(INPUT24TO31SEL::Aport4ch24to31),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL::Aport0ch0to7
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL::Aport0ch8to15
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL::Aport1ch0to7
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL::Aport1ch8to15
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT24TO31SEL::Aport1ch16to23
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT24TO31SEL::Aport1ch24to31
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL::Aport2ch0to7
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL::Aport2ch8to15
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT24TO31SEL::Aport2ch16to23
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT24TO31SEL::Aport2ch24to31
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL::Aport3ch0to7
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL::Aport3ch8to15
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT24TO31SEL::Aport3ch16to23
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT24TO31SEL::Aport3ch24to31
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL::Aport4ch0to7
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL::Aport4ch8to15
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT24TO31SEL::Aport4ch16to23
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT24TO31SEL::Aport4ch24to31
    }
}
#[doc = "Field `INPUT24TO31SEL` writer - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
pub type INPUT24TO31SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, INPUT24TO31SEL>;
impl<'a, REG> INPUT24TO31SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport0ch0to7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport0ch8to15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport1ch0to7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport1ch8to15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport1ch16to23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport1ch24to31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport2ch0to7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport2ch8to15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport2ch16to23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport2ch24to31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport3ch0to7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport3ch8to15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport3ch16to23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport3ch24to31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport4ch0to7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport4ch8to15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport4ch16to23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT24TO31SEL::Aport4ch24to31)
    }
}
impl R {
    #[doc = "Bits 0:4 - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input0to7sel(&self) -> INPUT0TO7SEL_R {
        INPUT0TO7SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input8to15sel(&self) -> INPUT8TO15SEL_R {
        INPUT8TO15SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input16to23sel(&self) -> INPUT16TO23SEL_R {
        INPUT16TO23SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input24to31sel(&self) -> INPUT24TO31SEL_R {
        INPUT24TO31SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
    #[inline(always)]
    #[must_use]
    pub fn input0to7sel(&mut self) -> INPUT0TO7SEL_W<SCANINPUTSELrs> {
        INPUT0TO7SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
    #[inline(always)]
    #[must_use]
    pub fn input8to15sel(&mut self) -> INPUT8TO15SEL_W<SCANINPUTSELrs> {
        INPUT8TO15SEL_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
    #[inline(always)]
    #[must_use]
    pub fn input16to23sel(&mut self) -> INPUT16TO23SEL_W<SCANINPUTSELrs> {
        INPUT16TO23SEL_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
    #[inline(always)]
    #[must_use]
    pub fn input24to31sel(&mut self) -> INPUT24TO31SEL_W<SCANINPUTSELrs> {
        INPUT24TO31SEL_W::new(self, 24)
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
#[doc = "Input Selection Register for Scan Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scaninputsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scaninputsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANINPUTSELrs;
impl crate::RegisterSpec for SCANINPUTSELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scaninputsel::R`](R) reader structure"]
impl crate::Readable for SCANINPUTSELrs {}
#[doc = "`write(|w| ..)` method takes [`scaninputsel::W`](W) writer structure"]
impl crate::Writable for SCANINPUTSELrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCANINPUTSEL to value 0"]
impl crate::Resettable for SCANINPUTSELrs {
    const RESET_VALUE: u32 = 0;
}
