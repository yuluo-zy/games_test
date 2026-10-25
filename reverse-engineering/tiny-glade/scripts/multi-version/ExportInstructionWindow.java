import ghidra.app.script.GhidraScript;
import java.nio.file.*;
public class ExportInstructionWindow extends GhidraScript {
    public void run() throws Exception {
        String[] args=getScriptArgs();Path out=Paths.get(args[0]);Files.createDirectories(out);
        var from=toAddr(args[1]);var to=toAddr(args[2]);StringBuilder text=new StringBuilder();
        var instructions=currentProgram.getListing().getInstructions(from,true);
        while(instructions.hasNext()) {
            var instruction=instructions.next();if(instruction.getAddress().compareTo(to)>0)break;
            text.append(instruction.getAddress()).append(" ").append(instruction).append("\n");
        }
        Files.writeString(out.resolve(args[1]+"-"+args[2]+".asm.txt"),text.toString());
        byte[] bytes=new byte[(int)(to.subtract(from)+1)];currentProgram.getMemory().getBytes(from,bytes);Files.write(out.resolve("window.bin"),bytes);
        println("Exported instruction window "+from+".."+to);
    }
}
