import ghidra.app.script.GhidraScript;
import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileResults;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import java.nio.file.*;
import java.util.*;
import com.google.gson.GsonBuilder;

/** Read existing analyzed database without modifying or reanalyzing it. */
public class ExportFocusedFunctions extends GhidraScript {
    public void run() throws Exception {
        String[] args = getScriptArgs();
        Path out = Paths.get(args[0]);
        Files.createDirectories(out);
        Map<String,Object> report = new LinkedHashMap<>();
        report.put("program", currentProgram.getName());
        report.put("executable_sha256", currentProgram.getExecutableSHA256());
        report.put("database_read_only", true);
        report.put("target_execution", false);
        report.put("auto_analysis", false);
        List<Object> rows = new ArrayList<>();
        DecompInterface decompiler = new DecompInterface();
        try {
            if (!decompiler.openProgram(currentProgram)) throw new Exception("Decompiler failed to open program");
            for (int i=1;i<args.length;i++) {
                if (args[i].startsWith("mem:")) {
                    String[] request = args[i].substring(4).split(":");
                    byte[] bytes = new byte[Integer.parseInt(request[1])];
                    currentProgram.getMemory().getBytes(toAddr(request[0]), bytes);
                    Files.write(out.resolve("memory-"+request[0].replace("0x", "")+".bin"), bytes);
                    continue;
                }
                Function fn = getFunctionAt(toAddr(args[i]));
                if (fn==null) throw new Exception("No exact function at " + args[i]);
                DecompileResults result = decompiler.decompileFunction(fn, 90, monitor);
                String name = args[i].replace("0x", "");
                Map<String,Object> row = new LinkedHashMap<>();
                row.put("address", args[i]);
                row.put("name", fn.getName(true));
                row.put("body_bytes", fn.getBody().getNumAddresses());
                row.put("decompiled", result.decompileCompleted());
                row.put("error", result.getErrorMessage());
                if (result.decompileCompleted()) {
                    Files.writeString(out.resolve(name+".c"), result.getDecompiledFunction().getC());
                }
                StringBuilder assembly = new StringBuilder();
                for (Instruction ins : currentProgram.getListing().getInstructions(fn.getBody(), true)) {
                    assembly.append(ins.getAddress()).append(" ").append(ins).append("\n");
                }
                Files.writeString(out.resolve(name+".asm.txt"), assembly.toString());
                rows.add(row);
                println("Exported " + args[i] + " bytes=" + fn.getBody().getNumAddresses());
            }
        } finally {
            decompiler.dispose();
        }
        report.put("functions", rows);
        Files.writeString(out.resolve("report.json"), new GsonBuilder().setPrettyPrinting().create().toJson(report));
    }
}
