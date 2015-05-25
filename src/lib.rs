use std::mem::transmute;

pub trait WriteToBitBuf {
	fn write_to_bitbuf(&self, buf: &mut BitBuf);
}

pub trait FromBitBuf {
    fn from_bitbuf(buf: &mut BitBuf) -> Self;
}

struct FourByte {
    b1: u8,
    b2: u8,
    b3: u8,
    b4: u8,
}

impl FourByte {
    pub fn trans_from_f32(value: f32) -> FourByte {
        unsafe { transmute::<f32, FourByte>(value) } 
    }

    pub fn trans_to_f32(self) -> f32 {
        unsafe { transmute::<FourByte, f32>(self) }
    }
}

struct EightByte {
    b1: u8,
    b2: u8,
    b3: u8,
    b4: u8,
    b5: u8,
    b6: u8,
    b7: u8,
    b8: u8,
}

impl EightByte {
    pub fn trans_from_f64(value: f64) -> EightByte {
        unsafe { transmute::<f64, EightByte>(value) }
    }

    pub fn trans_to_f64(self) -> f64 {
        unsafe { transmute::<EightByte, f64>(self) }
    }
}

#[derive(Clone)]
pub struct BitBuf {
    buf: Vec<u8>,
    pos: usize,       // The current bit position of the cursor.
    size: usize,      // Size in bits.
}

impl BitBuf {

    /// Creates a new BitBuf, initializing a new Vec<u8>.
    /// for the underlying buffer.
    pub fn with_len(len: usize) -> BitBuf {
        let mut vec = Vec::with_capacity(len);
        unsafe { vec.set_len(len) };
        for x in &mut vec { *x = 0; }
        BitBuf {
            buf: vec,
            pos: 0,
            size: len * 8,
        }
    }

    /// Consumes the BitBuf, returning the underlying Vec<u8>.
    pub fn to_vec(self) -> Vec<u8> {
        self.buf
    }

    /// Returns a slice into the underlying Vec<u8> buffer.
    //pub fn buf_as_slice(&self) -> &[u8] {
    //    self.buf.as_slice()
    //}

    /// The current bit size of the Vec<u8>.
    pub fn bit_size(&self) -> usize {
        self.size
    }

    /// The current position of the cursor. The BitBuf
    /// does not insert, and will overwrite any data currently
    /// at the cursor position during writing.
    pub fn bit_pos(&self) -> usize {
        self.pos
    }

    pub fn byte_pos(&self) -> usize {
        self.pos / 8
    }

    pub fn can_write_bits(&self, bit_count: usize) -> bool {
        (bit_size + self.pos) < self.size
    }

    pub fn can_read_bits(&self, bit_count: usize) -> bool {
        (bit_size + self.pos) < self.size
    }

    pub fn can_write_bytes(&self, byte_count: usize) -> bool {
        self.can_write_bits(byte_count * 8)
    }

    pub fn can_read_bytes(&self, byte_count: usize) -> bool {
        self.can_read_bits(byte_count * 8)
    }

    pub fn write_bool(&mut self, value: bool) {
        self.in_write_byte((if value {1} else {0}), 1);
    }

    pub fn read_bool(&mut self) -> bool {
        self.in_read_byte(1) == 1
    }

    pub fn write_i8(&mut self, value: i8) {
        self.write_i8_part(value, 8);
    }

    pub fn read_i8(&mut self) -> i8 {
        self.read_i8_part(8)
    }

    fn write_i8_part(&mut self, value: i8, bits: u8) {
        self.in_write_byte(value as u8, bits);
    }

    fn read_i8_part(&mut self, bits: u8) -> i8 {
        self.in_read_byte(bits) as i8
    }

    pub fn write_u8(&mut self, value: u8) {
        self.write_u8_part(value, 8);
    }

    pub fn read_u8(&mut self) -> u8 {
        self.read_u8_part(8)
    }

    pub fn write_u8_part(&mut self, value: u8, bits: u8) {
        self.in_write_byte(value, bits);
    }

    pub fn read_u8_part(&mut self, bits: u8) -> u8 {
        self.in_read_byte(bits)
    }

    pub fn write_u16(&mut self, value: u16) {
        self.write_u16_part(value, 16);
    }

    pub fn read_u16(&mut self) -> u16 {
        self.read_u16_part(16)
    }

