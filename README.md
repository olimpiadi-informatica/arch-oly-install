

    ./prepare.sh --with --some-options && \
    sudo rm -rf /var/tmp/archiso-workdir/ out/* && \
    sudo mkarchiso -v -w /var/tmp/archiso-workdir archiso-profile/ && \
    ./test_iso.sh
