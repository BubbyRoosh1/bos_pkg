cat <<EOF
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
EOF > /usr/local/bin/rustup-install
chmod a+x /usr/local/bin/rustup-install