    pub fn write_u16_part(&mut self, value: u16, bits: u8) {
        let a = (value >> 0) as u8;
        let b = (value >> 8) as u8;

        match (bits + 7) / 8 {
            1 => {
                self.in_write_byte(a, bits);
            },
            2 => {
                self.in_write_byte(a, 8);
                self.in_write_byte(b, bits - 8);
            },
            _ => {
                //panic!("Must write between 1 and 32 bits.")
            }
        }
    }

    pub fn read_u16_part(&mut self, bits: u8) -> u16 {
        let mut a = 0u16;
        let mut b = 0u16;

        match (bits + 7) / 8 {
            1 => {
                a = self.in_read_byte(bits) as u16;
            },
            2 => {
                a = self.in_read_byte(8) as u16;
                b = self.in_read_byte(bits - 8) as u16;
            },
            _ => {
                //panic!("Must read between 1 and 32 bits.")
            }
        }

        (a | (b << 8)) as u16
    }

    pub fn write_i16(&mut self, value: i16) {
        self.write_i16_part(value, 16);
    }

    pub fn read_i16(&mut self) -> i16 {
        self.read_i16_part(16)
    }

    fn write_i16_part(&mut self, value: i16, bits: u8) {
        self.write_u16_part(value as u16, bits);
    }

    fn read_i16_part(&mut self, bits: u8) -> i16 {
        self.read_u16_part(bits) as i16
    }

    pub fn write_u32(&mut self, value: u32) {
        self.write_u32_part(value, 32);
    }

    pub fn read_u32(&mut self) -> u32 {
        self.read_u32_part(32)
    }

    pub fn write_u32_part(&mut self, value: u32, bits: u8) {
        let a = (value >> 0) as u8;
        let b = (value >> 8) as u8;
        let c = (value >> 16) as u8;
        let d = (value >> 24) as u8;

        match (bits + 7) / 8 {
            1 => {
                self.in_write_byte(a, bits);
            },
            2 => {
                self.in_write_byte(a, 8);
                self.in_write_byte(b, bits - 8);
            },
            3 => {
                self.in_write_byte(a, 8);
                self.in_write_byte(b, 8);
                self.in_write_byte(c, bits - 16);
            },
            4 => {
                self.in_write_byte(a, 8);
                self.in_write_byte(b, 8);
                self.in_write_byte(c, 8);
                self.in_write_byte(d, bits - 24);
            },
            _ => {
                //panic!("Must write between 1 and 32 bits.")
            }
        }
    }

    pub fn read_u32_part(&mut self, bits: u8) -> u32 {
        let mut a = 0i32;
        let mut b = 0i32;
        let mut c = 0i32;
        let mut d = 0i32;

        match (bits + 7) / 8 {
            1 => {
                a = self.in_read_byte(bits) as i32;
            },
            2 => {
                a = self.in_read_byte(8) as i32;
                b = self.in_read_byte(bits - 8) as i32;
            },
            3 => {
                a = self.in_read_byte(8) as i32;
                b = self.in_read_byte(8) as i32;
                c = self.in_read_byte(bits - 16) as i32;
            },
            4 => {
                a = self.in_read_byte(8) as i32;
                b = self.in_read_byte(8) as i32;
                c = self.in_read_byte(8) as i32;
                d = self.in_read_byte(bits - 24) as i32;
            },
            _ => {
                //panic!("Must read between 1 and 32 bits.")
            }
        }

        (a | (b << 8) | (c << 16) | (d << 24)) as u32
    }

    pub fn read_i32(&mut self) -> i32 {
        self.read_i32_part(32)
    }

    pub fn write_i32(&mut self, value: i32) {
        self.write_i32_part(value, 32);
    }

    fn write_i32_part(&mut self, value: i32, bits: u8) {
        self.write_u32_part(value as u32, bits);
    }

    fn read_i32_part(&mut self, bits: u8) -> i32 {
        self.read_u32_part(bits) as i32
    }

    pub fn write_u64(&mut self, value: u64) {
        self.write_u64_part(value, 64);
    }

    pub fn read_u64(&mut self) -> u64 {
        self.read_u64_part(64)
    }

    pub fn write_u64_part(&mut self, value: u64, bits: u8) {
        if bits <= 32 {
            self.write_u32_part((value & 0xFFFFFFFF) as u32, bits);
        } else {
            self.write_u32_part(value as u32, 32);
            self.write_u32_part((value >> 32) as u32, bits - 32);
        }
    }

    pub fn read_u64_part(&mut self, bits: u8) -> u64 {
        if bits <= 32 {
            self.read_u32_part(bits) as u64
        } else {
            let a = self.read_u32_part(32) as u64;
            let b = self.read_u32_part(bits - 32) as u64;
            a | (b << 32)
        }
    }

