// VMF+1 Assembler - create fponeasm.o with g++ -c fponeasm.cpp

#include<iostream>
#include<cstdlib>
#include<cstring>
#include<functional>
#include <unordered_map>
using namespace std;

// hashmap of opcodes AND register names
// This heap-allocated data structure cannot use const char* as keys...
static unordered_map<string, uint16_t> Opcodes = {
  {"nop",0}, {"add",1}, {"sub",2}, {"mult",3},{"div",4},{"push",5},{"pop",6},
  {"mov",7},{"load",8},{"store",9},{"jmp",10},{"jnz",11},{"jz",12},{"jn",13},
  {"call",14},{"ret",15}
};
static unordered_map<string, uint16_t> Registers = {
  {"ax",0}, {"bx",1},{"cx",2},{"sp",3},{"bp",4},{"ma",5}
};
// a hashmap is a heap-allocated structure, so this is a bit of cheating...

// determine type of character:
// 0  character code 0 (end of string)
// 1 for whitespace
// 2 for base-10 digit 0-9
// 3 for alphabetical letter
// 4 for printable ascii chars such as &, +, etc
// 5 for other characters
int8_t char_type(char c)
{
  if (c==0) return 0;
  if (c==' ' || c=='\t' || c=='\n') return 1;
  int ac = (int)c; // ascii code
  if (ac>47 && ac<58) return 2;
  if ((ac>65 && ac<91) || (ac>96 && ac<='z')) return 3;
  if (ac>31 && ac<127) return 4;
  return 5;
}//char_type
  
// find next token, if token is a symbol, store symbol in token_out,
// return length of token.  If token is a number, return the number and
// set token_out to empty string (set first byte to 0).
// argument int *position is the current position of input
int next_token(const char input[], int* position, char token_out[])
{
  int answer = 0;
  int i = *position;
  int ti = 0;  // indexes token_out
  // skip white spaces
  while (char_type(input[i])==1) i++;
  *position = i;
  if (input[i]==0) return -1; // end of input reached

  // check for symbol
  while (char_type(input[i])>2)
    {
      token_out[ti++] = input[i++];
      answer++;
    } // symbol loop
  token_out[ti] = 0; // terminate string
  *position = i;
  if (answer>0) return answer;

  // check for digit:
  while (char_type(input[i])==2)
    {
      int digit_value = (int)input[i] - 48;
      answer = answer*10 + digit_value;
      i++;
    }
  *position = i;
  return answer;
}//next_token

uint16_t assemble(const char input[])
{
  uint16_t instruction; // machine instruction
  int i = 0; // indexes input
  char operation[8];
  int result = next_token(input,&i,operation);
  if (operation[0]==0) throw 4; // illegal opcode
  uint16_t opcode = Opcodes[string(operation)];
  instruction = opcode<<12; // left-shift 12 bits
  if (opcode==0 || opcode == 15) return instruction; //nop or ret
  // decode source operand
  int maximm = 256;
  result = next_token(input,&i,operation);
  if (operation[0]==0) { // immediate operand
    if (opcode<5 || opcode==7) {// ALU op or mov
      if (result >= maximm) throw 5; // invalid operand
      instruction = instruction | (result<<3);
      result = next_token(input,&i,operation); //
      if (operation[0]==0 || result!=2) { throw 5;} // illegal operand
      uint16_t dst = Registers[string(operation)];
      instruction = instruction | dst;
      return instruction;
    }//ALU op
    else if (opcode==5 || opcode==9) {// 11 bit operand (push and store)
      if (result>=2048) throw 5;
      instruction = instruction | result;
      return instruction;
    } else { // 12 bit operand
      if (result>=4096) throw 5;
      instruction = instruction | result;
      return instruction;
    }
  }// immediate operand
  else { // register first operand
    if (result!=2) throw 5;
    uint16_t src = Registers[string(operation)];
    if (opcode<5 || opcode==7) { // 2 operands
      instruction = instruction | 0x0800 | (src<<3);
      result = next_token(input,&i,operation); //
      if (operation[0]==0 || result!=2) throw 5; // illegal operand
      src = Registers[operation];
    }
    if (opcode==5 || opcode==9) {
      instruction = instruction | 0x0800;
      src = src << 3;  //push, store uses src
    }
    instruction = instruction | src;
  } 
  return instruction;
}//assemble machine instruction from assembly language instruction

/*
Abstract Machine AMF+1

Instruction Format: 16bit fixed:
4 bit operation, 0=nop
1 bit register or immediate source (1=register)

if both source and destination needed:
max 8 bit immediate operand or 3 bit src register operand
3 bit destination register operand

if only one operand needed, 
1 bit register or immediate (1=register)
3 bit register or 11 bit immediate.  (push 1000)

memory ops must use the mar and mbr, which can serve as general purpose regs.
cx is tested by all conditional branches.
*/
