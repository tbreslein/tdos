tommy_dir := if os() == "linux" { "/home/tommy" } else { "/Users/tommy" }
dots_dir := f'{{ tommy_dir }}/dots'

syke:
    if [ ! -d "{{ dots_dir }}" ]; then cd {{ tommy_dir }} && \
        git clone ssh://git@codeberg.org/tbreslein/dots.git; \
    else \
        cd {{ dots_dir }} && git pull \
    fi

    {{ dots_dir }}/scripts/syke

