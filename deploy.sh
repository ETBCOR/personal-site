docker build -t etbcor/portfolio-site:latest .
docker push etbcor/portfolio-site:latest
fly deploy
