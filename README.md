# rocket-mattiarubinicom
My personal website created in [rust](https://www.rust-lang.org/) with [rocket](https://rocket.rs/).

I had a lot of fun developing with container technology.

## How to build with podman
    podman build -f container/mattiarubinicom.Dockerfile --tag mattiarubinicom .

    podman run --publish 8080:80/tcp mattiarubinicom

## Using podman-compose
    podman-compose build

    podman-compose up

    podman-compose down

## **Podman notes**
To bind the containers on a ***port lower `1024`*** you need to run the previous command as root.

# Moving to K8s

## Using podman-compose
[podman-compose](https://github.com/containers/podman-compose) is a great projects that turn your [docker-compose.yaml](https://docs.docker.com/compose/compose-file/) into a pod ready to be used on k8s.

    podman-compose up -d --build

Once you created a pod in podman, you can generate a kubernetes config file

    podman generate kube -s rocket-mattiarubinicom > container/mattiarubinicom-kube-podman.yaml

Rembember to take down the podman-compose pod

    podman-compose down


## Kompose
[Kompose](https://github.com/kubernetes/kompose) is a great tool to turn your [docker-compose.yaml](https://docs.docker.com/compose/compose-file/) into a kubernetes configuration.

    kompose convert -o container/kompose/ 


# K8s testing
There are serveral way to test the kubernetes configurations

## K8s 
There are several project aimed to deploy lightweight k8s such as: [minikube](https://minikube.sigs.k8s.io/docs/), [minishif](https://www.okd.io/minishift/) or [k3s](https://k3s.io/).

To apply to k8s the configuration:

    kubectl apply -f <kubernetes config>

## podman play
Podman offers to run some of the kubernetes configurations via `podman play` tool:

    podman play kube <kubernetes config>