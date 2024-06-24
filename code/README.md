# The code
The code of this project is split into a fullstack leptos application and a generic mandelbrot renderer/explorer.

# How to run:
You need:
  - PostgreSQL database

Run the Docker image. Set the DATABASE_URL environment variable to point to your database. It should look like this:
`postgres://<USERNAME>:<PASSWORD>@<SERVER_IP_ADDR>/mandelguessr`

In case the server just crashes randomly consider trying this fix: https://github.com/diesel-rs/diesel/discussions/2947#discussioncomment-2025857