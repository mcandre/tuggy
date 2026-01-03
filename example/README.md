# DEMO

```console
$ cd example

$ cat Dockerfile
FROM tianon/toybox:0.8
RUN echo "Hello World!" >/banner

$ tuggy -t n4jm4/tuggy-demo --load

$ docker run --rm n4jm4/tuggy-demo cat /banner
Hello World!
```
