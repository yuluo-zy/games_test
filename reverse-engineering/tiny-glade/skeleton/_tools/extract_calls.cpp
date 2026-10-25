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
#include "llvm/Support/MemoryBuffer.h"
#include "llvm/Support/JSON.h"
#include "llvm/TargetParser/Triple.h"
#include <memory>
using namespace llvm;
static std::string hexbytes(ArrayRef<uint8_t> a){
  static const char *h="0123456789abcdef";std::string s;for(auto c:a){s+=h[c>>4];s+=h[c&15];}return s;
}
static void emit(json::Object o){outs()<<json::Value(std::move(o))<<'\n';}
int main(int argc,char **argv){
  if(argc!=3)return 2;
  auto binary=MemoryBuffer::getFile(argv[1]);auto input=MemoryBuffer::getFile(argv[2]);
  if(!binary||!input)return 2;
  auto parsed=json::parse((*input)->getBuffer());if(!parsed){errs()<<toString(parsed.takeError());return 2;}
  auto *ranges=parsed->getAsArray();if(!ranges)return 2;
  InitializeAllTargetInfos();InitializeAllTargetMCs();InitializeAllDisassemblers();
  std::string err;Triple triple("x86_64-pc-windows-msvc");auto *target=TargetRegistry::lookupTarget(triple.str(),err);
  if(!target){errs()<<err;return 1;}
  std::unique_ptr<MCRegisterInfo> regs(target->createMCRegInfo(triple.str()));
  MCTargetOptions opts;std::unique_ptr<MCAsmInfo> ai(target->createMCAsmInfo(*regs,triple.str(),opts));
  std::unique_ptr<MCSubtargetInfo> sti(target->createMCSubtargetInfo(triple.str(),"",""));
  std::unique_ptr<MCInstrInfo> ii(target->createMCInstrInfo());
  MCContext context(triple,ai.get(),regs.get(),sti.get());
  std::unique_ptr<MCDisassembler> dis(target->createMCDisassembler(*sti,context));
  std::unique_ptr<MCInstrAnalysis> analysis(target->createMCInstrAnalysis(ii.get()));
  std::unique_ptr<MCInstPrinter> printer(target->createMCInstPrinter(triple,0,*ai,*ii,*regs));
  auto data=(*binary)->getBuffer();uint64_t instructions=0,calls=0,gaps=0;
  for(auto &value:*ranges){auto *r=value.getAsObject();if(!r)return 2;
    uint64_t start=*r->getInteger("va"),file=*r->getInteger("file_offset"),length=*r->getInteger("length");
    if(file+length>data.size())return 2;
    auto bytes=ArrayRef<uint8_t>((const uint8_t*)data.data()+file,length);uint64_t offset=0,rangeInstructions=0;
    while(offset<length){MCInst inst;uint64_t size=0;
      auto status=dis->getInstruction(inst,size,bytes.drop_front(offset),start+offset,nulls());
      if(status==MCDisassembler::Fail||size==0){
        emit(json::Object{{"event","decode_gap"},{"range_va",start},{"va",start+offset},{"remaining_bytes",length-offset},{"raw_hex",hexbytes(bytes.drop_front(offset))}});
        ++gaps;break;
      }
      ++instructions;++rangeInstructions;
      bool call=analysis->isCall(inst),jump=analysis->isUnconditionalBranch(inst);
      uint64_t dest=0;bool direct=analysis->evaluateBranch(inst,start+offset,size,dest);
      if(call||(jump&&direct&&(dest<start||dest>=start+length))){
        std::string text;raw_string_ostream os(text);printer->printInst(&inst,start+offset,"",*sti,os);
        json::Object row{{"event",call?"call":"external_jump"},{"range_va",start},{"va",start+offset},{"size",size},
          {"raw_hex",hexbytes(bytes.slice(offset,size))},{"assembly",text},{"llvm_opcode",ii->getName(inst.getOpcode()).str()},
          {"direct",direct},{"soft_fail",status==MCDisassembler::SoftFail}};
        if(direct)row["target_va"]=dest;
        emit(std::move(row));++calls;
      }
      offset+=size;
    }
    emit(json::Object{{"event","range_end"},{"range_va",start},{"decoded_bytes",offset},{"length",length},{"instructions",rangeInstructions},{"complete",offset==length}});
  }
  emit(json::Object{{"event","complete"},{"ranges",ranges->size()},{"instructions",instructions},{"control_transfers",calls},{"decode_gaps",gaps}});
  return 0;
}
