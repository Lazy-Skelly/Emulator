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


#[allow(non_snake_case)]
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

    pub fn Read_memory(&mut self, adress:u16) -> u8{
        self.memory[adress as usize]
    }
    
    pub fn Write_memory(&mut self, adress:u16, data:u8){
        self.memory[adress as usize] = data;
    }
    
    pub fn Read_memory_16(&mut self, adress:u16) ->u16{
        let low :u16= self.Read_memory(adress) as u16;
        let high : u16= self.Read_memory(adress +1) as u16;
        (high << 8) | low
    }
    
    pub fn Write_memory_16(&mut self, adress:u16, data:u16){
        let high = (data >> 8) as u8;
        let low = (data & 0xff) as u8;
        self.Write_memory(adress, low);
        self.Write_memory(adress+1, high);
    }
     
    pub fn Get_operand_adress(&mut self, mode :Adressing_mode) -> u16{
        match mode{
            Adressing_mode::Immediate => self.pc,
            Adressing_mode::Zeropage => self.Read_memory(self.pc) as u16,
            Adressing_mode::Zeropage_X => (self.Read_memory_16(self.pc)+self.reg_x as u16) &0xff,
            Adressing_mode::Zeropage_Y => (self.Read_memory_16(self.pc)+self.reg_y as u16) &0xff,
            Adressing_mode::Absolute => self.Read_memory_16(self.pc),
            Adressing_mode::Absolute_X => self.Read_memory_16(self.pc) + self.reg_x as u16,
            Adressing_mode::Absolute_Y => self.Read_memory_16(self.pc) + self.reg_y as u16,
            Adressing_mode::Indirect_X  => {
                let adress = (self.Read_memory_16(self.pc)+self.reg_x as u16) &0xff;
                let low = self.Read_memory(adress) as u16;
                let high = self.Read_memory((adress +1)&0xff) as u16;
                high << 8 | low
            },
            Adressing_mode::Indirect_Y => {
                let adress = self.Read_memory(self.pc) as u16;
                let low = self.Read_memory(adress) as u16;
                let high = self.Read_memory((adress+1)&0xff) as u16;
                let adress = high << 8 | low;
                adress + self.reg_y as u16
            },
        }
    } 
     
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
        }         
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
