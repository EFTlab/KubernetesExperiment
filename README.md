minikube dashboard

cd ~/git/kubernetes1/rust-k8s-s1/build/
docker-compose run --rm compile
cd ~/git/kubernetes1/rust-k8s-s1
docker build -t dawesc1981/rust-k8s-s1:0.3.0 . && docker push dawesc1981/rust-k8s-s1:0.3.0
cd ~/git/kubernetes1/rust-k8s-s1
kubectl apply -f bb.yaml

cd ~/git/kubernetes1/rust-k8s-s2/build/
docker-compose run --rm compile
cd ~/git/kubernetes1/rust-k8s-s2
docker build -t dawesc1981/rust-k8s-s2:0.3.0 . && docker push dawesc1981/rust-k8s-s2:0.3.0
cd ~/git/kubernetes1/rust-k8s-s2
kubectl apply -f bb.yaml

minikube service rust-k8s-entrypoint-web-s1  --url & minikube service rust-k8s-entrypoint-web-s2  --url &
%1
%2
# KubernetesExperiment
