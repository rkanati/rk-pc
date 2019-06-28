
mod instruction;

#[derive(Clone, Copy, Debug, Default)]
struct SplitReg { l: u8, h: u8 }

impl SplitReg {
}

impl From<SplitReg> for u16 {
    fn from(r: SplitReg) -> u16 {
        (r.h as u16) << 8 | r.l as u16
    }
}

impl From<u16> for SplitReg {
    fn from(w: u16) -> SplitReg {
        SplitReg { l: w as u8, h: (w >> 8) as u8 }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct SegReg(u16);

impl SegReg {
    fn make_addr(self, offset: u16) -> u32 {
        (self.0 as u32) << 4 + offset as u32
    }
}

bitfield! {
    #[derive(Clone, Copy, Debug, Default)]
    struct Flags(u16);
    pub c, set_c:  0;
    pub p, set_p:  2;
    pub a, set_a:  4;
    pub z, set_z:  6;
    pub s, set_s:  7;
    pub t, set_t:  8;
    pub i, set_i:  9;
    pub d, set_d: 10;
    pub o, set_o: 11;
}

struct Regs {
    a: SplitReg,
    b: SplitReg,
    c: SplitReg,
    d: SplitReg,

    sp: u16,
    bp: u16,

    si: u16,
    di: u16,

    cs: SegReg,
    ds: SegReg,
    es: SegReg,
    ss: SegReg,

    ip: u16,
    flags: Flags,
}

pub struct I8086 {
    regs: Regs,
}

