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
        if(self.pc != 0xFFFF){
        self.pc = self.pc + 1;}
        else{
            println!(r"¯\_(ツ)_/¯");
        }

    }
    pub fn push(&mut self, val : u8){
        
        if(self.stack != 0xFF){
        self.stack += 1;
        self.Write_memory(0x0100 + self.stack as u16, val);}
        else{self.stack = 0x01;
            self.Write_memory(0x0100 + self.stack as u16, val);}

    }
    pub fn pop(&mut self) -> u8{
        let x : u8;
        
        if(self.stack != 0x00){
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
    pub fn is_negative(&mut self, x : u8) -> bool{
        if (x & 0b10000000 == 0x00 ){true}
        else{false}

    }
    pub fn neg(x :u8)-> u8{
        (!x + 1)

    }
    pub fn ADC(&mut self, code:opcode){ //STILL BEING TESTED THIS IS NOT FULL CODE
        let adress = self.Get_operand_adress(code.mode);
        let value = self.Read_memory(adress);
        if (!self.is_negative(value)){
            if ((value as u16 ) + (self.reg_a as u16) + {if(self.status & 0b00000001 == 1 ){1}else{0}} > 0xFF ){
                self.reg_a = {if(value>self.reg_a){value - self.reg_a}else{self.reg_a - value}};
                if(!self.is_negative(self.reg_a)){self.status = self.status | 0b00000001;}else{self.status = self.status | 0b10000001;}
            }
            else{
                if(self.is_negative(value) & !self.is_negative(self.reg_a)){}
                self.reg_a = self.reg_a + value  ; 
                self.status = self.status | 0b00000000; 
                



                

        }}
        else{}}
    pub fn ASL(&mut self,mode :Adressing_mode){
        let adress = self.Get_operand_adress(mode);
        let value = self.Read_memory(adress);
        if(value & 0b10000000 == 0x00){self.reg_a = (value)*2;}
        else{self.reg_a = (value-255) * 2 ; self.status = self.status | 0b00000001;}
        if(self.reg_a == 0){self.status = self.status  | 0b00000010;}
        if(self.is_negative(self.reg_a)){self.status = self.status | 0b10000000;}
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
    }
    
    pub fn LDX(&mut self, code :opcode){
        let adress = self.Get_operand_adress(code.mode);
        let x = self.Read_memory(adress);
        self.pc += code.length -1;
        self.reg_x = x;
        self.Set_zero_negative(x);
    }

    pub fn LDY(&mut self, code :opcode){
        let adress = self.Get_operand_adress(code.mode);
        let x = self.Read_memory(adress);
        self.pc += code.length -1;
        self.reg_y = x;
        self.Set_zero_negative(x);
       
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
        
    pub fn TAX(&mut self){
        self.reg_a = self.reg_x;
        let x = self.reg_a;
        self.Set_zero_negative(x);
    }
    
    pub fn AND(&mut self, mode :Adressing_mode){
        let adress = self.Get_operand_adress(mode);
        self.reg_a = self.reg_a & self.Read_memory(adress) as u8;
        if(self.reg_a == 0x00){
            self.status = self.status | 0b00000010;
        }
        if (self.status >= 0b10000000){
            self.status = self.status | 0b10000000;
        }
        self.nextpc();
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
        if (self.status & 64 == 0b01000000){
            if(  x >= 0b10000000){
                self.pc -= !x as u16 + 1; 
            }
            else{
                self.pc += x as u16;
            }
            
        }
        self.nextpc();
    }
    
    pub fn BVC(&mut self,code : opcode){
        let adress = self.Get_operand_adress(code.mode);
        let x = self.Read_memory(adress);
        if (self.status & 64 == 0b00000000){
            if(  x >= 0b10000000){
                self.pc -= !x as u16 + 1; 
            }
            else{
                self.pc += x as u16;
            }
            
        }
        self.nextpc();
    }
    
    pub fn BPL(&mut self,code :opcode){
        let adress = self.Get_operand_adress(code.mode);
        let x = self.Read_memory(adress);
        if (self.status & 0b10000000 == 0b00000000){
            if(  x >= 0b10000000){
                self.pc -= !x as u16 + 1; 
            }
            else{
                self.pc += x as u16;
            }
            
        }}
    pub fn BNE(&mut self,code :opcode){
        let adress = self.Get_operand_adress(code.mode);
        let x = self.Read_memory(adress);
        if (self.status & 0b00000010 == 0b00000000){
            if(  x >= 0b10000000){
                self.pc -= !x as u16 + 1; 
            }
            else{
                self.pc += x as u16;
            }
            
        }    }


    

    pub fn BNL(&mut self,code :opcode){
        let adress = self.Get_operand_adress(code.mode);
        let x = self.Read_memory(adress);
        if (self.status & 0b00000010 == 0b00000000){
            if(  x >= 0b10000000){
                self.pc -= !x as u16 + 1; 
            }
            else{
                self.pc += x as u16;
            }
            
        }
        self.nextpc();

    }

    pub fn BMI(&mut self,code :opcode){
        let adress = self.Get_operand_adress(code.mode);
        let x = self.Read_memory(adress);
        if (self.status & 0b10000000 == 0b10000000){
            if(  x >= 0b10000000){
                self.pc -= !x as u16 + 1; 
            }
            else{
                self.pc += x as u16;
            }
            
        }
        self.nextpc();

    }
    pub fn BCC(&mut self,mode :Adressing_mode){        
        let adress = self.Get_operand_adress(mode);
        let x = self.Read_memory(adress);
        if (self.status & 0b00000001 == 0b00000000){
            if(  x >= 0b10000000){
                self.pc -= !x as u16 + 1; 
            }
            else{
                self.pc += x as u16;
            }
            
        }
        self.nextpc();
}
pub fn BCS(&mut self,mode :Adressing_mode){        
    let adress = self.Get_operand_adress(mode);
    let x = self.Read_memory(adress);
    if (self.status & 0b00000001 != 0b00000000){
        if(  x >= 0b10000000){
            self.pc -= !x as u16 + 1; 
        }
        else{
            self.pc += x as u16;
        }
        
    }
    self.nextpc();}


    
    pub fn BEQ(&mut self,mode :Adressing_mode){        
        let adress = self.Get_operand_adress(mode);
        let x = self.Read_memory(adress);
        if (self.status & 0b00000010 != 0b00000010){
            if(  x >= 0b10000000){
                self.pc -= !x as u16 + 1; 
            }
            else{
                self.pc += x as u16;
            }
            
        }
        self.nextpc();}

pub fn BIT(){ //NEEDS TESTING

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
/*  ToDo:
        Create Stack {$0100-$0200}

 */

fn main() {
    let mut c = Cpu::new();
    let b = opcode::new("LSR".to_string(),0x4a,1,2,Adressing_mode::Immediate);
    c.Write_memory(0,0);
    c.Write_memory(1,5);
    c.Write_memory(0x0609,75);
    c.Write_memory(5,10);
    c.Write_memory(6,5);
    c.reg_a = 0xf0;
    c.reg_x = 1;
    c.LSR(b);
    println!("{}",c.status);
    println!("{}",c.memory[0]);




    c.push(13);
    println!("{}",c.memory[0x0100]);
    let z :u8;
    z = c.pop();
    println!("{}",z);




}
