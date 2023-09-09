_notes2() {
    _notes "$1" "$2" "$3"
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
