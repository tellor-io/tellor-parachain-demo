### Build image
```
docker build --platform linux/amd64 -t tellor-parachain-demo .
```
### Tag image
```
docker tag tellor-parachain-demo tellorofficial/tellor-parachain-demo:latest
```
### Push image to Docker Hub
```
docker push tellorofficial/tellor-parachain-demo:latest
```