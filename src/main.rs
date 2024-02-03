#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]

pub enum Adressing_mode{
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
    pub memory : [u8 ; 0x10000],
}


  
impl Cpu{
    pub fn new() -> Self { 
        Cpu {
            reg_a : 0,
            reg_x : 0,
            reg_y : 0,
            pc : 0b00000000,
            status : 0b00100000,
            memory : [0 ; 0x10000],

        }}
  
    pub fn BRK(&mut self){
            //push(self.pc); push(self.status);
            self.status = self.status | 0b00010000;
            self.pc = 0x0000 | (self.memory[0xFFFE] as u16);
            self.pc = self.pc | ((self.memory[0xFFFE] as u16)*256);            
        
        }
    pub fn INX(&mut self){        
        let x = self.reg_x;
        if x == 0b11111111 {
            self.Set_zero_flag(true);
            self.Set_negative_flag(false);
            self.reg_x = 0b00000000;
        }else{
        self.reg_x += 1; 
        if 0b10000000 == 0b10000000 & self.reg_x {
            self.Set_negative_flag(true);
        }else{
            self.Set_negative_flag(false);
        }
        }}
         
    }

    pub fn LDA(&mut self, mode :Adressing_mode){
        match mode {
            Adressing_mode::Immediate => {
                let x = self.memory[self.pc as usize];
                self.reg_a = x;
                if x == 0 {
                    self.Set_zero_flag(true);
                }else{
                    self.Set_zero_flag(false);
                }
                if 0b10000000 == 0b10000000 & x {
                    self.Set_negative_flag(true);
                }else{
                    self.Set_negative_flag(false);
                }
                self.reg_a = x;
            }
            _ => (),
            
        }
    }
    
    pub fn TAX(&mut self){
        self.reg_a = self.reg_x;
        let x = self.reg_a;
        if x == 0 {
            self.Set_zero_flag(true);
        }else{
            self.Set_zero_flag(false);
        }
        if 0b10000000 == 0b10000000 & x {
            self.Set_negative_flag(true);
        }else{
            self.Set_negative_flag(false);
        }
    }
    
    pub fn Set_carry_flag(&mut self, stat:bool){
        if stat{
            self.status = self.status | 0b00000001;
        }else{
            self.status = self.status & 0b11111110;
        }
    }
    
    pub fn Set_zero_flag(&mut self, stat:bool){
        if stat{
            self.status = self.status | 0b00000010;
        }else{
            self.status = self.status & 0b11111101;
        }
    }
    
    pub fn Set_interupt_flag(&mut self, stat:bool){
        if stat{
            self.status = self.status | 0b00000100;
        }else{
            self.status = self.status & 0b11111011;
        }
    }
    
    pub fn Set_decimal_flag(&mut self, stat:bool){
        if stat{
            self.status = self.status | 0b00001000;
        }else{
            self.status = self.status & 0b11110111;
        }
    }
    
    pub fn Set_b_flag(&mut self, stat:bool){
        if stat{
            self.status = self.status | 0b00010000;
        }else{
            self.status = self.status & 0b11101111;
        }
    }
    
    pub fn Set_overflow_flag(&mut self, stat:bool){
        if stat{
            self.status = self.status | 0b01000000;
        }else{
            self.status = self.status & 0b10111111;
        }
    }
    
    pub fn Set_negative_flag(&mut self, stat:bool){
        if stat{
            self.status = self.status | 0b10000000;
        }else{
            self.status = self.status & 0b01111111;
        }
    }
    
}
#[allow(non_camel_case_types)]
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
