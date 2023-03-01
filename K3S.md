# K3S

## INSTALL

curl -sfL https://get.k3s.io | sh - 

cp ~/.kube/config ~/.kube/config.zf

sudo chmod 644 /etc/rancher/k3s/k3s.yaml

KUBECONFIG=~/.kube/config.zf:/etc/rancher/k3s/k3s.yaml kubectl config view --merge --flatten > ~/.kube/config

kubectx default

kubectl get namespace

## UNINSTALL

/usr/local/bin/k3s-uninstall.sh

 cp ~/.kube/config.zf ~/.kube/config
