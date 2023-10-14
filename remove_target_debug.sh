#!/bin/bash

# Remove target and debug directories
remove_target_debug() {
    for dir in "$1"/*; do
        if [[ -d "$dir" ]]; then
            if [[ "$(basename "$dir")" == "target" || "$(basename "$dir")" == "debug" ]]; then
                echo "Removing directory: $dir"
                rm -r "$dir"
            else
                remove_target_debug "$dir"
            fi
        fi
    done
}
