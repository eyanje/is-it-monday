# Is it Monday?

[Is it Monday?](https://is-it-monday.eyanje.net) is an online survey that
collects responses on the day of the week.

For detailed documentation, refer to the [front end](frontend/README.md) and the
[back end](backend/README.md).

## Building

### Docker

To build Docker images, run
```bash
make
```

To build and push images to the Harbor repository, run
```bash
make push
```

## Deployment

Is it Monday? can be deployed to a Kubernetes cluster, after customization.

1. Update [frontend/js/src/api.ts](frontend/js/src/api.ts) to point to the API's
   endpoint.
2. Update [kubernetes/ingress.yaml](kubernetes/ingress.yaml) to match deployed
   hosts and paths.
3. Copy `.env.example` to `.env` and set appropriate values. Refer to
   [the backend](backend/README.md)
4. Apply resources to the server using Kustomize.
   ```bash
   kubectl apply -k .
   ```
5. Don't forget to restart the appropriate resources.
   ```bash
   kubectl rollout restart \
       statefulset/is-it-monday-backend \
       deployment/is-it-monday-frontend
   ```

