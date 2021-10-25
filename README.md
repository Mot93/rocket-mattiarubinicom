# rocket_mattiarubinicom
My personal website created in rust with rocket

I had a lot of fun developing everything with containers

## How to build with podman

    podman build mattiarubinicom.Dockerfile --tag mattiarubinicom

    podman run --publish 8080:80/tcp mattiarubinicom 

## Using podman-compose

    podman-compose build

    podman-compose up

    podman-compose down

## Going generating a yaml for kubernetes

    podman-compose up

    podman pod ps

    podman generate kube -s rocket_mattiarubinicom > mattiarubinicom-kube.yaml

    kubectl apply -f mattiarubinicom-kube.yaml

# Notes

To bind the containers on a port < 1024 you need to run the previous command as root.