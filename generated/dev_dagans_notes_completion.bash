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
            opts="-h --git --rg --exec --dir --list --sort-by --help [NOTE]"
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
                    COMPREPLY=($(compgen -W "last-access alphabetical none" -- "${cur}"))
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
