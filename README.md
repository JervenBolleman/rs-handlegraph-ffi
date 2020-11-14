# rs-handlegraph-ffi

Make it easy to access rs-handlegraph implementations in an other language.


# java

Uses early access build of project panama e.g. 

```
 ~/bin/jdk-16/bin/java \
    -Dforeign.restricted=permit \
    --add-modules jdk.incubator.foreign \
    src/main/java/swiss/sib/swissprot/libhandlegraph-panama.java
```
```
jextract \
    -l rs_handlegraph_ffi \
    -d target/java_classes \
    -t rs.handlegraph \
    libhandlegraph-ffi-c.h 
```

```
jar cvf rs-handlegraph.jar -C target/java_classes/ .
```
# install into local mvn

mvn install:install-file  -DgroupId=rs.handlegraph -DartifactId=jextract -Dpackaging=jar -Dversion=0.0.1 -Dfile=rs-handlegraph.jar


