# rgit

## Overview

Tool to detect and work on multiple, independent repositories.

It may work supplementary `repo` tool (https://android.googlesource.com/tools/repo) or as an independent way of dealing with multiple git repositories, for example in projects based on Yocto, where every meta layer is separate repository (not necessarily managed by `repo` tool).

## Examples

**Scan repositories and execute `git` command on each of them**

`rgit scan --relative | rgit exec -c "status"`

**Save scan results in the file and use later to execute some other command**

`rgit scan --relative -s`

`rgit exec -c "status --porcelain"`

Example output:

```bash
VideoStation-FFMPEG-Patcher

collectd/collectd-graph-generator
 M src/processes/processes_graph.rs

design-patterns
 M bridge/src/sorter/int_sorter.rs
 M bridge/src/sorter/mod.rs
 M bridge/src/sorter_impl/bubble_sort.rs
 M bridge/src/sorter_impl/insertion_sort.rs
 M bridge/src/sorter_impl/sorter_impl.rs
 M bridge/uml/class_diagram.png
 M bridge/uml/class_diagram.txt
```

## Using docker to build

### Build image

`docker buildx build -t rgit .`

### Start `bash` session

`docker run -it rgit bash`

### Build `rgit` in container and copy restult to host PC

`docker buildx build -t rgit --target debug-ubuntu-16-04 --output . .`

Result is in `target/ubuntu-16-04/debug`
