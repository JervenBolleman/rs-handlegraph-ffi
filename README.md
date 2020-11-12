# rs-handlegraph-ffi

Make it easy to access rs-handlegraph implementations in an other language.


# java

Uses early access build of project panama

```
 ~/bin/jdk-16/bin/java \
    -Dforeign.restricted=permit \
    --add-modules jdk.incubator.foreign \
    src/main/java/swiss/sib/swissprot/libhandlegraph-panama.java
```
```
jextract  -C -x -C c++ \
    -l $(pwd)/target/debug/librs_handlegraph_ffi.so \
    -t rs.handlegraph \
    libhandlegraph-ffi-c.h 
```


