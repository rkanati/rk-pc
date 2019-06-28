
struct Ctx<'a> {
    bytes:    &'a [u8],
    consumed: u32,
}

impl Ctx {
    fn byte(&mut self) -> u8 {
        let byte = self.bytes[0];
        self.bytes = &self.bytes[1..];
        self.consumed += 1;
        byte
    }

    fn word(&mut self) -> u16 {
        let word = u16::from_le_bytes(&self.bytes[0..1]);
        self.bytes = &self.bytes[2..];
        self.consumed += 2;
        word
    }
}

pub enum BaseReg { BX, BP }
pub enum IndexReg { SI, DI }

#[derive(Clone, Copy, Debug)]
pub enum ModRMRef {
    Reg(u8),
    Mem {
        base:  Option<BaseReg>,
        index: Option<IndexReg>,
        disp:  Option<u16>,
    },
}

pub struct ModRM {
    reference: ModRMRef,
    middle:    u8
}

impl ModRM {
    fn get(ctx: &mut Ctx) -> ModRM {
        let byte = ctx.byte();

        let mode   =  byte >> 6;
        let middle = (byte >> 3) & 7;
        let rm     = (byte >> 0) & 3;

        let reference = if mode != 3 {
            let (base, index) = match rm {
                0 => (Some(BaseReg::BX), Some(IndexReg::SI)),
                1 => (Some(BaseReg::BX), Some(IndexReg::DI)),
                2 => (Some(BaseReg::BP), Some(IndexReg::SI)),
                3 => (Some(BaseReg::BP), Some(IndexReg::DI)),
                4 => (None,              Some(IndexReg::SI)),
                5 => (None,              Some(IndexReg::DI)),
                6 => (None,              None,             ),
                7 => (Some(BaseReg::BX), None,             ),
                _ => unreachable!()
            };

            let disp = match mode {
                0 if rm == 6 => Some(ctx.word()),
                1            => Some(ctx.byte().sign_extend()),
                2            => Some(ctx.word()),
                _            => None
            };

            ModRMRef::Mem { base, index, disp }
        }
        else {
            ModRMRef::Reg(rm)
        };

        ModRM { middle, reference }
    }
}

#[derive(Clone, Copy, Debug)]
enum Error {
}

impl Display for Error {
    // TODO
}

impl error::Error for Error { }

type Result = result::Result<(Instruction, usize), Error>;
type DecodeFn = fn(Ctx) -> Result;

