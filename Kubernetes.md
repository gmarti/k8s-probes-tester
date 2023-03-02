# Kubernetes


## Requirements
K3S or minikube
kubectx
kubens
kubectl
fzf
watch


## Context
```
cat ~/.kube/config
kubectl
kubectl config --help
kubectl config view
kubectx
```

## Cluster
```
kubectl get nodes
kubectl get nodes -o wide
kubectl get nodes -L node-lifecycle -L kubernetes.io/arch
kubectl top nodes
kubectl describe node 
kubectl describe node | grep Event
 
```

## Namespace [namespace.yml]
```
kubens
kubectl get namespace
kubectl get pods --all-namespaces

cd deploy/kubernetes

kubectl apply -f namespace.yml

kubens > training

kubectl delete -f namespace.yml / kubectl delete namespace training

kubectl apply -f namespace.yml
```

## Pod [pod.yml]
```
kubectl apply -f pod.yml

kubectl get pods
kubectl get pods -o wide
kubectl get pods -o yaml
watch kubectl get pods

kubectl describe pod 

kubectl describe node

kubectl logs -f k8s-probes-tester

kubectl port-forward k8s-probes-tester 8080

curl 'http://localhost:8080/health/alive'
```
> change image to v1.0.3
```
kubectl apply -f pod.yml

kubectl get pod
kubectl describe pod

kubectl delete -f pod.yml | kubectl delete pod k8s-probes-tester
```
***How to deploy multiple replicas of a pod ?***
***How to roll the updates ?***

## Labels

kubectl get all -l app=booking-service

kubectl api-resources --verbs=list --namespaced -o name \           
  xargs -n 1 kubectl get --show-kind --ignore-not-found -l app=booking-service


## Deployment [deploy.yml]
```
kubectl apply -f deploy.yml

kubectl get deploy
kubectl get deploy -o wide
kubectl describe deploy

watch kubectl get pods

kubectl get replicaset -o wide
kubectl describe replicaset

watch kubectl get replicaset
```
> change image to v1.0.3
```
kubectl apply -f deploy.yml
```
> change replicas to 10
```
kubectl apply -f deploy.yml

kubectl scale deploy k8s-probes-tester --replicas=20
```
> change image to v1.0.2
```
kubectl apply -f deploy.yml
```
> change image to v1.0.3 and maxUnavailable to 2
```
kubectl apply -f deploy.yml

kubectl scale deploy k8s-probes-tester --replicas=20

kubectl rollout restart deployment k8s-probes-tester

kubectl port-forward k8s-probes-tester-123455667678-12345 8080
```
***How to load balance traffic to my pods ?***

## Service  [service.yml]
```
kubectl apply -f service.yml

kubectl get service -o wide
kubectl describe service
```
> change type to NodePort and uncomment nodePort
```
kubectl apply -f service.yml

kubectl get service -o wide
kubectl describe service

kubectl get node -o wide

curl http://10.125.0.27:30000/health/alive
curl http://localhost:30000/health/ready
```
>Service does not provide load balancing
>External load balancer or Ingress provide the load balancing features

```
kubectl port-forward svc/k8s-probes-tester 8081
```

>will select the first endpoint available
```
curl -X POST http://localhost:8081/health/ready
curl -X POST http://localhost:8081/health/ready
```
***How a pod can tell the service it's not ready ?***

## Probes  [probes.yml]
```
kubectl apply -f probes.yml

kubectl describe pod
kubectl describe service
```
### Liveness
```
curl http://localhost:30000/health/alive

watch kubectl get pods

curl -X POST http://localhost:30000/health/alive
```
>If liveness probe fail => Kill the container
```
kubectl describe pod

# 5x 
curl -X POST http://localhost:30000/health/alive 
```
>CrashLoopBackoff will wait a bit before retrying your pod

### Readyness
```
watch kubectl get pods

curl http://localhost:30000/health/ready
```
> change image to v1.0.3
```
kubectl apply -f probles.yml

kubectl rollout restart deployment k8s-probes-tester

curl -X POST http://localhost:30000/health/ready
```
>If readyness probe fail => Stop sending traffic
```
curl http://localhost:30000/health/ready

kubectl describe service
kubectl get deploy
kubectl get replicaset

kubectl scale deploy k8s-probes-tester --replicas=2

kubectl describe service

curl http://localhost:30000/health/ready

curl -X POST http://localhost:30000/health/ready

curl http://localhost:30000/health/ready

kubectl port-forward k8s-probes-tester-56599657cc-zpmjf 8080

curl -X POST 'http://localhost:8080/health/ready'

kubectl delete pod k8s-probes-tester-56599657cc-mgppk
```
> change replicas to 10 and maxUnavailable to 2
```
kubectl rollout restart deployment k8s-probes-tester
```
>always 8 pods available and 4 restarting

## PodDisruptionBudget [pdb.yml]
```
kubectl scale deploy k8s-probes-tester --replicas=2

kubectl-evict-pod

kubectl apply -f pdb.yml

kubectl get pdb
kubectl describe pdb
kubectl get pdb -o yaml

kubectl-evict-pod

kubectl scale deploy k8s-probes-tester --replicas=3

kubectl get pdb

kubectl-evict-pod

curl -X POST http://localhost:30000/health/ready

kubectl get pdb

kubectl-evict-pod

kubectl delete pod 
```

## Ingress [ingress.yml]


## ConfigMap and Secret
## [configmap.yml]
## [secret.yml]
## [pod_configmap_secret.yml]
kubectl apply -f configmap.yml

kubectl get configmap 

kubectl describe configmap

kubectl apply -f secret.yml

kubectl get secret

kubectl describe secret

kubectl get secret k8s-probes-tester -o jsonpath='{.data.password}' | base64 --decode -

echo -n 'Security!' | base64

kubectl create configmap k8s-probes-tester-logs --from-file=logback.xml

kubectl get configmap k8s-probes-tester-logs

kubectl describe configmap k8s-probes-tester-logs
