# papergraph

Papergraph is a rust library to build and analyze the citation graph of [Semantic Scholar](https://www.semanticscholar.org/), focused on AI/ML papers (for now). It comes with a frontend to explore the citation graph. The backend is a simple postgresql database.

## Usage

```bash
# Download database snapshot: TODO
# TODO
```

Start the UI

```bash
# TODO
```

## Use Cases

### 1. Finding landmark papers

Papers with a large citations count can be considered landmark papers. The techniques describes in such papers typically form the foundation of further incremental improvements. Given an arbitrary (non-landmark) paper, you may want to know which landmark papers you should study to gain the necessary background knowledge.

### 2. Reference Search

When writing a paper, you don't want to miss an important reference. 


## Freshness

`papergraph` is updated when new [data snapshots](http://s2-public-api-prod.us-west-2.elasticbeanstalk.com/corpus/download/) become available. This typically happens once a month. This means it will not contain all the latest papers.

## Usage from Python

`papergraph` comes with Python bindings.

