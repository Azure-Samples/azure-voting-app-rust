## Setup Environment
az deployment sub create --template-file ./deploy/main.bicep --location eastus
AcrName=(az deployment sub show --name main --query 'properties.outputs.acr_name.value' -o tsv)
AksName=(az deployment sub show --name main --query 'properties.outputs.aks_name.value' -o tsv)
ResourceGroup=(az deployment sub show --name main --query 'properties.outputs.resource_group_name.value' -o tsv)
az aks get-credentials --resource-group $ResourceGroup --name $AksName

## Build App Container
az acr build --registry $AcrName --image cnny2023/azure-voting-app-rust:{{.Run.ID}} .
BuildTag=(az acr repository show-tags \
                            --name $AcrName \
                            --repository cnny2023/azure-voting-app-rust \
                            --orderby time_desc \
                            --query '[0]' \
                            -o tsv)
