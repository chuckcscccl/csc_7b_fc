/*                Abstract Machine AMF+1

Abstract Machine AMF+1 is a 16bit computer architecture with 2**16=64K bytes
of memory (RAM). Each memory "word" is also 16 bits so there are 32K memory
addresses.  The memory is divided into the following segments:

Code Segment: Address 0-4095 (4K)
Stack Segment: Address 4096-12287  (8K)
Heap Segment: Address 65535 down to 12288 (20K)

CPU Architecture:

  The CPU of AMF+1 consists of an Arithmetic Logic Unit (ALU) capable of
  16-bit operations "add", "sub", "mult" and "div" as well as "mov" and "nop"

  The CPU contains 8 registers.  Six of these are accessible from software
  while two others are internal (software cannot directly read or change
  their values).  

    AX  : Accumulator register
    BX  : Alternate Accumulator register
    CX  : Counter register
    SP  : Stack pointer register
    BP  : Stack base register
    MA  : Memory address register

    PC  : program counter register  (internal)
    IR  : instruction register (internal)

Instruction Set Architecture:

ALU Instructions:

  opcode src dst   : operation on source operand src and destination dst.

  The opcode can be add, sub, mult or div.  The dst must name a (non-internal)
  register.  The src operand can be "register" or "immediate", meaning a
  constant.  For examples,

  sub ax bx  :  has effect bx = bx - ax;
  mult 3 cx  :  has effect cx = cx * 3, 3 is an "immediate operand"

  ** the maximum value of an immediate operand is 255 (8 bits).  This is
  because the entire instruction must also fit into a 16-bit word.

  The div instruction calculates both the quotient and remainder, and
  always stores the remainder in register cx:

  div 2 ax :  cx=ax%2; ax = ax/2;

  If the dst operand is also cx, then the quotient is discarded.

  All alu operations work on 16-bit two's complement integers (int16_t),
  but immediate operands cannot be signed. 
    mov 0 ax
    sub 1 ax   
  has the effect of placing -1 in ax


Other CPU-Bound Instructions:

  mov src dst : "moves" source operand to destination.  The src can be
  immediate or register, the dst can only be a register (non-internal)

  nop:  does nothing at all, but it has an important role.


Memory Instructions:

  push src : pushes the (register or immediate source) src onto the stack,
             incrementing the SP register.

  pop dst  : pops the contents of the top-of-stack into the dst register,
             decreasing the SP register.

  store src : stores the value of (register or immediate) src into the
             memory location indicated by the ma (memory address) register.
             The only way to address memory is to place the address in ma.

  load dst  : loads the value of memory at address held in the ma register 
             into the dst register.

  In the case of push and store, an immediate source operand is allowed to
  be 11 bits in size, with a value 0-2047 (2K).


Branch Instructions:

  jmp target : unconditional jump to instruction at memory location indicated
               by target.  By "jump" we mean setting the value of PC to the
               target.  Valid values of target are 1-4095 (within the
               code segment).  Usually, programs always start at memory 
               address 1.
  jnz target : jump to target instruction address if the value in the cx
              (counter) register is NOT ZERO.
  jz target :  jump to target instruction address if the value in the cx
              (counter) register IS ZERO.
  jn target :  jump to target instruction address if the value in the cx
              (counter) register IS NEGATIVE
  call target : pushes the current value of PC on the stack, then jump to
               the target instruction
  ret         : pops the value on top of the stack into PC and continue
               executing the instruction at address PC+1.

  
The PC register holds the address of the next instruction to execute.  After
the execution of each instruction, the PC is automatically incremented by one.

The instruction to execute is loaded into the IR (instruction register).


INSTRUCTION FORMAT:

  All instructions are stored in single 16-bit words (uint16_t).
  Since there are exactly 16 instructions, the first 4 bits (most significant
  bits) of the instruction stores the "opcode".  The opcode of each instruction
  is defined by their index in the following array:

  static const char* Instruction[16] = {
    "nop","add","sub","mult","div","push","pop","mov","load","store",
    "jmp","jnz","jz","jn","call","ret"};

  For example, the opcode for "mult" is 3.

  Depending on the instruction type, the other 12 bits of the instruction
  are interpreted as follows:

  The lowest (least significant) 3 bits identifies the destination register,
  for those instructions that require one.  Registers are identified by their
  indices in the following array:

  static const char* Register[6] = {"ax","bx","cx","sp","bp","ma"};

  So if the last 3 bits holds value 101 (5), it identies the ma register.

  IR bits usage for ALU instructions and the mov instruction:

     0123 4 56789ABC DEF
    ---------------------
    | op |i|     src|dst|  
    ---------------------

  op = opcode
  i = 1 if src is a register operand, i=0 if src is immediate.
  src : 8-bit immediate or register operand.  In case of a register src
        operand, on the lower 3 bits of these 8 bits (bits ABC) are used.
  dst : destination register.
  
  The pop and load instructions have the same format, except the src fields
  (bits 4-C) are not used.

  The push and store instructions have the same format, except that the
  lowest 3 bits (DEF) can also be used for an 11-bit immediate src operand.
  In case these instructions have a register src, then bits ABC will still
  represent the register.

  The branch instructions use all 12 bits after the 4 bit opcode (bits 4-F)
  to represent the target memory address.  12 bits can address 4K locations,
  which is exactly the size of the Code Segment of RAM.


Consult the sample programs for further illustration of how the instructions
work.  AMF+1 is a simple architecture but it has the essential flavors of 
a working computer and instruction set.

An "abstract machine" is only the specification of an architecture.
A "virtual machine" is the software implementation of an abstract machine.
*/

