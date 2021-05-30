# Overview
The motivation for this project is simplification of my daily operations with kafka.
Often I: 
- check topics lag for partitions;
- offset on topics for all partitions;
- look for a specific record using key or some regex.

To perform these activities I use: [birdayz/kaf](https://github.com/birdayz/kaf) which is awesome and 
if only searching capabilities would be better then I wouldn't have motivation to
write my own tool.

# Acknowledges
## HowTos
- [project's structure](https://bencane.com/2020/12/29/how-to-structure-a-golang-cli-project/)
- [version go application](https://www.forkingbytes.com/blog/dynamic-versioning-your-go-application/)
## Libraries
- [kafka-go](https://github.com/segmentio/kafka-go) - used to communication with kafka
- [clean env](https://github.com/ilyakaznacheev/cleanenv) - configuration management
- [urfave/cli](https://github.com/urfave/cli) - ease parsing & handling command line arguments

