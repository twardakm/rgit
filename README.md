# `rgit`

## Using docker to build

### Build image

`docker buildx build -t rgit .`

### Start `bash` session

`docker run -it rgit bash`

### Build `rgit` in container and copy restult to host PC

`docker buildx build -t rgit --target debug-ubuntu-16-04 --output . .`

Result is in `target/ubuntu-16-04/debug`
