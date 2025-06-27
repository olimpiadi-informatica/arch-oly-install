

    ./prepare.sh --with --some-options && \
    sudo rm -rf /tmp/archiso-workdir/ out/* && \
    sudo mkarchiso -v -w /tmp/archiso-workdir archiso-profile/ && \
    ./test_iso.sh
