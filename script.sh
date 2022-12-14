#!/bin/zsh
export THREATS=1000 
export EVENTS=10000

fakebeat ~/projects/fakebeat/examples/threat_url.json -i logs-ti_test_url -c $THREATS ~/projects/fakebeat/examples/log_url.json -i filebeat-url -c $EVENTS