// VMF+1 implementation of AMF+1

#include<iostream>
#include<cstdlib>
#include<cstring>
#include<cstdio>
#include<functional>
using namespace std;

static const char* Instruction[16] = {
  "nop","add","sub","mult","div","push","pop","mov","load","store",
  "jmp","jnz","jz","jn","call","ret"};
static const char* Register[6] = {"ax","bx","cx","sp","bp","ma"};

static const int MEMSIZE = 65536/2;    //2*32K = 64K = 2**16

const int16_t AX = 0;
const int16_t BX = 1;
const int16_t CX = 2;
const int16_t SP = 3;
const int16_t BP = 4;
const int16_t MAR = 5;

static bool TRACE = 1;

// to be imported from fponeasm.o :
uint16_t assemble(const char input[]);

  //exercise
void print_inst(uint16_t inst)
{
    int16_t opcode = (inst & 0xf000) >> 12;
    const char* iss = Instruction[opcode];
    cout << iss;
    if (opcode==0 || opcode==15) { return; }
    // decode 1st operand
    if (opcode>9) { // 12 bit operand (jump/call)
      cout << " " << (inst & 0x0fff) ;  return;
    }
    // determine if 1st operand is register or immediate
    if (opcode<5 || opcode==7) { // 2 operands
      uint16_t dst = inst%8;
      if (inst & 0x0800) { // register
        cout << " " << Register[ (inst & 0x07f8)>>3 ];
      } else { //immediate
        cout << " " << ((inst & 0x7f8)>>3);
      }
      cout << " " << Register[dst];
    }// ALU or mov
    else if (opcode==9 || opcode==5) { // push, store
      if (inst & 0x0800) { // register
        cout << " " << Register[ (inst & 0x07ff)>>3 ]; return;
      } else { //immediate
        cout << " " << (inst & 0x07ff); return;
      }
    }
    else { // dst only
      cout << " " << Register[inst%8];
    }
}//print_instruction
  

