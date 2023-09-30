# Tellor Parachain Integration Tests

### Docker
Run tests in Docker container:
- [Install Docker](https://docs.docker.com/get-docker/)
- Build and run the `tellor-parachain-integration-tests` image defined in `Dockerfile` using the command:
```shell
docker build -t tellor-parachain-integration-tests . && docker run --rm tellor-parachain-integration-tests
```

Run individual tests with output using the command:
```shell
docker run --rm tellor-parachain-integration-tests --test test_name --nocapture
```
