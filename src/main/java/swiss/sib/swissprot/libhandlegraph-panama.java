package swiss.sib.swissprot;

import java.lang.invoke.*;
import jdk.incubator.foreign.*;
import jdk.incubator.foreign.GroupLayout;
import java.nio.file.Paths;

class PanamaMain {

    private static MethodHandle node_is_reverse;
    private static MethodHandle parse_gfa_into_hash_graph;

    public static void main(String[] args) throws Throwable {
        // get System linker
        var gfaFile = args[0];
        var linker = CLinker.getInstance();
        var lookup = LibraryLookup.ofPath(Paths.get(System.getProperty("user.dir") + "/target/debug/librs_handlegraph_ffi.so"));
        System.err.println("Tryin to parse " + gfaFile);
//        node_is_reverse = linker.downcallHandle(lookup.lookup("node_is_reverse").get(),
//           MethodType.methodType(byte.class, MemoryAddress.class)
//        ,
//           FunctionDescriptor.of(CLinker.C_CHAR, CLinker.C_POINTER)
//        );
//        parse_gfa_into_hash_graph = linker.downcallHandle(lookup.lookup("load_hashgraph").get(),
//                MethodType.methodType(MemorySegment.class, MemoryAddress.class, long.class),
//                FunctionDescriptor.of(MemoryLayout.ofStruct(MemoryLayouts.ADDRESS), CLinker.C_POINTER, CLinker.C_LONG));
        parse_gfa_into_hash_graph = linker.downcallHandle(lookup.lookup("load_hashgraph").get(),
                MethodType.methodType(MemoryAddress.class, MemoryAddress.class, long.class),
                FunctionDescriptor.of(MemoryLayout.ofStruct(MemoryLayouts.ADDRESS), CLinker.C_POINTER, CLinker.C_LONG));
        final MemorySegment toCString = CLinker.toCString(gfaFile);

        MethodHandle strlen = CLinker.getInstance().downcallHandle(
                LibraryLookup.ofDefault().lookup("strlen").get(),
                MethodType.methodType(long.class, MemoryAddress.class),
                FunctionDescriptor.of(CLinker.C_LONG, CLinker.C_POINTER)
        );
//        MethodHandle strlen = CLinker.getInstance().downcallHandle(
//                LibraryLookup.ofDefault().lookup("strlen").get(),
//                MethodType.methodType(int.class, MemoryAddress.class),
//                FunctionDescriptor.of(CLinker.C_INT, CLinker.C_POINTER)
//        );
        final MemoryAddress address = toCString.address();
        final long cStringLength = (long) strlen.invokeExact(address);
        parse_gfa_into_hash_graph.invokeExact(address, cStringLength);
    }

//  public boolean nodeIsReverse(MemoryAddress nodeAddress) throws Throwable {
//      return  (byte) node_is_reverse.invokeExact(nodeAddress) > 0;
//    }
}