struct vmfplusone
{
  int16_t RAM[MEMSIZE]; // 2*32*1024 = 64K RAM
  // uint8_t ROM[16*1024]
  uint16_t IR; // instruction register
  uint16_t PC; // program counter register
  bool IDLE;   // machine IDLE flag (1=true)
  int16_t REG[8]; // registers ax, bx, cx, sp, bp, mar

  static constexpr int CodeSegment = 0;     // 4K code segment
  static constexpr int StackSegment = 4096;
  static constexpr int StackLimit = 12*1024;  // 8K stack
  // Heap memory will grow downwards from max mem addr towards StackLimit
  static constexpr int HEAPBASE = 65535;

  vmfplusone() { // constructor
    IR = 0;
    PC = 1;
    IDLE = 1;
    memset(RAM,0,MEMSIZE); // zero memory contents
    memset(REG,0,16);       // zero register contents
    REG[SP] = StackSegment;
  }

  // write a series of 16 functions
  // decode source operand from IR register
  int16_t decode_src_8() {
    int16_t src;
      if (IR & 0x0800) { // register operand
        return REG[ (IR & 0x0038) >> 3 ];
      }
      else {  // immediate operand for next 8 bits
        return (IR & 0x07f8) >> 3;
      }
  }//decode_src

  // decode destination register operand: value must index a register
  uint16_t decode_dst() { return IR % 8; }
  
  void nop() { }

  void add() {   // must decode operands from IR
    int16_t src = decode_src_8();
    uint16_t dst = decode_dst();
    REG[dst] += src;
  }
  void sub() {   // must decode operands from IR
    int16_t src = decode_src_8();
    uint16_t dst = decode_dst();
    REG[dst] -= src;
  }
  void mult() {   // must decode operands from IR
    int16_t src = decode_src_8();
    uint16_t dst = decode_dst();
    REG[dst] *= src;
  }
  void div() {   // must decode operands from IR
    int16_t src = decode_src_8();
    uint16_t dst = decode_dst();
    int16_t tmp = REG[dst];
    REG[dst] = tmp / src;
    REG[CX] = tmp % src;
  }    

  void push() {
    // decode src
    int16_t src;
    if (IR & 0x0800) { // register
      int16_t ri = (IR & 0x07f8) >> 3;
      src = REG[ri];
    }
    else { //immediate
      src = (IR & 0x07ff);
    }
    int16_t tos = REG[SP];
    if (tos >= StackLimit)  throw 1; // stack overflow
    RAM[tos] = src;
    REG[SP] = tos+1;
  }//push

  void pop() {
    int16_t dst = decode_dst();
    if (dst == SP)  throw 3; // stack pointer corruption
    int16_t tos = REG[SP];
    if (tos<=StackSegment) throw 2; // stack underflow
    REG[dst] = RAM[tos-1];
    REG[SP] = tos-1;
  }//pop

  void mov() {
    int16_t src = decode_src_8();
    int16_t dst = decode_dst();
    REG[dst] = src;
  }

  void load() { // load from [mar] to dst
    int16_t dst = decode_dst();
    REG[dst] = RAM[(uint16_t)REG[MAR]];
  }//load

  void store() {  // store src operand to memory
    int16_t src;
    if (IR & 0x0800) { // register
      int16_t ri = (IR & 0x07f8) >> 3;
      src = REG[ri];
    }
    else { //immediate
      src = (IR & 0x07ff);
    }
    RAM[(uint16_t)REG[MAR]] = src;
  }//store