static OPCODES: [DecodeFn; 256] = [
//  0/8     1/9     2/a     3/b     4/c     5/d     6/e     7/f
    add,    add,    add,    add,    add,    add,    push,   pop,    // 00
    or,     or,     or,     or,     or,     or,     push,   pop,
    adc,    adc,    adc,    adc,    adc,    adc,    push,   pop,    // 10
    sbb,    sbb,    sbb,    sbb,    sbb,    sbb,    push,   pop,
    and,    and,    and,    and,    and,    and,    segpfx, daa,    // 20
    sub,    sub,    sub,    sub,    sub,    sub,    segpfx, das,
    xor,    xor,    xor,    xor,    xor,    xor,    segpfx, aaa,    // 30
    cmp,    cmp,    cmp,    cmp,    cmp,    cmp,    segpfx, aas,
    inc,    inc,    inc,    inc,    inc,    inc,    inc,    inc,    // 40
    dec,    dec,    dec,    dec,    dec,    dec,    dec,    dec,
    push,   push,   push,   push,   push,   push,   push,   push,   // 50
    pop,    pop,    pop,    pop,    pop,    pop,    pop,    pop,
    j_cc,   j_cc,   j_cc,   j_cc,   j_cc,   j_cc,   j_cc,   j_cc,   // 60
    j_cc,   j_cc,   j_cc,   j_cc,   j_cc,   j_cc,   j_cc,   j_cc,
    j_cc,   j_cc,   j_cc,   j_cc,   j_cc,   j_cc,   j_cc,   j_cc,   // 70
    j_cc,   j_cc,   j_cc,   j_cc,   j_cc,   j_cc,   j_cc,   j_cc,
    etc1,   etc1,   etc1,   etc1,   test,   test,   xchg,   xchg,   // 80
    mov,    mov,    mov,    mov,    mov,    lea,    mov,    pop,
    nop,    xchg,   xchg,   xchg,   xchg,   xchg,   xchg,   xchg,   // 90
    cbw,    cwd,    call,   wait,   pushf,  popf,   sahf,   lahf,
    mov,    mov,    mov,    mov,    movsb,  movsw,  cmpsb,  cmpsw,  // a0
    test,   test,   stosb,  stosw,  lodsb,  lodsw,  scasb,  scasw,
    mov,    mov,    mov,    mov,    mov,    mov,    mov,    mov,    // b0
    mov,    mov,    mov,    mov,    mov,    mov,    mov,    mov,
    unk,    unk,    retn,   retn,   les,    lds,    mov,    mov,    // c0
    unk,    unk,    retf,   retf,   int,    int,    into,   iret,
    etc2,   etc2,   etc2,   etc2,   aam,    aad,    salc,   xlat,   // d0
    esc,    esc,    esc,    esc,    esc,    esc,    esc,    esc,
    loop_,  loop_,  loop_,  j_cc,   in_,    in_,    out,    out,    // e0
    call,   jmp,    jmp,    jmp,    in_,    in_,    out,    out,
    lock,   unk,    rep,    rep,    hlt,    cmc,    etc3a,  etc3b,  // f0
    clc,    stc,    cli,    sti,    cld,    std,    etc4,   etc5
];

fn aaa (ctx: Ctx) -> Result {
}

fn aad (ctx: Ctx) -> Result {
}

fn aam (ctx: Ctx) -> Result {
}

fn adc (ctx: Ctx) -> Result {
}

fn add (ctx: Ctx) -> Result {
}

fn and (ctx: Ctx) -> Result {
}

fn arith (ctx: Ctx) -> Result {
}

fn call (ctx: Ctx) -> Result {
}

fn cbw (ctx: Ctx) -> Result {
}

fn clc (ctx: Ctx) -> Result {
}

fn cld (ctx: Ctx) -> Result {
}

fn cli (ctx: Ctx) -> Result {
}

fn cmc (ctx: Ctx) -> Result {
}

fn cmp (ctx: Ctx) -> Result {
}

fn cmpsb (ctx: Ctx) -> Result {
}

fn cmpsw (ctx: Ctx) -> Result {
}

fn cwd (ctx: Ctx) -> Result {
}

fn daa (ctx: Ctx) -> Result {
}

fn das (ctx: Ctx) -> Result {
}

fn dec (ctx: Ctx) -> Result {
}

fn esc (ctx: Ctx) -> Result {
}

fn etc (ctx: Ctx) -> Result {
}

fn hlt (ctx: Ctx) -> Result {
}

fn imul (ctx: Ctx) -> Result {
}

fn in_ (ctx: Ctx) -> Result {
}

fn inc (ctx: Ctx) -> Result {
}

fn int (ctx: Ctx) -> Result {
}

fn into (ctx: Ctx) -> Result {
}

fn iret (ctx: Ctx) -> Result {
}

fn j_cc (ctx: Ctx) -> Result {
    // 0x6x are just aliases for 0x7x on 8086/8088
    let first = if first < 0x60 { first + 0x10 } else { first };
}

fn jmp (ctx: Ctx) -> Result {
}

fn lahf (ctx: Ctx) -> Result {
}

fn lds (ctx: Ctx) -> Result {
}

fn lea (ctx: Ctx) -> Result {
}

fn les (ctx: Ctx) -> Result {
}

fn lock (ctx: Ctx) -> Result {
}

fn lodsb (ctx: Ctx) -> Result {
}

fn lodsw (ctx: Ctx) -> Result {
}

