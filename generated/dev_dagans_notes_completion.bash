#!/usr/bin/env bash
_notes() {
    local i cur prev opts cmd
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="notes"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        notes)
            opts="-h -V --git --rg --exec --dir --list --sort-by --help --version [NOTE]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --git)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rg)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --exec)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --sort-by)
                    COMPREPLY=($(compgen -W "access-time alphabetical none" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

#!/usr/bin/env bash
_notes2() {
    _notes "$1" "$2" "$3"

    # clap-complete for some reason adds '[NOTE]' to the suggestions
    # So that needs to be removed
    local -a filtered_suggestions
    for suggestion in "${COMPREPLY[@]}"; do
        if [[ ! "$suggestion" =~ ^\[ ]]; then
            filtered_suggestions+=( "$suggestion" )
        fi
    done
    COMPREPLY=( "${filtered_suggestions[@]}" )

    # Skip adding notes to completions if preceded by certain arguments
    for longopt in '--sort-by' '--exec' '--git' '--rg'; do
        if [[ "$3" == "$longopt" ]]; then
            return 0
        fi
    done

    # Add notes to completions
    if [[ ! "$2" =~ ^- ]]; then
        for value in $(notes --list); do
            if [[ "$value" =~ ^$2 ]]; then
                COMPREPLY+=( "$value" )
            fi
        done
    fi
}

complete -F _notes2 -o nosort -o bashdefault -o default notes
