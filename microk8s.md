# microk8s

## INSTALL

brew install ubuntu/microk8s/microk8s

microk8s install --cpu=2 --mem=8

cp ~/.kube/config ~/.kube/config.zf

KUBECONFIG=~/.kube/config.zf:/etc/rancher/k3s/k3s.yaml kubectl config view --merge --flatten > ~/.kube/config

kubectx default

kubectl get namespace

## UNINSTALL

minikube delete

brew uninstall minikube