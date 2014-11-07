#!/bin/bash

# Call 'emacsclient' with the arguments.  Hide all output from emacsclient.
# 
# 2014-01-27 / 2014-01-27
# Bence Kalmar

emacsclient -a '' -c "${@}" &>/dev/null &
