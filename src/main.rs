#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]




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
            pc : 0b00000000,
            status : 0b00100000,
            memory : [0 ; 0xFFFF],

        }}
    pub fn BRK(&mut self){
            //push(self.pc); push(self.status);
            self.status = self.status | 0b00010000;
            self.pc = 0x0000 | (self.memory[0xFFFE] as u16);
            self.pc = self.pc | ((self.memory[0xFFFE] as u16)*256);            
        
        }
    pub fn INX(&mut self){ 
        if self.reg_x == 0b11111111{
            self.status = self.status | 0b00000010;
        }
        self.status += 1; 
        if (self.status  & 0b10000000 == 0b10000000 ){
            self.status = self.status | 0b10000000;
        }
         
        


    }
    
}


struct opcode{
    pub name : String,
    pub code : u8,
    pub length : u8,
    pub cycle : u8,
    pub mode : Adressing_mode,
}
impl opcode{
    fn new(s : String, c : u8,l : u8, cy : u8 , a : Adressing_mode) -> Self {
        opcode { name: (s), code: (c), length: (l), cycle: (cy), mode: (a) }
    }
}
/*  ToDo:
        Create Stack {$0100-$0200}

 */




fn main() {
   
}
