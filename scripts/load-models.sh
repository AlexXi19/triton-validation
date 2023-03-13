#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <pod_name>"
    exit 1
fi

script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
pod_name=$1

# Copy the folder and its contents to the specified pod
kubectl cp ${script_dir}/../models/triton/. ${pod_name}:/models/
