## Run integration tests using Docker
### Prerequisites
- System requirements: >= 20GB RAM, >= 8 CPU cores, Linux or Amd64 architecture. For example, an [e2-standard-8 instance on Google Cloud Compute](https://cloud.google.com/compute/vm-instance-pricing#general-purpose_machine_type_family) is sufficient.
- [Docker](https://docs.docker.com/engine/install/)
### Build image and launch container
- You can build the image yourself using the command `docker build --platform linux/amd64 -t <image-name> .` in the root directory of the repository. Alternatively, you can pull the image from Docker Hub using `docker pull tellorofficial/tellor-parachain-demo:latest`.
- Run the container using your built image with `docker run -it --name <container-name> <image-name>`, or using image pulled from Docker Hub with `docker run -it --name <container-name> tellorofficial/tellor-parachain-demo:latest`. If you decide to use a VM instance on Google Cloud Compute to [deploy the container](https://cloud.google.com/compute/docs/containers/deploying-containers), then you can use `docker exec -it <container-id> /bin/bash` to open an interactive terminal session in the container.
### Launch chains
- Once you have an interactive terminal session in the container, launch a network in the background (`rococo-local`, `statemine-local`, `moonbase-local` and oracle consumer parachain (with Tellor pallet)) using the `launch` script:
```
./scripts/launch.sh &
```
### Deploy contracts
- Use the `deploy` script to deploy the Tellor contracts to Moonbeam as well as perform required chain state initialisation (in the background):
```
./scripts/deploy.sh &
```
### Run tests
- todo:
