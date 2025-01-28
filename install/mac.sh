#!/bin/bash

latest_release_url=$(curl -s https://api.github.com/repos/FilipBoe/rust-cli-todo-app/releases/latest | grep '"browser_download_url"' | grep -Eo 'https://[^\"]+todo-app[^\"]+\.tar\.gz')

DESTINATION="${HOME}/.local/bin"

mkdir -p "${DESTINATION}/todo-app"

curl -L "${latest_release_url}" -o /tmp/todo-app.tar.gz
tar -xzf /tmp/todo-app.tar.gz -C "${DESTINATION}/todo-app"
echo "DATABASE_URL=sqlite://${DESTINATION}/todo-app/database.db" > "${DESTINATION}/todo-app/.env"

ln -sf "${DESTINATION}/todo-app/tap" "${DESTINATION}/tap"

if [[ ":$PATH:" != *":$DESTINATION:"* ]]; then
  echo "export PATH=\"\$PATH:${DESTINATION}\"" >> "${HOME}/.bashrc"
  echo "Installation successful! Restart your terminal or run 'source ~/.bashrc' to use the CLI."
else
  echo "Installation successful! Try 'tap'."
fi


