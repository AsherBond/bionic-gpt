# Running Locally

We have a very lightweight version of Bionic for running locally for for limited Proofs of concept. If you require features such as user management, document pipelines etc from the enterprise version then install the enterprise version instead.

## Prerequisites

The easiest way to get running with Bionic is with our `docker-compose.yml` file. You'll need [Docker](https://docs.docker.com/engine/install/) installed on your machine.

### OSX and Linux

```sh
curl -O https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/01c9e8365e48b72892763b37561e4eb5be4de1e4/infra-as-code/docker-compose.yml
```

### Windows

```sh
Invoke-WebRequest -Uri https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/01c9e8365e48b72892763b37561e4eb5be4de1e4/infra-as-code/docker-compose.yml -OutFile docker-compose.yml
```

### And run

```sh
docker-compose up
```

You can then access the front end from `http://localhost:3000`.

## Screenshot

![Alt text](/landing-page/bionic-console.png "Start Screen")