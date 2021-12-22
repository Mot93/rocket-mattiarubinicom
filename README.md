# rocket-mattiarubinicom
My personal website created in [rust](https://www.rust-lang.org/) with [rocket](https://rocket.rs/).

I had a lot of fun developing with container technology.

# How to build and run the image 
All the following instruction can be performed once inside the `container/` folder
    
    cd container/

## Using `podman`

1. Build the image usign the `Dockerfile`:

        podman build -f mattiarubinicom.Dockerfile --tag mattiarubinicom ./../

2. Start the container

    The first time:

        podman run --publish 8080:80/tcp -d --name mattiarubinicom mattiarubinicom 

    All subsequence times:

        podman run -d mattiarubinicom

3. Stop the container

        podman stop mattiarubinicom

## **Podman notes**
To bind the containers on a ***port lower `1024`*** you need to run the previous command as root.

## Using `docker`
If you prefer to use docker, simply substitute `podman` with `docker`.

Unlike podman, docker always runs as root and can bind any container to any port.

## Using `podman-compose`

1. Build all the necessary images

        podman-compose -p pod-mattiarubinicom build

2. Build the infrastructure and start all the containers
    
    **NOTE**: Unlike with docker-compose, all container are inside the same pod and cannot listen on the same port.

        podman-compose -p pod-mattiarubinicom up -d

3. Stop all the container and dismantle the infrastructure

        podman-compose -p pod-mattiarubinicom down

## Using `docker-compose`
Just replace `podman-compose` with `docker-compose`.

**NOTE**: unlike `docker-compose` all container are place in a virtual network and can listen on the same port.

# Uploading to a container registry

## Using `podman` 

1. Download the jenkins war file of the version you would like to use. 
    
    If the latest version is needed, just launch the `download_latest.sh` file.

2. Build the image usign the `Dockerfile`:

        podman build . --tag <username>/mattiarubinicom:<tagname>

3. Login to your account

        podman login <registry>

    For example

        podman login index.docker.io

4. Push the image

        podman push <username>/mattiarubinicom:<tagname>

## Using `docker`
If you prefer to use docker, simply substitute `podman` with `docker`.

# Moving to K8s

## Using `podman-compose`
[podman-compose](https://github.com/containers/podman-compose) is a great projects that turn your [docker-compose.yaml](https://docs.docker.com/compose/compose-file/) into a pod ready to be used on k8s.

    podman-compose -p pod-mattiarubinicom up -d --build 

Once you created a pod in podman, you can generate a kubernetes config file

    podman generate kube -s pod-mattiarubinicom > podman-k8s/kube-mattiarubinicom.yaml

Rembember to take down the podman-compose pod

    podman-compose -p pod-mattiarubinicom down


## Using `kompose`
[Kompose](https://github.com/kubernetes/kompose) is a great tool to turn your [docker-compose.yaml](https://docs.docker.com/compose/compose-file/) into a kubernetes configuration.

    kompose convert -o kompose/ 


# K8s testing
There are serveral way to test the kubernetes configurations

## K8s 
There are several project aimed to deploy lightweight k8s such as: [minikube](https://minikube.sigs.k8s.io/docs/), [minishif](https://www.okd.io/minishift/) or [k3s](https://k3s.io/).

To apply to k8s the configuration:

    kubectl apply -f <kubernetes config>

## `podman play`
Podman offers to run some of the kubernetes configurations via `podman play` tool:

    podman play kube <kubernetes config>
