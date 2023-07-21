docker build -t portfolio-site .
docker image tag portfolio-site etbcor/portfolio-site:latest
docker push etbcor/portfolio-site:latest