#!/bin/bash

if [ $# -ne 2 ]; then
    echo "Usage: $0 <fixed_url> <websites_file>"
    exit 1
fi

fixed_url=$1
websites_file=$2

if [ ! -f "$websites_file" ]; then
    echo "Error: Websites file '$websites_file' not found."
    exit 1
fi

make_request() {
    local url=$1
    local fixed_url=$2
    local request_url="http://$fixed_url/register/$url"
    echo "Registering: $request_url"
    curl -s "$request_url"
}

while read -r website; do
    make_request "$website" "$fixed_url" &
done < "$websites_file"

wait

echo "All requests completed."
