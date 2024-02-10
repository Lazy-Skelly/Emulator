use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
//NV1B DIZC are the flags
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
    No_Adress
   }
pub struct Cpu{
    pub reg_a : u8,
    pub reg_x : u8,
    pub reg_y : u8,
    pub pc : u16,
    pub status : u8,
    pub memory : [u8 ; 0x10000],
    pub stack : u8,
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
            stack : 0,

        }}
    pub fn  nextpc(&mut self){
        if self.pc != 0xFFFF {
        self.pc = self.pc + 1;}
        else{
            println!(r"¯\_(ツ)_/¯");
        }

    }
    pub fn push(&mut self, val : u8){
        
        if self.stack != 0xFF {
        self.stack += 1;
        self.Write_memory(0x0100 + self.stack as u16, val);}
        else{self.stack = 0x01;
            self.Write_memory(0x0100 + self.stack as u16, val);}

    }
    pub fn pop(&mut self) -> u8{
        let x : u8;
        
        if self.stack != 0x00 {
            x = self.Read_memory(0x0100+self.stack as u16);
            self.stack -=1;}
        else{
             x = self.Read_memory(0x0100+self.stack as u16);
            self.stack = 0xFF;
        }
        x
    
    }
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
            Adressing_mode::No_Adress => {
                assert!(false, "There is no memory adressing");
                0x00
            }
        }
    } 
     
    pub fn AND(&mut self, code :opcode){
        let adress = self.Get_operand_adress(code.mode);
        self.reg_a = self.reg_a & self.Read_memory(adress) as u8;
        if self.reg_a == 0x00 {
            self.status = self.status | 0b00000010;
        }
        if self.status >= 0b10000000 {
            self.status = self.status | 0b10000000;
        }
        self.pc += code.length -1;
    }
    pub fn CLV(&mut self){
        self.status = self.status & 0b10111111;
    }
    pub fn CLI(&mut self){
        self.status = self.status & 0b11111011;
    }
    pub fn CLD(&mut self){
        self.status = self.status & 0b11110111;
    }
    pub fn CLC(&mut self){
        self.status = self.status & 0b11111110;
    }
    pub fn BVS(&mut self,code : opcode){
        let adress = self.Get_operand_adress(code.mode);
        let x = self.Read_memory(adress);
        if self.status & 64 == 0b01000000{
            if x >= 0b10000000{
                self.pc -= !x as u16 + 1; 
            }
            else{
                self.pc += x as u16;
            }
            
        }
        self.pc += code.length -1;
    }
  
    pub fn BRK(&mut self){
            self.push(self.status);//self.push(self.pc);
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

    pub fn LDA(&mut self, code :opcode){
        let adress = self.Get_operand_adress(code.mode);
        let x = self.Read_memory(adress);
        self.pc += code.length -1;
        self.reg_a = x;
        self.Set_zero_negative(x);
        self.pc += code.length -1;
    }
    
    pub fn LDX(&mut self, code :opcode){
        let adress = self.Get_operand_adress(code.mode);
        let x = self.Read_memory(adress);
        self.pc += code.length -1;
        self.reg_x = x;
        self.Set_zero_negative(x);
        self.pc += code.length -1;
    }

    pub fn LDY(&mut self, code :opcode){
        let adress = self.Get_operand_adress(code.mode);
        let x = self.Read_memory(adress);
        self.pc += code.length -1;
        self.reg_y = x;
        self.Set_zero_negative(x);
        self.pc += code.length -1;
    }
    
    pub fn LSR(&mut self, code :opcode){
        if let Adressing_mode::No_Adress = code.mode{
            self.Set_carry_flag((self.reg_a & 0x01)== 1);
            self.reg_a = self.reg_a >> 1;
            self.Set_zero_negative(self.reg_a);
        }else{
            let adress = self.Get_operand_adress(code.mode);
            self.Set_carry_flag((self.memory[adress as usize] & 0x01)== 1);
            self.memory[adress as usize] = self.memory[adress as usize] >> 1;
            self.Set_zero_negative(self.memory[adress as usize]);
        }
        self.pc += code.length -1;
    }
    
    pub fn NOP(&mut self, _code :opcode){}
    
    pub fn ORA(&mut self, code :opcode){
        let adress = self.Get_operand_adress(code.mode);
        let x = self.Read_memory(adress);
        self.reg_a = self.reg_a | x;
        self.pc += code.length -1;
    }
    
    pub fn PHA(&mut self, _code :opcode){
        self.push(self.reg_a);
    }
    
    pub fn PHP(&mut self, _code :opcode){
        self.push(self.status);
    }
    
    pub fn PLA(&mut self, _code :opcode){
        self.reg_a = self.pop();
        self.Set_zero_negative(self.reg_a);
    }
    
    pub fn PLP(&mut self, _code :opcode){
        self.status = self.pop();
    }
    
    pub fn ROL(&mut self, code :opcode){
        if let Adressing_mode::No_Adress = code.mode{
            let c = self.reg_a & 0x80 == 0x80;
            self.reg_a = self.reg_a << 1;
            if self.status & 0x01 == 0x01 {
                self.reg_a += 1;
            }
            self.Set_carry_flag(c);
            self.pc += code.length -1;
        }else{
            let adress = self.Get_operand_adress(code.mode);
            let c = self.memory[adress as usize] & 0x80 == 0x80;
            self.memory[adress as usize] = self.memory[adress as usize] << 1;
            if self.status & 0x01 == 0x01 {
                self.memory[adress as usize] += 1;
            }
            self.Set_carry_flag(c);
            self.pc += code.length -1;
        }
    }
    
    pub fn ROR(&mut self, code :opcode){
        if let Adressing_mode::No_Adress = code.mode{
            let c = self.reg_a & 0x01 == 0x01;
            self.reg_a = self.reg_a >> 1;
            if self.status & 0x01 == 0x01 {
                self.reg_a += 0x80;
            }
            self.Set_carry_flag(c);
            self.pc += code.length -1;
        }else{
            let adress = self.Get_operand_adress(code.mode);
            let c = self.memory[adress as usize] & 0x01 == 0x01;
            self.memory[adress as usize] = self.memory[adress as usize] >> 1;
            if self.status & 0x01 == 0x01 {
                self.memory[adress as usize] += 0x80;
            }
            self.Set_carry_flag(c);
            self.pc += code.length -1;
        }
    }
    
    pub fn RTI(&mut self, _code :opcode){
        self.status = self.pop();
        let low = self.pop() as u16;
        let mut high = self.pop() as u16;
        high = high << 8;
        self.pc = high | low;
    }
    
    pub fn RTS(&mut self, _code :opcode){
        self.status = self.pop();
        let low = self.pop() as u16;
        let mut high = self.pop() as u16;
        high = high << 8;
        self.pc = high | low -1;
    }
    
    pub fn SBC(&mut self, code :opcode){
        
        let adress = self.Get_operand_adress(code.mode);
        let data = self.Read_memory(adress);
        let data = (data as i8).wrapping_neg().wrapping_sub(1);
        let data = data as u8;
        let result = self.reg_a as u16 + data as u16
        + (if self.status & 0x01 == 0x01 {
            1
        } else{ 
            0 
        }) as u16;

        self.Set_carry_flag(result > 0xff);
        let result = result as u8;
        
        if(data as u8 ^ result) & (result ^ self.reg_a) & 0x80 != 0 {
            self.Set_overflow_flag(true);
        }else{
            self.Set_overflow_flag(false);
        }
        
        self.reg_a = result as u8;
        self.Set_zero_negative(self.reg_a);
        self.pc += code.length -1;
    }
    
    pub fn SEC(&mut self, _code :opcode){
        self.Set_carry_flag(true);
    }
    
    pub fn SED(&mut self, _code :opcode){
        self.Set_decimal_flag(true);
    }
    
    pub fn SEI(&mut self, _code :opcode){
        self.Set_interupt_flag(true);
    }
    
    pub fn STA(&mut self, code :opcode){
        let adress = self.Get_operand_adress(code.mode);
        self.Write_memory(adress, self.reg_a);
        self.pc += code.length-1;
    }
    
    pub fn STX(&mut self, code :opcode){
        let adress = self.Get_operand_adress(code.mode);
        self.Write_memory(adress, self.reg_x);
        self.pc += code.length-1;
    }
    
    pub fn STY(&mut self, code :opcode){
        let adress = self.Get_operand_adress(code.mode);
        self.Write_memory(adress, self.reg_y);
        self.pc += code.length-1;
    }
        
    pub fn TAX(&mut self){
        self.reg_x = self.reg_a;
        let x = self.reg_x;
        self.Set_zero_negative(x);
    }
    
    pub fn TAY(&mut self){
        self.reg_y = self.reg_a;
        let x = self.reg_y;
        self.Set_zero_negative(x);
    }

    pub fn TXA(&mut self){
        self.reg_a = self.reg_x;
        let x = self.reg_a;
        self.Set_zero_negative(x);
    }
    
    pub fn TXS(&mut self){
        self.push(self.reg_x);
    }

    pub fn TYA(&mut self){
        self.reg_a = self.reg_y;
        let x = self.reg_a;
        self.Set_zero_negative(x);
    }

    pub fn Set_zero_negative(&mut self, x:u8){
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
    
    pub fn Load(&mut self, program :Vec<u8>){
        self.memory[0.. program.len()].copy_from_slice(program.as_slice());
    }
    
    pub fn Run(&mut self){
        loop{
        let code = self.Read_memory(self.pc);
        self.pc += 1;
        
            
        }
    }
    
}

#[allow(non_camel_case_types)]
pub struct opcode{
    pub name : String,
    pub code : u8,
    pub length : u16,
    pub cycle : u8,
    pub mode : Adressing_mode,
}
impl opcode{
    fn new(s : String, c : u8,l : u16, cy : u8 , a : Adressing_mode) -> Self {
        opcode { name: (s), code: (c), length: (l), cycle: (cy), mode: (a) }
    }
}

pub struct Hash_code{
    code_list :HashMap<u8, opcode>
}

impl Hash_code{
    fn new() -> Self {
        let mut c = Hash_code {code_list: HashMap::<u8,opcode>::new()};
        //ADC
        c.code_list.insert(0x69,opcode::new("ADC".to_string(),0x69,2,2,Adressing_mode::Immediate));
        c.code_list.insert(0x65,opcode::new("ADC".to_string(),0x65,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0x75,opcode::new("ADC".to_string(),0x75,2,4,Adressing_mode::Zeropage_X));
        c.code_list.insert(0x6d,opcode::new("ADC".to_string(),0x6d,3,4,Adressing_mode::Absolute));
        c.code_list.insert(0x7d,opcode::new("ADC".to_string(),0x7d,3,4,Adressing_mode::Absolute_X));
        c.code_list.insert(0x79,opcode::new("ADC".to_string(),0x79,3,4,Adressing_mode::Absolute_Y));
        c.code_list.insert(0x61,opcode::new("ADC".to_string(),0x61,2,6,Adressing_mode::Indirect_X));
        c.code_list.insert(0x71,opcode::new("ADC".to_string(),0x71,2,5,Adressing_mode::Indirect_Y));
        //AND
        c.code_list.insert(0x29,opcode::new("AND".to_string(),0x29,2,2,Adressing_mode::Immediate));
        c.code_list.insert(0x25,opcode::new("AND".to_string(),0x25,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0x35,opcode::new("AND".to_string(),0x35,2,4,Adressing_mode::Zeropage_X));
        c.code_list.insert(0x2d,opcode::new("AND".to_string(),0x2d,3,4,Adressing_mode::Absolute));
        c.code_list.insert(0x3d,opcode::new("AND".to_string(),0x3d,3,4,Adressing_mode::Absolute_X));
        c.code_list.insert(0x39,opcode::new("AND".to_string(),0x39,3,4,Adressing_mode::Absolute_Y));
        c.code_list.insert(0x21,opcode::new("AND".to_string(),0x21,2,6,Adressing_mode::Indirect_X));
        c.code_list.insert(0x31,opcode::new("AND".to_string(),0x31,2,5,Adressing_mode::Indirect_Y));
        //ASL
        c.code_list.insert(0x0a,opcode::new("ASL".to_string(),0x0a,1,2,Adressing_mode::No_Adress));
        c.code_list.insert(0x06,opcode::new("ASL".to_string(),0x06,2,5,Adressing_mode::Zeropage));
        c.code_list.insert(0x16,opcode::new("ASL".to_string(),0x16,2,6,Adressing_mode::Zeropage_X));
        c.code_list.insert(0x0e,opcode::new("ASL".to_string(),0x0e,3,6,Adressing_mode::Absolute));
        c.code_list.insert(0x1e,opcode::new("ASL".to_string(),0x1e,3,7,Adressing_mode::Absolute_X));
        //BCC
        c.code_list.insert(0x90,opcode::new("BCC".to_string(),0x90,2,2,Adressing_mode::No_Adress));
        //BCS
        c.code_list.insert(0xb0,opcode::new("BCS".to_string(),0xb0,2,2,Adressing_mode::No_Adress));
        //BEQ
        c.code_list.insert(0xf0,opcode::new("BEQ".to_string(),0xf0,2,2,Adressing_mode::No_Adress));
        //BIT
        c.code_list.insert(0x24,opcode::new("BIT".to_string(),0x24,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0x2c,opcode::new("BIT".to_string(),0x2c,3,4,Adressing_mode::Absolute));
        //BMI
        c.code_list.insert(0x30,opcode::new("BMI".to_string(),0x30,2,2,Adressing_mode::No_Adress));
        //BNE
        c.code_list.insert(0xd0,opcode::new("BNE".to_string(),0xd0,2,2,Adressing_mode::No_Adress));
        //BPL
        c.code_list.insert(0x10,opcode::new("BPL".to_string(),0x10,2,2,Adressing_mode::No_Adress));
        //BRK
        c.code_list.insert(0x00,opcode::new("BRK".to_string(),0x00,1,7,Adressing_mode::No_Adress));
        //BVC
        c.code_list.insert(0x50,opcode::new("BVC".to_string(),0x50,2,2,Adressing_mode::No_Adress));
        //BVS
        c.code_list.insert(0x70,opcode::new("BVS".to_string(),0x70,2,2,Adressing_mode::No_Adress));
        //CLC
        c.code_list.insert(0x18,opcode::new("CLC".to_string(),0x18,1,2,Adressing_mode::No_Adress));
        //CLD
        c.code_list.insert(0xd8,opcode::new("CLD".to_string(),0xd8,1,2,Adressing_mode::No_Adress));
        //CLI
        c.code_list.insert(0x58,opcode::new("CLI".to_string(),0x58,1,2,Adressing_mode::No_Adress));
        //CLV
        c.code_list.insert(0xb8,opcode::new("CLV".to_string(),0xb8,1,2,Adressing_mode::No_Adress));
        //CMP
        c.code_list.insert(0xc9,opcode::new("CMP".to_string(),0xc9,2,2,Adressing_mode::Immediate));
        c.code_list.insert(0xc5,opcode::new("CMP".to_string(),0xc5,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0xd5,opcode::new("CMP".to_string(),0xd5,2,4,Adressing_mode::Zeropage_X));
        c.code_list.insert(0xcd,opcode::new("CMP".to_string(),0xcd,3,4,Adressing_mode::Absolute));
        c.code_list.insert(0xdd,opcode::new("CMP".to_string(),0xdd,3,4,Adressing_mode::Absolute_X));
        c.code_list.insert(0xd9,opcode::new("CMP".to_string(),0xd9,3,4,Adressing_mode::Absolute_Y));
        c.code_list.insert(0xc1,opcode::new("CMP".to_string(),0xc1,2,6,Adressing_mode::Indirect_X));
        c.code_list.insert(0xd1,opcode::new("CMP".to_string(),0xd1,2,5,Adressing_mode::Indirect_Y));
        //CPX
        c.code_list.insert(0xe0,opcode::new("CPX".to_string(),0xe0,2,2,Adressing_mode::Immediate));
        c.code_list.insert(0xe4,opcode::new("CPX".to_string(),0xe4,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0xec,opcode::new("CPX".to_string(),0xec,3,4,Adressing_mode::Absolute));
        //CPY
        c.code_list.insert(0xc0,opcode::new("CPY".to_string(),0xc0,2,2,Adressing_mode::Immediate));
        c.code_list.insert(0xc4,opcode::new("CPY".to_string(),0xc4,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0xcc,opcode::new("CPY".to_string(),0xcc,3,4,Adressing_mode::Absolute));
        //DEC
        c.code_list.insert(0xc6,opcode::new("DEC".to_string(),0xc6,2,5,Adressing_mode::Zeropage));
        c.code_list.insert(0xd6,opcode::new("DEC".to_string(),0xd6,2,6,Adressing_mode::Zeropage_X));
        c.code_list.insert(0xce,opcode::new("DEC".to_string(),0xce,3,6,Adressing_mode::Absolute));
        c.code_list.insert(0xde,opcode::new("DEC".to_string(),0xde,3,7,Adressing_mode::Absolute_X));
        //DEX
        c.code_list.insert(0xca,opcode::new("DEX".to_string(),0xca,1,2,Adressing_mode::No_Adress));
        //DEY
        c.code_list.insert(0x88,opcode::new("DEY".to_string(),0x88,1,2,Adressing_mode::No_Adress));
        //EOR
        c.code_list.insert(0x49,opcode::new("EOR".to_string(),0x49,2,2,Adressing_mode::Immediate));
        c.code_list.insert(0x45,opcode::new("EOR".to_string(),0x45,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0x55,opcode::new("EOR".to_string(),0x55,2,4,Adressing_mode::Zeropage_X));
        c.code_list.insert(0x4d,opcode::new("EOR".to_string(),0x4d,3,4,Adressing_mode::Absolute));
        c.code_list.insert(0x5d,opcode::new("EOR".to_string(),0x5d,3,4,Adressing_mode::Absolute_X));
        c.code_list.insert(0x59,opcode::new("EOR".to_string(),0x59,3,4,Adressing_mode::Absolute_Y));
        c.code_list.insert(0x41,opcode::new("EOR".to_string(),0x41,2,6,Adressing_mode::Indirect_X));
        c.code_list.insert(0x51,opcode::new("EOR".to_string(),0x51,2,5,Adressing_mode::Indirect_Y));
        //INC
        c.code_list.insert(0xe6,opcode::new("INC".to_string(),0xe6,2,5,Adressing_mode::Zeropage));
        c.code_list.insert(0xf6,opcode::new("INC".to_string(),0xf6,2,6,Adressing_mode::Zeropage_X));
        c.code_list.insert(0xee,opcode::new("INC".to_string(),0xee,3,6,Adressing_mode::Absolute));
        c.code_list.insert(0xfe,opcode::new("INC".to_string(),0xfe,3,7,Adressing_mode::Absolute_X));
        //INX
        c.code_list.insert(0xe8,opcode::new("INX".to_string(),0xe8,1,2,Adressing_mode::No_Adress));
        //INY
        c.code_list.insert(0xc8,opcode::new("INY".to_string(),0xc8,1,2,Adressing_mode::No_Adress));
        //JMP
        c.code_list.insert(0x4c,opcode::new("JMP".to_string(),0x4c,3,3,Adressing_mode::Absolute));
        c.code_list.insert(0x6c,opcode::new("JMP".to_string(),0x6c,3,5,Adressing_mode::Absolute));
        //JSR
        c.code_list.insert(0x20,opcode::new("JSR".to_string(),0x20,3,6,Adressing_mode::Absolute));
        //LDA
        c.code_list.insert(0xa9,opcode::new("LDA".to_string(),0xa9,2,2,Adressing_mode::Immediate));
        c.code_list.insert(0xa5,opcode::new("LDA".to_string(),0xa5,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0xb5,opcode::new("LDA".to_string(),0xb5,2,4,Adressing_mode::Zeropage_X));
        c.code_list.insert(0xad,opcode::new("LDA".to_string(),0xad,3,4,Adressing_mode::Absolute));
        c.code_list.insert(0xbd,opcode::new("LDA".to_string(),0xbd,3,4,Adressing_mode::Absolute_X));
        c.code_list.insert(0xb9,opcode::new("LDA".to_string(),0xb9,3,4,Adressing_mode::Absolute_Y));
        c.code_list.insert(0xa1,opcode::new("LDA".to_string(),0xa1,2,6,Adressing_mode::Indirect_X));
        c.code_list.insert(0xb1,opcode::new("LDA".to_string(),0xb1,2,5,Adressing_mode::Indirect_Y));
        //LDX
        c.code_list.insert(0xa2,opcode::new("LDX".to_string(),0xa2,2,2,Adressing_mode::Immediate));
        c.code_list.insert(0xa6,opcode::new("LDX".to_string(),0xa6,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0xb6,opcode::new("LDX".to_string(),0xb6,2,4,Adressing_mode::Zeropage_Y));
        c.code_list.insert(0xae,opcode::new("LDX".to_string(),0xae,3,4,Adressing_mode::Absolute));
        c.code_list.insert(0xbe,opcode::new("LDX".to_string(),0xbe,3,4,Adressing_mode::Absolute_Y));
        //LDY
        c.code_list.insert(0xa0,opcode::new("LDY".to_string(),0xa0,2,2,Adressing_mode::Immediate));
        c.code_list.insert(0xa4,opcode::new("LDY".to_string(),0xa4,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0xb4,opcode::new("LDY".to_string(),0xb4,2,4,Adressing_mode::Zeropage_X));
        c.code_list.insert(0xac,opcode::new("LDY".to_string(),0xac,3,4,Adressing_mode::Absolute));
        c.code_list.insert(0xbc,opcode::new("LDY".to_string(),0xbc,3,4,Adressing_mode::Absolute_X));
        //LSR
        c.code_list.insert(0x4a,opcode::new("LSR".to_string(),0x4a,1,2,Adressing_mode::No_Adress));
        c.code_list.insert(0x46,opcode::new("LSR".to_string(),0x46,2,5,Adressing_mode::Zeropage));
        c.code_list.insert(0x56,opcode::new("LSR".to_string(),0x56,2,6,Adressing_mode::Zeropage_X));
        c.code_list.insert(0x4e,opcode::new("LSR".to_string(),0x4e,3,6,Adressing_mode::Absolute));
        c.code_list.insert(0x5e,opcode::new("LSR".to_string(),0x5e,3,7,Adressing_mode::Absolute_X));
        //NOP
        c.code_list.insert(0xea,opcode::new("NOP".to_string(),0xea,1,2,Adressing_mode::No_Adress));
        //ORA
        c.code_list.insert(0x09,opcode::new("ORA".to_string(),0x09,2,2,Adressing_mode::Immediate));
        c.code_list.insert(0x05,opcode::new("ORA".to_string(),0x05,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0x15,opcode::new("ORA".to_string(),0x15,2,4,Adressing_mode::Zeropage_X));
        c.code_list.insert(0x0d,opcode::new("ORA".to_string(),0x0d,3,4,Adressing_mode::Absolute));
        c.code_list.insert(0x1d,opcode::new("ORA".to_string(),0x1d,3,4,Adressing_mode::Absolute_X));
        c.code_list.insert(0x19,opcode::new("ORA".to_string(),0x19,3,4,Adressing_mode::Absolute_Y));
        c.code_list.insert(0x01,opcode::new("ORA".to_string(),0x01,2,6,Adressing_mode::Indirect_X));
        c.code_list.insert(0x11,opcode::new("ORA".to_string(),0x11,2,5,Adressing_mode::Indirect_Y));
        //PHA
        c.code_list.insert(0x48,opcode::new("PHA".to_string(),0x48,1,3,Adressing_mode::No_Adress));
        //PHP
        c.code_list.insert(0x08,opcode::new("PHP".to_string(),0x08,1,3,Adressing_mode::No_Adress));
        //PLA
        c.code_list.insert(0x68,opcode::new("PLA".to_string(),0x68,1,4,Adressing_mode::No_Adress));
        //PLP
        c.code_list.insert(0x28,opcode::new("PLP".to_string(),0x28,1,4,Adressing_mode::No_Adress));
        //ROL
        c.code_list.insert(0x2a,opcode::new("ROL".to_string(),0x2a,1,2,Adressing_mode::No_Adress));
        c.code_list.insert(0x26,opcode::new("ROL".to_string(),0x26,2,5,Adressing_mode::Zeropage));
        c.code_list.insert(0x36,opcode::new("ROL".to_string(),0x36,2,6,Adressing_mode::Zeropage_X));
        c.code_list.insert(0x2e,opcode::new("ROL".to_string(),0x2e,3,6,Adressing_mode::Absolute));
        c.code_list.insert(0x3e,opcode::new("ROL".to_string(),0x3e,3,7,Adressing_mode::Absolute_X));
        //ROR
        c.code_list.insert(0x6a,opcode::new("ROR".to_string(),0x6a,1,2,Adressing_mode::No_Adress));
        c.code_list.insert(0x66,opcode::new("ROR".to_string(),0x66,2,5,Adressing_mode::Zeropage));
        c.code_list.insert(0x76,opcode::new("ROR".to_string(),0x76,2,6,Adressing_mode::Zeropage_X));
        c.code_list.insert(0x6e,opcode::new("ROR".to_string(),0x6e,3,6,Adressing_mode::Absolute));
        c.code_list.insert(0x7e,opcode::new("ROR".to_string(),0x7e,3,7,Adressing_mode::Absolute_X));
        //RTI
        c.code_list.insert(0x40,opcode::new("RTI".to_string(),0x40,1,6,Adressing_mode::No_Adress));
        //RTS
        c.code_list.insert(0x60,opcode::new("RTS".to_string(),0x60,1,6,Adressing_mode::No_Adress));
        //SBC
        c.code_list.insert(0xe9,opcode::new("SBC".to_string(),0xe9,2,2,Adressing_mode::Immediate));
        c.code_list.insert(0xe5,opcode::new("SBC".to_string(),0xe5,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0xf5,opcode::new("SBC".to_string(),0xf5,2,4,Adressing_mode::Zeropage_X));
        c.code_list.insert(0xed,opcode::new("SBC".to_string(),0xed,3,4,Adressing_mode::Absolute));
        c.code_list.insert(0xfd,opcode::new("SBC".to_string(),0xfd,3,4,Adressing_mode::Absolute_X));
        c.code_list.insert(0xf9,opcode::new("SBC".to_string(),0xf9,3,4,Adressing_mode::Absolute_Y));
        c.code_list.insert(0xe1,opcode::new("SBC".to_string(),0xe1,2,6,Adressing_mode::Indirect_X));
        c.code_list.insert(0xf1,opcode::new("SBC".to_string(),0xf1,2,5,Adressing_mode::Indirect_Y));
        //SEC
        c.code_list.insert(0x38,opcode::new("SEC".to_string(),0x38,1,2,Adressing_mode::No_Adress));
        //SED
        c.code_list.insert(0xf8,opcode::new("SED".to_string(),0xf8,1,2,Adressing_mode::No_Adress));
        //SEI
        c.code_list.insert(0x78,opcode::new("SEI".to_string(),0x78,1,2,Adressing_mode::No_Adress));
        //STA
        c.code_list.insert(0x85,opcode::new("STA".to_string(),0x85,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0x95,opcode::new("STA".to_string(),0x95,2,4,Adressing_mode::Zeropage_X));
        c.code_list.insert(0x8d,opcode::new("STA".to_string(),0x8d,3,4,Adressing_mode::Absolute));
        c.code_list.insert(0x9d,opcode::new("STA".to_string(),0x9d,3,5,Adressing_mode::Absolute_X));
        c.code_list.insert(0x99,opcode::new("STA".to_string(),0x99,3,5,Adressing_mode::Absolute_Y));
        c.code_list.insert(0x81,opcode::new("STA".to_string(),0x81,2,6,Adressing_mode::Indirect_X));
        c.code_list.insert(0x91,opcode::new("STA".to_string(),0x91,2,6,Adressing_mode::Indirect_Y));
        //STX
        c.code_list.insert(0x86,opcode::new("STX".to_string(),0x86,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0x96,opcode::new("STX".to_string(),0x96,2,4,Adressing_mode::Zeropage_Y));
        c.code_list.insert(0x8e,opcode::new("STX".to_string(),0x8e,3,4,Adressing_mode::Absolute));
        //STY
        c.code_list.insert(0x84,opcode::new("STY".to_string(),0x84,2,3,Adressing_mode::Zeropage));
        c.code_list.insert(0x94,opcode::new("STY".to_string(),0x94,2,4,Adressing_mode::Zeropage_X));
        c.code_list.insert(0x8c,opcode::new("STY".to_string(),0x8c,3,4,Adressing_mode::Absolute));
        //TAX
        c.code_list.insert(0xaa,opcode::new("TAX".to_string(),0xaa,1,2,Adressing_mode::No_Adress));
        //TAY
        c.code_list.insert(0xa8,opcode::new("TAY".to_string(),0xa8,1,2,Adressing_mode::No_Adress));
        //TSX
        c.code_list.insert(0xba,opcode::new("TSX".to_string(),0xba,1,2,Adressing_mode::No_Adress));
        //TXA
        c.code_list.insert(0x8a,opcode::new("TXA".to_string(),0x8a,1,2,Adressing_mode::No_Adress));
        //TXS
        c.code_list.insert(0x9a,opcode::new("TXS".to_string(),0x9a,1,2,Adressing_mode::No_Adress));
        //TYA
        c.code_list.insert(0x98,opcode::new("TYA".to_string(),0x98,1,2,Adressing_mode::No_Adress));
        //
        c
    }
}
/*  ToDo:
        Create Stack {$0100-$0200}

 */

fn main() {
    let mut c = Cpu::new();
//    let b = opcode::new("LSR".to_string(),0x4a,1,2,Adressing_mode::Immediate);
    c.Load([0xa9, 0xc0, 0xaa, 0xe8, 0x00].to_vec());
    let b = Hash_code::new();
    let mut i =1;
    for (key, value) in &b.code_list{
        println!("{}. {key:#X}: {}: {}",i,value.name, value.length);
        i += 1;
    }
}