  void jmp() { // here operand is a 4K mem addr, so 12 bits
    int16_t dst = IR & 0x0fff;
    PC = dst - 1; // -1 to offset auto increment
  }
  void jnz() { // here operand is a 4K mem addr, so 12 bits
    int16_t dst = IR & 0x0fff;
    if (REG[CX]!=0) 
      PC = dst - 1; // -1 to offset auto increment
  }  
  void jz() { // here operand is a 4K mem addr, so 12 bits
    int16_t dst = IR & 0x0fff;
    if (REG[CX]==0) 
      PC = dst - 1; // -1 to offset auto increment
  }  
  void jn() { // here operand is a 4K mem addr, so 12 bits
    int16_t dst = IR & 0x0fff;
    if (REG[CX]<0) 
      PC = dst - 1; // -1 to offset auto increment
  }
  void call() {  // push PC on stack then jump
    int16_t tos = REG[SP];
    if (tos >= StackLimit)  throw 1; // stack overflow    
    int16_t dst = IR & 0x0fff;
    RAM[tos] = PC;
    REG[SP] = tos+1;
    PC = dst - 1;
  }
  void ret() {
    int16_t tos = REG[SP];
    if (tos <= StackSegment)  throw 2; // stack overflow
    PC = RAM[tos-1];  // allow it to increment!
    REG[SP] = tos-1;
  }
  
  void execute_instruction()
  { 
    IR = RAM[CodeSegment+PC];   // load instruction register
    uint16_t opcode = IR >> 12; // decode opcode
    // dispatch vector of functions
    static function<void()> ops[] = {
      [&](){nop();},
      [&](){add();},
      [&](){sub();},
      [&](){mult();},
      [&](){div();},
      [&](){push();},
      [&](){pop();},
      [&](){mov();},
      [&](){load();},
      [&](){store();},
      [&](){jmp();},
      [&](){jnz();},
      [&](){jz();},
      [&](){jn();},      
      [&](){call();},
      [&](){ret();}
    };  
    ops[opcode]();   // dispatch and call function implementing opcode
    PC++;
  }// execute instruction


  void load_instruction(uint16_t inst) {
    RAM[PC++] = inst;
  }

  void status() // prints status
  {
    printf("ax=%d, bx=%d, cx=%d, sp=%d, bp=%d, ma=%d, pc=%d",REG[AX],REG[BX],REG[CX],(uint16_t)REG[SP],(uint16_t)REG[BP],(uint16_t)REG[MAR],(uint16_t)PC);
    if (REG[SP]>StackSegment) printf(", tos=%d",RAM[REG[SP]-1]);
    printf("\n");
  }//status
  
  void run(uint16_t start, uint16_t limit) { // run until max pc value = limit
    PC = start;
    while (PC<limit) {
      if (TRACE) {
        print_inst(RAM[PC]);  cout << ":\t";
	//printf("%16c",':');
      }
      execute_instruction();
      if (TRACE) status();
    }
  }//run with trace option
  
}; //vmfplusone


static const char* err_message[] = {
  "Unspecified Error",
  "Stack Overflow",
  "Stack Underflow",
  "Stack Pointer Corruption",
  "Illegal Opcode",
  "Invalid Operand",
  "Segmentation Fault. It's not my fault, it's your fault."
};

////////////main
int main(int argc, char* argv[])
{
  vmfplusone VM; // instance of virtual machine
  // load program into machine
  char line[80];
  char* lineaddr = line;
  size_t linemax = 80; //setup using getline
  uint16_t startpc = VM.PC;
  int linenum = 0;
  bool stop = false;
  try {
   while (!stop) {
    int lsize = getline(&lineaddr,&linemax,stdin);
    //cout << "read line: " << line << " size " << lsize << endl;
    if (lsize<=0) stop=true;
    else {
      linenum++;
      if (line[0]=='.' || line[0] == EOF) stop=true;
      else {
	if (line[0]=='\n' || line[0]=='\r' || line[0]=='#' || lsize<3)
        {continue;}
	uint16_t inst = assemble(line);
	VM.load_instruction(inst);
      }//else (line not end in .
    }//else (!stop)
   }//while loop 
  } catch(int ec) {
    cout << err_message[ec] << ", line " << linenum << endl;
    linenum = -1; // signals error
  }//try-catch
  uint16_t endpc = VM.PC;
  TRACE = 1;
  if (endpc > startpc && linenum>=0) VM.run(startpc,endpc);
  return 0;
}//main
