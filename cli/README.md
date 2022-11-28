# Fakebeat - friendly and flexible utility to produce random Elasticsearch documents

## About

Fakebeat allows you to generate fake data with ease using [Tera](https://tera.netlify.app) templates.

This is similar to already existing `elastic/makelog`, but offers far more flexibility.

1. Configure your mappings
2. Setup document value generation using our flexible templating engine
3. Generate your fixtures based on the template with the handy CLI utility.

## Usage

Define custom document templates (as text files), consisting of `index` configuration and `values` for each field, like this:

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

Note: you can copy the `index` section straight from Kibana, it accepts anything permitted with [create index api](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html)

Each of the _values_ can be constructed using random value _generators_. You can check the available generators using
`fakebeat -g`. Generated values can be combined and used in conditional statements as well - see the Tera manual for reference on what is possible with the templating.

Once your template is ready, save it in a file and run `filebeat you_file.json --index index-name --count 100` to
create 100 documents within your local ES instance. It is also possible to use different hosts or cloud deployments,
consult `fakebeat -h` for how to do that.

See the [examples](/examples/) for reference on how a template might look like.

Usage example (assuming the default `url`, `password` and `username` options):

Single document template:
`fakebeat examples/event_file.json -i filebeat-file -c 10000`

Multiple examples:
`fakebeat examples/event_file.json -i filebeat-file -c 10000 examples/threat_url.json -i filebeat-url -c 10000`

Append to indices instead of recreating:
`fakebeat -a examples/event_file.json -i filebeat-file -c 10000 examples/threat_url.json -i filebeat-url -c 10000`
