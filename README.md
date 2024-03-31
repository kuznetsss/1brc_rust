# brc

Simple solution for [1 billion rows challenge](https://github.com/gunnarmorling/1brc).

On my mac it takes 49 seconds.

## Getting data

File `measurements.txt` is required to be in the repo to measure performance.
It could be generated by steps from [1brc's instruction](https://github.com/gunnarmorling/1brc?tab=readme-ov-file#running-the-challenge).
I used docker, because I don't have java on my mac:
```bash
git clone https://github.com/gunnarmorling/1brc/
cd 1brc
docker run -it --mount source=$(pwd),target=/home eclipse-temurin:21 /bin/bash
# inside docker:
cd home
./mvnw clean verify
./create_measurements.sh 1000000000
exit
# on host
mv ./measurements.txt <1brc_rust path>

```

