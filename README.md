# Rust-Project

To create the docker image for yugabyte database, you can run those commands:
  - docker run -d --name yugabyte  -p7000:7000 -p9000:9000 -p5433:5433 -p9042:9042 yugabytedb/yugabyte:latest bin/yugabyted start --daemon=false
  - mkdir ~/yb_data
  - docker run -d --name yugabyte -p7000:7000 -p9000:9000 -p5433:5433 -p9042:9042 -v ~/yb_data:/home/yugabyte/yb_data yugabytedb/yugabyte:latest bin/yugabyted start --base_dir=/home/yugabyte/yb_data --daemon=false

Then check that it is created by this command 'docker ps'

You need also to create the Postgres db.

To init the database, you can use this command 'diesel setup' in the cmd in this path: yugabyte/migration

Then, use the APIs
