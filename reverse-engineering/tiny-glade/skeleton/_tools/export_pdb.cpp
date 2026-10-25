#include "llvm/DebugInfo/PDB/Native/NativeSession.h"
#include "llvm/DebugInfo/PDB/Native/PDBFile.h"
#include "llvm/DebugInfo/PDB/Native/DbiStream.h"
#include "llvm/DebugInfo/PDB/Native/InfoStream.h"
#include "llvm/DebugInfo/PDB/Native/TpiStream.h"
#include "llvm/DebugInfo/PDB/Native/SymbolStream.h"
#include "llvm/DebugInfo/PDB/Native/ModuleDebugStream.h"
#include "llvm/DebugInfo/CodeView/DebugChecksumsSubsection.h"
#include "llvm/DebugInfo/CodeView/DebugLinesSubsection.h"
#include "llvm/DebugInfo/CodeView/DebugInlineeLinesSubsection.h"
#include "llvm/DebugInfo/CodeView/SymbolDeserializer.h"
#include "llvm/DebugInfo/CodeView/Line.h"
#include "llvm/ObjectYAML/CodeViewYAMLSymbols.h"
#include "llvm/ObjectYAML/CodeViewYAMLTypes.h"
#include "llvm/ObjectYAML/COFFYAML.h"
#include "llvm/Support/JSON.h"
#include "llvm/Support/FormatVariadic.h"
#include "llvm/Support/BinaryStreamReader.h"
#include <memory>
using namespace llvm;
using namespace llvm::pdb;
using namespace llvm::codeview;
static uint64_t Errors=0;
static COFF::header CoffHeader;
static std::string hexbytes(ArrayRef<uint8_t> a) {
  static const char *h="0123456789abcdef"; std::string s; s.reserve(a.size()*2);
  for(auto c:a){s+=h[c>>4];s+=h[c&15];}return s;
}
static void emit(json::Object o){outs()<<json::Value(std::move(o))<<'\n';}
static void error(json::Object &o,Error e){o["decode_error"]=toString(std::move(e));++Errors;}
static void yamlSym(json::Object &o,CVSymbol s){
  auto y=CodeViewYAML::SymbolRecord::fromCodeViewSymbol(s);
  if(!y){error(o,y.takeError());return;}
  std::string text;raw_string_ostream os(text);{yaml::Output out(os,&CoffHeader);out<<*y;}o["yaml"]=text;
}
static json::Object symbol(CVSymbol s,uint32_t off,int64_t mod){
  json::Object o{{"event","symbol"},{"module",mod},{"record_offset",off},
    {"kind",(int64_t)s.kind()},{"raw_hex",hexbytes(s.data())}};
  switch(s.kind()){
  case S_GPROC32:case S_LPROC32:case S_GPROC32_ID:case S_LPROC32_ID:
  case S_LPROC32_DPC:case S_LPROC32_DPC_ID:{
    auto r=SymbolDeserializer::deserializeAs<ProcSym>(s);if(!r){error(o,r.takeError());break;}
    o["class"]="procedure";o["name"]=r->Name.str();o["section"]=r->Segment;
    o["section_offset"]=r->CodeOffset;o["code_size"]=r->CodeSize;
    o["type_index"]=r->FunctionType.getIndex();o["parent"]=r->Parent;o["end"]=r->End;
    o["next"]=r->Next;o["debug_start"]=r->DbgStart;o["debug_end"]=r->DbgEnd;
    o["flags"]=(int64_t)r->Flags;o["type_namespace"]=(s.kind()==S_GPROC32_ID||s.kind()==S_LPROC32_ID||s.kind()==S_LPROC32_DPC_ID)?"IPI":"TPI";
    break;}
  case S_INLINESITE:{
    auto r=SymbolDeserializer::deserializeAs<InlineSiteSym>(s);if(!r){error(o,r.takeError());break;}
    o["class"]="inline_site";o["inlinee"]=r->Inlinee.getIndex();o["parent"]=r->Parent;o["end"]=r->End;
    o["annotations_hex"]=hexbytes(r->AnnotationData);json::Array ops;
    for(auto &a:r->annotations())ops.push_back(json::Array{a.Name.str(),a.U1,a.U2,a.S1});
    o["annotations"]=std::move(ops);break;}
  case S_END:case S_INLINESITE_END:case S_PROC_ID_END:o["class"]="scope_end";break;
  default:yamlSym(o,s);break;
  }return o;
}
int main(int argc,char **argv){
  if(argc<2)return 2;
  std::unique_ptr<IPDBSession> base;
  if(auto e=NativeSession::createFromPdbPath(argv[1],base)){errs()<<toString(std::move(e));return 1;}
  auto &ses=static_cast<NativeSession&>(*base);auto &pdb=ses.getPDBFile();
  auto machineDbi=pdb.getPDBDbiStream();
  if(!machineDbi){errs()<<toString(machineDbi.takeError());return 1;}
  CoffHeader.Machine=(COFF::MachineTypes)machineDbi->getMachineType();
  auto inf=pdb.getPDBInfoStream();if(!inf){errs()<<toString(inf.takeError());return 1;}
  std::string guid;raw_string_ostream gs(guid);gs<<inf->getGuid();
  json::Array streams,named;
  for(uint32_t i=0;i<pdb.getNumStreams();++i)streams.push_back(json::Array{i,pdb.getStreamByteSize(i)});
  for(auto &kv:inf->named_streams())named.push_back(json::Array{kv.first().str(),kv.second});
  emit(json::Object{{"event","identity"},{"guid",guid},{"age",inf->getAge()},{"streams",std::move(streams)},{"named_streams",std::move(named)}});
  for(bool ids:{false,true}){
    auto t=ids?pdb.getPDBIpiStream():pdb.getPDBTpiStream();
    if(!t){json::Object o{{"event","error"}};error(o,t.takeError());emit(std::move(o));continue;}
    uint32_t index=t->TypeIndexBegin();bool bad=false;
    for(auto rec:t->types(&bad)){
      json::Object o{{"event","type"},{"namespace",ids?"IPI":"TPI"},{"index",index++},
        {"kind",(int64_t)rec.kind()},{"raw_hex",hexbytes(rec.data())}};
      auto y=CodeViewYAML::LeafRecord::fromCodeViewRecord(rec);
      if(y){std::string text;raw_string_ostream os(text);{yaml::Output out(os);out<<*y;}o["yaml"]=text;}
      else error(o,y.takeError());emit(std::move(o));
    }
    if(bad){emit(json::Object{{"event","error"},{"message","type iterator failed"}});++Errors;}
  }
  auto sy=pdb.getPDBSymbolStream();
  if(sy){bool bad=false;uint32_t off=0;for(auto s:sy->getSymbols(&bad)){emit(symbol(s,off,-1));off+=s.length();}
    if(bad){emit(json::Object{{"event","error"},{"message","global symbol iterator failed"}});++Errors;}}
  else {json::Object o{{"event","error"}};error(o,sy.takeError());emit(std::move(o));}
  auto db=pdb.getPDBDbiStream();if(!db){errs()<<toString(db.takeError());return 1;}
  auto str=pdb.getStringTable();if(!str){errs()<<toString(str.takeError());return 1;}
  auto &mods=db->modules();uint64_t totalSymbols=0,totalLines=0,totalInlinees=0;
  for(uint32_t mi=0;mi<mods.getModuleCount();++mi){
    auto md=mods.getModuleDescriptor(mi);json::Array sources;
    for(auto f:mods.source_files(mi))sources.push_back(f.str());
    emit(json::Object{{"event","module"},{"index",mi},{"name",md.getModuleName().str()},
      {"object",md.getObjFileName().str()},{"stream",md.getModuleStreamIndex()},{"files",std::move(sources)}});
    if(md.getModuleStreamIndex()==0xffff){emit(json::Object{{"event","module_end"},{"module",mi}});continue;}
    auto m=ses.getModuleDebugStream(mi);
    if(!m){json::Object o{{"event","error"},{"module",mi}};error(o,m.takeError());emit(std::move(o));emit(json::Object{{"event","module_end"},{"module",mi}});continue;}
    uint32_t suboff=0;
    for(auto &sub:m->subsections()){
      auto data=sub.getRecordData();ArrayRef<uint8_t> bytes;
      if(auto e=data.readBytes(0,data.getLength(),bytes)){json::Object o{{"event","error"},{"module",mi}};error(o,std::move(e));emit(std::move(o));continue;}
      json::Object raw{{"event","subsection"},{"module",mi},{"c13_offset",suboff},{"kind",(int64_t)sub.kind()},{"raw_hex",hexbytes(bytes)}};
      emit(std::move(raw));suboff+=sub.getRecordLength();suboff=(suboff+3)&~3;
      if(sub.kind()==DebugSubsectionKind::FileChecksums){
        DebugChecksumsSubsectionRef checks;
        if(auto e=checks.initialize(data)){json::Object o{{"event","error"},{"module",mi}};error(o,std::move(e));emit(std::move(o));continue;}
        auto &arr=checks.getArray();for(auto it=arr.begin();it!=arr.end();++it){auto &c=*it;auto path=str->getStringForID(c.FileNameOffset);
          json::Object o{{"event","checksum"},{"module",mi},{"file_id",it.offset()},{"name_offset",c.FileNameOffset},{"algorithm",(int64_t)c.Kind},{"checksum",hexbytes(c.Checksum)}};
          if(path)o["path"]=path->str();else error(o,path.takeError());emit(std::move(o));}
      }else if(sub.kind()==DebugSubsectionKind::Lines){
        DebugLinesSubsectionRef lines;
        if(auto e=lines.initialize(BinaryStreamReader(data))){json::Object o{{"event","error"},{"module",mi}};error(o,std::move(e));emit(std::move(o));continue;}
        auto h=lines.header();for(auto &block:lines){json::Array ls;uint32_t li=0;
          for(auto &l:block.LineNumbers){LineInfo info(l.Flags);json::Object row{{"offset",(uint32_t)l.Offset},{"flags",(uint32_t)l.Flags},{"line",info.getStartLine()},{"end_line",info.getEndLine()},{"statement",info.isStatement()}};
            if(lines.hasColumnInfo()){auto c=block.Columns[li];row["start_column"]=(uint16_t)c.StartColumn;row["end_column"]=(uint16_t)c.EndColumn;}++li;ls.push_back(std::move(row));++totalLines;}
          emit(json::Object{{"event","lines"},{"module",mi},{"file_id",(uint32_t)block.NameIndex},{"section",(uint16_t)h->RelocSegment},{"section_offset",(uint32_t)h->RelocOffset},{"code_size",(uint32_t)h->CodeSize},{"flags",(uint16_t)h->Flags},{"lines",std::move(ls)}});}
      }else if(sub.kind()==DebugSubsectionKind::InlineeLines){
        DebugInlineeLinesSubsectionRef il;
        if(auto e=il.initialize(data)){json::Object o{{"event","error"},{"module",mi}};error(o,std::move(e));emit(std::move(o));continue;}
        for(auto &l:il){json::Array extras;for(auto x:l.ExtraFiles)extras.push_back((uint32_t)x);
          emit(json::Object{{"event","inlinee"},{"module",mi},{"inlinee",l.Header->Inlinee.getIndex()},{"file_id",(uint32_t)l.Header->FileID},{"line",(uint32_t)l.Header->SourceLineNum},{"extra_files",std::move(extras)}});++totalInlinees;}
      }
    }
    bool bad=false;uint32_t off=4;
    for(auto s:m->symbols(&bad)){emit(symbol(s,off,mi));off+=s.length();++totalSymbols;}
    if(bad){emit(json::Object{{"event","error"},{"module",mi},{"message","module symbol iterator failed"}});++Errors;}
    emit(json::Object{{"event","module_end"},{"module",mi}});
    if(mi%100==0)errs()<<"module "<<mi<<"/"<<mods.getModuleCount()<<" symbols "<<totalSymbols<<"\n";
  }
  emit(json::Object{{"event","complete"},{"modules",mods.getModuleCount()},{"module_symbols",totalSymbols},{"line_entries",totalLines},{"inlinee_records",totalInlinees},{"decode_errors",Errors}});
  return 0;
}