    pub fn write_i64(&mut self, value: i64) {
        self.write_u64_part(value as u64, 64);
    }

    pub fn read_i64(&mut self) -> i64 {
        self.read_u64_part(64) as i64
    }

    fn write_i64_part(&mut self, value: i64, bits: u8) {
        self.write_u64_part(value as u64, bits);
    }

    fn read_i64_part(&mut self, bits: u8) -> i64 {
        self.read_u64_part(bits) as i64
    }

    pub fn write_f32(&mut self, value: f32) {
        let trans = FourByte::trans_from_f32(value);
        self.in_write_byte(trans.b1, 8);
        self.in_write_byte(trans.b2, 8);
        self.in_write_byte(trans.b3, 8);
        self.in_write_byte(trans.b4, 8);
    }

    pub fn read_f32(&mut self) -> f32 {
        FourByte {
            b1: self.in_read_byte(8),
            b2: self.in_read_byte(8),
            b3: self.in_read_byte(8),
            b4: self.in_read_byte(8),
        }.trans_to_f32()
    }

    pub fn write_f64(&mut self, value: f64) {
        let trans = EightByte::trans_from_f64(value);
        self.in_write_byte(trans.b1, 8);
        self.in_write_byte(trans.b2, 8);
        self.in_write_byte(trans.b3, 8);
        self.in_write_byte(trans.b4, 8);
        self.in_write_byte(trans.b5, 8);
        self.in_write_byte(trans.b6, 8);
        self.in_write_byte(trans.b7, 8);
        self.in_write_byte(trans.b8, 8);
    }

    pub fn read_f64(&mut self) -> f64 {
        EightByte {
            b1: self.in_read_byte(8),
            b2: self.in_read_byte(8),
            b3: self.in_read_byte(8),
            b4: self.in_read_byte(8),
            b5: self.in_read_byte(8),
            b6: self.in_read_byte(8),
            b7: self.in_read_byte(8),
            b8: self.in_read_byte(8),
        }.trans_to_f64()
    }

    pub fn write_u8_slice(&mut self, value: &[u8]) {
        for i in 0..value.len() {
            self.in_write_byte(value[i], 8);
        }
    }

    pub fn read_vec_u8(&mut self, length: usize) -> Vec<u8> {
        (0..length).map(|_| self.in_read_byte(8)).collect()
    }

    pub fn write_string(&mut self, value: &str) {
        self.write_u32(value.len() as u32);
        self.write_u8_slice(value.as_bytes());
    }

    pub fn read_string(&mut self) -> String {
        let len = self.read_u32() as usize;
        String::from_utf8(self.read_vec_u8(len)).unwrap()
    }

    #[inline(always)]
    fn in_write_byte(&mut self, mut value: u8, bits: u8) {
        value = value & (0xFF >> (8 - bits));

        let p = (self.pos >> 3) as usize;
        let bits_used = self.pos & 0x7;

        if bits_used == 0 {
            self.buf[p] = value;
        } else {
            let bits_free = 8 - bits_used;
            let bits_left: i16 = bits_free as i16 - bits as i16;

            if bits_left >= 0 {
                let mask = (0xFF >> bits_free) | (0xFF << (8 - bits_left));
                self.buf[p] = (self.buf[p] & mask) | (value << bits_used);
            } else {
                self.buf[p] = (self.buf[p] & (0xFF >> bits_free)) | (value << bits_used);
                self.buf[p + 1] = (self.buf[p + 1] & (0xFF << (bits - bits_free as u8))) | (value >> bits_free);
            }
        }

        self.pos += bits as usize;
    }

    #[inline(always)]
    fn in_read_byte(&mut self, bits: u8) -> u8 {
        let value: u8;
        let p = (self.pos >> 3) as usize;
        let bits_used = self.pos % 8;

        if bits_used == 0 && bits == 8 {
            value = self.buf[p];
        } else {
            let first = self.buf[p] >> bits_used;
            let remainder = bits - (8 - bits_used as u8);
            if remainder < 1 {
                value = first & (0xFF >> (8 - bits));
            } else {
                let second = self.buf[p + 1] & (0xFF >> (8 - remainder));
                value = first | (second << (bits - remainder));
            }
        }

        self.pos += bits as usize;
        value
    }

}



#[test]
fn bool_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = true;
    buf.write_bool(testval);
    buf.pos = 0;
    assert!(buf.read_bool() == testval);
}

