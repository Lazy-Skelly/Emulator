
enum Adressing_mode{
    Immediate,
    Zeropage,
    Zeropage_X,
    Zeropage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
   }
pub struct Cpu{
    pub reg_a : u8,
    pub reg_x : u8,
    pub reg_y : u8,
    pub pc : u16,
    pub status : u8,
    pub memory : [u8 ; 0xFFFF],
}

impl Cpu{
    pub fn new() -> Self { 
        Cpu {
            reg_a : 0,
            reg_x : 0,
            reg_y : 0,
            pc : 0,
            status : 0,
            memory : [0 ; 0xFFFF],

        }
    }
}

struct opcode{
    pub name : String,
    pub code : u8,
    pub length : u8,
    pub cycle : u8,
    mode : Adressing_mode,
}
impl opcode{
    fn new(s : String, c : u8,l : u8, cy : u8 , a : Adressing_mode) -> Self {
        opcode { name: (s), code: (c), length: (l), cycle: (cy), mode: (a) }
    }
}
/*tp do:
        Flag modifiers
 */




fn main() {
   
}
