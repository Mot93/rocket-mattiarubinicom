# rocket_mattiarubinicom
My personal website created in rust with rocket

I had a lot of fun developing everything with containers

## How to build with podman

    podman build . --tag mattiarubinicom

    podman run --publish 8080:80/tcp mattiarubinicom 

## Using podman-compose

    podmna-compose up

    podman-compose down

## Going generating a yaml for kubernetes

    podman generate kube <missing>

# Notes

To bind the containers on a port < 1024 you need to run the previous command as root.