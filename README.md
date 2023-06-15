# Container Runtime Prototype

### Current State
1. Main function extracts a ubuntu:22.04 docker image to `/tmp/crp/UUID`
2. Starts new Linux namespace
3. Executes `chroot` to the extracted image path
4. Executes `ls -alh` in the new container

```
git clone https://github.com/lukasmetzner/container-runtime-prototype && cd container-runtime-prototype
docker pull ubuntu:22.04 && docker save ubuntu:22.04 > ./images/ubuntu.tar
./run.sh
```