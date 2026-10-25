import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import java.nio.file.*;
import java.util.*;
import com.google.gson.GsonBuilder;
/** Metadata-only ownership lookup; no decompiler or auto-analysis. */
public class DescribeFocusedFunctions extends GhidraScript {
 public void run() throws Exception {
  String[] args=getScriptArgs(); List<Object> rows=new ArrayList<>();
  for(int i=1;i<args.length;i++) {
   Function fn=getFunctionAt(toAddr(args[i]));Map<String,Object> row=new LinkedHashMap<>();row.put("address",args[i]);
   if(fn!=null){row.put("name",fn.getName(true));row.put("body_bytes",fn.getBody().getNumAddresses());}
   else row.put("exact_function_missing",true);rows.add(row);
  }
  Files.writeString(Paths.get(args[0]),new GsonBuilder().setPrettyPrinting().create().toJson(rows));println("Metadata rows: "+rows.size());
 }
}
