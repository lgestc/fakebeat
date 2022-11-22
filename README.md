# Fakebeat - friendly and flexible utility to produce random Elasticsearch documents

## About

I often find myself in the need of doing some quick testing on how well my queries perform.

This is similar to already existing `makelog`, but offers more flexibility.

## Usage

`fakelog` allows you to generate fake data using [Handlebars](https://handlebarsjs.com/guide/) templates with multiple helpers, allowing random data generation.

`fakelog -h` will render an overview of possible parameters, just as below:

```
Fake documents generator for Elasticsearch

Usage: fakebeat [OPTIONS] --index <INDEX> --count <COUNT> <TEMPLATE>...

Arguments:
  <TEMPLATE>...  Template file path

Options:
  -u, --username <USERNAME>  User name [default: elastic]
  -p, --password <PASSWORD>  [default: changeme]
      --url <URL>            [default: http://localhost:9200]
  -b, --batch <BATCH>        Batch size for inserts [default: 1000]
  -i, --index <INDEX>        Index to store documents in
  -c, --count <COUNT>        How many documents you want generated (per template)
  -a, --append               Append to the existing indices, instead of recreating them
  -h, --help                 Print help information
  -V, --version              Print version information

```

See `templates/` for reference on how the template might look like.

Usage example (assuming the default `url`, `password` and `username` options):

Single document template:
`fakebeat templates/event_file.json -i filebeat-file -c 10000`

Multiple templates:
`fakebeat templates/event_file.json -i filebeat-file -c 10000 templates/event_url.json -i filebeat-url -c 10000`

Append to indices instead of recreating:
`fakebeat -a templates/event_file.json -i filebeat-file -c 10000 templates/event_url.json -i filebeat-url -c 10000`

## Roadmap

[ ] better error handling
[ ] add tests

[ ] support cloud setups

[ ] configure github actions
[ ] provide mac os and linux releases
