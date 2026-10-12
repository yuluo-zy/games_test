import ghidra.app.script.GhidraScript;
import ghidra.program.model.data.*;
import java.nio.file.*;
import java.util.*;
import com.google.gson.GsonBuilder;

/** Read-only exact private type metadata; not a source or ABI recovery claim. */
public class DescribeFocusedDataTypes extends GhidraScript {
 public void run() throws Exception {
  String[] args=getScriptArgs(); List<Object> rows=new ArrayList<>();
  Iterator<DataType> types=currentProgram.getDataTypeManager().getAllDataTypes();
  while(types.hasNext()) {
   DataType type=types.next(); String name=type.getName(); boolean match=false;
   for(int i=1;i<args.length;i++) match |= name.equals(args[i]) || name.endsWith("::"+args[i]);
   if(!match) continue;
   Map<String,Object> row=new LinkedHashMap<>(); row.put("path",type.getPathName());
   row.put("name",name); row.put("length",type.getLength());
   if(type instanceof Composite) {
    List<Object> fields=new ArrayList<>();
    for(DataTypeComponent field:((Composite)type).getComponents()) {
     Map<String,Object> f=new LinkedHashMap<>(); f.put("offset",field.getOffset());
     f.put("length",field.getLength()); f.put("name",field.getFieldName());
     f.put("type",field.getDataType().getPathName());fields.add(f);
    }
    row.put("fields",fields);
   } else if(type instanceof TypeDef) {
    row.put("base",((TypeDef)type).getBaseDataType().getPathName());
   }
   rows.add(row);
  }
  Map<String,Object> report=new LinkedHashMap<>();report.put("program",currentProgram.getName());
  report.put("database_sha256",currentProgram.getExecutableSHA256());report.put("read_only",true);
  report.put("types",rows);Files.writeString(Paths.get(args[0]),new GsonBuilder().setPrettyPrinting().create().toJson(report));
  println("Focused types: "+rows.size());
 }
}
