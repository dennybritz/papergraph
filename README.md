# papergraph

papergraph is a rust library and binary to build and manage a citation graph of [Semantic Scholar](https://www.semanticscholar.org/), focused on AI/ML papers (for now). Data is stored in a postgres database with a [Hasura](https://hasura.io/) GraphQL backend on top for easy graph queries. It comes with Jupyter notebooks that show you how to analyze and visualize the data.

In the future, papergraph may ship with a frontend that allows you to interactively explore the graph.

## Use Cases

- **Finding landmark papers**  - Papers with a large citations may be considered landmark papers. The techniques describes in such papers often form the foundation of further incremental improvements. Given some arbitrary paper you're interested in, you may want to know which landmark papers you should study for the required background knowledge.
- **Reference research** - When writing a paper, you don't want to miss prior work. Looking through the citation graph for a related paper can help you find potentially interesting papers to read and cite.
- **Graph Analysis** - Run sophisticated graph algorithms on the dataset to gain insights


## Notebooks

The folllowing notebooks work out of the box using a publicly available API endpoint for the data. You can run them locally, or in the cloud via Google Colab. **Please read the caveats about the public endpoint below!**

- [Simple Graph Analysis](notebooks/simple_analysis.ipynb) | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/dennybritz/papergraph/blob/master/notebooks/simple_analysis.ipynb)
  - Example to query the citation graph for a specific paper and analyze it with pandas
- [Advanced Graph Analysis with networkx]()
  - TODO


## IMPORTANT! Using the public endpoint

The database is publicly available at `http://papergraph.dennybritz.com/v1/graphql`, so **please be gentle with your queries!** This is running on a small postgres server that I'm paying for, so please don't overload it with automated scripts. Be nice :) As long as you're running queries by hand through notebooks everything should be fine.

If you want to do lots of queries you are better of cloning this repo and building the database yourself. Instructions for this are below.

## Building the database from a postgresql snapshot


<!-- The easiest way is to download an existing postgresql data dump (~2GB). To keep the size relatively small, this dataset only contains a subset of papers from Computer Science. Papers with no citations are excluded. -->

TODO

## Building the database from scratch

Requirements:

- Docker

If you want to build the database from scratch, you must download the full [S2 research corpus](http://s2-public-api-prod.us-west-2.elasticbeanstalk.com/corpus/download/). The total compressed size is currently around ~120GB.

```bash
aws s3 sync --no-sign-request s3://ai2-s2-research-public/open-corpus/2020-04-10/ data/s2-research-corpus
```

Start up an empty postgres database server and create the schema

```bash
export DATABASE_URL=postgres://papergraph:papergraph@postgres:5432/papergraph
export RUST_LOG=info

# Run the postgres docker container
docker-compose up postgres

# Setup the datase and run migrations
docker run --rm --network papergraph_default \
  -e DATABASE_URL \
  dennybritz/papergraph \
  diesel database setup
```

Now that we have a postgres server with the right database schema running, we need to insert the data:

```bash
# Assuming you downloaded the data into /data 
# as shown in the AWS command above
DATA_PATH=data/s2-research-corpus/s2-corpus-017.gz

# Repeat this for all files you want to insert
# This will take a while
docker run --rm -it --network papergraph_default \
  -e DATABASE_URL -e RUST_LOG \
  -v `pwd`/${DATA_PATH}:/data/${DATA_PATH} \
  dennybritz/papergraph \
  papergraph insert -d /data/${DATA_PATH}
```

Now that have seeded the database, we can also start the Hasura server. Stop the postgres docker process with `ctrl+c` and run

```bash
docker-compose up
```

You should now be able to access the API on `http://localhost:8080`.


## Freshness

`papergraph` is updated when new [data snapshots](http://s2-public-api-prod.us-west-2.elasticbeanstalk.com/corpus/download/) become available. This typically happens once a month. This means it will not contain all the latest papers.

## Misc

Generating postgres database dumps

```bash
pg_dump -h localhost -p 15432 -F tar -U papergraph papergraph > pg_dump.tar
```

Build docker image

```bash
docker build -t dennybritz/papergraph .
```