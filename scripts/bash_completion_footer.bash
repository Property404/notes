_notes2() {
    _notes "$1" "$2" "$3"

    # Skip adding notes to completions if preceded by certain arguments
    for longopt in '--sort-by' '--exec' '--git' '--rg'; do
        if [[ "$3" == "$longopt" ]]; then
            return 0
        fi
    done

    # Add notes to completions
    if [[ ! "$2" =~ ^- ]]; then
        local -r values="$(notes --list)"
        for value in $(notes --list); do
            if [[ "$value" =~ ^$2 ]]; then
                COMPREPLY+=( "$value" )
            fi
        done
    fi
}

complete -F _notes2 -o nosort -o bashdefault -o default notes
