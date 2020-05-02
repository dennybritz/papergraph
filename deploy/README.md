## Useful commands

Reset database

```bash
# Need to temporarily shutdown hasura we can't have users accessing the db while resetting
kubectl scale --replicas=0 deployment/papergraph-hasura 
argo submit --watch deploy/k8s/workflows/run.yaml -p cmd="diesel database reset"
kubectl scale --replicas=1 deployment/papergraph-hasura 
```

Seed database

```bash
argo submit --watch deploy/k8s/workflows/seed.yaml 
```