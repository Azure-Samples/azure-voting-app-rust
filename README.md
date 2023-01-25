# Project Name

This is a re-implementation of the [Azure Voting App Redis] project in Rust with Postgres.  There are a few features added to allow the demonstration of some of the basic capabilities of Kubernetes for application hosting.

## Getting Started

### Prerequisites

- Azure Subscription
- Azure CLI

### Quickstart

```bash
## Clone Repository
git clone https://github.com/Azure-Samples/azure-voting-app-rust
cd azure-voting-app-rust
az login

## Setup Environment
az deployment sub create --template-file ./deploy/main.bicep --location eastus
AcrName=$(az deployment sub show --name main --query 'properties.outputs.acr_name.value' -o tsv)
AksName=$(az deployment sub show --name main --query 'properties.outputs.aks_name.value' -o tsv)
ResourceGroup=$(az deployment sub show --name main --query 'properties.outputs.resource_group_name.value' -o tsv)
az aks get-credentials --resource-group $ResourceGroup --name $AksName

## Build App Container
az acr build --registry $AcrName --image cnny2023/azure-voting-app-rust:{{.Run.ID}} .
BuildTag=$(az acr repository show-tags \
                            --name $AcrName \
                            --repository cnny2023/azure-voting-app-rust \
                            --orderby time_desc \
                            --query '[0]' \
                            -o tsv)

## Pods
kubectl run azure-voting-db \
    --image "postgres:15.0-alpine" \
    --env "POSTGRES_PASSWORD=mypassword" \
    --output yaml \
    --dry-run=client > manifests/pod-db.yaml
kubectl apply -f ./manifests/pod-db.yaml

DB_IP=$(kubectl get pod azure-voting-db -o jsonpath='{.status.podIP}')

kubectl run azure-voting-app \
    --image "$AcrName.azurecr.io/cnny2023/azure-voting-app-rust:$BuildTag" \
    --env "DATABASE_URL=postgres://postgres:mypassword@$DB_IP" \
    --output yaml \
    --dry-run=client > manifests/pod-app.yaml
kubectl apply -f ./manifests/pod-app.yaml

## Access the app
kubectl port-forward pod/azure-voting-app 8080:8080
```

## Resources

TODO: Cloud Native New Year link
