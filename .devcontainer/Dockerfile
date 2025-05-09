FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

# Install Node.js and npm for frontend development
USER root
RUN apt-get update && apt-get install -y curl && \
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs

# Switch back to vscode user
USER vscode

# Install Svelte language tools and other useful global npm packages if needed
RUN su vscode -c "npm install -g svelte-language-server typescript eslint prettier"

# Install Turso CLI (useful for database interaction, though auth is currently an issue)
USER root
RUN curl -sSfL https://get.tur.so/install.sh | bash
USER vscode
RUN echo 'export PATH="$HOME/.turso/bin:$PATH"' >> ~/.bashrc && \
    echo 'export PATH="$HOME/.turso/bin:$PATH"' >> ~/.zshrc

# Set the working directory in the container
WORKDIR /workspace

