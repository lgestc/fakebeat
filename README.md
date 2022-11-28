# Fakebeat - friendly and flexible utility to produce random Elasticsearch documents

## About

This is similar to already existing `makelog`, but offers more flexibility.

1. Configure your indices
2. Setup document value generation using our flexible templating engine
3. Generate your fixtures

## Usage

Define custom document templates, consisting of `index` configuration and `values` for each field, like this:

```
{
  "values": {
    "@timestamp": "{{date()}}",
    "threat": {
      "indicator": {
        "type": "file",
        "first_seen": "{{date(sub_rnd_days=30)}}",
        "file": {
          "hash": {
            "md5": "{{hash()}}"
          }
        },
        "marking": {
          "tlp": "RED"
        }
      },
      "feed": {
        "name": "fakebeat_{{random_value(options='file|host')}}"
      }
    },
    "event": {
      "type": "indicator",
      "category": "threat",
      "dataset": "ti_*",
      "kind": "enrichment"
    }
  },
  "index": {
    "mappings": {
      "properties": {
        "@timestamp": { "type": "date" },

        "threat": {
          "properties": {
            "indicator": {
              "properties": {
                "type": { "type": "keyword" },
                "first_seen": { "type": "date" },
                "file": {
                  "properties": {
                    "hash": {
                      "properties": {
                        "md5": {
                          "type": "keyword"
                        }
                      }
                    }
                  }
                },
                "marking": {
                  "properties": {
                    "tlp": { "type": "keyword" }
                  }
                }
              }
            },
            "feed": {
              "properties": {
                "name": {
                  "type": "keyword"
                }
              }
            }
          }
        },

        "event": {
          "properties": {
            "type": { "type": "keyword" },
            "category": { "type": "keyword" },
            "dataset": { "type": "keyword" },
            "kind": { "type": "keyword" }
          }
        }
      }
    }
  }
}

```

`fakelog` allows you to generate fake data using [Tera](https://tera.netlify.app) templates.

Each of the _values_ can be constructed using random value _generators_. You can check the available generators using
`fakebeat -g`. Generated values can be combined and used in conditional statements as well - see the Tera manual for reference on what is possible with the templating.

Once your template is ready, save it in a file and run `filebeat you_file.json --index indexName --count 100` to
create 100 documents within your local ES instance. It is also possible to use different hosts or cloud deployments,
consult `fakelog -h` for how to do that.

```
Fake documents generator for Elasticsearch

Usage: fakebeat [OPTIONS] [TEMPLATE]...

Arguments:
  [TEMPLATE]...  Template file path

Options:
  -u, --username <USERNAME>  User name [default: elastic]
  -p, --password <PASSWORD>  Password [default: changeme]
      --url <URL>            Elasticsearch host [default: http://localhost:9200]
      --cloud <CLOUD>        Elastic cloud id. If specified, overrides the url setting
  -b, --batch <BATCH>        Batch size for inserts [default: 1000]
  -i, --index <INDEX>        Index to store documents in (per template)
  -c, --count <COUNT>        How many documents you want generated (per template)
  -a, --append               Append to the existing indices, instead of recreating them
  -g, --generators           Print available generators
  -h, --help                 Print help information
  -V, --version              Print version information

```

See the [examples](/examples/) for reference on how a template might look like.

Usage example (assuming the default `url`, `password` and `username` options):

Single document template:
`fakebeat examples/event_file.json -i filebeat-file -c 10000`

Multiple examples:
`fakebeat examples/event_file.json -i filebeat-file -c 10000 examples/threat_url.json -i filebeat-url -c 10000`

Append to indices instead of recreating:
`fakebeat -a examples/event_file.json -i filebeat-file -c 10000 examples/threat_url.json -i filebeat-url -c 10000`
