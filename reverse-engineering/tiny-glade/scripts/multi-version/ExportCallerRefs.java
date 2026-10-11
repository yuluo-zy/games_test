import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import java.nio.file.*;
public class ExportCallerRefs extends GhidraScript {
 public void run() throws Exception {
  StringBuilder s=new StringBuilder();
  String[] a=getScriptArgs();
  for(int i=1;i<a.length;i++) {
   s.append("TARGET ").append(a[i]).append("\n");
   for(var r:getReferencesTo(toAddr(a[i]))) {
    Function f=getFunctionContaining(r.getFromAddress());
    s.append(r.getFromAddress()).append(" ").append(f==null?"?":f.getEntryPoint()+" "+f.getName(true)).append("\n");
   }
  }
  Files.writeString(Paths.get(a[0]),s.toString());
 }
}
