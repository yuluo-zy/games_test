import ghidra.app.script.GhidraScript;
import java.nio.file.*;
public class FindWallCacheWriters extends GhidraScript {
 public void run() throws Exception {
  StringBuilder out=new StringBuilder();
  for(var f:currentProgram.getFunctionManager().getFunctions(true)) {
   String n=f.getName(true);
   if(n.startsWith("bevy_ecs::system::system_param"))continue;
   StringBuilder hits=new StringBuilder(); boolean a=false,b=false;
   for(var i:currentProgram.getListing().getInstructions(f.getBody(),true)) {
    if(!i.getMnemonicString().startsWith("MOV"))continue;
    String dst=i.getDefaultOperandRepresentation(0);
    if(dst.contains("RBP")||dst.contains("RSP"))continue;
    if(dst.contains("0x108")||dst.contains("0x110")||dst.contains("0x10c")||dst.contains("0x114")) {
     if(dst.contains("0x108"))a=true;if(dst.contains("0x110"))b=true;
     hits.append(i.getAddress()).append(" ").append(i).append("\n");
    }
   }
   if(a)out.append(f.getEntryPoint()).append(" ").append(n).append("\n").append(hits);
  }
  Files.writeString(Paths.get(getScriptArgs()[0]),out.toString());
 }
}
