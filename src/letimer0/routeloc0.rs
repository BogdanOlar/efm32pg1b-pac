#[doc = "Register `ROUTELOC0` reader"]
pub type R = crate::R<ROUTELOC0rs>;
#[doc = "Register `ROUTELOC0` writer"]
pub type W = crate::W<ROUTELOC0rs>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUT0LOC {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
    #[doc = "7: Location 7"]
    Loc7 = 7,
    #[doc = "8: Location 8"]
    Loc8 = 8,
    #[doc = "9: Location 9"]
    Loc9 = 9,
    #[doc = "10: Location 10"]
    Loc10 = 10,
    #[doc = "11: Location 11"]
    Loc11 = 11,
    #[doc = "12: Location 12"]
    Loc12 = 12,
    #[doc = "13: Location 13"]
    Loc13 = 13,
    #[doc = "14: Location 14"]
    Loc14 = 14,
    #[doc = "15: Location 15"]
    Loc15 = 15,
    #[doc = "16: Location 16"]
    Loc16 = 16,
    #[doc = "17: Location 17"]
    Loc17 = 17,
    #[doc = "18: Location 18"]
    Loc18 = 18,
    #[doc = "19: Location 19"]
    Loc19 = 19,
    #[doc = "20: Location 20"]
    Loc20 = 20,
    #[doc = "21: Location 21"]
    Loc21 = 21,
    #[doc = "22: Location 22"]
    Loc22 = 22,
    #[doc = "23: Location 23"]
    Loc23 = 23,
    #[doc = "24: Location 24"]
    Loc24 = 24,
    #[doc = "25: Location 25"]
    Loc25 = 25,
    #[doc = "26: Location 26"]
    Loc26 = 26,
    #[doc = "27: Location 27"]
    Loc27 = 27,
    #[doc = "28: Location 28"]
    Loc28 = 28,
    #[doc = "29: Location 29"]
    Loc29 = 29,
    #[doc = "30: Location 30"]
    Loc30 = 30,
    #[doc = "31: Location 31"]
    Loc31 = 31,
}
impl From<OUT0LOC> for u8 {
    #[inline(always)]
    fn from(variant: OUT0LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OUT0LOC {
    type Ux = u8;
}
impl crate::IsEnum for OUT0LOC {}
#[doc = "Field `OUT0LOC` reader - I/O Location"]
pub type Out0locR = crate::FieldReader<OUT0LOC>;
impl Out0locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OUT0LOC> {
        match self.bits {
            0 => Some(OUT0LOC::Loc0),
            1 => Some(OUT0LOC::Loc1),
            2 => Some(OUT0LOC::Loc2),
            3 => Some(OUT0LOC::Loc3),
            4 => Some(OUT0LOC::Loc4),
            5 => Some(OUT0LOC::Loc5),
            6 => Some(OUT0LOC::Loc6),
            7 => Some(OUT0LOC::Loc7),
            8 => Some(OUT0LOC::Loc8),
            9 => Some(OUT0LOC::Loc9),
            10 => Some(OUT0LOC::Loc10),
            11 => Some(OUT0LOC::Loc11),
            12 => Some(OUT0LOC::Loc12),
            13 => Some(OUT0LOC::Loc13),
            14 => Some(OUT0LOC::Loc14),
            15 => Some(OUT0LOC::Loc15),
            16 => Some(OUT0LOC::Loc16),
            17 => Some(OUT0LOC::Loc17),
            18 => Some(OUT0LOC::Loc18),
            19 => Some(OUT0LOC::Loc19),
            20 => Some(OUT0LOC::Loc20),
            21 => Some(OUT0LOC::Loc21),
            22 => Some(OUT0LOC::Loc22),
            23 => Some(OUT0LOC::Loc23),
            24 => Some(OUT0LOC::Loc24),
            25 => Some(OUT0LOC::Loc25),
            26 => Some(OUT0LOC::Loc26),
            27 => Some(OUT0LOC::Loc27),
            28 => Some(OUT0LOC::Loc28),
            29 => Some(OUT0LOC::Loc29),
            30 => Some(OUT0LOC::Loc30),
            31 => Some(OUT0LOC::Loc31),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == OUT0LOC::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == OUT0LOC::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == OUT0LOC::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == OUT0LOC::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == OUT0LOC::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == OUT0LOC::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == OUT0LOC::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == OUT0LOC::Loc7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == OUT0LOC::Loc8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == OUT0LOC::Loc9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == OUT0LOC::Loc10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == OUT0LOC::Loc11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == OUT0LOC::Loc12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == OUT0LOC::Loc13
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == OUT0LOC::Loc14
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == OUT0LOC::Loc15
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == OUT0LOC::Loc16
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == OUT0LOC::Loc17
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == OUT0LOC::Loc18
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == OUT0LOC::Loc19
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == OUT0LOC::Loc20
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == OUT0LOC::Loc21
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == OUT0LOC::Loc22
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == OUT0LOC::Loc23
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == OUT0LOC::Loc24
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == OUT0LOC::Loc25
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == OUT0LOC::Loc26
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == OUT0LOC::Loc27
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == OUT0LOC::Loc28
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == OUT0LOC::Loc29
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == OUT0LOC::Loc30
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == OUT0LOC::Loc31
    }
}
#[doc = "Field `OUT0LOC` writer - I/O Location"]
pub type Out0locW<'a, REG> = crate::FieldWriter<'a, REG, 6, OUT0LOC>;
impl<'a, REG> Out0locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0LOC::Loc31)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUT1LOC {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
    #[doc = "7: Location 7"]
    Loc7 = 7,
    #[doc = "8: Location 8"]
    Loc8 = 8,
    #[doc = "9: Location 9"]
    Loc9 = 9,
    #[doc = "10: Location 10"]
    Loc10 = 10,
    #[doc = "11: Location 11"]
    Loc11 = 11,
    #[doc = "12: Location 12"]
    Loc12 = 12,
    #[doc = "13: Location 13"]
    Loc13 = 13,
    #[doc = "14: Location 14"]
    Loc14 = 14,
    #[doc = "15: Location 15"]
    Loc15 = 15,
    #[doc = "16: Location 16"]
    Loc16 = 16,
    #[doc = "17: Location 17"]
    Loc17 = 17,
    #[doc = "18: Location 18"]
    Loc18 = 18,
    #[doc = "19: Location 19"]
    Loc19 = 19,
    #[doc = "20: Location 20"]
    Loc20 = 20,
    #[doc = "21: Location 21"]
    Loc21 = 21,
    #[doc = "22: Location 22"]
    Loc22 = 22,
    #[doc = "23: Location 23"]
    Loc23 = 23,
    #[doc = "24: Location 24"]
    Loc24 = 24,
    #[doc = "25: Location 25"]
    Loc25 = 25,
    #[doc = "26: Location 26"]
    Loc26 = 26,
    #[doc = "27: Location 27"]
    Loc27 = 27,
    #[doc = "28: Location 28"]
    Loc28 = 28,
    #[doc = "29: Location 29"]
    Loc29 = 29,
    #[doc = "30: Location 30"]
    Loc30 = 30,
    #[doc = "31: Location 31"]
    Loc31 = 31,
}
impl From<OUT1LOC> for u8 {
    #[inline(always)]
    fn from(variant: OUT1LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OUT1LOC {
    type Ux = u8;
}
impl crate::IsEnum for OUT1LOC {}
#[doc = "Field `OUT1LOC` reader - I/O Location"]
pub type Out1locR = crate::FieldReader<OUT1LOC>;
impl Out1locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OUT1LOC> {
        match self.bits {
            0 => Some(OUT1LOC::Loc0),
            1 => Some(OUT1LOC::Loc1),
            2 => Some(OUT1LOC::Loc2),
            3 => Some(OUT1LOC::Loc3),
            4 => Some(OUT1LOC::Loc4),
            5 => Some(OUT1LOC::Loc5),
            6 => Some(OUT1LOC::Loc6),
            7 => Some(OUT1LOC::Loc7),
            8 => Some(OUT1LOC::Loc8),
            9 => Some(OUT1LOC::Loc9),
            10 => Some(OUT1LOC::Loc10),
            11 => Some(OUT1LOC::Loc11),
            12 => Some(OUT1LOC::Loc12),
            13 => Some(OUT1LOC::Loc13),
            14 => Some(OUT1LOC::Loc14),
            15 => Some(OUT1LOC::Loc15),
            16 => Some(OUT1LOC::Loc16),
            17 => Some(OUT1LOC::Loc17),
            18 => Some(OUT1LOC::Loc18),
            19 => Some(OUT1LOC::Loc19),
            20 => Some(OUT1LOC::Loc20),
            21 => Some(OUT1LOC::Loc21),
            22 => Some(OUT1LOC::Loc22),
            23 => Some(OUT1LOC::Loc23),
            24 => Some(OUT1LOC::Loc24),
            25 => Some(OUT1LOC::Loc25),
            26 => Some(OUT1LOC::Loc26),
            27 => Some(OUT1LOC::Loc27),
            28 => Some(OUT1LOC::Loc28),
            29 => Some(OUT1LOC::Loc29),
            30 => Some(OUT1LOC::Loc30),
            31 => Some(OUT1LOC::Loc31),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == OUT1LOC::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == OUT1LOC::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == OUT1LOC::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == OUT1LOC::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == OUT1LOC::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == OUT1LOC::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == OUT1LOC::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == OUT1LOC::Loc7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == OUT1LOC::Loc8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == OUT1LOC::Loc9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == OUT1LOC::Loc10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == OUT1LOC::Loc11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == OUT1LOC::Loc12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == OUT1LOC::Loc13
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == OUT1LOC::Loc14
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == OUT1LOC::Loc15
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == OUT1LOC::Loc16
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == OUT1LOC::Loc17
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == OUT1LOC::Loc18
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == OUT1LOC::Loc19
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == OUT1LOC::Loc20
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == OUT1LOC::Loc21
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == OUT1LOC::Loc22
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == OUT1LOC::Loc23
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == OUT1LOC::Loc24
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == OUT1LOC::Loc25
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == OUT1LOC::Loc26
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == OUT1LOC::Loc27
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == OUT1LOC::Loc28
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == OUT1LOC::Loc29
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == OUT1LOC::Loc30
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == OUT1LOC::Loc31
    }
}
#[doc = "Field `OUT1LOC` writer - I/O Location"]
pub type Out1locW<'a, REG> = crate::FieldWriter<'a, REG, 6, OUT1LOC>;
impl<'a, REG> Out1locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1LOC::Loc31)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn out0loc(&self) -> Out0locR {
        Out0locR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn out1loc(&self) -> Out1locR {
        Out1locR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTELOC0")
            .field("out0loc", &self.out0loc())
            .field("out1loc", &self.out1loc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn out0loc(&mut self) -> Out0locW<ROUTELOC0rs> {
        Out0locW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn out1loc(&mut self) -> Out1locW<ROUTELOC0rs> {
        Out1locW::new(self, 8)
    }
}
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC0rs;
impl crate::RegisterSpec for ROUTELOC0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc0::R`](R) reader structure"]
impl crate::Readable for ROUTELOC0rs {}
#[doc = "`write(|w| ..)` method takes [`routeloc0::W`](W) writer structure"]
impl crate::Writable for ROUTELOC0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTELOC0 to value 0"]
impl crate::Resettable for ROUTELOC0rs {
    const RESET_VALUE: u32 = 0;
}
