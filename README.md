# rocket_mattiarubinicom
My personal website created in rust with rocket

I had a lot of fun developing everything with containers

## How to build with podman

    sudo podman build . --tag mattiarubinicom

    sudo podman run --publish 80:80/tcp mattiarubinicom 

## Using podman-compose

    sudo podmna-compose up