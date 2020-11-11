import java.lang.invoke.*;
import jdk.incubator.foreign.*;
import java.nio.file.Paths;

class PanamaMain {
  public static void main(String[] args) throws Throwable {
    // get System linker
    var linker = CLinker.getInstance();
    var lookup = LibraryLookup.ofPath(Paths.get(System.getProperty("user.dir")+"/target/debug/librs_handlegraph_ffi.so"));

    // get a native method handle for 'getpid' function
    var getpid = linker.downcallHandle(
           lookup.lookup("node_is_reverse").get(),
           MethodType.methodType(boolean.class, MemoryAddress.class),
           FunctionDescriptor.of(CLinker.C_CHAR, CLinker.C_POINTER));

    // invoke it!
    System.out.println((int)getpid.invokeExact());
  }
}
