# DEMO

```console
$ cd example

$ cat Dockerfile
FROM tianon/toybox:0.8
RUN echo "Hello World!" >/banner

$ tuggy -t mcandre/tuggy-demo --load

$ docker run --rm mcandre/tuggy-demo cat /banner
Hello World!
```
