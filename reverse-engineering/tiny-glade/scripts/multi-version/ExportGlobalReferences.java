import ghidra.app.script.GhidraScript;
import ghidra.program.model.symbol.Reference;
import ghidra.program.model.listing.Function;
import java.nio.file.*;
import java.util.*;
import com.google.gson.GsonBuilder;
public class ExportGlobalReferences extends GhidraScript {
    public void run() throws Exception {
        String[] args=getScriptArgs();Path out=Paths.get(args[0]);Files.createDirectories(out);
        var address=toAddr(args[1]);Map<String,Object> report=new LinkedHashMap<>();
        report.put("program",currentProgram.getName());report.put("database_sha256",currentProgram.getExecutableSHA256());
        report.put("address",address.toString());report.put("database_read_only",true);
        var symbol=getSymbolAt(address);report.put("symbol",symbol==null?null:symbol.getName(true));
        report.put("initial_byte",Byte.toUnsignedInt(currentProgram.getMemory().getByte(address)));
        var block=currentProgram.getMemory().getBlock(address);report.put("memory_block",block==null?null:block.getName());
        List<Object> refs=new ArrayList<>();
        for(Reference ref:getReferencesTo(address)) {
            Map<String,Object> row=new LinkedHashMap<>();row.put("from",ref.getFromAddress().toString());row.put("type",ref.getReferenceType().toString());row.put("write",ref.getReferenceType().isWrite());
            Function fn=getFunctionContaining(ref.getFromAddress());row.put("function_address",fn==null?null:fn.getEntryPoint().toString());row.put("function_name",fn==null?null:fn.getName(true));
            var instruction=getInstructionAt(ref.getFromAddress());row.put("instruction",instruction==null?null:instruction.toString());refs.add(row);
        }
        report.put("references",refs);Files.writeString(out.resolve("global-references.json"),new GsonBuilder().setPrettyPrinting().create().toJson(report));
        println("Exported global references "+address+" count="+refs.size());
    }
}
