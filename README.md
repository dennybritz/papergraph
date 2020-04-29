# papergraph

## **THIS IS WORK IN PROGRESS!** 

papergraph is a rust library and binary to build and manage a citation graph of [Semantic Scholar](https://www.semanticscholar.org/), focused on AI/ML papers (for now). Data is stored in a postgres database with a [Hasura](https://hasura.io/) GraphQL backend on top for easy graph queries. It comes with Jupyter notebooks that show you how to analyze and visualize the data.

In the future, papergraph may ship with a frontend that allows you to interactively explore the graph.

## Use Cases

### 1. Finding landmark papers

Notebook: TODO

Papers with a large citations count can be considered landmark papers. The techniques describes in such papers typically form the foundation of further incremental improvements. Given an arbitrary (non-landmark) paper, you may want to know which landmark papers you should study to gain the required background knowledge.

### 2. Reference Search

Notebook: TODO

When writing a paper, you don't want to miss an important reference. 

### 3. Graph Analysis

Notebook: TODO


## Downloading Data

The easiest way is to download an existing postgresql data dump (~2GB). To keep the size relatively small, this dataset only contains a subset of papers from Computer Science. Papers with no citations are excluded.

```bash
# TODO
```

If you want to build the database from scratch, you must download the full [S2 research corpus](http://s2-public-api-prod.us-west-2.elasticbeanstalk.com/corpus/download/). The total compressed size is currently around ~120GB.

```
aws s3 sync --no-sign-request s3://ai2-s2-research-public/open-corpus/2020-04-10/ data/s2-research-corpus
```

## Local Usage

Start a postgres and Hasura server in a docker container:

```bash
docker-compose up

# The Hasura console to run queries is now available at 
open http://localhost:8080/console
```

If you have an existig postgres data dump, load it into postgres:

```bash
# TODO 
pg_dump
```

If you want to build your own database from a raw data dump you have to load it manually:

```bash
# TODO - this will take a while
cargo run --release -- insert -d $DATA_PATH 
```


## Freshness

`papergraph` is updated when new [data snapshots](http://s2-public-api-prod.us-west-2.elasticbeanstalk.com/corpus/download/) become available. This typically happens once a month. This means it will not contain all the latest papers.

## Misc

Generating postgres database dumps

```bash
pg_dump -h localhost -p 15432 -F tar -U papergraph papergraph > pg_dump.tar
```

Build docker image

```bash
docker build -t papergraph/papergraph .
```