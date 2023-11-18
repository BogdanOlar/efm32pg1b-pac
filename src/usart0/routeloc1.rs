#[doc = "Register `ROUTELOC1` reader"]
pub type R = crate::R<ROUTELOC1_SPEC>;
#[doc = "Register `ROUTELOC1` writer"]
pub type W = crate::W<ROUTELOC1_SPEC>;
#[doc = "Field `CTSLOC` reader - I/O Location"]
pub type CTSLOC_R = crate::FieldReader<CTSLOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
    #[doc = "6: Location 6"]
    LOC6 = 6,
    #[doc = "7: Location 7"]
    LOC7 = 7,
    #[doc = "8: Location 8"]
    LOC8 = 8,
    #[doc = "9: Location 9"]
    LOC9 = 9,
    #[doc = "10: Location 10"]
    LOC10 = 10,
    #[doc = "11: Location 11"]
    LOC11 = 11,
    #[doc = "12: Location 12"]
    LOC12 = 12,
    #[doc = "13: Location 13"]
    LOC13 = 13,
    #[doc = "14: Location 14"]
    LOC14 = 14,
    #[doc = "15: Location 15"]
    LOC15 = 15,
    #[doc = "16: Location 16"]
    LOC16 = 16,
    #[doc = "17: Location 17"]
    LOC17 = 17,
    #[doc = "18: Location 18"]
    LOC18 = 18,
    #[doc = "19: Location 19"]
    LOC19 = 19,
    #[doc = "20: Location 20"]
    LOC20 = 20,
    #[doc = "21: Location 21"]
    LOC21 = 21,
    #[doc = "22: Location 22"]
    LOC22 = 22,
    #[doc = "23: Location 23"]
    LOC23 = 23,
    #[doc = "24: Location 24"]
    LOC24 = 24,
    #[doc = "25: Location 25"]
    LOC25 = 25,
    #[doc = "26: Location 26"]
    LOC26 = 26,
    #[doc = "27: Location 27"]
    LOC27 = 27,
    #[doc = "28: Location 28"]
    LOC28 = 28,
    #[doc = "29: Location 29"]
    LOC29 = 29,
    #[doc = "30: Location 30"]
    LOC30 = 30,
    #[doc = "31: Location 31"]
    LOC31 = 31,
}
impl From<CTSLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSLOC_A {
    type Ux = u8;
}
impl CTSLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTSLOC_A> {
        match self.bits {
            0 => Some(CTSLOC_A::LOC0),
            1 => Some(CTSLOC_A::LOC1),
            2 => Some(CTSLOC_A::LOC2),
            3 => Some(CTSLOC_A::LOC3),
            4 => Some(CTSLOC_A::LOC4),
            5 => Some(CTSLOC_A::LOC5),
            6 => Some(CTSLOC_A::LOC6),
            7 => Some(CTSLOC_A::LOC7),
            8 => Some(CTSLOC_A::LOC8),
            9 => Some(CTSLOC_A::LOC9),
            10 => Some(CTSLOC_A::LOC10),
            11 => Some(CTSLOC_A::LOC11),
            12 => Some(CTSLOC_A::LOC12),
            13 => Some(CTSLOC_A::LOC13),
            14 => Some(CTSLOC_A::LOC14),
            15 => Some(CTSLOC_A::LOC15),
            16 => Some(CTSLOC_A::LOC16),
            17 => Some(CTSLOC_A::LOC17),
            18 => Some(CTSLOC_A::LOC18),
            19 => Some(CTSLOC_A::LOC19),
            20 => Some(CTSLOC_A::LOC20),
            21 => Some(CTSLOC_A::LOC21),
            22 => Some(CTSLOC_A::LOC22),
            23 => Some(CTSLOC_A::LOC23),
            24 => Some(CTSLOC_A::LOC24),
            25 => Some(CTSLOC_A::LOC25),
            26 => Some(CTSLOC_A::LOC26),
            27 => Some(CTSLOC_A::LOC27),
            28 => Some(CTSLOC_A::LOC28),
            29 => Some(CTSLOC_A::LOC29),
            30 => Some(CTSLOC_A::LOC30),
            31 => Some(CTSLOC_A::LOC31),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CTSLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CTSLOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CTSLOC_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CTSLOC_A::LOC3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CTSLOC_A::LOC4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CTSLOC_A::LOC5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CTSLOC_A::LOC6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CTSLOC_A::LOC7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CTSLOC_A::LOC8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CTSLOC_A::LOC9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CTSLOC_A::LOC10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CTSLOC_A::LOC11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CTSLOC_A::LOC12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CTSLOC_A::LOC13
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == CTSLOC_A::LOC14
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == CTSLOC_A::LOC15
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == CTSLOC_A::LOC16
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == CTSLOC_A::LOC17
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == CTSLOC_A::LOC18
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == CTSLOC_A::LOC19
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == CTSLOC_A::LOC20
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == CTSLOC_A::LOC21
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == CTSLOC_A::LOC22
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == CTSLOC_A::LOC23
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == CTSLOC_A::LOC24
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == CTSLOC_A::LOC25
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == CTSLOC_A::LOC26
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == CTSLOC_A::LOC27
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == CTSLOC_A::LOC28
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == CTSLOC_A::LOC29
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == CTSLOC_A::LOC30
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == CTSLOC_A::LOC31
    }
}
#[doc = "Field `CTSLOC` writer - I/O Location"]
pub type CTSLOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CTSLOC_A>;
impl<'a, REG, const O: u8> CTSLOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC_A::LOC31)
    }
}
#[doc = "Field `RTSLOC` reader - I/O Location"]
pub type RTSLOC_R = crate::FieldReader<RTSLOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTSLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
    #[doc = "6: Location 6"]
    LOC6 = 6,
    #[doc = "7: Location 7"]
    LOC7 = 7,
    #[doc = "8: Location 8"]
    LOC8 = 8,
    #[doc = "9: Location 9"]
    LOC9 = 9,
    #[doc = "10: Location 10"]
    LOC10 = 10,
    #[doc = "11: Location 11"]
    LOC11 = 11,
    #[doc = "12: Location 12"]
    LOC12 = 12,
    #[doc = "13: Location 13"]
    LOC13 = 13,
    #[doc = "14: Location 14"]
    LOC14 = 14,
    #[doc = "15: Location 15"]
    LOC15 = 15,
    #[doc = "16: Location 16"]
    LOC16 = 16,
    #[doc = "17: Location 17"]
    LOC17 = 17,
    #[doc = "18: Location 18"]
    LOC18 = 18,
    #[doc = "19: Location 19"]
    LOC19 = 19,
    #[doc = "20: Location 20"]
    LOC20 = 20,
    #[doc = "21: Location 21"]
    LOC21 = 21,
    #[doc = "22: Location 22"]
    LOC22 = 22,
    #[doc = "23: Location 23"]
    LOC23 = 23,
    #[doc = "24: Location 24"]
    LOC24 = 24,
    #[doc = "25: Location 25"]
    LOC25 = 25,
    #[doc = "26: Location 26"]
    LOC26 = 26,
    #[doc = "27: Location 27"]
    LOC27 = 27,
    #[doc = "28: Location 28"]
    LOC28 = 28,
    #[doc = "29: Location 29"]
    LOC29 = 29,
    #[doc = "30: Location 30"]
    LOC30 = 30,
    #[doc = "31: Location 31"]
    LOC31 = 31,
}
impl From<RTSLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTSLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTSLOC_A {
    type Ux = u8;
}
impl RTSLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RTSLOC_A> {
        match self.bits {
            0 => Some(RTSLOC_A::LOC0),
            1 => Some(RTSLOC_A::LOC1),
            2 => Some(RTSLOC_A::LOC2),
            3 => Some(RTSLOC_A::LOC3),
            4 => Some(RTSLOC_A::LOC4),
            5 => Some(RTSLOC_A::LOC5),
            6 => Some(RTSLOC_A::LOC6),
            7 => Some(RTSLOC_A::LOC7),
            8 => Some(RTSLOC_A::LOC8),
            9 => Some(RTSLOC_A::LOC9),
            10 => Some(RTSLOC_A::LOC10),
            11 => Some(RTSLOC_A::LOC11),
            12 => Some(RTSLOC_A::LOC12),
            13 => Some(RTSLOC_A::LOC13),
            14 => Some(RTSLOC_A::LOC14),
            15 => Some(RTSLOC_A::LOC15),
            16 => Some(RTSLOC_A::LOC16),
            17 => Some(RTSLOC_A::LOC17),
            18 => Some(RTSLOC_A::LOC18),
            19 => Some(RTSLOC_A::LOC19),
            20 => Some(RTSLOC_A::LOC20),
            21 => Some(RTSLOC_A::LOC21),
            22 => Some(RTSLOC_A::LOC22),
            23 => Some(RTSLOC_A::LOC23),
            24 => Some(RTSLOC_A::LOC24),
            25 => Some(RTSLOC_A::LOC25),
            26 => Some(RTSLOC_A::LOC26),
            27 => Some(RTSLOC_A::LOC27),
            28 => Some(RTSLOC_A::LOC28),
            29 => Some(RTSLOC_A::LOC29),
            30 => Some(RTSLOC_A::LOC30),
            31 => Some(RTSLOC_A::LOC31),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == RTSLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == RTSLOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == RTSLOC_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == RTSLOC_A::LOC3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == RTSLOC_A::LOC4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == RTSLOC_A::LOC5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == RTSLOC_A::LOC6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == RTSLOC_A::LOC7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == RTSLOC_A::LOC8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == RTSLOC_A::LOC9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == RTSLOC_A::LOC10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == RTSLOC_A::LOC11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == RTSLOC_A::LOC12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == RTSLOC_A::LOC13
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == RTSLOC_A::LOC14
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == RTSLOC_A::LOC15
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == RTSLOC_A::LOC16
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == RTSLOC_A::LOC17
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == RTSLOC_A::LOC18
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == RTSLOC_A::LOC19
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == RTSLOC_A::LOC20
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == RTSLOC_A::LOC21
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == RTSLOC_A::LOC22
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == RTSLOC_A::LOC23
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == RTSLOC_A::LOC24
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == RTSLOC_A::LOC25
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == RTSLOC_A::LOC26
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == RTSLOC_A::LOC27
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == RTSLOC_A::LOC28
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == RTSLOC_A::LOC29
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == RTSLOC_A::LOC30
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == RTSLOC_A::LOC31
    }
}
#[doc = "Field `RTSLOC` writer - I/O Location"]
pub type RTSLOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, RTSLOC_A>;
impl<'a, REG, const O: u8> RTSLOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC_A::LOC31)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ctsloc(&self) -> CTSLOC_R {
        CTSLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn rtsloc(&self) -> RTSLOC_R {
        RTSLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTELOC1")
            .field("ctsloc", &format_args!("{}", self.ctsloc().bits()))
            .field("rtsloc", &format_args!("{}", self.rtsloc().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ROUTELOC1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ctsloc(&mut self) -> CTSLOC_W<ROUTELOC1_SPEC, 0> {
        CTSLOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn rtsloc(&mut self) -> RTSLOC_W<ROUTELOC1_SPEC, 8> {
        RTSLOC_W::new(self)
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
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC1_SPEC;
impl crate::RegisterSpec for ROUTELOC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc1::R`](R) reader structure"]
impl crate::Readable for ROUTELOC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routeloc1::W`](W) writer structure"]
impl crate::Writable for ROUTELOC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTELOC1 to value 0"]
impl crate::Resettable for ROUTELOC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