fn loop_ (ctx: Ctx) -> Result {
}

fn movsb (ctx: Ctx) -> Result {
}

fn movsw (ctx: Ctx) -> Result {
}

fn nop (ctx: Ctx) -> Result {
}

fn or (ctx: Ctx) -> Result {
}

fn out (ctx: Ctx) -> Result {
}

fn pop (ctx: Ctx) -> Result {
    match ctx[0] {
        0x8f => {
            let modrm = ModRM::from(ctx);
            assert!(modrm.middle() == 0);
            Ok((Instruction::PopRM(modrm), modrm.len()))
        }
        0x58 ... 0x5f => Ok((Instruction::PopR(ctx[0] & 7), 1))
        0x06 | 0x0e | 0x16 | 0x1e => Ok((Instruction::PopSR((ctx[0] >> 3) & 3), 1))
    }
}

fn popf (ctx: Ctx) -> Result {
}

fn push (ctx: Ctx) -> Result {
    match ctx[0] {
        0xff => {
            let modrm = ModRM::from(ctx);
            assert!(modrm.middle() == 6);
            Ok((Instruction::PushRM(modrm), modrm.len()))
        }
        0x50 ... 0x57 => Ok((Instruction::PushR(ctx[0] & 7), 1))
        0x06 | 0x0e | 0x16 | 0x1e => Ok((Instruction::PushSR((ctx[0] >> 3) & 3), 1))
    }
}

fn pushf (ctx: Ctx) -> Result {
}

fn r_s (ctx: Ctx) -> Result {
}

fn rep (ctx: Ctx) -> Result {
}

fn retf (ctx: Ctx) -> Result {
}

fn retn (ctx: Ctx) -> Result {
}

fn sahf (ctx: Ctx) -> Result {
}

fn sbb (ctx: Ctx) -> Result {
}

fn scasb (ctx: Ctx) -> Result {
}

fn scasw (ctx: Ctx) -> Result {
}

fn stc (ctx: Ctx) -> Result {
}

fn std (ctx: Ctx) -> Result {
}

fn sti (ctx: Ctx) -> Result {
}

fn stosb (ctx: Ctx) -> Result {
}

fn stosw (ctx: Ctx) -> Result {
}

fn sub (ctx: Ctx) -> Result {
}

fn test (ctx: Ctx) -> Result {
}

fn wait (ctx: Ctx) -> Result {
}

fn xchg (ctx: Ctx) -> Result {
}

fn xlat (ctx: Ctx) -> Result {
}

fn xor (ctx: Ctx) -> Result {
}

fn mov (ctx: Ctx) -> Result {
    let b0 = ctx.byte();
    match b0 {
        0x88 ..= 0x8b => {
            let d = b0 & 2 != 0;
            let w = b0 & 1 != 0;
            let modrm = ModRM::from(ctx);
            Ok((Instruction::MovR_RM(d, w, modrm), modrm.len()))
        }
        0xc6 | 0xc7 => {
            let w = b0 & 1 != 0;
            let modrm = ModRM::from(ctx);
            let ctx = ctx.consume(modrm.len());
            if b0 == 0xc6 {
                Ok((Instruction::MovRM_IB(w, modrm, b0), modrm.len() + 1))
            }
            else {
                let word = ctx.immw();
                Ok((Instruction::MovRM_IW(w, modrm, word), modrm.len() + 2))
            }
        }
        0xb0 ..= 0xbf => {
            let w = b0 & 8 != 0;
            let reg = b0 & 7;
            if w {
                let (ctx, word) = ctx.word();
                Ok((Instruction::MovR_IW(reg, word), 3))
            }
            else {
                let (ctx, byte) = ctx.byte();
                Ok((Instruction::MovR_IB(reg, byte), 2))
            }
        }
        0xa0 | 0xa1 => {
            let addr = ctx.word();
            if b0 == 0xa0 {
                Ok((Instruction::MovA_M(
            }
            else {
            }
        }
    }
}

