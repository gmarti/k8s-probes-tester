# microk8s

## INSTALL

brew install ubuntu/microk8s/microk8s

microk8s install --cpu=1 --mem=4

cp ~/.kube/config ~/.kube/config.zf

microk8s config > ~/.kube/microk8s-config

KUBECONFIG=~/.kube/config.zf:~/.kube/microk8s-config kubectl config view --merge --flatten > ~/.kube/config

kubectx microk8s

kubectl get namespace

## UNINSTALL

microk8s reset # start fresh

microk8s stop

brew uninstall microk8s