#[test]
fn u8_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = 211;
    buf.write_u8(testval);
    buf.pos = 0;
    assert!(buf.read_u8() == testval);
}

#[test]
fn u8_part_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = 15;
    buf.write_u8_part(testval, 4);
    buf.pos = 0;
    assert!(buf.read_u8_part(4) == testval);
}

#[test]
fn i8_part_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = 6;
    buf.write_i8_part(testval, 4);
    buf.pos = 0;
    assert!(buf.read_i8_part(4) == testval);
}

#[test]
fn i8_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = -109;
    buf.write_i8(testval);
    buf.pos = 0;
    assert!(buf.read_i8() == testval);
}

#[test]
fn u16_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = 34507;
    buf.write_u16(testval);
    buf.pos = 0;
    assert!(buf.read_u16() == testval);
}

#[test]
fn u16_part_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = 448;
    buf.write_u16_part(testval, 13);
    buf.pos = 0;
    let result = buf.read_u16_part(13);
    println!("{}", result);
    assert!(result == testval);
}

#[test]
fn i16_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = -11066;
    buf.write_i16(testval);
    buf.pos = 0;
    assert!(buf.read_i16() == testval);
}

#[test]
fn i16_part_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = 10034;
    buf.write_i16_part(testval, 15);
    buf.pos = 0;
    assert!(buf.read_i16_part(15) == testval);
}

#[test]
fn u32_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = 193772;
    buf.write_u32(testval);
    buf.pos = 0;
    assert!(buf.read_u32() == testval);
}

#[test]
fn u32_part_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = 839011;
    buf.write_u32_part(testval, 27);
    buf.pos = 0;
    assert!(buf.read_u32_part(27) == testval);
}

#[test]
fn i32_part_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = 54397;
    buf.write_i32_part(testval, 22);
    buf.pos = 0;
    assert!(buf.read_i32_part(22) == testval);
}

#[test]
fn i32_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = -23498225;
    buf.write_i32(testval);
    buf.pos = 0;
    assert!(buf.read_i32() == testval);
}

#[test]
fn u64_part_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = 32944949231715;
    buf.write_u64_part(testval, 59);
    buf.pos = 0;
    assert!(buf.read_u64_part(59) == testval);
}

#[test]
fn u64_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = 248394023907611;
    buf.write_u64(testval);
    buf.pos = 0;
    assert!(buf.read_u64() == testval);
}

#[test]
fn i64_part_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = 1998372011;
    buf.write_i64_part(testval, 50);
    buf.pos = 0;
    assert!(buf.read_i64_part(50) == testval);
}

#[test]
fn i64_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = -24839402390;
    buf.write_i64(testval);
    buf.pos = 0;
    assert!(buf.read_i64() == testval);
}

#[test]
fn f32_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = 3.0393124f32;
    buf.write_f32(testval);
    buf.pos = 0;
    assert!(buf.read_f32() == testval);
}

#[test]
fn f64_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = 3.0395831239485302f64;
    buf.write_f64(testval);
    buf.pos = 0;
    assert!(buf.read_f64() == testval);
}

#[test]
fn string_writeread_equal() {
    let mut buf = BitBuf::with_len(1400);
    let testval = "This is a test string. Nothing to see here. No, really!";
    buf.write_string(testval);
    buf.pos = 0;
    assert!(buf.read_string() == testval);
}

