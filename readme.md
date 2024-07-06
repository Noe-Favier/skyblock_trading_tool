# Skyblock Trading Tool (S2T)

This project aims to populate a database with auction items retrieved from an external API. The code periodically fetches auction data and inserts new items into a PostgreSQL database.

then, we'll create diagrams, and analysis tools based on this large amount of data.

## Setup and run project

in first place, fill the `.env` file with your own values.

> `cp .env.example .env`

then, you can use the docker-compose file to run the project.

```bash
docker-compose -p skyblock-tt up -d
```

Otherwise, you can run the project manually,
a manual tuto can be found in [./manual-project-setup.md](./manual-project-setup.md)
