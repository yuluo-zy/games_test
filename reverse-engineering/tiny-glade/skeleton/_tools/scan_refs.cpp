#include "llvm/MC/MCAsmInfo.h"
#include "llvm/MC/MCContext.h"
#include "llvm/MC/MCDisassembler/MCDisassembler.h"
#include "llvm/MC/MCInstrAnalysis.h"
#include "llvm/MC/MCInstrInfo.h"
#include "llvm/MC/MCInstPrinter.h"
#include "llvm/MC/MCRegisterInfo.h"
#include "llvm/MC/MCSubtargetInfo.h"
#include "llvm/MC/TargetRegistry.h"
#include "llvm/Support/TargetSelect.h"
#include "llvm/TargetParser/Triple.h"
#include "llvm/Support/JSON.h"
#include "llvm/Support/raw_ostream.h"
#include <fstream>
#include <vector>
#include <set>
#include <cstring>
#include <iterator>
using namespace llvm;
static std::string hex(const uint8_t *p,size_t n){const char*h="0123456789abcdef";std::string s;for(size_t i=0;i<n;++i){s+=h[p[i]>>4];s+=h[p[i]&15];}return s;}
static void emit(json::Object o){outs()<<json::Value(std::move(o))<<'\n';}
int main(int argc,char**argv){
 if(argc!=3)return 2;
 InitializeAllTargetInfos();InitializeAllTargetMCs();InitializeAllDisassemblers();
 std::string err;Triple triple("x86_64-pc-windows-msvc");auto *target=TargetRegistry::lookupTarget(triple.str(),err);if(!target)return 3;
 std::unique_ptr<MCRegisterInfo> regs(target->createMCRegInfo(triple.str()));
 MCTargetOptions opts;std::unique_ptr<MCAsmInfo> ai(target->createMCAsmInfo(*regs,triple.str(),opts));
 std::unique_ptr<MCSubtargetInfo> sti(target->createMCSubtargetInfo(triple.str(),"",""));
 std::unique_ptr<MCInstrInfo> ii(target->createMCInstrInfo());MCContext context(triple,ai.get(),regs.get(),sti.get());
 std::unique_ptr<MCDisassembler> dis(target->createMCDisassembler(*sti,context));
 std::unique_ptr<MCInstrAnalysis> analysis(target->createMCInstrAnalysis(ii.get()));
 std::unique_ptr<MCInstPrinter> printer(target->createMCInstPrinter(triple,0,*ai,*ii,*regs));
 std::ifstream bin(argv[1],std::ios::binary);std::vector<uint8_t> bytes((std::istreambuf_iterator<char>(bin)),{});
 std::ifstream ranges(argv[2]);uint64_t rva,fo,size,total=0,ins=0,edges=0;
 while(ranges>>rva>>fo>>size){
  if(fo+size>bytes.size()||size==0)return 4;
  std::vector<uint64_t> todo{0};std::set<uint64_t> visited;uint64_t gaps=0,ijmp=0;
  while(!todo.empty()){
   uint64_t at=todo.back();todo.pop_back();
   while(at<size && !visited.count(at)){
    MCInst inst;uint64_t len=0;auto status=dis->getInstruction(inst,len,ArrayRef<uint8_t>(bytes.data()+fo+at,size-at),rva+at,nulls());
    if(status==MCDisassembler::Fail||!len){++gaps;break;}visited.insert(at);++ins;
    std::string asmtext;raw_string_ostream aos(asmtext);printer->printInst(&inst,rva+at,"",*sti,aos);auto *p=bytes.data()+fo+at;
    bool call=analysis->isCall(inst),jump=analysis->isUnconditionalBranch(inst),branch=analysis->isConditionalBranch(inst);
    bool ret=analysis->isReturn(inst)||ii->getName(inst.getOpcode())=="TRAP"||ii->getName(inst.getOpcode())=="INT3";
    uint64_t dest=0;bool direct=analysis->evaluateBranch(inst,rva+at,len,dest);uint64_t target=dest;
    if(call||(jump&&(!direct||target<rva||target>=rva+size))){
     json::Object row{{"event","reference"},{"caller_entry_rva",rva},{"instruction_rva",rva+at},{"length",len},{"bytes",hex(p,len)},{"assembly",asmtext},{"llvm_opcode",ii->getName(inst.getOpcode()).str()},{"soft_fail",status==MCDisassembler::SoftFail},{"kind",call?(direct?"direct_call":"indirect_call_unknown"):direct?"direct_jump_outside_procedure":"indirect_jump_unknown"}};
     if(direct)row["target_rva"]=target;else row["target_rva"]=nullptr;
     emit(std::move(row));++edges;
    }
    if((branch||jump)&&direct&&target>=rva&&target<rva+size)todo.push_back(target-rva);
    if(jump){if(!direct)++ijmp;break;}
    if(ret)break;
    at+=len;
   }
  }
  emit(json::Object{{"event","coverage"},{"entry_rva",rva},{"pdb_code_size",size},{"decoded_instruction_starts",visited.size()},{"decode_gaps",gaps},{"unresolved_indirect_jumps",ijmp}});++total;
 }
 emit(json::Object{{"event","complete"},{"ranges",total},{"instructions",ins},{"reference_records",edges}});
 return 0;
}