//struct BenchPerson {
//    first_name: String,
//    last_name: String,
//    age: i8,
//    alive: bool,
//    weight: i16,
//}
//
//impl WriteToBitBuf for BenchPerson {
//    fn write_to_bitbuf(&self, buf: &mut BitBuf) {
//        buf.write_string(&self.first_name);
//        buf.write_string(&self.last_name);
//        buf.write_i8(self.age);
//        buf.write_bool(self.alive);
//        buf.write_i16(self.weight);
//    }
//}
//
//impl FromBitBuf for BenchPerson {
//    fn from_bitbuf(buf: &mut BitBuf) -> BenchPerson {
//        BenchPerson {
//            first_name: buf.read_string(),
//            last_name: buf.read_string(),
//            age: buf.read_i8(),
//            alive: buf.read_bool(),
//            weight: buf.read_i16(),
//        }
//    }
//}
//#[bench]
//fn benchperson_write1400_bench(b: &mut Bencher) {
//    let mut buf = BitBuf::with_len(1400);
//    let person = BenchPerson {
//        first_name: String::from_str("John"),
//        last_name: String::from_str("Johnson"),
//        age: 47,
//        alive: true,
//        weight: 203,
//    };
//    b.iter(|| {
//        buf.pos = 0;
//        for _ in 0..63 {
//            person.write_to_bitbuf(&mut buf);
//        }
//    })
//}
//
//#[bench]
//fn benchperson_read1400_bench(b: &mut Bencher) {
//    let mut buf = BitBuf::with_len(1400);
//    let person = BenchPerson {
//        first_name: String::from_str("John"),
//        last_name: String::from_str("Johnson"),
//        age: 47,
//        alive: true,
//        weight: 203,
//    };
//    for _ in 0..63 {
//        person.write_to_bitbuf(&mut buf);
//    }
//    b.iter(|| {
//        buf.pos = 0;
//        for _ in 0..63 {
//            let p: BenchPerson = FromBitBuf::from_bitbuf(&mut buf);
//        }
//    })
//}
//
//#[bench]
//fn bitbuf_create_bench(b: &mut Bencher) {
//    b.iter(|| {
//        let mut buf = BitBuf::with_len(1400);
//    })
//}
//
//#[bench]
//fn in_byte_write1400_bench(b: &mut Bencher) {
//    let mut buf = BitBuf::with_len(1400);
//    b.iter(|| {
//        buf.pos = 0;
//        for _ in 0..1400 {
//            buf.in_write_byte(240, 8);
//        }
//    })
//}
//
//#[bench]
//fn in_byte_read1400_bench(b: &mut Bencher) {
//    let mut buf = BitBuf::with_len(1400);
//    for _ in 0..1400 {
//            buf.in_write_byte(240, 8);
//    }
//    b.iter(|| {
//        buf.pos = 0;
//        for _ in 0..1400 {
//            let b = buf.in_read_byte(8);
//        }
//    })
//}
//
//#[bench]
//fn string_write1400_bench(b: &mut Bencher) {
//    let mut buf = BitBuf::with_len(1400);
//    b.iter(|| {
//        buf.pos = 0;
//        for _ in 0..50 {
//            buf.write_string("This is a string. Woo!!!");
//        }
//    })
//}
//
//#[bench]
//fn string_read1400_bench(b: &mut Bencher) {
//    let mut buf = BitBuf::with_len(1400);
//    for _ in 0..50 {
//        buf.write_string("This is a string. Woo!!!");
//    }
//    b.iter(|| {
//        buf.pos = 0;
//        for _ in 0..50 {
//            let s = buf.read_string();
//        }
//    })
//}
//
//#[bench]
//fn i32_write1400_bench(b: &mut Bencher) {
//    let mut buf = BitBuf::with_len(1400);
//    b.iter(|| {
//        buf.pos = 0;
//        for _ in 0..350 {
//            buf.write_i32(123239012);
//        }
//    })
//}
//
//#[bench]
//fn i64_write1400_bench(b: &mut Bencher) {
//    let mut buf = BitBuf::with_len(1400);
//    b.iter(|| {
//        buf.pos = 0;
//        for _ in 0..175 {
//            buf.write_i64(12352390123458);
//        }
//    })
//}
//
//#[bench]
//fn i64_read1400_bench(b: &mut Bencher) {
//    let mut buf = BitBuf::with_len(1400);
//    for _ in 0..175 {
//        buf.write_i64(12352390123458);
//    }
//    b.iter(|| {
//        buf.pos = 0;
//        for _ in 0..175 {
//            let i = buf.read_i64();
//        }
//    })
//}
//
//#[bench]
//fn f32_write1400_bench(b: &mut Bencher) {
//    let mut buf = BitBuf::with_len(1400);
//    b.iter(|| {
//        buf.pos = 0;
//        for _ in 0..350 {
//            buf.write_f32(123.239012f32);
//        }
//    })
//}
//
//#[bench]
//fn f64_write1400_bench(b: &mut Bencher) {
//    let mut buf = BitBuf::with_len(1400);
//    b.iter(|| {
//        buf.pos = 0;
//        for _ in 0..175 {
//            buf.write_f64(1235.2390123458f64);
//        }
//    })
//}
//
//#[bench]
//fn f64_read1400_bench(b: &mut Bencher) {
//    let mut buf = BitBuf::with_len(1400);
//    for _ in 0..175 {
//        buf.write_f64(1235.2390123458f64);
//    }
//    b.iter(|| {
//        buf.pos = 0;
//        for _ in 0..175 {
//            let f = buf.read_f64();
//        }
//    })
//}
