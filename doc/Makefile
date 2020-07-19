VERSION := $(shell git describe --tags)

all: pathfix.1.gz #pathfix.yml.5.gz

pathfix.1.gz: pathfix.1
	gzip -kf --best pathfix.1

pathfix.1: pathfix.1.md
	pandoc $< -s -t man -o $@

.PHONY: pathfix.1.md
pathfix.1.md: pathfix.1.template.md
	sed "s/\$${VERSION}/${VERSION}/g" pathfix.1.template.md > pathfix.1.md

pathfix.yml.5: pathfix.yml.5.md
	pandoc $< -s -t man -o $@

.PHONY: clean
clean:
	rm -rf pathfix.1 pathfix.1.md pathfix.yml.5 pathfix.yml.5.md
