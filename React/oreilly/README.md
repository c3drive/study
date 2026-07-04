# Build
docker build --no-cache -f ./Dockerfile . -t miharasatsuki/react_oreilly

# enter the container
docker run -it miharasatsuki/react_oreilly bash