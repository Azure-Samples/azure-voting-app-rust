# Project Name

This is a re-implementation of the [Azure Voting App Redis] project in Rust with Postgres.  There are a few features added to allow the demonstration of some of the basic capabilities of Kubernetes for application hosting.

## Getting Started

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/)
- [KIND](https://kind.sigs.k8s.io/docs/user/quick-start/)
- [kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl/)

> [!NOTE]
> If you are compiling the Rust code locally on your machine, you will need to install the `libpq-dev` package.  This package is required for the `diesel` crate to compile. See the following command to install the package on Ubuntu.

```bash
sudo apt-get install libpq-dev
```

### Quickstart

```bash
## Clone Repository
git clone https://github.com/Azure-Samples/azure-voting-app-rust
cd azure-voting-app-rust

## Create a KIND cluster
kind create cluster

## Install the Azure Voting App
kubectl apply -f ./manifests

## Access the app
kubectl port-forward svc/azure-voting-app 8080:80

## Browse to the app
echo "http://localhost:8080"
```