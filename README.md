# RESTful server to wrap NEO4j http Edpoint
Listens on port 9090 for RESTful requests
 - family Returns all - slow
 - search/{name} perfect match only name search

Both return the Neo4j JSON - simply proxying the service wrapping response in JSON.


## Deps
Uses a verison of rsuted_cyher in my github, forked the repo. and modified to use the graph response olnly of Neo4j v4


## Builds

#### Github Workflows/Actions
see `.github` folder

# Local
for now building with a symlink to local dir
REST  Proxy  for NEO4J
