import ghidra.app.script.GhidraScript;
import ghidra.program.model.symbol.Reference;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.listing.Function;
import java.nio.file.*;
import java.util.*;
import com.google.gson.GsonBuilder;
public class ExportReferenceContexts extends GhidraScript {
 public void run() throws Exception {
  String[] args=getScriptArgs();Path out=Paths.get(args[0]);Files.createDirectories(out);List<Object> rows=new ArrayList<>();
  for(int a=1;a<args.length;a++) for(Reference ref:getReferencesTo(toAddr(args[a]))){
   Function fn=getFunctionContaining(ref.getFromAddress());Map<String,Object> row=new LinkedHashMap<>();row.put("target",args[a]);row.put("from",ref.getFromAddress().toString());row.put("reference_type",ref.getReferenceType().toString());row.put("function",fn==null?null:fn.getEntryPoint().toString());row.put("name",fn==null?null:fn.getName(true));
   Instruction ins=getInstructionAt(ref.getFromAddress());for(int n=0;n<24 && ins!=null && ins.getPrevious()!=null;n++)ins=ins.getPrevious();List<String> lines=new ArrayList<>();for(int n=0;n<28 && ins!=null;n++,ins=ins.getNext())lines.add(ins.getAddress()+" "+ins);row.put("context",lines);rows.add(row);
  }
  Files.writeString(out.resolve("references.json"),new GsonBuilder().setPrettyPrinting().create().toJson(rows));println("Exported bounded reference contexts: "+rows.size());
 }
}
